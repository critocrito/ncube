use futures::try_join;
use ncube_data::{ReqCtx, SearchResponse, SourceRequest, SuccessResponse};
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

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
