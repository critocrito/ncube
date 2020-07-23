use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Segment, SegmentRequest};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::params;

pub fn segment_store(wrapped_db: Database) -> Box<dyn SegmentStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(SegmentStoreSqlite { db }),
        Database::Http(client) => Box::new(SegmentStoreHttp { client }),
    }
}

#[async_trait]
pub trait SegmentStore {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError>;
    async fn create(&self, query: &str, title: &str, slug: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct SegmentStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl SegmentStore for SegmentStoreSqlite {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    async fn create(&self, query: &str, title: &str, slug: &str) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/create.sql"))?;

        stmt.execute(params![
            &query,
            &title,
            &slug,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SegmentStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl SegmentStore for SegmentStoreHttp {
    async fn create(&self, query: &str, title: &str, _slug: &str) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments",
            self.client.workspace.slug
        ));

        let payload = SegmentRequest {
            query: query.to_string(),
            title: title.to_string(),
        };

        self.client.post::<(), SegmentRequest>(url, payload).await?;

        Ok(())
    }

    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments/{}",
            self.client.workspace.slug, slug,
        ));

        match self.client.get::<Segment>(url).await {
            Ok(_) => Ok(true),
            _ => Ok(false),
        }
    }
}
