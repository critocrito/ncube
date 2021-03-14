use ncube_actors_common::{ActorError, Registry};
use ncube_actors_db::{DatabaseActor, LookupDatabase};
use ncube_actors_host::{HostActor, RequirePool};
use ncube_data::{Workspace, WorkspaceKind};
use ncube_db::{Database, DatabaseError};
use ncube_errors::HostError;
use ncube_stores::{workspace_store, WorkspaceStore};
use thiserror::Error;
use tracing::{error, instrument};

pub mod account;
pub mod config;
pub mod host;
pub mod investigation;
pub mod source;
pub mod stat;
pub mod task;
pub mod unit;
pub mod workspace;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Host(#[from] HostError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("{0}")]
    NotAllowed(String),

    #[error("{0}")]
    Invalid(String),

    #[error("{0}")]
    NotFound(String),
}

impl From<ActorError> for HandlerError {
    fn from(err: ActorError) -> Self {
        match err {
            ActorError::Database(inner_err) => HandlerError::Database(inner_err),
            ActorError::Invalid(msg) | ActorError::Host(msg) => HandlerError::Invalid(msg),
        }
    }
}

impl warp::reject::Reject for HandlerError {}

#[instrument]
pub async fn ensure_workspace(workspace: &str) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let database = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(database.clone());

    if let false = workspace_store.exists(&workspace).await? {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    Ok(())
}

#[instrument]
pub async fn lookup_workspace(workspace: &str) -> Result<Workspace, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let database = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(database.clone());

    let workspace = workspace_store
        .show_by_slug(&workspace)
        .await
        .map_err(|e| {
            let msg = format!("Workspace `{}` doesn't exist.", workspace);
            error!("{:?}", e.to_string());
            HandlerError::Invalid(msg)
        })?;

    Ok(workspace)
}

#[instrument]
pub async fn workspace_database(workspace: &str) -> Result<Database, HandlerError> {
    let database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    Ok(database)
}

#[instrument]
pub async fn host_database() -> Result<Database, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let database = host_actor.call(RequirePool).await??;

    Ok(database)
}

#[instrument]
pub async fn database(workspace: &Workspace) -> Result<Database, HandlerError> {
    // When we list processes for a local workspace we need to use the local
    // sqlite host database, but for remote workspaces we still have to query
    // the remote workspace database.
    match workspace.kind {
        WorkspaceKind::Local(_) => host_database().await,
        WorkspaceKind::Remote(_) => workspace_database(&workspace.slug).await,
    }
}
