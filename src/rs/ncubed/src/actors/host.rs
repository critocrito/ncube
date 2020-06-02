use async_trait::async_trait;
use ncube_data::{ConfigSetting, NcubeConfig, Workspace};
use std::path::Path;
use std::result::Result;
use xactor::{message, Actor, Context, Handler, Message};

use crate::actors::task::{SetupWorkspace, TaskActor};
use crate::db::{sqlite, Database};
use crate::errors::ActorError;
use crate::fs::{mkdirp, unzip_workspace};
use crate::registry::Registry;
use crate::stores::{config_store, workspace_store, ConfigStore, WorkspaceStore};
use crate::types::{DatabaseRequest, WorkspaceRequest};

pub(crate) struct HostActor {
    db: sqlite::Database,
}

#[async_trait]
impl Actor for HostActor {
    async fn started(&mut self, _ctx: &Context<Self>) {
        let store = config_store(Database::Sqlite(self.db.clone()));
        store.upgrade().await.unwrap();
        store.init().await.unwrap();
    }
}

impl Registry for HostActor {}

impl HostActor {
    pub fn new(host_db: &str) -> Result<Self, ActorError> {
        let config = host_db.parse::<sqlite::Config>()?;
        let db = sqlite::Database::new(config, 1);

        Ok(Self { db })
    }

    pub async fn workspace_root(&self) -> Result<Option<ConfigSetting>, ActorError> {
        let store = config_store(Database::Sqlite(self.db.clone()));
        let config = store.show().await?;
        let setting = config.into_iter().find(|setting| {
            let comparator: String = "workspace_root".into();
            comparator == setting.name
        });
        Ok(setting)
    }
}

#[message(result = "Result<sqlite::Database, ActorError>")]
#[derive(Debug)]
pub(crate) struct RequirePool;

#[message(result = "Result<bool, ActorError>")]
#[derive(Debug)]
pub(crate) struct IsBootstrapped;

#[message(result = "Result<NcubeConfig, ActorError>")]
#[derive(Debug)]
pub(crate) struct ShowConfig;

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

#[message(result = "Result<bool, ActorError>")]
#[derive(Debug)]
pub(crate) struct WorkspaceExists {
    pub slug: String,
}

#[derive(Debug)]
pub(crate) struct CreateWorkspace {
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) description: Option<String>,
    pub(crate) kind: String,
    pub(crate) database: DatabaseRequest,
}

impl Message for CreateWorkspace {
    type Result = Result<(), ActorError>;
}

impl From<WorkspaceRequest> for CreateWorkspace {
    fn from(w: WorkspaceRequest) -> CreateWorkspace {
        CreateWorkspace {
            name: w.name.clone(),
            slug: w.slug(),
            description: w.description,
            kind: w.kind,
            database: w.database,
        }
    }
}

#[message(result = "Result<Workspace, ActorError>")]
#[derive(Debug)]
pub(crate) struct ShowWorkspace {
    pub(crate) slug: String,
}

#[message(result = "Result<Vec<Workspace>, ActorError>")]
#[derive(Debug)]
pub(crate) struct ListWorkspaces;

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub(crate) struct RemoveWorkspace {
    pub slug: String,
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub(crate) struct UpdateWorkspace {
    pub current_slug: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[async_trait]
impl Handler<RequirePool> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: RequirePool,
    ) -> Result<sqlite::Database, ActorError> {
        let db = self.db.clone();

        Ok(db)
    }
}

#[async_trait]
impl Handler<IsBootstrapped> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _: IsBootstrapped,
    ) -> Result<bool, ActorError> {
        let store = config_store(Database::Sqlite(self.db.clone()));
        let is_bootstrapped = store.is_bootstrapped().await?;
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
        let store = config_store(Database::Sqlite(self.db.clone()));
        let config = store.show().await?;
        Ok(config)
    }
}

#[async_trait]
impl Handler<InsertSetting> for HostActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: InsertSetting) -> Result<(), ActorError> {
        let store = config_store(Database::Sqlite(self.db.clone()));
        store.insert(&msg.name, &msg.value).await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<WorkspaceExists> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: WorkspaceExists,
    ) -> Result<bool, ActorError> {
        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        let exists = workspace_store.exists(&msg.slug).await?;
        Ok(exists)
    }
}

#[async_trait]
impl Handler<CreateWorkspace> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: CreateWorkspace,
    ) -> Result<(), ActorError> {
        let workspace_root = self
            .workspace_root()
            .await?
            .ok_or_else(|| ActorError::Invalid("missing the workspace root to continue".into()))?;
        let database = match msg.database {
            DatabaseRequest::Sqlite => "sqlite",
        };
        let location = Path::new(&workspace_root.value).join(Path::new(&msg.slug));
        let database_path = location.join("sugarcube.db");

        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        workspace_store
            .create(
                &msg.name,
                &msg.slug,
                &msg.description,
                &msg.kind,
                &location.to_string_lossy(),
                &database,
                &database_path.to_string_lossy(),
            )
            .await?;

        mkdirp(&location)?;
        unzip_workspace(&location)
            .map_err(|_| ActorError::Invalid("Failed to create project directory".into()))?;
        let mut actor = TaskActor::from_registry().await.unwrap();
        actor
            .call(SetupWorkspace {
                location: location.to_string_lossy().to_string(),
            })
            .await??;
        Ok(())
    }
}

#[async_trait]
impl Handler<ListWorkspaces> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        _msg: ListWorkspaces,
    ) -> Result<Vec<Workspace>, ActorError> {
        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        let workspaces = workspace_store.list().await?;
        Ok(workspaces)
    }
}

#[async_trait]
impl Handler<ShowWorkspace> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: ShowWorkspace,
    ) -> Result<Workspace, ActorError> {
        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        let workspace = workspace_store.show_by_slug(&msg.slug).await?;
        Ok(workspace)
    }
}

#[async_trait]
impl Handler<RemoveWorkspace> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: RemoveWorkspace,
    ) -> Result<(), ActorError> {
        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        workspace_store.delete_by_slug(&msg.slug).await?;

        Ok(())
    }
}

#[async_trait]
impl Handler<UpdateWorkspace> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: UpdateWorkspace,
    ) -> Result<(), ActorError> {
        let workspace_store = workspace_store(Database::Sqlite(self.db.clone()));
        workspace_store
            .update(&msg.current_slug, &msg.name, &msg.slug, &msg.description)
            .await?;

        Ok(())
    }
}
