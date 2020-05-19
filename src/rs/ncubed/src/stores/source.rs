use async_trait::async_trait;
use ncube_data::Source;
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;

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
    async fn list(&mut self, db: Self::Database) -> Result<Vec<Source>, StoreError>;
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

    async fn list(&mut self, db: Self::Database) -> Result<Vec<Source>, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/list.sql"))?;

        let mut sources: Vec<Source> = vec![];
        for source in from_rows::<Source>(stmt.query(NO_PARAMS)?) {
            sources.push(source?);
        }

        Ok(sources)
    }
}
