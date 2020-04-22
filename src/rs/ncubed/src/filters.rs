use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter};

use crate::errors::{DataStoreError, RouteRejection};
use crate::handlers;

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

impl warp::reject::Reject for RouteRejection {}

impl From<RouteRejection> for warp::Rejection {
    fn from(rejection: RouteRejection) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}

impl From<DataStoreError> for RouteRejection {
    fn from(_: DataStoreError) -> RouteRejection {
        RouteRejection::DataError
    }
}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    eprintln!("{:?}", err);
    // warp::reject::not_found() returns a 405 Method Not Allowed status
    // code. To ensure a 404 Not Found I create a custom rejection
    // error (RouteRejection::NotFound).
    // See: https://github.com/seanmonstar/warp/issues/77
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(RouteRejection::NotFound) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = "NOT FOUND";
    } else if let Some(RouteRejection::ChannelError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "channel error";
    } else if let Some(RouteRejection::DataError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "data error";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

pub mod ncube_config {
    use super::*;

    fn show() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path::end())
            .and_then(handlers::ncube_config::show)
    }

    fn create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(handlers::ncube_config::create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .map(|reply| warp::reply::with_header(reply, "location", "/"))
    }

    fn update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::put()
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(handlers::ncube_config::insert)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT))
    }

    pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        show().or(create()).or(update())
    }
}
