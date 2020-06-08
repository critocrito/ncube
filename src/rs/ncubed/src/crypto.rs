use aes::Aes256;
use argon2::{self, Config};
use base64::{decode, encode};
use block_modes::block_padding::Iso7816;
use block_modes::BlockMode;
use block_modes::Cbc;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use rand::Rng;
use secstr::SecVec;
use sha2::{Digest, Sha256, Sha512};

use crate::errors::HostError;

type Aes256Cbc = Cbc<Aes256, Iso7816>;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 30;

pub fn hash<R: Rng>(mut rng: R, password: &[u8]) -> String {
    let salt = rng.gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub fn mkpass<R: Rng>(mut rng: R) -> String {
    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<String>()
}

pub(crate) fn jwt_sign(key: &str, email: &str, workspace: &str) -> Result<String, HostError> {
    let key: Hmac<Sha512> =
        Hmac::new_varkey(key.as_bytes()).expect("HMAC can take key of any size");
    let now = Utc::now();
    let expire = now + Duration::hours(1);

    let claims = RegisteredClaims {
        subject: Some(email.into()),
        audience: Some(workspace.into()),
        expiration: Some(expire.timestamp() as u64),
        not_before: Some(now.timestamp() as u64),
        ..Default::default()
    };

    claims.sign_with_key(&key).map_err(|_| HostError::AuthError)
}

pub(crate) fn jwt_verify(key: &str, token: &str) -> Result<RegisteredClaims, HostError> {
    let key: Hmac<Sha512> = Hmac::new_varkey(key.as_bytes()).map_err(|_| HostError::AuthError)?;
    let claims: RegisteredClaims =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_| HostError::AuthError)?;
    let now = Utc::now();
    let expiration = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(
            claims.expiration.ok_or_else(|| HostError::AuthError)? as i64,
            0,
        ),
        Utc,
    );
    let not_before = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(
            claims.not_before.ok_or_else(|| HostError::AuthError)? as i64,
            0,
        ),
        Utc,
    );

    if not_before <= now && expiration >= now {
        Ok(claims)
    } else {
        Err(HostError::AuthError)
    }
}

// Fixme: Supply Rng
pub fn gen_secret_key(seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(&seed);
    let hash = hasher.result();
    let hex_digest = format!("{:x}", hash);
    hex_digest
}

pub fn gen_nonce<R: Rng>(mut rng: R) -> [u8; 16] {
    let mut arr = [0u8; 16];
    rng.fill(&mut arr);
    arr
}

pub fn gen_symmetric_key<R: Rng>(mut rng: R) -> SecVec<u8> {
    let mut arr = [0u8; 32];
    rng.fill(&mut arr);
    SecVec::new(arr.to_vec())
}

pub fn aes_encrypt<R: Rng>(rng: R, key: &SecVec<u8>, plaintext: &[u8]) -> String {
    let iv = gen_nonce(rng);
    let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);
    let encoded_ciphertext = encode(ciphertext);
    let encoded_iv = encode(iv);
    format!("aes256cbc${}${}", encoded_iv, encoded_ciphertext)
}

pub fn aes_decrypt(key: SecVec<u8>, ciphertext: &str) -> Result<Vec<u8>, HostError> {
    let parts: Vec<&str> = ciphertext.split('$').take(3).collect();
    if parts.len() != 3 {
        return Err(HostError::AuthError);
    }
    let (encoded_iv, encoded_ciphertext) = (parts[1], parts[2]);
    let iv = decode(encoded_iv).map_err(|_| HostError::AuthError)?;
    let ciphertext = decode(encoded_ciphertext).map_err(|_| HostError::AuthError)?;
    let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
    let decrypted_ciphertext = cipher
        .decrypt_vec(&ciphertext)
        .map_err(|_| HostError::AuthError)?;
    Ok(decrypted_ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;
    use std::iter::repeat;

    const PASSWORD: &str = "abcd";
    const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8$t5fJGEKEOeaHTS8JrGC9GViRlfI1PeGscNHFYN4NCFE";
    const SHA256SUM: &str = "37b9403cf88cc2639d0a118d757a43a0ff6d4871823707ab6a8bb56bc68e8e79";

    #[test]
    fn aes_encrypt_decrypt() {
        let key = gen_symmetric_key(StepRng::new(0, 1));
        let plaintext: Vec<u8> = b"This is a secret I want to keep".to_vec();
        let ciphertext = aes_encrypt(StepRng::new(0, 1), &key, &plaintext);
        let decrypted_text = aes_decrypt(key, &ciphertext).unwrap();
        assert_eq!(plaintext, decrypted_text);
    }

    #[test]
    fn generate_random_passwords() {
        let rng = StepRng::new(0, 1);
        let expected = repeat("A".to_string())
            .take(30)
            .collect::<Vec<String>>()
            .join("");
        let result = mkpass(rng);
        assert_eq!(result, expected);
    }

    #[test]
    fn hashing_passwords() {
        let rng = StepRng::new(0, 1);
        let result = hash(rng, PASSWORD.as_bytes());
        assert_eq!(result, PASSWORD_HASH);
    }

    #[test]
    fn verifying_passwords() {
        let result = verify(&PASSWORD_HASH, PASSWORD.as_bytes());
        assert!(result)
    }

    #[test]
    fn signing_and_verifying_jwt_tokens() {
        let key = "some secret key";
        let token = jwt_sign(&key, "me@example.org", "syrian-archive").unwrap();
        let claims = jwt_verify(&key, &token).unwrap();

        assert_eq!(claims.subject.unwrap(), "me@example.org");
        assert_eq!(claims.audience.unwrap(), "syrian-archive");
        assert!(claims.expiration.is_some());
        assert!(claims.not_before.is_some());
    }

    #[test]
    fn generate_secret_keys() {
        let rng = StepRng::new(0, 1);
        let seed = mkpass(rng);
        let result = gen_secret_key(&seed);
        assert_eq!(result, SHA256SUM);
    }
}
