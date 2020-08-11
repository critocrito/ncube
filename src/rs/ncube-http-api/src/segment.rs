use ncube_data::{ReqCtx, SegmentRequest, SuccessResponse};
use ncube_handlers::workspace as handlers;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[instrument]
async fn create(
    _ctx: ReqCtx,
    workspace: String,
    segment_req: SegmentRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::create_segment(&workspace, &segment_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn show(
    _ctx: ReqCtx,
    workspace: String,
    segment: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let segment = handlers::show_segment(&workspace, &segment).await?;
    let response = SuccessResponse::new(segment);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let segments = handlers::list_segments(&workspace).await?;
    let response = SuccessResponse::new(segments);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn update(
    _ctx: ReqCtx,
    workspace: String,
    segment: String,
    segment_req: SegmentRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::update_segment(&workspace, &segment, &segment_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn remove(
    _ctx: ReqCtx,
    workspace: String,
    segment: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::remove_segment(&workspace, &segment).await?;

    Ok(warp::reply())
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "segments"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "segments"))
            .and(warp::get())
            .and_then(list))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "segments" / String))
            .and(warp::get())
            .and_then(show))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "segments" / String))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(update)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "segments" / String))
            .and(warp::delete())
            .and_then(remove)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
}
