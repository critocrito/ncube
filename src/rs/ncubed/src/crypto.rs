use argon2::{self, Config};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use rand::Rng;
use sha2::{Digest, Sha256, Sha512};

use crate::errors::HostError;

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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;
    use std::iter::repeat;

    const PASSWORD: &str = "abcd";
    const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8$t5fJGEKEOeaHTS8JrGC9GViRlfI1PeGscNHFYN4NCFE";
    const SHA256SUM: &str = "37b9403cf88cc2639d0a118d757a43a0ff6d4871823707ab6a8bb56bc68e8e79";

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
