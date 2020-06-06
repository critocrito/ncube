use async_trait::async_trait;
use chrono::Utc;
use ncube_data::Source;
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::from_rows;

use crate::db::{sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn source_store(wrapped_db: Database) -> impl SourceStore {
    match wrapped_db {
        Database::Sqlite(db) => SourceStoreSqlite { db },
        Database::Http(_client) => todo!(),
    }
}

#[async_trait]
pub(crate) trait SourceStore {
    async fn exists(&self, id: i32) -> Result<bool, StoreError>;
    async fn create(&self, kind: &str, term: &str) -> Result<(), StoreError>;
    async fn list(&self) -> Result<Vec<Source>, StoreError>;
    async fn delete(&self, id: i32) -> Result<(), StoreError>;
    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct SourceStoreSqlite {
    db: sqlite::Database,
}

#[async_trait]
impl SourceStore for SourceStoreSqlite {
    async fn exists(&self, id: i32) -> Result<bool, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&id], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    async fn create(&self, kind: &str, term: &str) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/create.sql"))?;

        stmt.execute(params![&kind, &term, &now.to_rfc3339(), &now.to_rfc3339()])?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Source>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/list.sql"))?;

        let mut sources: Vec<Source> = vec![];
        for source in from_rows::<Source>(stmt.query(NO_PARAMS)?) {
            sources.push(source?);
        }

        Ok(sources)
    }

    async fn delete(&self, id: i32) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/delete.sql"))?;

        stmt.execute(params![&id])?;

        Ok(())
    }

    async fn update(&self, id: i32, kind: &str, term: &str) -> Result<(), StoreError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/source/update.sql"))?;

        stmt.execute(params![&id, &kind, &term, &now.to_rfc3339()])?;

        Ok(())
    }
}
