use serde::{Serialize, Serializer};
use std::convert::Infallible;
use tracing::{error, info, instrument};
use warp::{
    http::{Method, StatusCode},
    Filter,
};

use crate::errors::HandlerError;

#[derive(Debug)]
struct SuccessStatus;

impl Serialize for SuccessStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("success")
    }
}

#[derive(Debug)]
struct ErrorStatus;

impl Serialize for ErrorStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("error")
    }
}

#[derive(Debug, Serialize)]
struct SuccessResponse<T>
where
    T: Serialize,
{
    status: SuccessStatus,
    data: T,
}

impl<T> SuccessResponse<T>
where
    T: Serialize,
{
    fn new(data: T) -> Self {
        Self {
            status: SuccessStatus,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: ErrorStatus,
    code: u16,
    errors: String,
}

impl ErrorResponse {
    fn new(code: StatusCode, errors: &str) -> Self {
        Self {
            status: ErrorStatus,
            code: code.as_u16(),
            errors: errors.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_response_envelope() {
        let response = ErrorResponse::new(StatusCode::BAD_REQUEST, "I am an error!");

        let expected = "{\"status\":\"error\",\"code\":400,\"errors\":\"I am an error!\"}";
        let result = serde_json::to_string(&response).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn http_success_response_envelope() {
        let response = SuccessResponse::new("I am data!");

        let expected = "{\"status\":\"success\",\"data\":\"I am data!\"}";
        let result = serde_json::to_string(&response).unwrap();

        assert_eq!(result, expected);
    }
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
        .or(warp::path!("images" / "icon_data.svg")
            .map(|| include_str!("../../../../target/dist/images/icon_data.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
        .or(warp::path!("images" / "icon_help.svg")
            .map(|| include_str!("../../../../target/dist/images/icon_help.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
        .or(warp::path!("images" / "icon_investigation.svg")
            .map(|| include_str!("../../../../target/dist/images/icon_investigation.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
        .or(warp::path!("images" / "icon_process.svg")
            .map(|| include_str!("../../../../target/dist/images/icon_process.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
        .or(warp::path!("images" / "icon_query.svg")
            .map(|| include_str!("../../../../target/dist/images/icon_query.svg"))
            .map(|reply| warp::reply::with_header(reply, "content-type", "image/svg+xml")))
}

pub(crate) fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type"]);

    warp::path("api")
        .and(
            config::routes()
                .or(workspace::routes())
                .or(source::routes()),
        )
        .with(cors)
}

pub(crate) mod config {
    use super::SuccessResponse;
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
        let response = SuccessResponse::new(config);

        Ok(warp::reply::json(&response))
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
    use super::SuccessResponse;
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
        let response = SuccessResponse::new(workspace);

        Ok(warp::reply::json(&response))
    }

    async fn list() -> Result<impl warp::Reply, warp::Rejection> {
        let workspaces = handlers::list_workspaces().await?;
        let response = SuccessResponse::new(workspaces);

        Ok(warp::reply::json(&response))
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

pub(crate) mod source {
    use super::SuccessResponse;
    use tracing::instrument;
    use warp::Filter;

    use crate::handlers::source as handlers;
    use crate::types::SourceRequest;

    #[instrument]
    async fn create(
        workspace_slug: String,
        source: SourceRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::create_source(&workspace_slug, source).await?;

        // FIXME: Set location header
        Ok(warp::reply())
    }

    #[instrument]
    async fn list(workspace_slug: String) -> Result<impl warp::Reply, warp::Rejection> {
        let sources = handlers::list_sources(&workspace_slug).await?;
        let response = SuccessResponse::new(sources);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn remove(workspace_slug: String, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::remove_source(&workspace_slug, id).await?;

        Ok(warp::reply())
    }

    #[instrument]
    async fn update(
        workspace_slug: String,
        id: i32,
        source: SourceRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::update_source(&workspace_slug, id, &source).await?;

        // FIXME: Set location header
        Ok(warp::reply())
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("workspaces" / String / "sources")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .or(warp::path!("workspaces" / String / "sources")
                .and(warp::get())
                .and_then(list))
            .or(warp::path!("workspaces" / String / "sources" / i32)
                .and(warp::delete())
                .and_then(remove)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
            .or(warp::path!("workspaces" / String / "sources" / i32)
                .and(warp::put())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}
