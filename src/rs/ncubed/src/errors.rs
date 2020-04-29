pub use crate::db::sqlite::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error(transparent)]
    SqliteConfig(#[from] crate::db::sqlite::ConfigError),
    #[error(transparent)]
    SqliteDatabase(#[from] rusqlite::Error),
    #[error(transparent)]
    SqlitePool(#[from] deadpool::managed::PoolError<rusqlite::Error>),
    #[error(transparent)]
    Runtime(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Upgrade(#[from] refinery_migrations::Error),
    #[error(transparent)]
    Invalid(#[from] serde_rusqlite::error::Error),
    #[error("Resource `{0}` does not exist in store.")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("The underlying store failed.: {0}")]
    Store(#[from] StoreError),
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Store(#[from] StoreError),
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
            ActorError::Store(err) => match err {
                StoreError::NotFound(inner_err) => HandlerError::NotFound(inner_err.to_string()),
                StoreError::Invalid(inner_err) => HandlerError::Invalid(inner_err.to_string()),
                _ => HandlerError::Store(err),
            },
        }
    }
}

impl warp::reject::Reject for HandlerError {}

impl From<HandlerError> for warp::Rejection {
    fn from(rejection: HandlerError) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Actor(#[from] ActorError),
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
