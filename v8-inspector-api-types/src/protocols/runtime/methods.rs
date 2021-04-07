use crate::methods::Method;
use crate::protocols::runtime::types::RemoteObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOn<'a> {
    pub object_id: &'a str,
    pub function_declaration: &'a str,
    pub return_by_value: bool,
    pub generate_preview: bool,
    pub silent: bool,
    pub await_promise: bool,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnReturnObject {
    pub result: RemoteObject,
}
impl<'a> Method for CallFunctionOn<'a> {
    const NAME: &'static str = "Runtime.callFunctionOn";
    type ReturnObject = CallFunctionOnReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Evaluate<'a> {
    pub expression: &'a str,
    pub include_command_line_api: bool,
    pub silent: bool,
    pub return_by_value: bool,
    pub generate_preview: bool,
    pub user_gesture: bool,
    pub await_promise: bool,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateReturnObject {
    pub result: RemoteObject,
}
impl<'a> Method for Evaluate<'a> {
    const NAME: &'static str = "Runtime.evaluate";
    type ReturnObject = EvaluateReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "Runtime.enable";
    type ReturnObject = EnableReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Disable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Runtime.disable";
    type ReturnObject = DisableReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunIfWaitingForDebugger {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunIfWaitingForDebuggerReturnObject {}
impl Method for RunIfWaitingForDebugger {
    const NAME: &'static str = "Runtime.runIfWaitingForDebugger";
    type ReturnObject = RunIfWaitingForDebuggerReturnObject;
}
