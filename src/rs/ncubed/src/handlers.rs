use ncube_data::NcubeConfig;

use crate::actors::NcubeActor;
use crate::errors::HandlerError;
use crate::messages::{InsertSetting, IsBootstrapped, ShowConfig};
use crate::registry::Registry;

use crate::types::NcubeConfigRequest;

pub mod ncube_config {
    use super::*;

    pub async fn show() -> Result<NcubeConfig, HandlerError> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(false) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::BootstrapMissing);
        }

        let result = actor.call(ShowConfig).await?;
        let config = result?;

        Ok(config)
    }

    pub async fn create(settings: NcubeConfigRequest) -> Result<(), HandlerError> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(true) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::NotAllowed(
                "Ncube already bootstrapped".into(),
            ));
        }

        for setting in settings {
            let _ = actor
                .call(InsertSetting::new(
                    setting.name.to_string(),
                    setting.value.to_string(),
                ))
                .await?;
        }
        Ok(())
    }

    pub async fn insert(settings: NcubeConfigRequest) -> Result<(), HandlerError> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(false) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::BootstrapMissing);
        }

        for setting in settings {
            let _ = actor
                .call(InsertSetting::new(
                    setting.name.to_string(),
                    setting.value.to_string(),
                ))
                .await;
        }

        Ok(())
    }
}
