use ncube_data::Account;
use rand;
use tracing::instrument;

use crate::actors::host::{HostActor, RequirePool, ShowSecretKey};
use crate::crypto;
use crate::errors::HandlerError;
use crate::registry::Registry;
use crate::stores::{account_store, workspace_store, AccountStore, WorkspaceStore};
use crate::types::{AccountRequest, JwtToken};

#[instrument]
pub async fn create_account(
    workspace: &str,
    account_request: AccountRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);

    let AccountRequest {
        email, password, ..
    } = account_request;
    let hash = crypto::hash(rand::thread_rng(), password.as_bytes());

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

    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let hash = account_store
        .show_password(&email, workspace.id)
        .await?
        .ok_or_else(|| HandlerError::NotAllowed("login failed".into()))?;

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

    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let hash = crypto::hash(rand::thread_rng(), password.as_bytes());

    account_store
        .update_password(&email, &hash, workspace.id)
        .await?;

    Ok(())
}

#[instrument]
pub async fn issue_token(
    workspace: &str,
    email: &str,
    password: &str,
) -> Result<JwtToken, HandlerError> {
    let logged_in = login_account(&workspace, &email, &password).await?;

    if !logged_in {
        // FIXME: This results in 403 Forbidden HTTP response, do I want a 410
        // Unauthorized instead?
        return Err(HandlerError::NotAllowed("login failed".into()));
    }

    let mut host_actor = HostActor::from_registry().await.unwrap();
    let key = host_actor.call(ShowSecretKey).await??;

    let token = crypto::jwt_sign(&key.value, &email, &workspace)
        .map_err(|_| HandlerError::Invalid("signing failed".into()))?;

    Ok(JwtToken { token })
}
