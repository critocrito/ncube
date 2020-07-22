use async_trait::async_trait;
use ncube_data::{ConfigSetting, NcubeConfig};
use ncube_db::{errors::DatabaseError, sqlite, Database};
use ncube_fs::expand_tilde;
use std::path::PathBuf;
use std::result::Result;
use xactor::{message, Actor, Context, Handler};

use crate::actors::Registry;
use crate::errors::ActorError;
use crate::stores::{config_store, ConfigStore};

pub(crate) struct HostActor {
    db: Database,
}

#[async_trait]
impl Actor for HostActor {
    async fn started(&mut self, _ctx: &Context<Self>) {
        let store = config_store(self.db.clone());
        store.upgrade().await.unwrap();
        store.init().await.unwrap();
    }
}

impl Registry for HostActor {}

impl HostActor {
    pub fn new(connection_str: &str) -> Result<Self, ActorError> {
        let db = sqlite::Database::from_str(&connection_str, 1)
            .map_err(|e| ActorError::Database(DatabaseError::SqliteConfig(e)))?;

        Ok(Self {
            db: Database::Sqlite(Box::new(db)),
        })
    }

    async fn get_setting(&self, name: &str) -> Result<Option<ConfigSetting>, ActorError> {
        let store = config_store(self.db.clone());
        let config = store.show_all().await?;
        let setting = config.into_iter().find(|setting| {
            let comparator: String = name.into();
            comparator == setting.name
        });
        Ok(setting)
    }

    pub async fn workspace_root(&self) -> Result<ConfigSetting, ActorError> {
        self.get_setting("workspace_root")
            .await?
            .ok_or_else(|| ActorError::Invalid("missing workspace root".into()))
    }

    pub async fn secret_key(&self) -> Result<ConfigSetting, ActorError> {
        self.get_setting("secret_key")
            .await?
            .ok_or_else(|| ActorError::Invalid("missing secret key".into()))
    }

    pub async fn endpoint(&self) -> Result<ConfigSetting, ActorError> {
        self.get_setting("endpoint")
            .await?
            .ok_or_else(|| ActorError::Invalid("missing http endpoint".into()))
    }
}

#[message(result = "Result<Database, ActorError>")]
#[derive(Debug)]
pub(crate) struct RequirePool;

#[async_trait]
impl Handler<RequirePool> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: RequirePool,
    ) -> Result<Database, ActorError> {
        let db = self.db.clone();

        Ok(db)
    }
}

#[message(result = "Result<PathBuf, ActorError>")]
#[derive(Debug)]
pub(crate) struct WorkspaceRootSetting;

#[async_trait]
impl Handler<WorkspaceRootSetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: WorkspaceRootSetting,
    ) -> Result<PathBuf, ActorError> {
        let workspace_root = self.workspace_root().await?;
        let value = workspace_root
            .value
            .ok_or_else(|| ActorError::Invalid("signing failed".into()))?;
        let expanded_root =
            expand_tilde(value).ok_or_else(|| ActorError::Invalid("signing failed".into()))?;

        Ok(expanded_root)
    }
}

#[message(result = "Result<ConfigSetting, ActorError>")]
#[derive(Debug)]
pub(crate) struct SecretKeySetting;

#[async_trait]
impl Handler<SecretKeySetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: SecretKeySetting,
    ) -> Result<ConfigSetting, ActorError> {
        let secret_key = self.secret_key().await?;
        Ok(secret_key)
    }
}

#[message(result = "Result<ConfigSetting, ActorError>")]
#[derive(Debug)]
pub(crate) struct EndpointSetting;

#[async_trait]
impl Handler<EndpointSetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: EndpointSetting,
    ) -> Result<ConfigSetting, ActorError> {
        let secret_key = self.endpoint().await?;
        Ok(secret_key)
    }
}

#[message(result = "Result<bool, ActorError>")]
#[derive(Debug)]
pub(crate) struct IsBootstrapped;

#[async_trait]
impl Handler<IsBootstrapped> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: IsBootstrapped,
    ) -> Result<bool, ActorError> {
        let store = config_store(self.db.clone());
        let is_bootstrapped = store.is_bootstrapped().await?;
        Ok(is_bootstrapped)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
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
impl Handler<InsertSetting> for HostActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: InsertSetting) -> Result<(), ActorError> {
        let store = config_store(self.db.clone());
        store.insert(&msg.name, &msg.value).await?;
        Ok(())
    }
}

#[message(result = "Result<NcubeConfig, ActorError>")]
#[derive(Debug)]
pub(crate) struct Settings;

#[async_trait]
impl Handler<Settings> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: Settings,
    ) -> Result<NcubeConfig, ActorError> {
        let store = config_store(self.db.clone());
        let config = store.show().await?;
        Ok(config)
    }
}

#[message(result = "Result<NcubeConfig, ActorError>")]
#[derive(Debug)]
pub(crate) struct AllSettings;

#[async_trait]
impl Handler<AllSettings> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: AllSettings,
    ) -> Result<NcubeConfig, ActorError> {
        let store = config_store(self.db.clone());
        let config = store.show_all().await?;
        Ok(config)
    }
}
