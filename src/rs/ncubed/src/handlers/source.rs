use chrono::Utc;
use ncube_data::Source;
use tracing::{error, instrument};

use crate::actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{HostActor, WorkspaceExists},
};
use crate::db::sqlite;
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{SourceStore, SourceStoreSqlite};
use crate::types::SourceRequest;

#[instrument]
pub async fn create_source(workspace: &str, source: SourceRequest) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    if let Ok(false) = host_actor
        .call(WorkspaceExists {
            slug: workspace.into(),
        })
        .await?
    {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.into(),
        })
        .await??;

    #[allow(clippy::match_single_binding)]
    match database {
        sqlite::Database { .. } => {
            let store = SourceStoreSqlite {};
            let now = Utc::now();
            store
                .create(database, &source.kind, &source.term, &now.to_rfc3339())
                .await?;
        }
    }

    Ok(())
}

#[instrument]
pub async fn list_sources(workspace: &str) -> Result<Vec<Source>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    if let Ok(false) = host_actor
        .call(WorkspaceExists {
            slug: workspace.into(),
        })
        .await?
    {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.into(),
        })
        .await??;

    #[allow(clippy::match_single_binding)]
    let sources = match database {
        sqlite::Database { .. } => {
            let store = SourceStoreSqlite {};
            store.list(database).await?
        }
    };

    Ok(sources)
}

#[instrument]
pub async fn remove_source(workspace: &str, id: i32) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    if let Ok(false) = host_actor
        .call(WorkspaceExists {
            slug: workspace.into(),
        })
        .await?
    {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.into(),
        })
        .await??;

    #[allow(clippy::match_single_binding)]
    match database {
        sqlite::Database { .. } => {
            let store = SourceStoreSqlite {};
            if let false = store.exists(database.clone(), id).await? {
                let msg = format!("Source `{}` doesn't exist.", id);
                error!("{:?}", msg);
                return Err(HandlerError::NotFound(msg));
            }

            store.delete(database, id).await?
        }
    };

    Ok(())
}
