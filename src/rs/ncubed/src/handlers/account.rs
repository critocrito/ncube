use ncube_data::Account;
use tracing::instrument;

use crate::actors::host::{HostActor, RequirePool};
use crate::crypto;
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{account_store, workspace_store2, AccountStore, WorkspaceStore};
use crate::types::AccountRequest;

#[instrument]
pub async fn create_account(
    workspace: &str,
    account_request: AccountRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store2(db.clone());
    let account_store = account_store(db);

    let AccountRequest {
        email, password, ..
    } = account_request;
    let hash = crypto::hash(password.as_bytes());

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    if account_store.exists(&email, workspace.id).await? {
        return Err(HandlerError::Invalid(
            "This email already exists for this workspace.".into(),
        ));
    }

    // FIXME: I'm not setting a name for the account yet.
    account_store
        .create(&email, &hash, &password, None, workspace.id)
        .await?;

    Ok(())
}

#[instrument]
pub async fn list_accounts() -> Result<Vec<Account>, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let store = account_store(db);

    let accounts = store.list().await?;

    Ok(accounts)
}

#[instrument]
pub async fn login_account(
    workspace: &str,
    email: &str,
    password: &str,
) -> Result<bool, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;

    let workspace_store = workspace_store2(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let hash = account_store
        .show_password(&email, workspace.id)
        .await?
        .ok_or_else(|| HandlerError::NotFound("No password found.".into()))?;

    Ok(crypto::verify(&hash, password.as_bytes()))
}

#[instrument]
pub async fn update_password(
    workspace: &str,
    email: &str,
    password: &str,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;

    let workspace_store = workspace_store2(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let hash = crypto::hash(password.as_bytes());

    account_store
        .update_password(&email, &hash, workspace.id)
        .await?;

    Ok(())
}
