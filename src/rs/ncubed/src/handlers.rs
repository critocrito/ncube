pub mod ncube {
    use ncube_data::NcubeConfig;

    use crate::actors::{
        ncube::{InsertSetting, IsBootstrapped, ShowConfig},
        NcubeHost,
    };
    use crate::errors::HandlerError;
    use crate::registry::Registry;

    pub async fn show() -> Result<NcubeConfig, HandlerError> {
        let mut actor = NcubeHost::from_registry().await.unwrap();

        if let Ok(false) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::BootstrapMissing);
        }

        let result = actor.call(ShowConfig).await?;
        let config = result?;

        Ok(config)
    }

    pub async fn create(name: &str, value: &str) -> Result<(), HandlerError> {
        let mut actor = NcubeHost::from_registry().await.unwrap();

        if let Ok(true) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::NotAllowed(
                "Ncube already bootstrapped".into(),
            ));
        }

        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;

        Ok(())
    }

    pub async fn upsert(name: &str, value: &str) -> Result<(), HandlerError> {
        let mut actor = NcubeHost::from_registry().await.unwrap();

        if let Ok(false) = actor.call(IsBootstrapped).await? {
            return Err(HandlerError::BootstrapMissing);
        }

        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;

        Ok(())
    }
}
