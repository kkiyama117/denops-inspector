use serde::Deserialize;

use crate::protocols::debugger::types::{BreakPointId, CallFrame, DebugSymbols, Location};
use crate::protocols::runtime::types::{ScriptId, StackTrace, StackTraceId};
use crate::types::JsUInt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BreakpointResolved {
    pub params: BreakpointResolvedParams,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointResolvedParams {
    breakpoint_id: BreakPointId,
    location: Location,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Paused {
    pub params: PausedParams,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PausedParams {
    pub call_frames: Vec<CallFrame>,
    pub reason: String,
    pub data: Option<serde_json::Value>,
    pub hit_breakpoints: Option<Vec<String>>,
    pub async_stack_trace: Option<StackTrace>,
    pub async_stack_trace_id: Option<StackTraceId>,
    #[deprecated]
    pub(crate) async_call_stack_trace_id: Option<StackTraceId>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Resumed {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptFailedToParse {
    pub params: ScriptFailedToParseParams,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFailedToParseParams {
    pub script_id: ScriptId,
    pub url: String,
    pub start_line: JsUInt,
    pub start_column: JsUInt,
    pub end_line: JsUInt,
    pub end_column: JsUInt,
    pub execution_context_id: JsUInt,
    pub hash: String,
    pub execution_context_aux_data: Option<serde_json::Value>,
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
    #[serde(rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    pub is_module: Option<bool>,
    pub length: Option<JsUInt>,
    pub stack_trace: Option<StackTrace>,
    pub code_offset: Option<JsUInt>,
    pub script_language: Option<String>,
    pub embedder_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptParsed {
    pub params: ScriptParsedParams,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParsedParams {
    pub script_id: ScriptId,
    pub url: String,
    pub start_line: JsUInt,
    pub start_column: JsUInt,
    pub end_line: JsUInt,
    pub end_column: JsUInt,
    pub execution_context_id: JsUInt,
    pub hash: String,
    pub execution_context_aux_data: Option<serde_json::Value>,
    pub is_live_edit: Option<bool>,
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
    #[serde(rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    pub is_module: Option<bool>,
    pub length: Option<JsUInt>,
    pub stack_trace: Option<StackTrace>,
    pub code_offset: Option<JsUInt>,
    pub script_language: Option<String>,
    pub debug_symbols: Option<DebugSymbols>,
    pub embedder_name: String,
}

mod test {
    #[test]
    fn can_parse_exception_thrown_event() {
        let message = r#"{
        "method":"Debugger.scriptParsed",
        "params":{
            "scriptId":"3",
            "url":"deno:core/core.js",
            "startLine":0,
            "startColumn":0,
            "endLine":279,
            "endColumn":0,
            "executionContextId":0,
            "hash":"25f9f7ec15fdc578530ee1930d06c48307c3c81f",
            "isLiveEdit":false,
            "sourceMapURL":"",
            "hasSourceURL":false,
            "isModule":false,
            "length":7413,
            "stackTrace":{
                "callFrames":[
                    {
                        "functionName":"handleAsyncMsgFromRust",
                        "scriptId":"3",
                        "url":"deno:core/core.js",
                        "lineNumber":157,
                        "columnNumber":33
                    }
                ]
            },
            "scriptLanguage":"JavaScript",
            "embedderName":"deno:core/core.js"
        }
   }
   "#;

        let _result =
            serde_json::from_str::<crate::protocols::debugger::events::ScriptParsed>(message)
                .unwrap();
    }
}
