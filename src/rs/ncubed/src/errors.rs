use ncube_errors::HostError;
use thiserror::Error;

pub use crate::{
    db::{http, sqlite},
    http::ErrorResponse,
};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    SqlitePool(#[from] deadpool::managed::PoolError<rusqlite::Error>),
    #[error(transparent)]
    SqliteConfig(#[from] crate::db::sqlite::SqliteConfigError),
    #[error(transparent)]
    Http(#[from] hyper::error::Error),
    #[error(transparent)]
    Resp(#[from] serde_json::error::Error),
    #[error(transparent)]
    HttpConfig(#[from] crate::db::http::HttpConfigError),
    #[error(transparent)]
    Runtime(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Upgrade(#[from] refinery_migrations::Error),
    #[error(transparent)]
    Invalid(#[from] serde_rusqlite::error::Error),
    #[error("Resource `{0}` does not exist in store.")]
    NotFound(String),
    #[error("Operation is not authorized.")]
    Unauthorized,
    #[error("{0:?}")]
    HttpFail(ErrorResponse),
}

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("The underlying store failed.: {0}")]
    Store(#[from] StoreError),
    #[error("The host gave an error: {0}")]
    Host(String),
    #[error("The request to the actor was invalid: {0}")]
    Invalid(String),
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Store(#[from] StoreError),
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

impl From<HostError> for ActorError {
    fn from(err: HostError) -> Self {
        ActorError::Host(err.to_string())
    }
}

impl From<anyhow::Error> for ActorError {
    fn from(err: anyhow::Error) -> Self {
        ActorError::Host(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ActorError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        ActorError::Host(err.to_string())
    }
}

impl From<ActorError> for HandlerError {
    fn from(err: ActorError) -> Self {
        match err {
            ActorError::Store(err) => match err {
                StoreError::NotFound(inner_err) => HandlerError::NotFound(inner_err),
                StoreError::Invalid(inner_err) => HandlerError::Invalid(inner_err.to_string()),
                _ => HandlerError::Store(err),
            },
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

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Actor(#[from] ActorError),
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Host(#[from] HostError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
