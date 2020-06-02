use async_trait::async_trait;
use chrono::Utc;
use ncube_data::Workspace;
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};

use crate::db::{sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn workspace_store(wrapped_db: Database) -> impl WorkspaceStore {
    match wrapped_db {
        Database::Sqlite(db) => WorkspaceStoreSqlite { db },
    }
}

pub(crate) fn workspace_store2(db: sqlite::Database) -> impl WorkspaceStore {
    WorkspaceStoreSqlite { db }
}

#[async_trait]
pub(crate) trait WorkspaceStore {
    async fn exists(&self, slug: &str) -> Result<bool, StoreError>;
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
    ) -> Result<(), StoreError>;
    async fn list(&self) -> Result<Vec<Workspace>, StoreError>;
    async fn show_by_slug(&self, slug: &str) -> Result<Workspace, StoreError>;
    async fn delete_by_slug(&self, slug: &str) -> Result<(), StoreError>;
    async fn update(
        &self,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
    ) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct WorkspaceStoreSqlite {
    db: sqlite::Database,
}

#[async_trait]
impl WorkspaceStore for WorkspaceStoreSqlite {
    async fn exists(&self, slug: &str) -> Result<bool, StoreError> {
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
    ) -> Result<(), StoreError> {
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
            &now.to_rfc3339(),
            &now.to_rfc3339()
        ])?;

        stmt2.execute(params![&workspace_id, &database, &database_path])?;

        conn.execute_batch("COMMIT;")?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Workspace>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/list.sql"))?;

        let workspaces_iter = from_rows::<Workspace>(stmt.query(NO_PARAMS)?);

        let mut workspaces: Vec<Workspace> = vec![];
        for workspace in workspaces_iter {
            workspaces.push(workspace?);
        }

        Ok(workspaces)
    }

    async fn show_by_slug(&self, slug: &str) -> Result<Workspace, StoreError> {
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
            _ => Err(StoreError::NotFound(format!("Workspace/{}", slug))),
        }
    }

    async fn delete_by_slug(&self, slug: &str) -> Result<(), StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/remove_by_slug.sql"))?;
        stmt.execute(params![&slug])?;

        Ok(())
    }

    async fn update(
        &self,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
    ) -> Result<(), StoreError> {
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
}
