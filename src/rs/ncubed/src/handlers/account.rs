use ncube_data::Account;
use tracing::{error, instrument};

use crate::actors::host::{CreateAccount, HostActor, ListAccounts, WorkspaceExists};
use crate::crypto;
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::types::AccountRequest;

#[instrument]
pub async fn create_account(
    workspace: &str,
    account_request: AccountRequest,
) -> Result<(), HandlerError> {
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

    let password = crypto::hash(account_request.password.as_bytes());

    let AccountRequest { email, .. } = account_request;
    let name = "name must change".to_string();

    host_actor
        .call(CreateAccount {
            workspace: workspace.into(),
            name: Some(name),
            otp: account_request.password,
            email,
            password,
        })
        .await??;

    Ok(())
}

#[instrument]
pub async fn list_accounts() -> Result<Vec<Account>, HandlerError> {
    let mut actor = HostActor::from_registry().await.unwrap();

    let accounts = actor.call(ListAccounts).await??;

    Ok(accounts)
}
