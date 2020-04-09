use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub name: String,
    pub value: String,
    pub required: bool,
    pub description: String,
}

pub type NcubeConfig = Vec<ConfigSetting>;
