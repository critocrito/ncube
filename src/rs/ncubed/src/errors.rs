use r2d2::Error as R2d2Error;
use refinery_migrations::Error as RefineryMigrationsError;
use rusqlite::Error as RusqliteError;
use serde_rusqlite::error::Error as SerdeRusqliteError;
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
    #[error("invalid")]
    Invalid,
}

// FIXME: Handle error cases and reasons
impl From<R2d2Error> for DataStoreError {
    fn from(_: R2d2Error) -> DataStoreError {
        DataStoreError::FailedConnection
    }
}

// FIXME: Handle error cases and reasons
impl From<RusqliteError> for DataStoreError {
    fn from(e: RusqliteError) -> DataStoreError {
        match e {
            RusqliteError::QueryReturnedNoRows => DataStoreError::Invalid,
            _ => DataStoreError::FailedConnection,
        }
    }
}

// FIXME: Handle error cases and reasons
impl From<SerdeRusqliteError> for DataStoreError {
    fn from(_: SerdeRusqliteError) -> DataStoreError {
        DataStoreError::Serialization
    }
}

// FIXME: Handle error cases and reasons
impl From<RefineryMigrationsError> for DataStoreError {
    fn from(_: RefineryMigrationsError) -> DataStoreError {
        DataStoreError::Upgrade
    }
}

#[derive(Error, Debug)]
pub enum RouteRejection {
    #[error("channel was dropped")]
    ChannelError,
    #[error("failed to fetch data")]
    DataError,
}
