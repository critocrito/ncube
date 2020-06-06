use ncube_data::Source;
use tracing::{error, instrument};

use crate::actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{HostActor, RequirePool},
};
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{source_store, workspace_store, SourceStore, WorkspaceStore};
use crate::types::SourceRequest;

#[instrument]
pub async fn create_source(workspace: &str, source: SourceRequest) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let store = source_store(database);
    store.create(&source.kind, &source.term).await?;

    Ok(())
}

#[instrument]
pub async fn list_sources(workspace: &str) -> Result<Vec<Source>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
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

    let store = source_store(database);
    let sources = store.list().await?;

    Ok(sources)
}

#[instrument]
pub async fn remove_source(workspace: &str, id: i32) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
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

    let store = source_store(database);

    if let false = store.exists(id).await? {
        let msg = format!("Source `{}` doesn't exist.", id);
        error!("{:?}", msg);
        return Err(HandlerError::NotFound(msg));
    }

    store.delete(id).await?;

    Ok(())
}

#[instrument]
pub async fn update_source(
    workspace: &str,
    id: i32,
    source: &SourceRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
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

    let store = source_store(database);

    if let false = store.exists(id).await? {
        let msg = format!("Source `{}` doesn't exist.", id);
        error!("{:?}", msg);
        return Err(HandlerError::NotFound(msg));
    }

    store.update(id, &source.kind, &source.term).await?;

    Ok(())
}
