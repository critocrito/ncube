use argon2::{self, Config};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use rand::Rng;
use sha2::{Digest, Sha256, Sha512};

use crate::errors::CryptoError;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 30;

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub fn mkpass() -> String {
    let mut rng = rand::thread_rng();

    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<String>()
}

pub(crate) fn jwt_sign(key: &str, email: &str, workspace: &str) -> Result<String, CryptoError> {
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

    claims.sign_with_key(&key).map_err(|_| CryptoError)
}

pub(crate) fn jwt_verify(key: &str, token: &str) -> Result<RegisteredClaims, CryptoError> {
    // FIXME: Don't unwrap error
    let key: Hmac<Sha512> = Hmac::new_varkey(key.as_bytes()).map_err(|_| CryptoError)?;
    let claims: RegisteredClaims =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_| CryptoError)?;
    let now = Utc::now();
    let expiration = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(claims.expiration.ok_or_else(|| CryptoError)? as i64, 0),
        Utc,
    );
    let not_before = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(claims.not_before.ok_or_else(|| CryptoError)? as i64, 0),
        Utc,
    );

    if not_before <= now && expiration >= now {
        Ok(claims)
    } else {
        Err(CryptoError)
    }
}

pub fn gen_secret_key() -> String {
    let key = mkpass();
    let mut hasher = Sha256::new();
    hasher.input(&key);
    let hash = hasher.result();
    let hex_digest = format!("{:x}", hash);
    hex_digest
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    #[test]
    fn hashing_passwords() {
        let mut rng = rand::thread_rng();

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        let hashed = hash(password.as_bytes());

        assert_eq!(verify(&hashed, password.as_bytes()), true);
    }

    #[test]
    fn jwt_tokens() {
        let key = "some secret key";
        let token = jwt_sign(&key, "me@example.org", "syrian-archive").unwrap();
        let claims = jwt_verify(&key, &token).unwrap();

        assert_eq!(claims.subject.unwrap(), "me@example.org");
        assert_eq!(claims.audience.unwrap(), "syrian-archive");
        assert!(claims.expiration.is_some());
        assert!(claims.not_before.is_some());
    }
}
