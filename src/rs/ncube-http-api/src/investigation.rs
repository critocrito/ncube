use ncube_data::{AnnotationReq, InvestigationReq, ReqCtx, SuccessResponse, VerifySegmentReq};
use ncube_handlers::{investigation as investigation_handlers, workspace as handlers};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[derive(Debug, Deserialize)]
pub struct UnitsOptions {
    pub state: Option<String>,
}

#[instrument]
async fn create(
    _ctx: ReqCtx,
    workspace: String,
    investigation_req: InvestigationReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::create_investigation(&workspace, &investigation_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn show(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let investigation = handlers::show_investigation(&workspace, &investigation).await?;
    let response = SuccessResponse::new(investigation);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn list(_ctx: ReqCtx, workspace: String) -> Result<impl warp::Reply, warp::Rejection> {
    let investigations = handlers::list_investigations(&workspace).await?;
    let response = SuccessResponse::new(investigations);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn remove(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

#[instrument]
async fn verify_segment(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    verify_segment_req: VerifySegmentReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    investigation_handlers::verify_segment(&workspace, &investigation, &verify_segment_req).await?;

    Ok(warp::reply())
}

#[instrument]
async fn list_segments(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let segments = investigation_handlers::list_segments(&workspace, &investigation).await?;
    let response = SuccessResponse::new(segments);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn list_units(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    segment: String,
    opts: UnitsOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let state = opts
        .state
        .map(|value| percent_decode_str(&value).decode_utf8_lossy().to_string());

    let segments =
        investigation_handlers::list_units(&workspace, &investigation, &segment, state).await?;
    let response = SuccessResponse::new(segments);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn update_state(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    segment: String,
    unit: i32,
    state: serde_json::Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    investigation_handlers::update_unit_state(&workspace, &investigation, &segment, unit, &state)
        .await?;

    Ok(warp::reply())
}

#[instrument]
async fn create_annotation(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    verification: i32,
    annotation_req: AnnotationReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    investigation_handlers::set_annotation(
        &workspace,
        &investigation,
        verification,
        &annotation_req,
    )
    .await?;

    Ok(warp::reply())
}

#[instrument]
async fn list_annotations(
    _ctx: ReqCtx,
    workspace: String,
    investigation: String,
    verification: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let annotations =
        investigation_handlers::list_annotations(&workspace, &investigation, verification).await?;

    let response = SuccessResponse::new(annotations);

    Ok(warp::reply::json(&response))
}

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "investigations"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED))
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "investigations"))
            .and(warp::get())
            .and_then(list))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::get())
            .and_then(show))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::delete())
            .and_then(remove)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String
            ))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(verify_segment)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::CREATED)))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String / "segments"
            ))
            .and(warp::get())
            .and_then(list_segments))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String / "segments" / String
            ))
            .and(warp::query::<UnitsOptions>())
            .and(warp::get())
            .and_then(list_units))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String / "segments" / String / i32
            ))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(update_state)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String / "annotations" / i32
            ))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(create_annotation)
            .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT)))
        .or(authenticate_remote_req()
            .and(warp::path!(
                "workspaces" / String / "investigations" / String / "annotations" / i32
            ))
            .and(warp::get())
            .and_then(list_annotations))
}
