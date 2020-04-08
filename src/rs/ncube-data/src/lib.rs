pub struct Ncube;

pub trait NcubeEntity {}

#[derive(Debug)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    // pub investigations: Vec<Investigation>,
    // pub data_segments: Vec<DataSegment>,
}

#[derive(Debug)]
pub struct Investigation {
    pub id: i32,
    pub title: String,
}

#[derive(Debug)]
pub struct DataSegment {
    pub id: i32,
    pub title: String,
    pub query: String, // FIXME: This should be a proper query
}

impl NcubeEntity for Collection {}

#[derive(Debug)]
pub struct NcubeConfig {
    pub workspace_root: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Default for NcubeConfig {
    fn default() -> Self {
        NcubeConfig {
            workspace_root: "./ncube.db".to_string(),
            name: None,
            email: None,
        }
    }
}
