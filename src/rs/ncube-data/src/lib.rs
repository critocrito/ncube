// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use chrono::prelude::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::default::Default;
use std::fmt::Debug;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Ncube;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigSetting {
    pub name: String,
    pub value: Option<String>,
    pub required: bool,
    pub restricted: bool,
    pub description: String,
}

pub type NcubeConfig = Vec<ConfigSetting>;

/// Ncube workspace databases can either be a Sqlite or PostgreSQL database.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::WorkspaceDatabase;
/// let db = WorkspaceDatabase::Sqlite { path: "path/to/file.db".into() };
/// assert_eq!(
///   "{\"database\":\"sqlite\",\"database_path\":\"path/to/file.db\"}",
///   serde_json::to_string(&db).unwrap()
/// );
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "database", rename_all = "lowercase")]
pub enum WorkspaceDatabase {
    Sqlite {
        #[serde(rename = "database_path")]
        path: String,
    },
    Http {
        #[serde(rename = "database_path")]
        path: String,
    },
}

impl Default for WorkspaceDatabase {
    fn default() -> Self {
        WorkspaceDatabase::Sqlite {
            path: "ncubed.db".to_string(),
        }
    }
}

/// There can be either `local` or `remote` workspaces.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::WorkspaceKind;
/// let local_kind = WorkspaceKind::Local("~/path/to/workspace".into());
/// let remote_kind = WorkspaceKind::Remote("https://ncube.cryptodrunks.net".into());
/// assert_eq!("{\"kind\":\"local\",\"location\":\"~/path/to/workspace\"}", serde_json::to_string(&local_kind).unwrap());
/// assert_eq!("{\"kind\":\"remote\",\"location\":\"https://ncube.cryptodrunks.net\"}", serde_json::to_string(&remote_kind).unwrap());
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase", content = "location")]
pub enum WorkspaceKind {
    Local(String),
    Remote(String),
}

impl Default for WorkspaceKind {
    fn default() -> Self {
        WorkspaceKind::Local("./workspace".to_string())
    }
}

/// A single Ncube workspace. Workspaces can either be `local` or `remote`. They
/// are differentiated by the `kind` attribute. Each workspace has an associated
/// database.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use chrono::prelude::{DateTime, Utc, TimeZone};
/// # use ncube_data::{Workspace, WorkspaceDatabase, WorkspaceKind};
/// let local_workspace = Workspace {
///   id: 1,
///   name: "Syrian Archive".into(),
///   slug: "syrian-archive".into(),
///   description: None,
///   created_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   updated_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   kind: WorkspaceKind::Local("~/path".into()),
///   database: WorkspaceDatabase::Sqlite { path: "path/to/file.db".into() },
/// };
/// assert_eq!(
///   "{\"id\":1,\"name\":\"Syrian Archive\",\"slug\":\"syrian-archive\",\"description\":null,\"created_at\":\"2014-11-28T12:00:09Z\",\"updated_at\":\"2014-11-28T12:00:09Z\",\"kind\":\"local\",\"location\":\"~/path\",\"database\":\"sqlite\",\"database_path\":\"path/to/file.db\"}",
///   serde_json::to_string(&local_workspace).unwrap()
/// );
///
/// let json_string = "{\"id\":1,\"name\":\"Syrian Archive\",\"slug\":\"syrian-archive\",\"description\":null,\"created_at\":\"2014-11-28T12:00:09Z\",\"updated_at\":\"2014-11-28T12:00:09Z\",\"kind\":\"remote\",\"location\":\"https://...\",\"database\":\"sqlite\",\"database_path\":\"path/to/file.db\"}";
/// let expected = Workspace {
///   id: 1,
///   name: "Syrian Archive".into(),
///   slug: "syrian-archive".into(),
///   description: None,
///   created_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   updated_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   kind: WorkspaceKind::Remote("https://...".into()),
///   database: WorkspaceDatabase::Sqlite { path: "path/to/file.db".into() },
/// };
/// assert_eq!(
///   expected,
///   serde_json::from_str::<Workspace>(json_string).unwrap(),
/// );
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub kind: WorkspaceKind,
    #[serde(flatten)]
    pub database: WorkspaceDatabase,
}

impl Default for Workspace {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: i32::default(),
            name: String::default(),
            slug: String::default(),
            description: Option::default(),
            created_at: now,
            updated_at: now,
            kind: WorkspaceKind::default(),
            database: WorkspaceDatabase::default(),
        }
    }
}

impl Workspace {
    /// Construct a valid database string for this workspace.
    ///
    /// # Example
    ///
    /// ```
    /// # use ncube_data::{Workspace, WorkspaceDatabase, WorkspaceKind};
    /// # use chrono::prelude::{DateTime, Utc, TimeZone};
    /// let workspace = Workspace {
    ///   id: 1,
    ///   name: "Syrian Archive".into(),
    ///   slug: "syrian-archive".into(),
    ///   description: None,
    ///   created_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
    ///   updated_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
    ///   kind: WorkspaceKind::Local("~/path".into()),
    ///   database: WorkspaceDatabase::Sqlite { path: "path/to/file.db".into() },
    /// };
    /// let expected = "sqlite://path/to/file.db".to_string();
    /// assert_eq!(workspace.connection_string(), expected);
    /// ```
    pub fn connection_string(&self) -> String {
        match &self.database {
            WorkspaceDatabase::Sqlite { path } => format!("sqlite://{}", path),
            WorkspaceDatabase::Http { path } => path.to_string(),
        }
    }
}

impl Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:?})", self.name, self.kind)
    }
}
/// There are different types of annotations.
///
/// - tags : simple labels that can categorize data.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::AnnotationKind;
/// let tag = AnnotationKind::Tag("xyz".into());
/// assert_eq!("{\"type\":\"tag\",\"term\":\"xyz\"}", serde_json::to_string(&tag).unwrap());
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type", content = "term")]
pub enum AnnotationKind {
    Tag(String),
}

/// Annotations are additional data that is layered over a source or unit of
/// data.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::{Annotation, AnnotationKind};
/// let annotation = Annotation {
///   id: 1,
///   kind: AnnotationKind::Tag("xyz".into()),
/// };
///
/// assert_eq!(
///   "{\"id\":1,\"type\":\"tag\",\"term\":\"xyz\"}",
///   serde_json::to_string(&annotation).unwrap()
/// );
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Annotation {
    pub id: i32,
    #[serde(flatten)]
    pub kind: AnnotationKind,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QueryTag {
    pub label: String,
    pub description: Option<String>,
}

/// A source represents a place where data can be fetched from. It is combined
/// with a data process to yield units of data.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::{Source, QueryTag};
/// let tag = QueryTag {
///   label: "code".to_string(),
///   description: None,
/// };
///
/// let source = Source {
///   id: 1,
///   kind: "youtube_video".into(),
///   term: "https://youtube.com/watch?v=123456".into(),
///   tags: vec![tag],
/// };
///
/// assert_eq!(
///   "{\"id\":1,\"type\":\"youtube_video\",\"term\":\"https://youtube.com/watch?v=123456\",\"tags\":[{\"label\":\"code\",\"description\":null}]}",
///   serde_json::to_string(&source).unwrap()
/// );
/// ```
// FIXME: Do I need the id?
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Source {
    pub id: i32,
    #[serde(rename = "type")]
    pub kind: String,
    pub term: String,
    #[serde(default)]
    pub tags: Vec<QueryTag>,
}

/// An account is required to gain access to a remote workspace.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub workspace: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: Option<String>,
    pub otp: Option<String>,
    pub is_otp: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Stat {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum MediaType {
    Video,
    Image,
    Url,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Media {
    pub id_hash: String,
    #[serde(flatten)]
    pub kind: MediaType,
    pub term: String,
    // pub data: Option<Map<String, Value>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Download {
    pub id_hash: String,
    #[serde(flatten)]
    pub kind: MediaType,
    pub term: String,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub location: Option<String>,
    // pub data: Option<Map<String, Value>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: i32,
    pub id_hash: String,
    pub content_hash: String,
    pub source: String,
    pub unit_id: Option<String>,
    pub body: Option<String>,
    pub href: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub fetched_at: DateTime<Utc>,
    // pub data: Option<Value>,
    #[serde(default)]
    pub media: Vec<Media>,
    #[serde(default)]
    pub downloads: Vec<Download>,
    #[serde(default)]
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "database")]
pub enum DatabaseRequest {
    Sqlite,
    Http,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "account")]
pub struct AccountRequest {
    pub email: String,
    pub password: String,
    pub password_again: String,
    pub otp: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum WorkspaceKindRequest {
    Local,
    Remote {
        workspace: String,
        endpoint: String,
        account: AccountRequest,
    },
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
        match &self.kind {
            WorkspaceKindRequest::Local => slugify!(&self.name),
            WorkspaceKindRequest::Remote { workspace, .. } => workspace.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub term: String,
    pub tags: Vec<QueryTag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePasswordRequest {
    pub email: String,
    pub password: String,
    pub password_again: String,
}

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

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse<T> {
    pub data: Vec<T>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "status")]
pub enum Status {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    #[serde(flatten)]
    pub status: Status,
    pub data: T,
}

impl<T> SuccessResponse<T>
where
    T: Debug,
{
    pub fn new(data: T) -> Self {
        Self {
            status: Status::Success,
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(flatten)]
    pub status: Status,
    pub code: u16,
    pub errors: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, errors: &str) -> Self {
        Self {
            status: Status::Error,
            code: code.as_u16(),
            errors: errors.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HttpResponse<T>
where
    T: Debug,
{
    Empty,
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Segment {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub query: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentRequest {
    pub title: String,
    pub query: String,
}

impl SegmentRequest {
    pub fn slug(&self) -> String {
        slugify!(&self.title)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ProcessConfigKind {
    Secret,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub name: String,
    pub key: String,
    pub description: String,
    #[serde(flatten)]
    pub kind: ProcessConfigKind,
    pub template: serde_json::Value,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessConfigReq {
    pub key: String,
    pub value: serde_json::Value,
}

/// A workspace has several data processes and each process may have
/// configuration dependencies that have to be fulfilled.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use std::collections::HashMap;
/// # use ncube_data::{ProcessConfigKind, ProcessConfig, Process};
///
/// let template = serde_json::from_str(r#"{"api_key":"Youtube API key"}"#).unwrap();
/// let template2 = serde_json::from_str(r#"{"api_key":"Youtube API key"}"#).unwrap();
/// let secrets = serde_json::from_str(r#"{"api_key":"some key"}"#).unwrap();
///
/// let process = Process {
///   id: 1,
///   key: "youtube_video".into(),
///   name: "Youtube Video".to_string(),
///   description: "Fetch individual Youtube videos.".to_string(),
///   config: vec![
///     ProcessConfig {
///       name: "Youtube API Key".to_string(),
///       key: "youtube".to_string(),
///       description: "Youtube API credentials.".to_string(),
///       kind: ProcessConfigKind::Secret,
///       value: Some(secrets),
///       template,
///     },
///     ProcessConfig {
///       name: "Other API Key".to_string(),
///       key: "youtube".to_string(),
///       description: "Youtube API credentials.".to_string(),
///       kind: ProcessConfigKind::Secret,
///       value: None,
///       template: template2,
///     }
///   ]
/// };
///
/// assert_eq!("{\"id\":1,\"key\":\"youtube_video\",\"name\":\"Youtube Video\",\"description\":\"Fetch individual Youtube videos.\",\"config\":[{\"name\":\"Youtube API Key\",\"key\":\"youtube\",\"description\":\"Youtube API credentials.\",\"kind\":\"secret\",\"template\":{\"api_key\":\"Youtube API key\"},\"value\":{\"api_key\":\"some key\"}},{\"name\":\"Other API Key\",\"key\":\"youtube\",\"description\":\"Youtube API credentials.\",\"kind\":\"secret\",\"template\":{\"api_key\":\"Youtube API key\"},\"value\":null}]}",
///   serde_json::to_string(&process).unwrap()
/// )
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub config: Vec<ProcessConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ProcessRunKind {
    All,
    Selection,
    New,
}

impl Default for ProcessRunKind {
    fn default() -> Self {
        ProcessRunKind::All
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessRunReq {
    pub key: String,
    #[serde(default, flatten)]
    pub kind: ProcessRunKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodologyReq {
    pub title: String,
    pub description: Option<String>,
    pub process: serde_json::Value,
}

impl MethodologyReq {
    pub fn slug(&self) -> String {
        slugify!(&self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Methodology {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub process: serde_json::Value,
    pub initial_state: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestigationReq {
    pub title: String,
    pub description: Option<String>,
    pub methodology: String,
}

impl InvestigationReq {
    pub fn slug(&self) -> String {
        slugify!(&self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Investigation {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub methodology: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySegmentReq {
    pub segment: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SegmentUnit {
    pub id: i32,
    pub source: String,
    pub title: Option<String>,
    pub videos: i32,
    pub images: i32,
    pub state: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_response_envelope() {
        let response = ErrorResponse::new(StatusCode::BAD_REQUEST, "I am an error!");

        let expected = "{\"status\":\"error\",\"code\":400,\"errors\":\"I am an error!\"}";
        let result = serde_json::to_string(&response).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn http_success_response_envelope() {
        let response = SuccessResponse::new("I am data!");

        let expected = "{\"status\":\"success\",\"data\":\"I am data!\"}";
        let result = serde_json::to_string(&response).unwrap();

        assert_eq!(result, expected);
    }
}
