use ncube_data::{Segment, SegmentUnit, VerifySegmentReq};
use ncube_stores::investigation_store;
use tracing::instrument;

use crate::{ensure_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn verify_segment(
    workspace: &str,
    investigation: &str,
    segment_req: &VerifySegmentReq,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let investigation_store = investigation_store(database.clone());

    if let None = investigation_store.show(&investigation).await? {
        return Err(HandlerError::NotFound(format!(
            "Investigation '{}' could not be found.",
            investigation
        )));
    };

    investigation_store
        .verify_segment(&investigation, &segment_req.segment)
        .await?;

    Ok(())
}

#[instrument]
pub async fn list_segments(
    workspace: &str,
    investigation: &str,
) -> Result<Vec<Segment>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let investigation_store = investigation_store(database.clone());

    if let None = investigation_store.show(&investigation).await? {
        return Err(HandlerError::NotFound(format!(
            "Investigation '{}' could not be found.",
            investigation
        )));
    };

    let segments = investigation_store.segments(&investigation).await?;

    Ok(segments)
}

#[instrument]
pub async fn list_units(
    workspace: &str,
    investigation: &str,
    segment: &str,
    state: Option<String>,
) -> Result<Vec<SegmentUnit>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let investigation_store = investigation_store(database.clone());

    if let None = investigation_store.show(&investigation).await? {
        return Err(HandlerError::NotFound(format!(
            "Investigation '{}' could not be found.",
            investigation
        )));
    };

    let units = match state {
        Some(state) => {
            investigation_store
                .units_by_state(&investigation, &segment, &state)
                .await?
        }
        _ => investigation_store.units(&investigation, &segment).await?,
    };

    Ok(units)
}
