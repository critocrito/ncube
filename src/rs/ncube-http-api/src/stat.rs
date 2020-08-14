use ncube_data::{ReqCtx, SuccessResponse};
use ncube_handlers::{stat as stat_handlers, workspace as handlers};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

// The query parameters for stats.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub q: Option<String>,
}

#[instrument]
async fn sources_total(
    _ctx: ReqCtx,
    workspace: String,
    opts: ListOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let query = opts
        .q
        .map(|query| percent_decode_str(&query).decode_utf8_lossy().to_string());

    let stat = handlers::stat_sources_total(&workspace, query).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn sources_types(
    _ctx: ReqCtx,
    workspace: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_sources_types(&workspace).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn data_total(
    _ctx: ReqCtx,
    workspace: String,
    opts: ListOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let query = opts
        .q
        .map(|query| percent_decode_str(&query).decode_utf8_lossy().to_string());

    let stat = handlers::stat_data_total(&workspace, query).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn data_sources(
    _ctx: ReqCtx,
    workspace: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_data_sources(&workspace).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn data_videos(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_data_videos(&workspace).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn data_segments(
    _ctx: ReqCtx,
    workspace: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_data_segments(&workspace).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn processes_all(
    _ctx: ReqCtx,
    workspace: String,
    process: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_process_all(&workspace, &process).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn segments_units(
    _ctx: ReqCtx,
    workspace: String,
    segment: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_segment_units(&workspace, &segment).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn investigations_data(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = stat_handlers::stat_investigation_data(&workspace, &investigation).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn investigations_segments(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = stat_handlers::stat_investigation_segments(&workspace, &investigation).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn investigations_total(
    _ctx: ReqCtx,
    workspace: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = handlers::stat_investigations_total(&workspace).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn investigations_verified(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = stat_handlers::stat_investigation_verified(&workspace, &investigation).await?;
    let response = SuccessResponse::new(stat.value);

    Ok(warp::reply::json(&response))
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!(
            "workspaces" / String / "stats" / "sources" / "total"
        ))
        .and(warp::query::<ListOptions>())
        .and(warp::get())
        .and_then(sources_total)
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "sources" / "types"
            ))
            .and(warp::get())
            .and_then(sources_types))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "data" / "total"
            ))
            .and(warp::query::<ListOptions>())
            .and(warp::get())
            .and_then(data_total))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "data" / "sources"
            ))
            .and(warp::get())
            .and_then(data_sources))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "data" / "videos"
            ))
            .and(warp::get())
            .and_then(data_videos))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "data" / "segments"
            ))
            .and(warp::get())
            .and_then(data_segments))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "processes" / String / "all"
            ))
            .and(warp::get())
            .and_then(processes_all))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "segments" / String / "units"
            ))
            .and(warp::get())
            .and_then(segments_units))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "investigations" / "total"
            ))
            .and(warp::get())
            .and_then(investigations_total))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "investigations" / String / "data"
            ))
            .and(warp::get())
            .and_then(investigations_data))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "investigations" / String / "verified"
            ))
            .and(warp::get())
            .and_then(investigations_verified))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "investigations" / String / "segments"
            ))
            .and(warp::get())
            .and_then(investigations_segments))
}
