use ncube_data::{MethodologyReq, ReqCtx, SuccessResponse};
use ncube_handlers::workspace as handlers;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[instrument]
async fn create(
    _ctx: ReqCtx,
    workspace: String,
    methodology_req: MethodologyReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::create_methodology(&workspace, &methodology_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn show(
    _ctx: ReqCtx,
    workspace: String,
    methodology: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let methodology = handlers::show_methodology(&workspace, &methodology).await?;
    let response = SuccessResponse::new(methodology);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let methodologies = handlers::list_methodologies(&workspace).await?;
    let response = SuccessResponse::new(methodologies);

    Ok(warp::reply::json(&response))
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "methodologies"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "methodologies"))
            .and(warp::get())
            .and_then(list))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "methodologies" / String
            ))
            .and(warp::get())
            .and_then(show))
}
