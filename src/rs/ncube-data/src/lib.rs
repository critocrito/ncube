// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Ncube;

pub trait NcubeEntity {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    // pub investigations: Vec<Investigation>,
    // pub data_segments: Vec<DataSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Investigation {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSegment {
    pub id: i32,
    pub title: String,
    pub query: String, // FIXME: This should be a proper query
}

impl NcubeEntity for Collection {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigSetting {
    pub name: String,
    pub value: String,
    pub required: bool,
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
            WorkspaceDatabase::Http { path } => format!("{}", path),
        }
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

/// A source represents a place where data can be fetched from. It is combined
/// with a data process to yield units of data.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use ncube_data::{Source, Annotation, AnnotationKind};
/// let annotation = Annotation {
///   id: 1,
///   kind: AnnotationKind::Tag("xyz".into()),
/// };
///
/// let source = Source {
///   id: 1,
///   kind: "youtube_video".into(),
///   term: "https://youtube.com/watch?v=123456".into(),
///   annotations: vec![annotation],
/// };
///
/// assert_eq!(
///   "{\"id\":1,\"type\":\"youtube_video\",\"term\":\"https://youtube.com/watch?v=123456\",\"annotations\":[{\"id\":1,\"type\":\"tag\",\"term\":\"xyz\"}]}",
///   serde_json::to_string(&source).unwrap()
/// );
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Source {
    pub id: i32,
    #[serde(rename = "type")]
    pub kind: String,
    pub term: String,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
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
}
