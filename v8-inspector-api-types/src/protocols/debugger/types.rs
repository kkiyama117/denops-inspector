use crate::protocols::runtime::types::{RemoteObject, ScriptId};
use crate::types::{JsInt, JsUInt};
use serde::{Deserialize, Serialize};

/// See https://chromedevtools.github.io/devtools-protocol/v8/Debugger/#type-BreakLocation
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation {
    script_id: ScriptId,
    line_number: JsInt,
    column_number: Option<JsInt>,
    types: Option<String>,
}

pub type BreakPointId = String;

/// Stack entry for runtime errors and assertions
/// See https://chromedevtools.github.io/devtools-protocol/v8/Debugger/#type-CallFrame
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    call_frame_id: CallFrameId,
    function_name: String,
    function_location: Option<Location>,
    location: Location,
    url: String,
    scope_chain: Vec<Scope>,
    this: RemoteObject,
    return_value: Option<RemoteObject>,
}
pub type CallFrameId = String;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DebugSymbols {
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "externalURL")]
    external_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    script_id: ScriptId,
    line_number: JsInt,
    column_number: Option<JsInt>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    #[serde(rename = "type")]
    _type: String,
    object: RemoteObject,
    name: Option<String>,
    start_location: Option<Location>,
    end_location: Option<Location>,
}

pub type ScriptLanguage = String;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    line_number: JsInt,
    line_content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocationLange {
    script_id: ScriptId,
    start: ScriptPosition,
    end: ScriptPosition,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPosition {
    line_number: JsUInt,
    column_number: JsUInt,
}
