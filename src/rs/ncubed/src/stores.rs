pub mod sqlite;

use async_trait::async_trait;
use ncube_data::Collection;

use crate::errors::DataStoreError;

#[async_trait]
pub trait NcubeStore {
    fn upgrade(&mut self) -> Result<(), DataStoreError>;
    async fn list_collections(&mut self) -> Result<Vec<Collection>, DataStoreError>;
}
