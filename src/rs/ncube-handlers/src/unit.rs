use ncube_data::Unit;
use ncube_stores::unit_store;
use tracing::instrument;

use crate::{ensure_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn show_data_unit(workspace: &str, id: i32) -> Result<Option<Unit>, HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;
    let unit_store = unit_store(database);
    let unit = unit_store.show(id).await?;

    Ok(unit)
}
