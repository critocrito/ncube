use chrono::{DateTime, Duration, Utc};
use ncube_data::Account;
use tracing::instrument;

use crate::actors::{
    host::{HostActor, RequirePool, SecretKeySetting},
    Registry,
};
use crate::crypto;
use crate::errors::HandlerError;
use crate::stores::{account_store, workspace_store, AccountStore, WorkspaceStore};
use crate::types::{AccountRequest, JwtToken};

// This function sets the OTP max age policy. At this time this is set to 48
// hours.
pub fn is_valid_otp(ts: DateTime<Utc>) -> bool {
    let now = Utc::now();
    let otp_max_age = now - Duration::hours(48);

    otp_max_age <= ts
}

#[instrument]
pub async fn create_account(
    workspace: &str,
    account_request: AccountRequest,
) -> Result<(), HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);
    let workspace = workspace_store.show_by_slug(&workspace).await?;

    let AccountRequest { email, .. } = account_request;

    if account_store.exists(&email, workspace.id).await? {
        return Err(HandlerError::Invalid(
            "This email already exists for this workspace.".into(),
        ));
    }

    let password = crypto::gen_secret_key(rand::thread_rng());
    let hash = crypto::hash(rand::thread_rng(), password.as_bytes());
    let key = crypto::gen_symmetric_key(rand::thread_rng());
    let otp = crypto::aes_encrypt(rand::thread_rng(), &key, &password.as_bytes().to_vec());

    account_store
        .create(&email, &hash, &otp, key, None, workspace.id)
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
    let account = account_store.show(&email, workspace.id).await?;

    // The account must have a still valid OTP password in case it is an active
    // OTP account.
    if account.is_otp && !is_valid_otp(account.updated_at) {
        return Err(HandlerError::NotAllowed("login failed".into()));
    }

    let hash = account_store
        .show_password(&email, workspace.id)
        .await?
        .ok_or_else(|| HandlerError::NotAllowed("login failed".into()))?;

    let key = account_store
        .show_key(&email, workspace.id)
        .await
        .map_err(|_| HandlerError::NotAllowed("login failed".into()))?;

    let decrypted_password = crypto::aes_decrypt(key, password)
        .map_err(|_| HandlerError::NotAllowed("login failed".into()))?;

    Ok(crypto::verify(&hash, &decrypted_password))
}

#[instrument]
pub async fn update_password(
    workspace: &str,
    email: &str,
    password: &str,
    password_again: &str,
) -> Result<String, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;

    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;
    let account = account_store.show(&email, workspace.id).await?;

    // The account must have a still valid OTP password in case it is an active
    // OTP account.
    if account.is_otp && !is_valid_otp(account.updated_at) {
        return Err(HandlerError::NotAllowed("update failed".into()));
    }

    // Passwords must match.
    if password != password_again {
        return Err(HandlerError::NotAllowed("update failed".into()));
    }

    let hash = crypto::hash(rand::thread_rng(), password.as_bytes());
    let key = crypto::gen_symmetric_key(rand::thread_rng());

    account_store
        .update_password(&email, &hash, &key, workspace.id)
        .await
        .map_err(|_| HandlerError::NotAllowed("update failed".into()))?;

    let new_password = crypto::aes_encrypt(rand::thread_rng(), &key, &password.as_bytes().to_vec());

    Ok(new_password)
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
    let key = host_actor.call(SecretKeySetting).await??;
    let value = key
        .value
        .ok_or_else(|| HandlerError::Invalid("signing failed".into()))?;
    let token = crypto::jwt_sign(&value, &email, &workspace)
        .map_err(|_| HandlerError::Invalid("signing failed".into()))?;

    Ok(JwtToken { token })
}

#[instrument]
pub async fn show_account(workspace: &str, email: &str) -> Result<Account, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;
    let accounts = account_store.show(&email, workspace.id).await?;

    Ok(accounts)
}
