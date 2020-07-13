use ncube_crypto::AuthError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HostError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("General host error: {0}")]
    General(String),
    #[error("Authentication failed: {0}")]
    AuthError(String),
}

impl warp::reject::Reject for HostError {}

impl From<HostError> for warp::Rejection {
    fn from(rejection: HostError) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}

impl From<AuthError> for HostError {
    fn from(e: AuthError) -> HostError {
        HostError::AuthError(e.to_string())
    }
}
