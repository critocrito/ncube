use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ncube_data::{ConfigSetting, NcubeConfig, Workspace};
use std::path::Path;
use std::result::Result;
use xactor::{message, Actor, Context, Handler, Message};

use crate::db::sqlite;
use crate::errors::{ActorError, StoreError};
use crate::fs::{mkdirp, unzip_workspace};
use crate::registry::Registry;
use crate::stores::{ConfigSqliteStore, ConfigStore, WorkspaceStore, WorkspaceStoreSqlite};
use crate::types::WorkspaceRequest;

pub(crate) struct HostActor {
    db: sqlite::Database,
    store: ConfigSqliteStore,
    workspace_store: WorkspaceStoreSqlite,
}

#[async_trait]
impl Actor for HostActor {
    async fn started(&mut self, _ctx: &Context<Self>) {
        self.store.upgrade(&self.db).await.unwrap();
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
        let workspace_store = WorkspaceStoreSqlite {};

        Ok(Self {
            store,
            workspace_store,
            db,
        })
    }

    pub async fn workspace_root(&mut self) -> Result<Option<ConfigSetting>, StoreError> {
        let config = self.store.show(&self.db).await?;
        let setting = config.into_iter().find(|setting| {
            let comparator: String = "workspace_root".into();
            &comparator == &setting.name
        });
        Ok(setting)
    }
}

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
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Message for CreateWorkspace {
    type Result = Result<(), ActorError>;
}

impl From<WorkspaceRequest> for CreateWorkspace {
    fn from(w: WorkspaceRequest) -> CreateWorkspace {
        let name = w.name.clone();
        let slug = w.slug();
        let description = w.description.clone();
        let created_at = Utc::now();
        let updated_at = created_at;
        let kind = w.kind;

        CreateWorkspace {
            name,
            slug,
            description,
            kind,
            created_at,
            updated_at,
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

#[async_trait]
impl Handler<WorkspaceExists> for HostActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: WorkspaceExists,
    ) -> Result<bool, ActorError> {
        let exists = self
            .workspace_store
            .exists(self.db.clone(), &msg.slug)
            .await?;
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
        let workspace_root = self.workspace_root().await?.ok_or(ActorError::Invalid(
            "missing the workspace root to continue".into(),
        ))?;
        let location = Path::new(&workspace_root.value).join(Path::new(&msg.slug));
        self.workspace_store
            .create(
                self.db.clone(),
                &msg.name,
                &msg.slug,
                &msg.description,
                &msg.kind,
                &location.to_string_lossy(),
                &msg.created_at.to_rfc3339(),
                &msg.updated_at.to_rfc3339(),
            )
            .await?;

        mkdirp(&location)?;
        unzip_workspace(&location)
            .map_err(|_| ActorError::Invalid("Failed to create project directory".into()))?;
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
        let workspaces = self.workspace_store.list(self.db.clone()).await?;
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
        let workspace = self
            .workspace_store
            .show_by_slug(self.db.clone(), &msg.slug)
            .await?;
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
        self.workspace_store
            .delete_by_slug(self.db.clone(), &msg.slug)
            .await?;

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
        let now = Utc::now();
        self.workspace_store
            .update(
                self.db.clone(),
                &msg.current_slug,
                &msg.name,
                &msg.slug,
                &msg.description,
                &now.to_rfc3339(),
            )
            .await?;

        Ok(())
    }
}
