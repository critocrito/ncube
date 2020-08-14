use ncube_data::VerifySegmentReq;
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

    investigation_store
        .verify_segment(&investigation, &segment_req.segment)
        .await?;

    Ok(())
}
