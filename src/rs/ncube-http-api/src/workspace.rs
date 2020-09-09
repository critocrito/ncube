use ncube_data::{ReqCtx, SuccessResponse, WorkspaceRequest};
use ncube_handlers::workspace as handlers;
use serde::Deserialize;
use warp::Filter;

use crate::http::restrict_to_local_req;

// The query parameters for delete workspace.
#[derive(Debug, Deserialize)]
pub struct DeleteOptions {
    pub remove_location: Option<bool>,
}

async fn create(
    _ctx: ReqCtx,
    workspace: WorkspaceRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let workspace = handlers::create_workspace(workspace).await?;
    let response = SuccessResponse::new(workspace);

    Ok(warp::reply::json(&response))
}

async fn show(_ctx: ReqCtx, slug: String) -> Result<impl warp::Reply, warp::Rejection> {
    let workspace = handlers::show_workspace(&slug).await?;
    let response = SuccessResponse::new(workspace);

    Ok(warp::reply::json(&response))
}

async fn list(_ctx: ReqCtx) -> Result<impl warp::Reply, warp::Rejection> {
    let workspaces = handlers::list_workspaces().await?;
    let response = SuccessResponse::new(workspaces);

    Ok(warp::reply::json(&response))
}

async fn delete(
    _ctx: ReqCtx,
    slug: String,
    opts: DeleteOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let remove_location = match opts.remove_location {
        Some(true) => true,
        _ => false,
    };

    handlers::remove_workspace(&slug, remove_location).await?;

    Ok(warp::reply())
}

async fn update(
    _ctx: ReqCtx,
    slug: String,
    workspace: WorkspaceRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::update_workspace(&slug, workspace).await?;

    // FIXME: Set location header
    Ok(warp::reply())
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("workspaces")
        .and(warp::get())
        .and(warp::path::end())
        .and(restrict_to_local_req())
        .and_then(list)
        .or(warp::path("workspaces")
            .and(warp::post())
            .and(warp::path::end())
            .and(restrict_to_local_req())
            .and(warp::body::json())
            .and_then(create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED)))
        .or(warp::any()
            .and(restrict_to_local_req())
            .and(warp::path!("workspaces" / String))
            .and(warp::get())
            .and_then(show))
        .or(warp::any()
            .and(restrict_to_local_req())
            .and(warp::path!("workspaces" / String))
            .and(warp::delete())
            .and(warp::query::<DeleteOptions>())
            .and_then(delete)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(warp::any()
            .and(restrict_to_local_req())
            .and(warp::path!("workspaces" / String))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(update)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
}
