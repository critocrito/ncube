pub mod sqlite;

use async_trait::async_trait;

use crate::errors::DataStoreError;

#[async_trait]
pub trait Pool {
    async fn exec(&self) -> Result<i64, DataStoreError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
