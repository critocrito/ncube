use ncubed::{
    crypto, handlers,
    types::{AccountRequest, DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest},
};
use rand;

use crate::fatal;

pub(crate) async fn account(workspace: &str, email: &str) {
    let rng = rand::thread_rng();
    let password = crypto::mkpass(rng);

    handlers::account::create_account(
        workspace,
        AccountRequest {
            email: email.to_string(),
            name: None,
            password: password.clone(),
        },
    )
    .await
    .unwrap_or_else(|e| fatal!("cannot create account: {}", e.to_string()));

    print!("{}", password);
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
