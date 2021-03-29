use crate::methods::Method;
use serde::{Deserialize, Serialize};

/// Debugger.Enable
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "Debugger.enable";
    type ReturnObject = EnableReturnObject;
}

/// Debugger.Enable
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Disable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Debugger.enable";
    type ReturnObject = DisableReturnObject;
}
