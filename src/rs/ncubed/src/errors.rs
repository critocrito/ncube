use ncube_actors::ActorError;
use ncube_db::errors::DatabaseError;
use ncube_errors::HostError;
use thiserror::Error;

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
