use ncube_data::{QueryTag, Source, SourceRequest};
use ncube_stores::{search_store, source_store};
use tracing::{error, instrument};

use crate::{ensure_workspace, lookup_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn create_source(workspace: &str, source: SourceRequest) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let store = source_store(database);
    store
        .create(&source.kind, &source.term, source.tags)
        .await?;

    Ok(())
}

#[instrument]
pub async fn show_source(workspace: &str, id: i32) -> Result<Option<Source>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let mut database = workspace_database(&workspace).await?;

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
    ensure_workspace(&workspace).await?;

    let mut database = workspace_database(&workspace).await?;

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
    ensure_workspace(&workspace).await?;

    let mut database = workspace_database(&workspace).await?;

    database.login().await?;

    let store = search_store(database);
    let sources = store.sources(&query, page, page_size).await?;

    Ok(sources)
}

#[instrument]
pub async fn remove_source(workspace: &str, id: i32) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
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
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
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
    let workspace = lookup_workspace(workspace).await?;

    let mut database = workspace_database(&workspace.slug).await?;

    database.login().await?;

    let store = source_store(database);
    let sources = store.list_source_tags().await?;

    Ok(sources)
}

#[instrument]
pub async fn remove_source_tag(workspace: &str, tag: &str) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;

    let store = source_store(database);
    store.remove_source_tag(&tag).await?;

    Ok(())
}
