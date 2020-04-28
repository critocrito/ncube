use xactor::Actor;

use crate::registry::Registry;
use crate::stores::sqlite::NcubeStoreSqlite;

pub(crate) struct NcubeHost {
    store: NcubeStoreSqlite,
}

impl Actor for NcubeHost {}

impl Registry for NcubeHost {}

impl NcubeHost {
    pub fn new(store: NcubeStoreSqlite) -> Self {
        Self { store }
    }
}

pub(crate) mod ncube {
    use super::NcubeHost;
    use crate::stores::NcubeStore;
    use async_trait::async_trait;
    use ncube_data::NcubeConfig;
    use std::result::Result;
    use xactor::{message, Context, Handler};

    use crate::errors::ActorError;

    #[message(result = "Result<bool, ActorError>")]
    pub(crate) struct IsBootstrapped;

    #[message(result = "Result<NcubeConfig, ActorError>")]
    pub(crate) struct ShowConfig;

    #[message(result = "Result<(), ActorError>")]
    pub(crate) struct InsertSetting {
        pub name: String,
        pub value: String,
    }

    impl InsertSetting {
        pub(crate) fn new(name: String, value: String) -> Self {
            Self { name, value }
        }
    }

    #[async_trait]
    impl Handler<IsBootstrapped> for NcubeHost {
        async fn handle(
            &mut self,
            _ctx: &Context<Self>,
            _: IsBootstrapped,
        ) -> Result<bool, ActorError> {
            let is_bootstrapped = self.store.is_bootstrapped().await?;
            Ok(is_bootstrapped)
        }
    }

    #[async_trait]
    impl Handler<ShowConfig> for NcubeHost {
        async fn handle(
            &mut self,
            _ctx: &Context<Self>,
            _: ShowConfig,
        ) -> Result<NcubeConfig, ActorError> {
            let config = self.store.show().await?;
            Ok(config)
        }
    }

    #[async_trait]
    impl Handler<InsertSetting> for NcubeHost {
        async fn handle(
            &mut self,
            _ctx: &Context<Self>,
            msg: InsertSetting,
        ) -> Result<(), ActorError> {
            self.store.insert(&msg.name, &msg.value).await?;
            Ok(())
        }
    }
}
