use ncube_data::NcubeConfig;

use crate::actors::{
    host::{InsertSetting, IsBootstrapped, ShowConfig},
    HostActor,
};
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

    let result = actor.call(ShowConfig).await?;
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

    for (name, value) in settings {
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
