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
pub enum RouteRejection {
    #[error("channel was dropped")]
    ChannelError,
    #[error("failed to fetch data")]
    DataError,
    #[error("resource not found")]
    NotFound,
}
