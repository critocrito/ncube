use async_trait::async_trait;
use ncube_data::NcubeConfig;
use std::result::Result;
use xactor::*;

use crate::errors::ActorError;
use crate::messages::*;
use crate::registry::Registry;
use crate::stores::sqlite::NcubeStoreSqlite;
use crate::stores::NcubeStore;

pub(crate) struct NcubeActor {
    store: NcubeStoreSqlite,
}

impl Actor for NcubeActor {}

impl Registry for NcubeActor {}

impl NcubeActor {
    pub fn new(store: NcubeStoreSqlite) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Handler<IsBootstrapped> for NcubeActor {
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
impl Handler<ShowConfig> for NcubeActor {
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
impl Handler<InsertSetting> for NcubeActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: InsertSetting) -> Result<(), ActorError> {
        self.store.insert(&msg.name, &msg.value).await?;
        Ok(())
    }
}
