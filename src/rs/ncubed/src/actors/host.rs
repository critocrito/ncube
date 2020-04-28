use async_trait::async_trait;
use ncube_data::NcubeConfig;
use std::result::Result;
use xactor::{message, Actor, Context, Handler};

use crate::db::sqlite;
use crate::errors::{ActorError, StoreError};
use crate::registry::Registry;
use crate::stores::{ConfigSqliteStore, ConfigStore};

pub(crate) struct HostActor {
    db: sqlite::Database,
    store: ConfigSqliteStore,
}

#[async_trait]
impl Actor for HostActor {
    async fn started(&mut self, _ctx: &Context<Self>) -> () {
        self.store.upgrade(&self.db).await.unwrap();

        ()
    }
}

impl Registry for HostActor {}

// #[async_trait]
impl HostActor {
    // FIXME: Probably I should something else than StoreError
    pub fn new(host_db: &str) -> Result<Self, StoreError> {
        let config = host_db.parse::<sqlite::Config>()?;
        let db = sqlite::Database::new(config, 10);
        let store = ConfigSqliteStore {};

        Ok(Self { store, db })
    }
}

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
impl Handler<IsBootstrapped> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: IsBootstrapped,
    ) -> Result<bool, ActorError> {
        let is_bootstrapped = self.store.is_bootstrapped(&self.db).await?;
        Ok(is_bootstrapped)
    }
}

#[async_trait]
impl Handler<ShowConfig> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: ShowConfig,
    ) -> Result<NcubeConfig, ActorError> {
        let config = self.store.show(&self.db).await?;
        Ok(config)
    }
}

#[async_trait]
impl Handler<InsertSetting> for HostActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: InsertSetting) -> Result<(), ActorError> {
        self.store.insert(&self.db, &msg.name, &msg.value).await?;
        Ok(())
    }
}
