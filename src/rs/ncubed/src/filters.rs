use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;

use crate::errors::{DataError, HandlerError};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(HandlerError::BootstrapMissing) = err.find() {
        // FIXME: Add documentation pointer to the bootstrap error
        code = StatusCode::NOT_FOUND;
        message = "Ncube requires initial bootstrapping.".into();
    } else if let Some(HandlerError::Data(DataError::NotFound(entity))) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = format!("Failure to fetch data entity: {}", entity);
    } else if let Some(HandlerError::NotAllowed(reason)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = reason.to_string();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".into();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

pub mod ncube_config {
    use warp::Filter;

    use crate::handlers::ncube_config as handlers;
    use crate::types::NcubeConfigRequest;

    async fn show() -> Result<impl warp::Reply, warp::Rejection> {
        let config = handlers::show().await?;
        Ok(warp::reply::json(&config))
    }

    async fn create(settings: NcubeConfigRequest) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::create(settings).await?;
        Ok(warp::reply())
    }

    async fn update(settings: NcubeConfigRequest) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::insert(settings).await?;
        Ok(warp::reply())
    }

    pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path::end())
            .and_then(show)
            .or(warp::post()
                .and(warp::path::end())
                .and(warp::body::json())
                .and_then(create)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
                .map(|reply| warp::reply::with_header(reply, "location", "/")))
            .or(warp::put()
                .and(warp::path::end())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}
