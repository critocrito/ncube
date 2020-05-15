use chrono::Utc;
use tracing::{error, instrument};

use crate::actors::{
    db::{DatabaseActor, LookupDatabase},
    host::{HostActor, WorkspaceExists},
};
use crate::db::sqlite;
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{SourceStore, SourceStoreSqlite};
use crate::types::SourceRequest;

#[instrument]
pub async fn create_source(workspace: &str, source: SourceRequest) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    if let Ok(false) = host_actor
        .call(WorkspaceExists {
            slug: workspace.into(),
        })
        .await?
    {
        let msg = format!("Workspace `{}` doesn't exist.", workspace);
        error!("{:?}", msg);
        return Err(HandlerError::Invalid(msg));
    };

    let mut database_actor = DatabaseActor::from_registry().await.unwrap();

    let database = database_actor
        .call(LookupDatabase {
            workspace: workspace.into(),
        })
        .await??;

    match database {
        sqlite::Database { .. } => {
            let mut store = SourceStoreSqlite {};
            let now = Utc::now();
            store
                .create(database, &source.kind, &source.term, &now.to_rfc3339())
                .await?;
        }
    }

    Ok(())
}
