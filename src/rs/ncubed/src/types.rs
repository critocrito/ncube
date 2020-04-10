use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SettingRequest {
    pub name: String,
    pub value: String,
}

pub type NcubeConfigRequest = Vec<SettingRequest>;
