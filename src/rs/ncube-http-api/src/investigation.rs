use ncube_data::{InvestigationReq, ReqCtx, SuccessResponse, VerifySegmentReq};
use ncube_handlers::{investigation as investigation_handlers, workspace as handlers};
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[instrument]
async fn create(
    _ctx: ReqCtx,
    workspace: String,
    investigation_req: InvestigationReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::create_investigation(&workspace, &investigation_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn show(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let investigation = handlers::show_investigation(&workspace, &investigation).await?;
    let response = SuccessResponse::new(investigation);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let investigations = handlers::list_investigations(&workspace).await?;
    let response = SuccessResponse::new(investigations);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn remove(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

#[instrument]
async fn verify_segment(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    verify_segment_req: VerifySegmentReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    investigation_handlers::verify_segment(&workspace, &investigation, &verify_segment_req).await?;

    Ok(warp::reply())
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "investigations"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "investigations"))
            .and(warp::get())
            .and_then(list))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::get())
            .and_then(show))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::delete())
            .and_then(remove)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(verify_segment)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED)))
}
