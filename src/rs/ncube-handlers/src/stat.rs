use ncube_data::Stat;
use ncube_search::parse_query;
use ncube_stores::stat_store;
use tracing::instrument;

use crate::{ensure_workspace, workspace::show_segment, workspace_database, HandlerError};

#[instrument]
pub async fn stat_sources_total(
    workspace: &str,
    query: Option<String>,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.sources_total(query).await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_sources_types(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.sources_types().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_total(workspace: &str, query: Option<String>) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store
        .data_total(query.map(|q| parse_query(&q)))
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_sources(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.data_sources().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_videos(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.data_videos().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_data_segments(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.data_segments().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_process_all(workspace: &str, process: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.processes_all(&process).await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_segment_units(workspace: &str, segment: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let segment = show_segment(&workspace, &segment).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .data_total(Some(parse_query(&segment.query)))
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_investigations_total(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;
    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);

    let stats = stat_store.investigations_total().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_investigation_segments(
    workspace: &str,
    investigation: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .investigation_segments_total(&investigation)
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_investigation_data(
    workspace: &str,
    investigation: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store.investigation_data_total(&investigation).await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_investigation_verified(
    workspace: &str,
    investigation: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .investigation_data_verified(&investigation)
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_segment_verified(
    workspace: &str,
    investigation: &str,
    segment: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .investigation_segment_verified(&investigation, &segment)
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_segment_progress(
    workspace: &str,
    investigation: &str,
    segment: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .investigation_segment_progress(&investigation, &segment)
        .await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_verified_total(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store.verified_total().await?;

    Ok(stats)
}

#[instrument]
pub async fn stat_in_process_total(workspace: &str) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store.in_process_total().await?;

    Ok(stats)
}
