use anyhow::Result;
use ncube_data::NcubeConfig;
use xactor::*;

use crate::errors::DataStoreError;

#[message(result = "Result<bool, DataStoreError>")]
pub(crate) struct IsBootstrapped;

#[message(result = "Result<NcubeConfig, DataStoreError>")]
pub(crate) struct ShowConfig;

#[message(result = "Result<(), DataStoreError>")]
pub(crate) struct InsertSetting {
    pub name: String,
    pub value: String,
}

impl InsertSetting {
    pub(crate) fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
