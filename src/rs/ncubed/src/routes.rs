use serde::Serialize;
use std::convert::Infallible;
use tracing::info;
use warp::{
    http::{Method, StatusCode},
    Filter,
};

use crate::errors::{DataError, HandlerError};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub(crate) async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
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

pub(crate) fn router(
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    let log = warp::log::custom(|info| {
        let method = info.method();
        let path = info.path();
        let status = info.status();
        let elapsed = info.elapsed();
        info!(req.method = %method, req.path = path, req.status = %status, req.elapsed = ?elapsed);
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
        .or(warp::path!("fonts" / "NotoSans-Regular.ttf")
            .map(|| {
                let file = include_bytes!("../../../../target/dist/fonts/NotoSans-Regular.ttf");
                file.to_vec()
            })
            .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
        .or(warp::path!("fonts" / "NotoSans-Bold.ttf")
            .map(|| {
                let file = include_bytes!("../../../../target/dist/fonts/NotoSans-Bold.ttf");
                file.to_vec()
            })
            .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
}

pub(crate) fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type"]);

    warp::path("api").and(config::routes().with(cors))
}

pub(crate) mod config {
    use serde::Deserialize;
    use warp::Filter;

    use crate::handlers::config as handlers;

    #[derive(Debug, Deserialize)]
    struct SettingRequest {
        name: String,
        value: String,
    }

    async fn show() -> Result<impl warp::Reply, warp::Rejection> {
        let config = handlers::show_config().await?;

        Ok(warp::reply::json(&config))
    }

    async fn create(settings: Vec<SettingRequest>) -> Result<impl warp::Reply, warp::Rejection> {
        let mut config = vec![];
        for SettingRequest { name, value } in settings {
            config.push((name, value));
        }

        handlers::bootstrap(config).await?;

        Ok(warp::reply())
    }

    async fn update(settings: Vec<SettingRequest>) -> Result<impl warp::Reply, warp::Rejection> {
        for SettingRequest { name, value } in settings {
            handlers::insert_config_setting(&name, &value).await?;
        }

        Ok(warp::reply())
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
