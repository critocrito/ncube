use ncube_data::{LoginRequest, ReqCtx, SuccessResponse, UpdatePasswordRequest};
use ncube_handlers::account as handlers;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[instrument]
async fn login(
    workspace: String,
    login: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = handlers::issue_token(&workspace, &login.email, &login.password).await?;
    let response = SuccessResponse::new(token);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn update(
    _req: ReqCtx,
    workspace: String,
    request: UpdatePasswordRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let UpdatePasswordRequest {
        email,
        password,
        password_again,
    } = request;
    let enc_password =
        handlers::update_password(&workspace, &email, &password, &password_again).await?;
    let response = SuccessResponse::new(enc_password);

    Ok(warp::reply::json(&response))
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workspaces" / String / "account")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "account"))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(update))
}
