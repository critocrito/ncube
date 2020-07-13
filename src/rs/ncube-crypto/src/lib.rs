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
use std::fmt::{Display, Formatter};
use thiserror::Error;

type Aes256Cbc = Cbc<Aes256, Iso7816>;

#[derive(Error, Debug)]
pub struct AuthError;

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError")
    }
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

pub fn sha256(key: SecVec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(&key.unsecure());
    let hash = hasher.result();
    format!("{:x}", hash)
}

pub fn gen_secret_key<R: Rng>(rng: R) -> String {
    let key = gen_symmetric_key(rng);
    sha256(key)
}

pub fn hash<R: Rng>(mut rng: R, password: &[u8]) -> String {
    let salt = rng.gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub fn jwt_sign(key: &str, email: &str, workspace: &str) -> Result<String, AuthError> {
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

    claims.sign_with_key(&key).map_err(|_| AuthError)
}

pub fn jwt_verify(key: &str, token: &str) -> Result<RegisteredClaims, AuthError> {
    let key: Hmac<Sha512> = Hmac::new_varkey(key.as_bytes()).map_err(|_| AuthError)?;
    let claims: RegisteredClaims =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_| AuthError)?;
    let now = Utc::now();
    let expiration = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(claims.expiration.ok_or_else(|| AuthError)? as i64, 0),
        Utc,
    );
    let not_before = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(claims.not_before.ok_or_else(|| AuthError)? as i64, 0),
        Utc,
    );

    if not_before <= now && now <= expiration {
        Ok(claims)
    } else {
        Err(AuthError)
    }
}

pub fn aes_encrypt<R: Rng>(rng: R, key: &SecVec<u8>, plaintext: &[u8]) -> String {
    let iv = gen_nonce(rng);
    let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);
    let encoded_ciphertext = encode(ciphertext);
    let encoded_iv = encode(iv);
    format!("aes256cbc${}${}", encoded_iv, encoded_ciphertext)
}

pub fn aes_decrypt(key: SecVec<u8>, ciphertext: &str) -> Result<Vec<u8>, AuthError> {
    let parts: Vec<&str> = ciphertext.split('$').take(3).collect();
    if parts.len() != 3 {
        return Err(AuthError);
    }
    let (encoded_iv, encoded_ciphertext) = (parts[1], parts[2]);
    let iv = decode(encoded_iv).map_err(|_| AuthError)?;
    let ciphertext = decode(encoded_ciphertext).map_err(|_| AuthError)?;
    let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).map_err(|_| AuthError)?;
    Ok(decrypted_ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    const PASSWORD: &str = "abcd";
    const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8$t5fJGEKEOeaHTS8JrGC9GViRlfI1PeGscNHFYN4NCFE";
    const SHA256SUM: &str = "a1e03200f1f82ad2c1cec8795c271aaecf98f5aa2d151d2229ec5fa0c177cf77";

    #[test]
    fn aes_encrypt_decrypt() {
        let key = gen_symmetric_key(StepRng::new(0, 1));
        let plaintext: Vec<u8> = b"This is a secret I want to keep".to_vec();
        let ciphertext = aes_encrypt(StepRng::new(0, 1), &key, &plaintext);
        let decrypted_text = aes_decrypt(key, &ciphertext).unwrap();
        assert_eq!(plaintext, decrypted_text);
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
    fn generate_secret_key() {
        let rng = StepRng::new(0, 1);
        let secret = gen_secret_key(rng);
        assert_eq!(secret, SHA256SUM);
    }
}
