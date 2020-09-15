use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Segment, SegmentRequest};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};
use tracing::instrument;

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
    async fn show(&self, slug: &str) -> Result<Option<Segment>, DatabaseError>;
    async fn list(&self) -> Result<Vec<Segment>, DatabaseError>;
    async fn delete(&self, slug: &str) -> Result<(), DatabaseError>;
    async fn update(&self, slug: &str, query: &str, title: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct SegmentStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl SegmentStore for SegmentStoreSqlite {
    #[instrument]
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
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

    #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Segment>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/show.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&slug], |row| {
            from_row_with_columns::<Segment>(row, &columns)
        })?;

        let mut segments: Vec<Segment> = vec![];
        for row in rows {
            segments.push(row?)
        }

        match segments.first() {
            Some(segment) => Ok(Some(segment.to_owned())),
            _ => Ok(None),
        }
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Segment>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/list.sql"))?;

        let mut segments: Vec<Segment> = vec![];
        for row in from_rows::<Segment>(stmt.query(NO_PARAMS)?) {
            segments.push(row?)
        }

        Ok(segments)
    }

    #[instrument]
    async fn delete(&self, slug: &str) -> Result<(), DatabaseError> {
        if let Some(segment) = self.show(&slug).await? {
            let conn = self.db.connection().await?;
            let mut stmt =
                conn.prepare_cached(include_str!("../sql/annotation/delete-by-segment.sql"))?;
            let mut stmt2 = conn.prepare_cached(include_str!(
                "../sql/investigation/remove-verifications.sql"
            ))?;
            let mut stmt3 = conn.prepare_cached(include_str!("../sql/segment/delete.sql"))?;

            conn.execute_batch("BEGIN;")?;

            stmt.execute(params![&segment.id])?;
            stmt2.execute(params![&segment.id])?;
            stmt3.execute(params![&slug])?;

            conn.execute_batch("COMMIT;")?;
        }

        Ok(())
    }

    #[instrument]
    async fn update(&self, slug: &str, query: &str, title: &str) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/segment/update.sql"))?;

        let segment_req = SegmentRequest {
            title: title.to_string(),
            query: query.to_string(),
        };

        stmt.execute(params![
            &segment_req.query,
            &segment_req.title,
            &segment_req.slug(),
            &now.to_rfc3339(),
            &slug
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
    #[instrument]
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

    #[instrument]
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

    #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Segment>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments/{}",
            self.client.workspace.slug, slug,
        ));

        let data: Option<Segment> = self.client.get(url).await?;

        Ok(data)
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Segment>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments",
            self.client.workspace.slug
        ));

        let data: Vec<Segment> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    #[instrument]
    async fn delete(&self, slug: &str) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments/{}",
            self.client.workspace.slug, slug,
        ));

        self.client.delete(url).await?;

        Ok(())
    }

    #[instrument]
    async fn update(&self, slug: &str, query: &str, title: &str) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/segments/{}",
            self.client.workspace.slug, slug
        ));

        let payload = SegmentRequest {
            title: title.to_string(),
            query: query.to_string(),
        };

        self.client.put::<(), SegmentRequest>(url, payload).await?;

        Ok(())
    }
}
