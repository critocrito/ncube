use ncubed::{
    handlers,
    types::{AccountRequest, DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest},
};

use crate::fatal;

pub(crate) async fn account(workspace: &str, email: &str) {
    handlers::account::create_account(
        workspace,
        AccountRequest {
            email: email.to_string(),
            name: None,
        },
    )
    .await
    .unwrap_or_else(|e| fatal!("cannot create account: {}", e.to_string()));
}

pub(crate) async fn workspace(name: &str, database: DatabaseRequest) {
    let request = WorkspaceRequest {
        name: name.to_string(),
        kind: WorkspaceKindRequest::Local,
        description: None,
        database,
    };
    // FIXME: project directory get's created asynchronously, need to
    // wait for that.
    handlers::workspace::create_workspace(request)
        .await
        .unwrap_or_else(|e| fatal!("cannot create workspace: {}", e.to_string()));
}
