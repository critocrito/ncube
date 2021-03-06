use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ConnectionOut {
    pub(crate) name: String,
    pub(crate) workspace: String,
    pub(crate) endpoint: String,
    pub(crate) description: Option<String>,
    pub(crate) email: String,
    pub(crate) otp: Option<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}
