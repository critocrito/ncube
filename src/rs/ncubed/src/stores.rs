pub mod sqlite;

use async_trait::async_trait;

use crate::errors::DataStoreError;

#[async_trait]
pub trait NcubeStore {
    fn upgrade(&mut self) -> Result<(), DataStoreError>;

    async fn show_number(&mut self) -> Result<i64, DataStoreError>;
}
