use argon2::{self, Config};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{error::Error, RegisteredClaims, SignWithKey, VerifyWithKey};
use rand::Rng;
use sha2::Sha512;
use std::collections::BTreeMap;

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

pub(crate) fn jwt_sign(key: &str, email: &str, workspace: &str) -> Result<String, Error> {
    let key: Hmac<Sha512> = Hmac::new_varkey(key.as_bytes()).unwrap();
    let now = Utc::now();
    let expire = now + Duration::hours(1);

    let claims = RegisteredClaims {
        subject: Some(email.into()),
        audience: Some(workspace.into()),
        expiration: Some(expire.timestamp() as u64),
        not_before: Some(now.timestamp() as u64),
        ..Default::default()
    };

    claims.sign_with_key(&key)
}

pub(crate) fn jwt_verify(key: &str, token: &str) -> Result<RegisteredClaims, Error> {
    let key: Hmac<Sha512> = Hmac::new_varkey(key.as_bytes()).unwrap();
    let claims: RegisteredClaims = VerifyWithKey::verify_with_key(token, &key).unwrap();
    Ok(claims)
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
