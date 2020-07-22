use chrono::{DateTime, Duration, Utc};
use ncube_crypto as crypto;
use ncube_data::{Account, JwtToken, WorkspaceKind};
use ncube_stores::{account_store, workspace_store, WorkspaceStore};
use tracing::{debug, error, info, instrument, warn};

use crate::actors::{
    db::{LookupDatabase, ResetDatabase},
    host::{RequirePool, SecretKeySetting},
    DatabaseActor, HostActor, Registry,
};
use crate::errors::HandlerError;

// This function sets the OTP max age policy. At this time this is set to 48
// hours.
pub fn is_valid_otp(ts: DateTime<Utc>) -> bool {
    let now = Utc::now();
    let otp_max_age = now - Duration::hours(48);

    otp_max_age <= ts
}

/// Creating an account works differently whether a workspace is local or
/// remote. A remote workspace is associated with one local account. This local
/// account only needs to store the AES encrypted password and requires nothing
/// else. In order to create such an account a AES encrypted OTP password has to
/// be provided.
///
/// This local account is mapped to a local account on the remote server, where
/// the workspace is regarded as local. These accounts are created using the the
/// `ncubectl` command and are created with `None` as value to the `otp`
/// argument. In this case a otp password and AES key generated for the account.
#[instrument]
pub async fn create_account(
    workspace: &str,
    email: &str,
    otp: Option<String>,
) -> Result<Account, HandlerError> {
    let mut host_actor = HostActor::from_registry().await.unwrap();

    let db = host_actor.call(RequirePool).await??;
    let workspace_store = workspace_store(db.clone());
    let account_store = account_store(db);
    let workspace = workspace_store.show_by_slug(&workspace).await?;

    if account_store.exists(&email, &workspace).await? {
        return Err(HandlerError::Invalid(
            "This email already exists for this workspace.".into(),
        ));
    }

    account_store.create(&email, otp, &workspace).await?;
    let account = account_store.show(&email, &workspace).await?;

    Ok(account)
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
    let account = account_store.show(&email, &workspace).await?;

    // The account must have a still valid OTP password in case it is an active
    // OTP account.
    if account.is_otp && !is_valid_otp(account.updated_at) {
        warn!("otp expired for {}", workspace.slug);
        return Err(HandlerError::NotAllowed("login failed".into()));
    }

    let hash = account_store
        .show_password(&email, &workspace)
        .await
        .map_err(|e| {
            error!("login failed to fetch password: {:?}", e.to_string());
            HandlerError::NotAllowed("login failed".into())
        })?;

    let key = account_store
        .show_key(&email, &workspace)
        .await
        .map_err(|e| {
            error!("login failed to fetch key: {:?}", e.to_string());
            HandlerError::NotAllowed("login failed".into())
        })?;

    let decrypted_password = crypto::aes_decrypt(key, password).map_err(|e| {
        error!("failed to decrypt password: {:?}", e.to_string());
        HandlerError::NotAllowed("login failed".into())
    })?;

    let is_verified = crypto::verify(&hash, &decrypted_password);

    if !is_verified {
        error!("password verification failed");
    }

    Ok(is_verified)
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
    let store = account_store(db);

    let workspace = workspace_store.show_by_slug(&workspace).await?;
    let account = store.show(&email, &workspace).await?;

    // The account must have a still valid OTP password in case it is an active
    // OTP account.
    if account.is_otp && !is_valid_otp(account.updated_at) {
        error!("otp expired for {:?}", workspace.slug);
        return Err(HandlerError::NotAllowed("update failed".into()));
    }

    // Passwords must match.
    if password != password_again {
        error!("passwords don't match for {:?}", workspace.slug);
        return Err(HandlerError::NotAllowed("update failed".into()));
    }

    // If the workspace is a remote one, we need to do the actual update
    // remotely and then update the local database. The full update process is
    // described in detail in the auth-workflow guide in the docs directory.
    let new_hash = match &workspace.kind {
        WorkspaceKind::Local(..) => {
            debug!("updating local password ({:?}/{:?})", workspace.slug, email);
            store.update_password(&email, &password, &workspace).await?
        }
        WorkspaceKind::Remote(..) => {
            debug!(
                "updating remote password ({:?}/{:?})",
                workspace.slug, email
            );

            let mut db_actor = DatabaseActor::from_registry().await.unwrap();
            let mut remote_db = db_actor
                .call(LookupDatabase {
                    workspace: workspace.slug.to_string(),
                })
                .await??;

            // Make sure we can login.
            remote_db.login().await?;
            let remote_store = account_store(remote_db.clone());

            // Update the password remotely and receive an AES encrypted hash to
            // store locally.
            let new_hash = remote_store
                .update_password(&email, &password, &workspace)
                .await?;

            // Store the AES encrypted hash in the local database. This hash
            // will be send when doing a login.
            store
                .update_hashed_password(&email, &new_hash, &workspace)
                .await?;

            // We make sure to replace the database in the cache to accept the new password.
            db_actor
                .call(ResetDatabase {
                    workspace: workspace.slug.to_string(),
                })
                .await??;

            new_hash
        }
    };

    Ok(new_hash)
}

#[instrument]
pub async fn issue_token(
    workspace: &str,
    email: &str,
    password: &str,
) -> Result<JwtToken, HandlerError> {
    let logged_in = login_account(&workspace, &email, &password).await?;

    info!("{:?} -> {:?}/{:?}", workspace, email, password);

    if !logged_in {
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
    let account = account_store.show(&email, &workspace).await?;

    Ok(account)
}
