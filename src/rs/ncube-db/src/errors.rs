use ncube_data::ErrorResponse;
use std::fmt::{Display, Formatter, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct SqliteConfigError;

impl Display for SqliteConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SqliteConfigError")
    }
}

#[derive(Error, Debug)]
pub struct HttpConfigError;

impl Display for HttpConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "HttpConfigError")
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    SqlitePool(#[from] deadpool::managed::PoolError<rusqlite::Error>),
    #[error(transparent)]
    Invalid(#[from] serde_rusqlite::error::Error),
    #[error(transparent)]
    SqliteConfig(#[from] SqliteConfigError),
    #[error(transparent)]
    Upgrade(#[from] refinery::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Runtime(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Resp(#[from] serde_json::error::Error),
    #[error("{0}")]
    HttpConfig(String),

    #[error("Resource `{0}` does not exist in store.")]
    NotFound(String),
    #[error("Operation is not authorized.")]
    Unauthorized,
    #[error("{0:?}")]
    HttpFail(ErrorResponse),
}
