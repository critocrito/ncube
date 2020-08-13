use async_trait::async_trait;
use chrono::Utc;
use ncube_data::{Investigation, InvestigationReq};
use ncube_db::{errors::DatabaseError, http, sqlite, Database};
use rusqlite::{params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};
use tracing::instrument;

pub fn investigation_store(wrapped_db: Database) -> Box<dyn InvestigationStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(InvestigationStoreSqlite { db }),
        Database::Http(client) => Box::new(InvestigationStoreHttp { client }),
    }
}

#[async_trait]
pub trait InvestigationStore {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError>;
    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        methodology: &str,
        slug: &str,
    ) -> Result<(), DatabaseError>;
    async fn show(&self, slug: &str) -> Result<Option<Investigation>, DatabaseError>;
    async fn list(&self) -> Result<Vec<Investigation>, DatabaseError>;
    async fn verify_segment(&self) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct InvestigationStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl InvestigationStore for InvestigationStoreSqlite {
    #[instrument]
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/investigation/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[instrument]
    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        methodology: &str,
        slug: &str,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt =
            conn.prepare_cached(include_str!("../sql/investigation/show_methodology.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/investigation/create.sql"))?;

        let methodology_id: i32 = stmt
            .query_row(params![&methodology], |row| row.get(0))
            .map_err(|_| DatabaseError::NotFound("couldn't retrieve methodology".into()))?;

        stmt2.execute(params![
            &title,
            &slug,
            &description,
            methodology_id,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        Ok(())
    }

    // #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Investigation>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/investigation/show.sql"))?;

        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&slug], |row| {
            from_row_with_columns::<Investigation>(row, &columns)
        })?;

        let mut investigations: Vec<Investigation> = vec![];
        for row in rows {
            investigations.push(row?)
        }

        match investigations.first() {
            Some(investigation) => Ok(Some(investigation.to_owned())),
            _ => Ok(None),
        }
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Investigation>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/investigation/list.sql"))?;

        let mut investigations: Vec<Investigation> = vec![];
        for row in from_rows::<Investigation>(stmt.query(NO_PARAMS)?) {
            investigations.push(row?)
        }

        Ok(investigations)
    }

    #[instrument]
    async fn verify_segment(&self) -> Result<(), DatabaseError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct InvestigationStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl InvestigationStore for InvestigationStoreHttp {
    #[instrument]
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations/{}",
            self.client.workspace.slug, slug,
        ));

        match self.client.get::<Investigation>(url).await {
            Ok(_) => Ok(true),
            _ => Ok(false),
        }
    }

    async fn create(
        &self,
        title: &str,
        description: &Option<String>,
        methodology: &str,
        _slug: &str,
    ) -> Result<(), DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations",
            self.client.workspace.slug
        ));

        let payload = InvestigationReq {
            title: title.to_string(),
            description: description.clone(),
            methodology: methodology.to_string(),
        };

        self.client
            .post::<(), InvestigationReq>(url, payload)
            .await?;

        Ok(())
    }

    #[instrument]
    async fn show(&self, slug: &str) -> Result<Option<Investigation>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations/{}",
            self.client.workspace.slug, slug,
        ));

        let data: Option<Investigation> = self.client.get(url).await?;

        Ok(data)
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<Investigation>, DatabaseError> {
        let mut url = self.client.url.clone();
        url.set_path(&format!(
            "/api/workspaces/{}/investigations",
            self.client.workspace.slug
        ));

        let data: Vec<Investigation> = self.client.get(url).await?.unwrap_or_else(|| vec![]);

        Ok(data)
    }

    #[instrument]
    async fn verify_segment(&self) -> Result<(), DatabaseError> {
        unimplemented!()
    }
}
