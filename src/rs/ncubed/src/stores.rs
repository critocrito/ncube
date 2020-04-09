pub mod sqlite;

use async_trait::async_trait;
use ncube_data::{Collection, NcubeConfig};

use crate::errors::DataStoreError;

#[async_trait]
pub trait NcubeStore {
    fn upgrade(&mut self) -> Result<(), DataStoreError>;
    async fn list_collections(&mut self) -> Result<Vec<Collection>, DataStoreError>;
    async fn is_bootstrapped(&mut self) -> Result<bool, DataStoreError>;
    async fn show(&mut self) -> Result<NcubeConfig, DataStoreError>;
}
