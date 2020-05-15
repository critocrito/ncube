use async_trait::async_trait;
use rusqlite::params;

use crate::db::sqlite;
use crate::errors::StoreError;

#[async_trait]
pub(crate) trait SourceStore {
    type Database;

    async fn create(
        &mut self,
        db: Self::Database,
        kind: &str,
        term: &str,
        now: &str,
    ) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct SourceStoreSqlite;

#[async_trait]
impl SourceStore for SourceStoreSqlite {
    type Database = sqlite::Database;

    async fn create(
        &mut self,
        db: Self::Database,
        kind: &str,
        term: &str,
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/source/create.sql"))?;

        stmt.execute(params![&kind, &term, &now, &now])?;

        Ok(())
    }
}
