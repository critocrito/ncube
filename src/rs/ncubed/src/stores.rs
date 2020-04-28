pub mod sqlite;

use async_trait::async_trait;
use ncube_data::{Collection, NcubeConfig};

use crate::errors::StoreError;

#[async_trait]
pub trait NcubeStore {
    async fn upgrade(&mut self) -> Result<(), StoreError>;
    async fn list_collections(&mut self) -> Result<Vec<Collection>, StoreError>;
    async fn is_bootstrapped(&mut self) -> Result<bool, StoreError>;
    async fn show(&mut self) -> Result<NcubeConfig, StoreError>;
    async fn insert(&mut self, name: &str, value: &str) -> Result<(), StoreError>;
}
