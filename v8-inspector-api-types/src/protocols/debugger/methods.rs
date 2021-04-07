use crate::methods::Method;
use crate::prelude::types::BreakLocation;
use crate::protocols::debugger::types::{CallFrameId, Location, LocationLange};
use crate::protocols::runtime::events::{ExceptionDetails, TimeDelta};
use crate::protocols::runtime::types::{RemoteObject, UniqueDebuggerId};
use crate::types::JsUInt;
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
pub struct Enable {
    pub max_script_cache_size: Option<JsUInt>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {
    debugger_id: Option<UniqueDebuggerId>,
}
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
pub struct EvaluateOnCallFrameReturnObject {
    pub result: RemoteObject,
    exception_details: Option<ExceptionDetails>,
}
impl Method for EvaluateOnCallFrame {
    const NAME: &'static str = "Debugger.evaluateOnCallFrame ";
    type ReturnObject = EvaluateOnCallFrameReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpoints {
    start: Location,
    end: Option<Location>,
    restrict_to_function: Option<bool>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturnObject {
    locations: Vec<BreakLocation>,
}
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
pub struct Resume {
    pub terminate_on_resume: Option<bool>,
}
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StepInto {
    pub break_on_async_call: Option<bool>,
    pub skip_list: Vec<LocationLange>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepIntoReturnObject {}
impl Method for StepInto {
    const NAME: &'static str = "Debugger.stepInto";
    type ReturnObject = StepIntoReturnObject;
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StepOut {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOutReturnObject {}
impl Method for StepOut {
    const NAME: &'static str = "Debugger.stepOut";
    type ReturnObject = StepOutReturnObject;
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StepOver {
    pub skip_list: Vec<LocationLange>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOverReturnObject {}
impl Method for StepOver {
    const NAME: &'static str = "Debugger.stepOver";
    type ReturnObject = StepOverReturnObject;
}
