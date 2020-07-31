use ncube_data::ErrorResponse;
use ncube_db::errors::DatabaseError;
use ncube_errors::HostError;
use ncube_handlers::HandlerError;
use std::convert::Infallible;
use tracing::{error, info, instrument};
use warp::{
    http::{Method, StatusCode},
    Filter,
};

mod config;
mod process;
mod segment;
mod source;
mod source_tag;
mod stat;
mod unit;
mod user;
mod workspace;

#[instrument]
pub(crate) async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    error!("{:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(HandlerError::Invalid(reason)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = reason.into();
    } else if let Some(HandlerError::NotFound(reason)) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = reason.into();
    } else if let Some(HandlerError::NotAllowed(reason)) = err.find() {
        code = StatusCode::FORBIDDEN;
        message = reason.to_string();
    } else if let Some(HostError::AuthError(reason)) = err.find() {
        error!("{:?}", reason);
        code = StatusCode::UNAUTHORIZED;
        message = "request did not authorize".to_string();
    } else if let Some(DatabaseError::HttpFail(reason)) = err.find() {
        error!("{:?}", reason);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    } else if let Some(rejection) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = rejection.to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorResponse::new(code, &message));

    Ok(warp::reply::with_status(json, code))
}

pub(crate) fn router(
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    let log = warp::log::custom(|info| {
        let method = info.method();
        let path = info.path();
        let status = info.status();
        let elapsed = info.elapsed();
        if status.as_u16() >= 400 {
            error!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        } else {
            info!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
        }
    });

    assets().or(api()).recover(handle_rejection).with(log)
}

pub(crate) fn assets() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("index.html")
        .map(|| include_str!("../../../../target/dist/index.html"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/html"))
        .or(warp::path("styles.css")
            .map(|| include_str!("../../../../target/dist/styles.css"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/css")))
        .or(warp::path("app.js")
            .map(|| include_str!("../../../../target/dist/app.js"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript")))
        .or(warp::path("styles.css.map")
            .map(|| include_str!("../../../../target/dist/styles.css.map"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/css")))
        .or(warp::path("app.js.map")
            .map(|| include_str!("../../../../target/dist/app.js.map"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript")))
}

pub(crate) fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["content-type"]);

    warp::path("api")
        .and(
            config::routes()
                .or(workspace::routes())
                .or(source::routes())
                .or(user::routes())
                .or(stat::routes())
                .or(unit::routes())
                .or(source_tag::routes())
                .or(segment::routes())
                .or(process::routes()),
        )
        .with(cors)
}
