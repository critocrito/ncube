use ncube_actors::{
    db::{DatabaseActor, LookupDatabase},
    host::RequirePool,
    ActorError, HostActor, Registry,
};
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
pub mod unit;
pub mod workspace;

#[derive(Error, Debug)]
pub enum HandlerError {
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

impl From<HandlerError> for warp::Rejection {
    fn from(rejection: HandlerError) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}

#[instrument]
pub async fn ensure_workspace(workspace: &str) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let false = workspace_store.exists(&workspace).await? {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    Ok(())
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
