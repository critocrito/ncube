use ncube_actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{HostActor, RequirePool},
    Registry,
};
use ncube_data::{QueryTag, Source, SourceRequest};
use ncube_stores::{search_store, source_store, workspace_store, WorkspaceStore};
use tracing::{error, instrument};

use crate::HandlerError;

#[instrument]
pub async fn create_source(workspace: &str, source: SourceRequest) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    let store = source_store(database);
    store
        .create(&source.kind, &source.term, source.tags)
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_source(workspace: &str, id: i32) -> Result<Option<Source>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let database_actor = DatabaseActor::from_registry().await.unwrap();

    let mut database = database_actor
        .call(LookupDatabase {
            workspace: workspace.slug.clone(),
        })
        .await??;

    database.login().await?;

    let store = source_store(database);
    let source = store.show(id).await?;

    Ok(source)
}

#[instrument]
pub async fn list_sources(
    workspace: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Source>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let database_actor = DatabaseActor::from_registry().await.unwrap();

    let mut database = database_actor
        .call(LookupDatabase {
            workspace: workspace.slug.clone(),
        })
        .await??;

    database.login().await?;

    let store = source_store(database);
    let sources = store.list(page, page_size).await?;

    Ok(sources)
}

#[instrument]
pub async fn search_sources(
    workspace: &str,
    query: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Source>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();
    let mut database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    database.login().await?;

    let store = search_store(database);
    let sources = store.sources(&query, page, page_size).await?;

    Ok(sources)
}

#[instrument]
pub async fn remove_source(workspace: &str, id: i32) -> Result<(), HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();

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
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let database_actor = DatabaseActor::from_registry().await.unwrap();

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

#[instrument]
pub async fn list_source_tags(workspace: &str) -> Result<Vec<QueryTag>, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let database_actor = DatabaseActor::from_registry().await.unwrap();

    let mut database = database_actor
        .call(LookupDatabase {
            workspace: workspace.slug.clone(),
        })
        .await??;

    database.login().await?;

    let store = source_store(database);
    let sources = store.list_source_tags().await?;

    Ok(sources)
}
