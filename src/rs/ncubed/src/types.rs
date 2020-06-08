pub use ncube_data::{AnnotationKind, WorkspaceKind};
use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "database")]
pub enum DatabaseRequest {
    Sqlite,
    Http,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum WorkspaceKindRequest {
    Local,
    Remote { endpoint: String },
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub kind: WorkspaceKindRequest,
    #[serde(flatten)]
    pub database: DatabaseRequest,
}

impl WorkspaceRequest {
    pub fn slug(&self) -> String {
        slugify!(&self.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub term: String,
    pub annotations: Vec<AnnotationKind>,
}

#[derive(Debug)]
pub struct AccountRequest {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub type UpdatePasswordRequest = LoginRequest;

#[derive(Debug, Serialize)]
pub struct JwtToken {
    pub token: String,
}

#[derive(Debug, Default)]
pub struct ReqCtx {
    pub is_local: bool,
    pub is_authorized: bool,
    pub email: Option<String>,
    pub workspace: Option<String>,
}
