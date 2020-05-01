use serde::Serialize;
use std::convert::Infallible;
use tracing::{error, info, instrument};
use warp::{
    http::{Method, StatusCode},
    Filter,
};

use crate::errors::HandlerError;

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

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
    } else if let Some(rejection) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = rejection.to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
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
        .or(warp::path!("images" / "logo_big.svg")
            .map(|| include_str!("../../../../target/dist/images/logo_big.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
}

pub(crate) fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type"]);

    warp::path("api")
        .and(config::routes().or(workspace::routes()))
        .with(cors)
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

pub(crate) mod workspace {
    use warp::Filter;

    use crate::handlers::workspace as handlers;
    use crate::types::WorkspaceRequest;

    async fn create(workspace: WorkspaceRequest) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::create_workspace(workspace).await?;

        // FIXME: Set location header
        Ok(warp::reply())
        // .map(|reply| {warp::reply::with_header(reply, "location", format!("/workspaces/{}", workspace.slug()),)})
    }

    async fn show(slug: String) -> Result<impl warp::Reply, warp::Rejection> {
        let workspace = handlers::show_workspace(&slug).await?;

        Ok(warp::reply::json(&workspace))
    }

    async fn list() -> Result<impl warp::Reply, warp::Rejection> {
        let workspaces = handlers::list_workspaces().await?;

        Ok(warp::reply::json(&workspaces))
    }

    async fn delete(slug: String) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::remove_workspace(&slug).await?;

        Ok(warp::reply())
    }

    async fn update(
        slug: String,
        workspace: WorkspaceRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::update_workspace(&slug, workspace).await?;

        // FIXME: Set location header
        Ok(warp::reply())
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("workspaces")
            .and(warp::get())
            .and(warp::path::end())
            .and_then(list)
            .or(warp::path("workspaces")
                .and(warp::post())
                .and(warp::path::end())
                .and(warp::body::json())
                .and_then(create)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED)))
            .or(warp::path!("workspaces" / String)
                .and(warp::get())
                .and_then(show))
            .or(warp::path!("workspaces" / String)
                .and(warp::delete())
                .and_then(delete)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
            .or(warp::path!("workspaces" / String)
                .and(warp::put())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}
