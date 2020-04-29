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

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub name: String,
    pub value: String,
    pub required: bool,
    pub description: String,
}

pub type NcubeConfig = Vec<ConfigSetting>;

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
/// are differentiated by the `kind` attribute.
///
/// # Example
///
/// ```
/// # use serde_json;
/// # use chrono::prelude::{DateTime, Utc, TimeZone};
/// # use ncube_data::{Workspace, WorkspaceKind};
/// let local_workspace = Workspace {
///   id: 1,
///   name: "Syrian Archive".into(),
///   slug: "syrian-archive".into(),
///   description: None,
///   created_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   updated_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   kind: WorkspaceKind::Local("~/path".into()),
/// };
/// assert_eq!(
///   "{\"id\":1,\"name\":\"Syrian Archive\",\"slug\":\"syrian-archive\",\"description\":null,\"created_at\":\"2014-11-28T12:00:09Z\",\"updated_at\":\"2014-11-28T12:00:09Z\",\"kind\":\"local\",\"location\":\"~/path\"}",
///   serde_json::to_string(&local_workspace).unwrap()
/// );
///
/// let json_string = "{\"id\":1,\"name\":\"Syrian Archive\",\"slug\":\"syrian-archive\",\"description\":null,\"created_at\":\"2014-11-28T12:00:09Z\",\"updated_at\":\"2014-11-28T12:00:09Z\",\"kind\":\"remote\",\"location\":\"https://...\"}";
/// let expected = Workspace {
///   id: 1,
///   name: "Syrian Archive".into(),
///   slug: "syrian-archive".into(),
///   description: None,
///   created_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   updated_at: Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
///   kind: WorkspaceKind::Remote("https://...".into()),
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
}
