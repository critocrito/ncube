use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("connection failed")]
    FailedConnection,
    #[error("database upgrade failed")]
    Upgrade,
    #[error("data serialization error")]
    Serialization,
    #[error("record not found")]
    NotFound,
}
