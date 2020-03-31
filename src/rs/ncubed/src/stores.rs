pub mod sqlite;

use crate::errors::DataStoreError;

pub trait NcubeStore {
    fn upgrade(&mut self) -> Result<(), DataStoreError>;
}
