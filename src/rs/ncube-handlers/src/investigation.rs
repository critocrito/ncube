use ncube_data::{Annotation, AnnotationReq, Segment, SegmentUnit, VerifySegmentReq};
use ncube_stores::{annotation_store, investigation_store};
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

#[instrument]
pub async fn update_unit_state(
    workspace: &str,
    investigation: &str,
    segment: &str,
    unit: i32,
    state: &serde_json::Value,
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

    // The update just doesn't happen if the unit doesn't exist (fails the where
    // clause).
    investigation_store
        .update_unit_state(&investigation, &segment, unit, &state)
        .await?;

    Ok(())
}

#[instrument]
pub async fn set_annotation(
    workspace: &str,
    investigation: &str,
    verification: i32,
    annotation_req: &AnnotationReq,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    // FIXME: verify this verification exists for this workspace and investigation

    let database = workspace_database(&workspace).await?;
    let annotation_store = annotation_store(database.clone());

    annotation_store
        .create(
            &annotation_req.key,
            &annotation_req.value,
            &annotation_req.name,
            &annotation_req.note,
            &investigation,
            verification,
        )
        .await?;

    Ok(())
}

#[instrument]
pub async fn list_annotations(
    workspace: &str,
    investigation: &str,
    verification: i32,
) -> Result<Vec<Annotation>, HandlerError> {
    ensure_workspace(&workspace).await?;

    // FIXME: verify this verification exists for this workspace and investigation

    let database = workspace_database(&workspace).await?;
    let annotation_store = annotation_store(database.clone());

    let annotations = annotation_store.list(&investigation, verification).await?;

    Ok(annotations)
}
