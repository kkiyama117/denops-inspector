use crate::methods::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocation {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocationReturnObject {}
impl Method for ContinueToLocation {
    const NAME: &'static str = "Debugger.continueToLocation";
    type ReturnObject = ContinueToLocationReturnObject;
}

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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Disable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Debugger.disable";
    type ReturnObject = DisableReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrame {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameReturnObject {}
impl Method for EvaluateOnCallFrame {
    const NAME: &'static str = "Debugger.evaluateOnCallFrame ";
    type ReturnObject = EvaluateOnCallFrameReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpoints {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturnObject {}
impl Method for GetPossibleBreakpoints {
    const NAME: &'static str = "Debugger.getPossibleBreakpoints ";
    type ReturnObject = GetPossibleBreakpointsReturnObject;
}

/// Debugger.Enable
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resume {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeReturnObject {}
impl Method for Resume {
    const NAME: &'static str = "Debugger.resume";
    type ReturnObject = ResumeReturnObject;
}
