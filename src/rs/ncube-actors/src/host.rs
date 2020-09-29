use async_trait::async_trait;
use ncube_actors_common::{message, Actor, ActorError, Context, Handler, Registry};
use ncube_cache::GuardedCache;
use ncube_data::{Client, ConfigSetting, NcubeConfig};
use ncube_db::{errors::DatabaseError, sqlite, Database};
use ncube_fs::expand_tilde;
use ncube_stores::{config_store, workspace_store, ConfigStore, WorkspaceStore};
use std::path::PathBuf;
use std::result::Result;
use tracing::error;
use uuid::Uuid;

pub struct HostActor {
    db: Database,
    clients: GuardedCache<Client>,
}

#[async_trait]
impl Actor for HostActor {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<(), anyhow::Error> {
        let store = config_store(self.db.clone());
        store.upgrade().await.unwrap();
        store.init().await.unwrap();
        Ok(())
    }
}

impl Registry for HostActor {}

impl HostActor {
    pub fn new(connection_str: &str) -> Result<Self, ActorError> {
        let db = sqlite::Database::from_str(&connection_str, 1)
            .map_err(|e| ActorError::Database(DatabaseError::SqliteConfig(e)))?;

        let clients: GuardedCache<Client> = GuardedCache::new();

        Ok(Self {
            db: Database::Sqlite(Box::new(db)),
            clients,
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
pub struct RequirePool;

#[async_trait]
impl Handler<RequirePool> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: RequirePool,
    ) -> Result<Database, ActorError> {
        let db = self.db.clone();

        Ok(db)
    }
}

#[message(result = "Result<PathBuf, ActorError>")]
#[derive(Debug)]
pub struct WorkspaceRootSetting;

#[async_trait]
impl Handler<WorkspaceRootSetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
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
pub struct SecretKeySetting;

#[async_trait]
impl Handler<SecretKeySetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: SecretKeySetting,
    ) -> Result<ConfigSetting, ActorError> {
        let secret_key = self.secret_key().await?;
        Ok(secret_key)
    }
}

#[message(result = "Result<ConfigSetting, ActorError>")]
#[derive(Debug)]
pub struct EndpointSetting;

#[async_trait]
impl Handler<EndpointSetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: EndpointSetting,
    ) -> Result<ConfigSetting, ActorError> {
        let secret_key = self.endpoint().await?;
        Ok(secret_key)
    }
}

#[message(result = "Result<bool, ActorError>")]
#[derive(Debug)]
pub struct IsBootstrapped;

#[async_trait]
impl Handler<IsBootstrapped> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _: IsBootstrapped,
    ) -> Result<bool, ActorError> {
        let store = config_store(self.db.clone());
        let is_bootstrapped = store.is_bootstrapped().await?;
        Ok(is_bootstrapped)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct InsertSetting {
    pub name: String,
    pub value: String,
}

impl InsertSetting {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[async_trait]
impl Handler<InsertSetting> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: InsertSetting,
    ) -> Result<(), ActorError> {
        let store = config_store(self.db.clone());
        store.insert(&msg.name, &msg.value).await?;
        Ok(())
    }
}

#[message(result = "Result<NcubeConfig, ActorError>")]
#[derive(Debug)]
pub struct Settings;

#[async_trait]
impl Handler<Settings> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _: Settings,
    ) -> Result<NcubeConfig, ActorError> {
        let store = config_store(self.db.clone());
        let config = store.show().await?;
        Ok(config)
    }
}

#[message(result = "Result<NcubeConfig, ActorError>")]
#[derive(Debug)]
pub struct AllSettings;

#[async_trait]
impl Handler<AllSettings> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _: AllSettings,
    ) -> Result<NcubeConfig, ActorError> {
        let store = config_store(self.db.clone());
        let config = store.show_all().await?;
        Ok(config)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct EnableWorkspace {
    pub workspace: String,
}

#[async_trait]
impl Handler<EnableWorkspace> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: EnableWorkspace,
    ) -> Result<(), ActorError> {
        let store = workspace_store(self.db.clone());
        store.enable(&msg.workspace).await?;
        Ok(())
    }
}

#[message(result = "Result<String, ActorError>")]
#[derive(Debug)]
pub struct RegisterClient {
    pub client_id: usize,
}

#[async_trait]
impl Handler<RegisterClient> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RegisterClient,
    ) -> Result<String, ActorError> {
        let uuid = Uuid::new_v4()
            .to_simple()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();

        self.clients.put(
            &uuid,
            Client {
                client_id: msg.client_id,
                topics: vec![String::from("host")],
                sender: None,
            },
        );

        Ok(uuid)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct UnregisterClient {
    pub uuid: String,
}

#[async_trait]
impl Handler<UnregisterClient> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UnregisterClient,
    ) -> Result<(), ActorError> {
        self.clients.delete(&msg.uuid);

        Ok(())
    }
}

#[message(result = "Result<Option<Client>, ActorError>")]
#[derive(Debug)]
pub struct ClientSubscription {
    pub uuid: String,
}

#[async_trait]
impl Handler<ClientSubscription> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ClientSubscription,
    ) -> Result<Option<Client>, ActorError> {
        let client = self.clients.get(&msg.uuid);

        Ok(client)
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct UpdateSubscription {
    pub uuid: String,
    pub client: Client,
}

#[async_trait]
impl Handler<UpdateSubscription> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UpdateSubscription,
    ) -> Result<(), ActorError> {
        self.clients.reset(&msg.uuid, msg.client);

        Ok(())
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub struct PublishMessage {
    pub msg: String,
}

#[async_trait]
impl Handler<PublishMessage> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: PublishMessage,
    ) -> Result<(), ActorError> {
        for client in self.clients.entries().iter() {
            if let Some(channel) = &client.1.sender {
                if let Err(e) = channel.send(Ok(warp::ws::Message::text(msg.msg.clone()))) {
                    error!(
                        "Error sending message to client {}: {}",
                        client.0,
                        e.to_string()
                    );
                }
            }
        }

        Ok(())
    }
}
