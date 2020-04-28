use ncube_data::NcubeConfig;
use std::result::Result;
use xactor::*;

use crate::errors::ActorError;

#[message(result = "Result<bool, ActorError>")]
pub(crate) struct IsBootstrapped;

#[message(result = "Result<NcubeConfig, ActorError>")]
pub(crate) struct ShowConfig;

#[message(result = "Result<(), ActorError>")]
pub(crate) struct InsertSetting {
    pub name: String,
    pub value: String,
}

impl InsertSetting {
    pub(crate) fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
