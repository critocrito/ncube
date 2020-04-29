use async_trait::async_trait;
use ncube_data::Workspace;
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, columns_from_statement, from_row_with_columns, from_rows};

use crate::db::sqlite;
use crate::errors::StoreError;

#[async_trait]
pub(crate) trait WorkspaceStore {
    type Database;

    async fn exists(&mut self, db: Self::Database, slug: &str) -> Result<bool, StoreError>;
    async fn create(
        &mut self,
        db: Self::Database,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        created_at: &str,
        updated_at: &str,
    ) -> Result<(), StoreError>;
    async fn list(&mut self, db: Self::Database) -> Result<Vec<Workspace>, StoreError>;
    async fn show_by_slug(
        &mut self,
        db: Self::Database,
        slug: &str,
    ) -> Result<Workspace, StoreError>;
    async fn delete_by_slug(&mut self, db: Self::Database, slug: &str) -> Result<(), StoreError>;
    async fn update(
        &mut self,
        db: Self::Database,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        updated_at: &str,
    ) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct WorkspaceStoreSqlite;

#[async_trait]
impl WorkspaceStore for WorkspaceStoreSqlite {
    type Database = sqlite::Database;

    async fn exists(&mut self, db: Self::Database, slug: &str) -> Result<bool, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/exists.sql"))?;

        let result: i32 = stmt.query_row(params![&slug], |row| row.get(0))?;

        match result {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    async fn create(
        &mut self,
        db: Self::Database,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        created_at: &str,
        updated_at: &str,
    ) -> Result<(), StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/create.sql"))?;

        stmt.execute(params![
            &name,
            &slug,
            &description,
            &kind,
            &location,
            &created_at,
            &updated_at
        ])?;

        Ok(())
    }

    async fn list(&mut self, db: Self::Database) -> Result<Vec<Workspace>, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/list.sql"))?;

        let workspaces_iter = from_rows::<Workspace>(stmt.query(NO_PARAMS)?);

        let mut workspaces: Vec<Workspace> = vec![];
        for workspace in workspaces_iter {
            workspaces.push(workspace?);
        }

        Ok(workspaces)
    }

    async fn show_by_slug(
        &mut self,
        db: Self::Database,
        slug: &str,
    ) -> Result<Workspace, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/show_by_slug.sql"))?;
        let columns = columns_from_statement(&stmt);
        let mut rows = stmt.query_and_then(params![&slug], |row| {
            from_row_with_columns::<Workspace>(row, &columns)
        })?;

        let mut workspaces: Vec<Workspace> = vec![];
        while let Some(workspace) = rows.next() {
            workspaces.push(workspace?);
        }

        match workspaces.first() {
            Some(workspace) => Ok(workspace.to_owned()),
            _ => Err(StoreError::NotFound(format!("Workspace/{}", slug)))?,
        }
    }

    async fn delete_by_slug(&mut self, db: Self::Database, slug: &str) -> Result<(), StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/remove_by_slug.sql"))?;
        stmt.execute(params![&slug])?;

        Ok(())
    }

    async fn update(
        &mut self,
        db: Self::Database,
        current_slug: &str,
        name: &str,
        slug: &str,
        description: &Option<String>,
        kind: &str,
        location: &str,
        updated_at: &str,
    ) -> Result<(), StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/workspace/update.sql"))?;

        stmt.execute(params![
            &name,
            &slug,
            &description,
            &kind,
            &location,
            &updated_at,
            &current_slug,
        ])?;

        Ok(())
    }
}
