use ncube_data::{ErrorResponse, SuccessResponse};
use ncube_db::errors::DatabaseError;
use ncube_errors::HostError;
use ncube_handlers::HandlerError;
use std::convert::Infallible;
use tracing::{error, info, instrument};
use warp::{
    http::{Method, StatusCode},
    Filter,
};

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
        .or(warp::path("82b1a58ddf26951345dcec05fb4d7b5c.ttf")
            .map(|| {
                let file =
                    include_bytes!("../../../../target/dist/82b1a58ddf26951345dcec05fb4d7b5c.ttf");
                file.to_vec()
            })
            .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
        .or(warp::path("e962f548522aa99bb8f9c3505bcf56a9.ttf")
            .map(|| {
                let file =
                    include_bytes!("../../../../target/dist/e962f548522aa99bb8f9c3505bcf56a9.ttf");
                file.to_vec()
            })
            .map(|reply| warp::reply::with_header(reply, "content-type", "font/ttf")))
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
                .or(segment::routes()),
        )
        .with(cors)
}

pub(crate) mod config {
    use super::SuccessResponse;
    use ncube_data::ReqCtx;
    use ncube_handlers::config as handlers;
    use serde::Deserialize;
    use warp::Filter;

    use crate::http::restrict_to_local_req;

    #[derive(Debug, Deserialize)]
    struct SettingRequest {
        name: String,
        value: String,
    }

    async fn show(_ctx: ReqCtx) -> Result<impl warp::Reply, warp::Rejection> {
        let config = handlers::show_config().await?;
        let response = SuccessResponse::new(config);

        Ok(warp::reply::json(&response))
    }

    async fn create(
        _ctx: ReqCtx,
        settings: Vec<SettingRequest>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut config = vec![];
        for SettingRequest { name, value } in settings {
            config.push((name, value));
        }

        handlers::bootstrap(config).await?;

        Ok(warp::reply())
    }

    async fn update(
        _ctx: ReqCtx,
        settings: Vec<SettingRequest>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        for SettingRequest { name, value } in settings {
            handlers::insert_config_setting(&name, &value).await?;
        }

        Ok(warp::reply())
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path::end())
            .and(restrict_to_local_req())
            .and_then(show)
            .or(warp::post()
                .and(warp::path::end())
                .and(restrict_to_local_req())
                .and(warp::body::json())
                .and_then(create)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
                .map(|reply| warp::reply::with_header(reply, "location", "/")))
            .or(warp::put()
                .and(warp::path::end())
                .and(restrict_to_local_req())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}

pub(crate) mod workspace {
    use super::SuccessResponse;
    use ncube_data::{ReqCtx, WorkspaceRequest};
    use ncube_handlers::workspace as handlers;
    use warp::Filter;

    use crate::http::restrict_to_local_req;

    async fn create(
        _ctx: ReqCtx,
        workspace: WorkspaceRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let workspace = handlers::create_workspace(workspace).await?;
        let response = SuccessResponse::new(workspace);

        Ok(warp::reply::json(&response))
    }

    async fn show(_ctx: ReqCtx, slug: String) -> Result<impl warp::Reply, warp::Rejection> {
        let workspace = handlers::show_workspace(&slug).await?;
        let response = SuccessResponse::new(workspace);

        Ok(warp::reply::json(&response))
    }

    async fn list(_ctx: ReqCtx) -> Result<impl warp::Reply, warp::Rejection> {
        let workspaces = handlers::list_workspaces().await?;
        let response = SuccessResponse::new(workspaces);

        Ok(warp::reply::json(&response))
    }

    async fn delete(_ctx: ReqCtx, slug: String) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::remove_workspace(&slug).await?;

        Ok(warp::reply())
    }

    async fn update(
        _ctx: ReqCtx,
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
            .and(restrict_to_local_req())
            .and_then(list)
            .or(warp::path("workspaces")
                .and(warp::post())
                .and(warp::path::end())
                .and(restrict_to_local_req())
                .and(warp::body::json())
                .and_then(create)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED)))
            .or(warp::any()
                .and(restrict_to_local_req())
                .and(warp::path!("workspaces" / String))
                .and(warp::get())
                .and_then(show))
            .or(warp::any()
                .and(restrict_to_local_req())
                .and(warp::path!("workspaces" / String))
                .and(warp::delete())
                .and_then(delete)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
            .or(warp::any()
                .and(restrict_to_local_req())
                .and(warp::path!("workspaces" / String))
                .and(warp::put())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}

pub(crate) mod source {
    use super::SuccessResponse;
    use futures::try_join;
    use ncube_data::{ReqCtx, SearchResponse, SourceRequest};
    use ncube_handlers::{source as handlers, workspace as workspace_handlers, HandlerError};
    use percent_encoding::percent_decode_str;
    use serde::Deserialize;
    use tracing::instrument;
    use warp::Filter;

    use crate::http::authenticate_remote_req;

    // The query parameters for list source.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub page: Option<i32>,
        pub size: Option<i32>,
        pub q: Option<String>,
    }

    #[instrument]
    async fn create(
        _ctx: ReqCtx,
        workspace_slug: String,
        source: SourceRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::create_source(&workspace_slug, source).await?;

        // FIXME: Set location header
        Ok(warp::reply())
    }

    #[instrument]
    async fn show(
        _ctx: ReqCtx,
        workspace_slug: String,
        id: i32,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let source = handlers::show_source(&workspace_slug, id).await?;
        match source {
            Some(value) => {
                let response = SuccessResponse::new(value);
                Ok(warp::reply::json(&response))
            }
            None => Err(HandlerError::NotFound(format!("Source {} not found.", id)))?,
        }
    }

    #[instrument]
    async fn list(
        _ctx: ReqCtx,
        workspace_slug: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let sources = handlers::list_sources(
            &workspace_slug,
            opts.page.unwrap_or(0),
            opts.size.unwrap_or(20),
        )
        .await?;
        let response = SuccessResponse::new(sources);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn search(
        _ctx: ReqCtx,
        workspace: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query = opts
            .q
            .map(|q| percent_decode_str(&q).decode_utf8_lossy().to_string());

        if let None = query {
            return Err(HandlerError::Invalid(
                "search requires a query parameter".into(),
            ))?;
        }

        let query_str = query.clone().unwrap();

        let (data, total) = try_join!(
            handlers::search_sources(
                &workspace,
                &query_str,
                opts.page.unwrap_or(0),
                opts.size.unwrap_or(20),
            ),
            workspace_handlers::stat_sources_total(&workspace, query)
        )?;

        let response = SuccessResponse::new(SearchResponse {
            data,
            total: total.value,
        });

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn remove(
        _ctx: ReqCtx,
        workspace_slug: String,
        id: i32,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::remove_source(&workspace_slug, id).await?;

        Ok(warp::reply())
    }

    #[instrument]
    async fn update(
        _ctx: ReqCtx,
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
        authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "sources"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "sources" / i32))
                .and(warp::get())
                .and_then(show))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "sources"))
                .and(warp::get())
                .and(warp::query::<ListOptions>())
                .and_then(list))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "sources" / "search"))
                .and(warp::get())
                .and(warp::query::<ListOptions>())
                .and_then(search))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "sources" / i32))
                .and(warp::delete())
                .and_then(remove)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "sources" / i32))
                .and(warp::put())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}

pub(crate) mod user {
    use super::SuccessResponse;
    use ncube_data::{LoginRequest, ReqCtx, UpdatePasswordRequest};
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

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
}

pub(crate) mod source_tag {
    use super::SuccessResponse;
    use ncube_data::ReqCtx;
    use ncube_handlers::source as handlers;
    use tracing::instrument;
    use warp::Filter;

    use crate::http::authenticate_remote_req;

    #[instrument]
    async fn list(
        _ctx: ReqCtx,
        workspace_slug: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let sources = handlers::list_source_tags(&workspace_slug).await?;
        let response = SuccessResponse::new(sources);

        Ok(warp::reply::json(&response))
    }

    // FIXME: turn this into /workspaces/<workspace_id>/sources/tags endpoint
    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "source-tags"))
            .and(warp::get())
            .and_then(list)
    }
}

pub(crate) mod segment {
    use ncube_data::{ReqCtx, SegmentRequest, SuccessResponse};
    use ncube_handlers::workspace as handlers;
    use tracing::instrument;
    use warp::Filter;

    use crate::http::authenticate_remote_req;

    #[instrument]
    async fn create(
        _ctx: ReqCtx,
        workspace: String,
        segment_req: SegmentRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::create_segment(&workspace, &segment_req).await?;

        Ok(warp::reply())
    }

    #[instrument]
    async fn show(
        _ctx: ReqCtx,
        workspace: String,
        segment: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let segment = handlers::show_segment(&workspace, &segment).await?;
        let response = SuccessResponse::new(segment);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
        let segments = handlers::list_segments(&workspace).await?;
        let response = SuccessResponse::new(segments);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn update(
        _ctx: ReqCtx,
        workspace: String,
        segment: String,
        segment_req: SegmentRequest,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::update_segment(&workspace, &segment, &segment_req).await?;

        Ok(warp::reply())
    }

    #[instrument]
    async fn remove(
        _ctx: ReqCtx,
        workspace: String,
        segment: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        handlers::remove_segment(&workspace, &segment).await?;

        Ok(warp::reply())
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "segments"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(create)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "segments"))
                .and(warp::get())
                .and_then(list))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "segments" / String))
                .and(warp::get())
                .and_then(show))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "segments" / String))
                .and(warp::put())
                .and(warp::body::json())
                .and_then(update)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "segments" / String))
                .and(warp::delete())
                .and_then(remove)
                .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
    }
}

pub(crate) mod stat {
    use super::SuccessResponse;
    use ncube_data::ReqCtx;
    use ncube_handlers::workspace as handlers;
    use percent_encoding::percent_decode_str;
    use serde::Deserialize;
    use tracing::instrument;
    use warp::Filter;

    use crate::http::authenticate_remote_req;

    // The query parameters for stats.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub q: Option<String>,
    }

    #[instrument]
    async fn sources_total(
        _ctx: ReqCtx,
        workspace: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query = opts
            .q
            .map(|query| percent_decode_str(&query).decode_utf8_lossy().to_string());

        let stat = handlers::stat_sources_total(&workspace, query).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn sources_types(
        _ctx: ReqCtx,
        workspace: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let stat = handlers::stat_sources_types(&workspace).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn data_total(
        _ctx: ReqCtx,
        workspace: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query = opts
            .q
            .map(|query| percent_decode_str(&query).decode_utf8_lossy().to_string());

        let stat = handlers::stat_data_total(&workspace, query).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn data_sources(
        _ctx: ReqCtx,
        workspace: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let stat = handlers::stat_data_sources(&workspace).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn data_videos(
        _ctx: ReqCtx,
        workspace: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let stat = handlers::stat_data_videos(&workspace).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn data_segments(
        _ctx: ReqCtx,
        workspace: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let stat = handlers::stat_data_segments(&workspace).await?;
        let response = SuccessResponse::new(stat.value);

        Ok(warp::reply::json(&response))
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "stats" / "sources" / "total"
            ))
            .and(warp::query::<ListOptions>())
            .and(warp::get())
            .and_then(sources_total)
            .or(authenticate_remote_req()
                .and(warp::path!(
                    "workspaces" / String / "stats" / "sources" / "types"
                ))
                .and(warp::get())
                .and_then(sources_types))
            .or(authenticate_remote_req()
                .and(warp::path!(
                    "workspaces" / String / "stats" / "data" / "total"
                ))
                .and(warp::query::<ListOptions>())
                .and(warp::get())
                .and_then(data_total))
            .or(authenticate_remote_req()
                .and(warp::path!(
                    "workspaces" / String / "stats" / "data" / "sources"
                ))
                .and(warp::get())
                .and_then(data_sources))
            .or(authenticate_remote_req()
                .and(warp::path!(
                    "workspaces" / String / "stats" / "data" / "videos"
                ))
                .and(warp::get())
                .and_then(data_videos))
            .or(authenticate_remote_req()
                .and(warp::path!(
                    "workspaces" / String / "stats" / "data" / "segments"
                ))
                .and(warp::get())
                .and_then(data_segments))
    }
}

pub(crate) mod unit {
    use super::SuccessResponse;
    use futures::try_join;
    use ncube_data::{ReqCtx, SearchResponse};
    use ncube_handlers::{workspace as handlers, HandlerError};
    use percent_encoding::percent_decode_str;
    use serde::Deserialize;
    use tracing::instrument;
    use warp::Filter;

    use crate::http::authenticate_remote_req;

    // The query parameters for list data.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub page: Option<i32>,
        pub size: Option<i32>,
        pub q: Option<String>,
    }

    #[instrument]
    async fn data(
        _ctx: ReqCtx,
        workspace: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let data = handlers::list_data(&workspace, opts.page.unwrap_or(0), opts.size.unwrap_or(20))
            .await?;
        let response = SuccessResponse::new(data);

        Ok(warp::reply::json(&response))
    }

    #[instrument]
    async fn search(
        _ctx: ReqCtx,
        workspace: String,
        opts: ListOptions,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query = opts
            .q
            .map(|query| percent_decode_str(&query).decode_utf8_lossy().to_string());

        if let None = query {
            return Err(HandlerError::Invalid(
                "search requires a query parameter".into(),
            ))?;
        }

        let query_str = query.clone().unwrap();

        let (data, total) = try_join!(
            handlers::search_data(
                &workspace,
                &query_str,
                opts.page.unwrap_or(0),
                opts.size.unwrap_or(20),
            ),
            handlers::stat_data_total(&workspace, query)
        )?;

        let response = SuccessResponse::new(SearchResponse {
            data,
            total: total.value,
        });

        Ok(warp::reply::json(&response))
    }

    pub(crate) fn routes(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "data"))
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and_then(data)
            .or(authenticate_remote_req()
                .and(warp::path!("workspaces" / String / "data" / "search"))
                .and(warp::get())
                .and(warp::query::<ListOptions>())
                .and_then(search))
    }
}
