use ncube_data::{QueryTag, Source, Stat};
use tracing::{error, instrument};

use crate::actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{HostActor, RequirePool},
    Registry,
};
use crate::errors::HandlerError;
use crate::stores::{search_store, source_store, stat_store, workspace_store, WorkspaceStore};
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
    store
        .create(&source.kind, &source.term, source.tags)
        .await?;

    Ok(())
}

#[instrument]
pub async fn list_sources(
    workspace: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Source>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

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
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();
    let mut database = database_actor
        .call(LookupDatabase {
            workspace: workspace.to_string(),
        })
        .await??;

    database.login().await?;

    let store = search_store(database);
    let sources = store.sources(&workspace, &query, page, page_size).await?;

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

#[instrument]
pub async fn list_source_tags(workspace: &str) -> Result<Vec<QueryTag>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());

    if let Ok(false) = workspace_store.exists(&workspace).await {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

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

#[instrument]
pub async fn stat_sources_total_search(workspace: &str, query: &str) -> Result<Stat, HandlerError> {
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

    let stat_store = stat_store(database);

    let stats = stat_store.sources_total_search(&query).await?;

    Ok(stats)
}
