use anyhow::Result;
use ncube_data::NcubeConfig;
use xactor::*;

use crate::errors::StoreError;
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

#[async_trait::async_trait]
impl Handler<IsBootstrapped> for NcubeActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: IsBootstrapped,
    ) -> Result<bool, StoreError> {
        self.store.is_bootstrapped().await
    }
}

#[async_trait::async_trait]
impl Handler<ShowConfig> for NcubeActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: ShowConfig,
    ) -> Result<NcubeConfig, StoreError> {
        self.store.show().await
    }
}

#[async_trait::async_trait]
impl Handler<InsertSetting> for NcubeActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: InsertSetting) -> Result<(), StoreError> {
        self.store.insert(&msg.name, &msg.value).await
    }
}
