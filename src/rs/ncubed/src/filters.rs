use serde::Serialize;
use std::convert::Infallible;
use tokio::sync::mpsc::{error::SendError, Sender};

use warp::{http::StatusCode, Filter};

use crate::errors::{DataStoreError, RouteRejection};
use crate::handlers;
use crate::services::NcubeStoreCmd;

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

impl From<SendError<NcubeStoreCmd>> for RouteRejection {
    fn from(_: SendError<NcubeStoreCmd>) -> RouteRejection {
        RouteRejection::ChannelError
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

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
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

fn with_ncube_store(
    tx: Sender<NcubeStoreCmd>,
) -> impl Filter<Extract = (Sender<NcubeStoreCmd>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

pub mod ncube_config {
    use super::*;

    fn show(
        tx: Sender<NcubeStoreCmd>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path::end())
            .and(with_ncube_store(tx))
            .and_then(handlers::ncube_config::show)
    }

    fn create(
        tx: Sender<NcubeStoreCmd>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path::end())
            .and(warp::body::json())
            .and(with_ncube_store(tx))
            .and_then(handlers::ncube_config::create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .map(|reply| warp::reply::with_header(reply, "location", "/"))
    }

    fn update(
        tx: Sender<NcubeStoreCmd>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::put()
            .and(warp::path::end())
            .and(warp::body::json())
            .and(with_ncube_store(tx))
            .and_then(handlers::ncube_config::insert)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT))
    }

    pub fn routes(
        tx: Sender<NcubeStoreCmd>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        show(tx.clone()).or(create(tx.clone())).or(update(tx))
    }
}
