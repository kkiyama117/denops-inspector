use crate::protocols::runtime::types::{RemoteObject, ScriptId, StackTrace};
use crate::types::{JsInt, JsUInt};
use serde::{Deserialize, Serialize};

/// Issued when exception was thrown and unhandled
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#event-exceptionThrown
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionThrown {
    pub timestamp: f64,
    pub exception_details: ExceptionDetails,
}

/// Detailed information about exception (or error) that was thrown during script compilation or execution
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ExceptionDetails
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    pub exception_id: JsInt,
    pub text: String,
    pub line_number: JsUInt,
    pub column_number: JsUInt,
    pub script_id: Option<ScriptId>,
    pub url: Option<String>,
    pub stack_trace: Option<StackTrace>,
    pub exception: Option<RemoteObject>,
    pub execution_context_id: Option<JsUInt>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionThrownEvent {
    pub params: ExceptionThrown,
}

/// Issued when object should be inspected
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-inspectRequested
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InspectRequested {
    pub object: RemoteObject,
    pub hints: serde_json::Value,
}

pub type TimeDelta = JsUInt;

#[test]
fn can_parse_exception_thrown_event() {
    let message = r#"
          {
              "timestamp": 1566067104960.9648,
              "exceptionDetails": {
                "exceptionId": 14,
                "text": "Uncaught",
                "lineNumber": 13,
                "columnNumber": 14,
                "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                "stackTrace": {
                  "callFrames": [
                    {
                      "functionName": "thatThrows",
                      "scriptId": "179",
                      "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                      "lineNumber": 13,
                      "columnNumber": 14
                    },
                    {
                      "functionName": "",
                      "scriptId": "179",
                      "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                      "lineNumber": 10,
                      "columnNumber": 6
                    }
                  ]
                },
                "exception": {
                  "type": "object",
                  "subtype": "error",
                  "className": "Error",
                  "description": "Error: Just an error thrown()\n    at thatThrows (http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:14:15)\n    at http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7",
                  "objectId": "{\"injectedScriptId\":45,\"id\":1}",
                  "preview": {
                    "type": "object",
                    "subtype": "error",
                    "description": "Error: Just an error thrown()\n    at thatThrows (http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:14:15)\n    at http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7",
                    "overflow": false,
                    "properties": [
                      {
                        "name": "stack",
                        "type": "string",
                        "value": "Error: Just an error thrown()\n    at thatThrows (https_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7"
                      },
                      {
                        "name": "message",
                        "type": "string",
                        "value": "Just an error thrown()"
                      }
                    ]
                  }
                },
                "executionContextId": 45
              }
            }
        "#;

    let _exception_thrown = serde_json::from_str::<ExceptionThrown>(message).unwrap();
}
