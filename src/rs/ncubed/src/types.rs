pub use ncube_data::WorkspaceKind;
use serde::Deserialize;
use slugify::slugify;

#[derive(Debug, Deserialize)]
pub struct WorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub kind: WorkspaceKind,
}

impl WorkspaceRequest {
    pub fn slug(&self) -> String {
        slugify!(&self.name)
    }
}
