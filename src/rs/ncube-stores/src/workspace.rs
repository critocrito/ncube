use async_trait::async_trait;
use chrono::Utc;
use ncube_data::Workspace;
use ncube_db::{errors::DatabaseError, sqlite, Database};
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};

pub fn workspace_store(wrapped_db: Database) -> impl WorkspaceStore {
    match wrapped_db {
        Database::Sqlite(db) => WorkspaceStoreSqlite { db },
        Database::Http(_client) => todo!(),
    }
}

#[async_trait]
pub trait WorkspaceStore {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError>;
    #[allow(clippy::too_many_arguments)]
    async fn create(
        &self,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        database: &str,
        database_path: &str,
    ) -> Result<(), DatabaseError>;
    async fn list(&self) -> Result<Vec<Workspace>, DatabaseError>;
    async fn show_by_slug(&self, slug: &str) -> Result<Workspace, DatabaseError>;
    async fn delete_by_slug(&self, slug: &str) -> Result<(), DatabaseError>;
    async fn update(
        &self,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
    ) -> Result<(), DatabaseError>;
    async fn enable(&self, slug: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct WorkspaceStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl WorkspaceStore for WorkspaceStoreSqlite {
    async fn exists(&self, slug: &str) -> Result<bool, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn create(
        &self,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        database: &str,
        database_path: &str,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;

        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/create.sql"))?;
        let mut stmt2 =
            conn.prepare_cached(include_str!("../sql/workspace/create_database.sql"))?;

        conn.execute_batch("BEGIN;")?;
        let workspace_id = stmt.insert(params![
            &name,
            &slug,
            &description,
            &kind,
            &location,
            0,
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        stmt2.execute(params![&workspace_id, &database, &database_path])?;

        conn.execute_batch("COMMIT;")?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Workspace>, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/list.sql"))?;

        let workspaces_iter = from_rows::<Workspace>(stmt.query(NO_PARAMS)?);

        let mut workspaces: Vec<Workspace> = vec![];
        for workspace in workspaces_iter {
            workspaces.push(workspace?);
        }

        Ok(workspaces)
    }

    async fn show_by_slug(&self, slug: &str) -> Result<Workspace, DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/show_by_slug.sql"))?;
        let columns = columns_from_statement(&stmt);
        let rows = stmt.query_and_then(params![&slug], |row| {
            from_row_with_columns::<Workspace>(row, &columns)
        })?;

        let mut workspaces: Vec<Workspace> = vec![];
        for row in rows {
            workspaces.push(row?)
        }

        match workspaces.first() {
            Some(workspace) => Ok(workspace.to_owned()),
            _ => Err(DatabaseError::NotFound(format!("Workspace/{}", slug))),
        }
    }

    async fn delete_by_slug(&self, slug: &str) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt =
            conn.prepare_cached(include_str!("../sql/workspace/remove_database_by_slug.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!(
            "../sql/workspace/remove_capability_by_slug.sql"
        ))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/workspace/remove_by_slug.sql"))?;

        conn.execute_batch("BEGIN;")?;
        stmt.execute(params![&slug])?;
        stmt2.execute(params![&slug])?;
        stmt3.execute(params![&slug])?;
        conn.execute_batch("COMMIT;")?;

        Ok(())
    }

    async fn update(
        &self,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/update.sql"))?;

        stmt.execute(params![
            &name,
            &slug,
            &description,
            &now.to_rfc3339(),
            &current_slug,
        ])?;

        Ok(())
    }

    async fn enable(&self, slug: &str) -> Result<(), DatabaseError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/enable.sql"))?;

        stmt.execute(params![&slug, 1])?;

        Ok(())
    }
}
