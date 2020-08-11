use ncube_data::{ProcessConfigReq, ProcessRunReq, ReqCtx, SuccessResponse};
use ncube_handlers::host as handlers;
use warp::Filter;

use crate::http::{authenticate_remote_req, restrict_to_local_req};

async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let processes = handlers::list_processes(&workspace).await?;
    let response = SuccessResponse::new(processes);

    Ok(warp::reply::json(&response))
}

async fn configure(
    _ctx: ReqCtx,
    workspace: String,
    config_request: ProcessConfigReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::configure_process(&workspace, &config_request).await?;

    Ok(warp::reply())
}

async fn run(
    _ctx: ReqCtx,
    workspace: String,
    process_request: ProcessRunReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::run_process(&workspace, &process_request).await?;

    Ok(warp::reply())
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "processes"))
        .and(warp::get())
        .and_then(list)
        .or(restrict_to_local_req()
            .and(warp::path!("workspaces" / String / "processes"))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(configure))
        .or(restrict_to_local_req()
            .and(warp::path!("workspaces" / String / "processes"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(run))
}
