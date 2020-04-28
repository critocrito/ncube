use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Sqlite backend failed.")]
    SqliteDatabase(#[from] rusqlite::Error),
    #[error("Deadpool has a problem")]
    SqlitePool(#[from] deadpool::managed::PoolError<rusqlite::Error>),
    #[error("Tokio runtime failed")]
    Runtime(#[from] tokio::task::JoinError),
    #[error("Serialization failed")]
    Serialization(#[from] serde_rusqlite::error::Error),
    #[error("Database upgrade failed.")]
    Upgrade(#[from] refinery_migrations::Error),
}

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("The underlying store failed.")]
    Store(#[from] StoreError),
    #[error("resource not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("`{0}` not found")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Actor(#[from] ActorError),
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Data(#[from] DataError),
    #[error("Ncube requires bootstrapping")]
    BootstrapMissing,
    #[error("handler action not allowed: {0}")]
    NotAllowed(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
