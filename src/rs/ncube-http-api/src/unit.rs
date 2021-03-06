use futures::try_join;
use ncube_data::{ReqCtx, SearchResponse, SuccessResponse};
use ncube_handlers::{
    stat as stat_handlers, unit as unit_handlers, workspace as handlers, HandlerError,
};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tracing::instrument;
use warp::Filter;

use crate::http::{authenticate_remote_req, conditionals, send_file, Conditionals};

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
    let data =
        handlers::list_data(&workspace, opts.page.unwrap_or(0), opts.size.unwrap_or(20)).await?;
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
        stat_handlers::stat_data_total(&workspace, query)
    )?;

    let response = SuccessResponse::new(SearchResponse {
        data,
        total: total.value,
    });

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn download(
    _ctx: ReqCtx,
    workspace: String,
    unit_id: String,
    kind: String,
    file: String,
    file_conditionals: Conditionals,
) -> Result<impl warp::Reply, warp::Rejection> {
    let file_path = format!("{}/{}/{}", &unit_id, &kind, &file);
    let file = handlers::show_download(&workspace, &file_path).await?;
    let response = send_file(file, file_path.to_string(), file_conditionals).await?;

    Ok(response)
}

#[instrument]
async fn download_metadata(
    _ctx: ReqCtx,
    workspace: String,
    unit_id: String,
    kind: String,
    file: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let file_path = format!("{}/{}/{}", &unit_id, &kind, &file);
    let data = handlers::show_download_meta(&workspace, &file_path).await?;

    let response = SuccessResponse::new(data);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn show(
    _ctx: ReqCtx,
    workspace: String,
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data = unit_handlers::show_data_unit(&workspace, id).await?;
    let response = SuccessResponse::new(data);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn data_by_ids(
    _ctx: ReqCtx,
    workspace: String,
    body: Vec<i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data = unit_handlers::list_data_ids(&workspace, body).await?;
    let response = SuccessResponse::new(data);

    Ok(warp::reply::json(&response))
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "data"))
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and_then(data)
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "data"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(data_by_ids))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "data" / "search"))
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and_then(search))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "data" / "units" / i32))
            .and(warp::get())
            .and_then(show))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "data" / String / String / String
            ))
            .and(warp::get())
            .and(conditionals())
            .and_then(download))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "data" / String / String / String / "meta"
            ))
            .and(warp::get())
            .and_then(download_metadata))
}
