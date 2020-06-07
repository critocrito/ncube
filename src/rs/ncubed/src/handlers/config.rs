use ncube_data::NcubeConfig;
use rand::{self, rngs::StdRng, SeedableRng};
use std::time::SystemTime;

use crate::actors::{
    host::{InsertSetting, IsBootstrapped, SecretKeySetting, Settings},
    HostActor,
};
use crate::crypto::{gen_secret_key, mkpass};
use crate::errors::HandlerError;
use crate::registry::Registry;

pub async fn is_bootstrapped() -> Result<bool, HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    let is_bootstrapped = actor.call(IsBootstrapped).await?;

    Ok(is_bootstrapped?)
}

pub async fn show_config() -> Result<NcubeConfig, HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    if !is_bootstrapped().await? {
        return Err(HandlerError::Invalid(
            "Ncube requires initial bootstrapping.".into(),
        ));
    }

    let result = actor.call(Settings).await?;
    let config = result?;

    Ok(config)
}

pub async fn bootstrap(settings: Vec<(String, String)>) -> Result<(), HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    if is_bootstrapped().await? {
        return Err(HandlerError::NotAllowed(
            "Ncube already bootstrapped".into(),
        ));
    }

    // The documentation has a warning about using StdRng::seed_from_u64, that
    // it should not be used for cryptographic applications. I think it's okay
    // here though, since bootstrapping happens only once, and the secret key is
    // only relevant for remote Ncube installations. This code path almost
    // certainly is only triggered for local installations where the actual
    // secret key doesn't matter.
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let rng = StdRng::seed_from_u64(d.as_secs());
    let seed = mkpass(rng);

    let restricted_settings = vec![("secret_key".to_string(), gen_secret_key(&seed))];

    for (name, value) in settings {
        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;
    }

    for (name, value) in restricted_settings {
        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;
    }

    Ok(())
}

pub async fn insert_config_setting(name: &str, value: &str) -> Result<(), HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    if !is_bootstrapped().await? {
        return Err(HandlerError::Invalid(
            "Ncube requires initial bootstrapping.".into(),
        ));
    }

    let _ = actor
        .call(InsertSetting::new(name.to_string(), value.to_string()))
        .await?;

    Ok(())
}

pub async fn show_secret_key() -> Result<String, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();
    let key = host_actor.call(SecretKeySetting).await??;
    Ok(key.value)
}
