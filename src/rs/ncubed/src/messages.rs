use anyhow::Result;
use ncube_data::NcubeConfig;
use xactor::*;

use crate::errors::StoreError;

#[message(result = "Result<bool, StoreError>")]
pub(crate) struct IsBootstrapped;

#[message(result = "Result<NcubeConfig, StoreError>")]
pub(crate) struct ShowConfig;

#[message(result = "Result<(), StoreError>")]
pub(crate) struct InsertSetting {
    pub name: String,
    pub value: String,
}

impl InsertSetting {
    pub(crate) fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
