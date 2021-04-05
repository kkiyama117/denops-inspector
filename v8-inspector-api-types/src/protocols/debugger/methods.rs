use crate::methods::Method;
use crate::protocols::debugger::types::{CallFrameId, Location};
use crate::protocols::runtime::events::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocation {
    location: Location,
    target_call_frames: Option<String>,
}

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
pub struct EvaluateOnCallFrame {
    call_frame_id: CallFrameId,
    expression: String,
    object_group: String,
    #[serde(rename = "includeCommandLineAPI")]
    include_command_line_api: Option<bool>,
    silent: Option<String>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    throw_on_side_effect: Option<bool>,
    timeout: Option<TimeDelta>,
}
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pause {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseReturnObject {}
impl Method for Pause {
    const NAME: &'static str = "Debugger.pause";
    type ReturnObject = PauseReturnObject;
}

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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPauses {
    pub skip: bool,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPausesReturnObject {}
impl Method for SetSkipAllPauses {
    const NAME: &'static str = "Debugger.setSkipAllPauses";
    type ReturnObject = SetSkipAllPausesReturnObject;
}
