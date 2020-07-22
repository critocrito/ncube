use ncube_errors::HostError;
use thiserror::Error;

pub use crate::db::{errors::DatabaseError, http, sqlite};

#[derive(Error, Debug)]
pub enum ActorError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("The host gave an error: {0}")]
    Host(String),

    #[error("The request to the actor was invalid: {0}")]
    Invalid(String),
}

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

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Actor(#[from] ActorError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Host(#[from] HostError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
