use serde::Deserialize;

use crate::protocols::runtime::methods::StackTrace;
use crate::types::JsUInt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptParsed {
    pub params: ScriptParsedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParsedParams {
    pub script_id: String,
    pub url: String,
    pub start_line: JsUInt,
    pub start_column: JsUInt,
    pub end_line: JsUInt,
    pub end_column: JsUInt,
    pub execution_context_id: JsUInt,
    pub hash: String,
    pub is_live_edit: bool,
    #[serde(rename(deserialize = "sourceMapURL"))]
    pub source_map_url: String,
    #[serde(rename(deserialize = "hasSourceURL"))]
    pub has_source_url: bool,
    pub is_module: bool,
    pub length: JsUInt,
    pub stack_trace: StackTrace,
    pub script_language: String,
    pub embedder_name: String,
}

mod test {
    use crate::protocols::debugger::events::ScriptParsed;

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

        let _result = serde_json::from_str::<ScriptParsed>(message).unwrap();
    }
}
