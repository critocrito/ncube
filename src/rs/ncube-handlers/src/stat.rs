use ncube_data::Stat;
use ncube_stores::stat_store;

use crate::{ensure_workspace, workspace_database, HandlerError};

pub async fn stat_investigation_segments(
    workspace: &str,
    investigation: &str,
) -> Result<Stat, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let stat_store = stat_store(database);
    let stats = stat_store
        .investigation_segments_total(&investigation)
        .await?;

    Ok(stats)
}
