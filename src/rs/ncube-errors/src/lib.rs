use thiserror::Error;

#[derive(Debug, Error)]
pub enum HostError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("General host error: {0}")]
    General(String),
    #[error("Authentication failed.")]
    AuthError,
}

impl warp::reject::Reject for HostError {}

impl From<HostError> for warp::Rejection {
    fn from(rejection: HostError) -> warp::Rejection {
        warp::reject::custom(rejection)
    }
}
