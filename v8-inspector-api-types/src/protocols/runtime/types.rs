use crate::types::JsUInt;
use serde::{Deserialize, Serialize};

/// Unique script identifier
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ScriptId
pub type ScriptId = String;

/// Stack entry for runtime errors and assertions
/// See https://chromedevtools.github.io/devtools-protocol/v8/Debugger/#type-CallFrame
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    function_name: String,
    script_id: ScriptId,
    url: String,
    line_number: JsUInt,
    column_number: JsUInt,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace {
    pub description: Option<String>,
    pub call_frame: Vec<CallFrame>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub value: Option<String>,
    pub value_preview: Option<Box<PropertyPreview>>,
    pub subtype: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview {
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub value: Option<String>,
    pub value_preview: Option<Box<PropertyPreview>>,
    pub subtype: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    #[serde(rename = "type")]
    pub object_type: String,
    pub subtype: Option<String>,
    pub description: Option<String>,
    pub overflow: bool,
    pub properties: Vec<PropertyPreview>,
}

/// Object type
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-RemoteObject
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RemoteObjectType {
    Object,
    Function,
    Undefined,
    String,
    Number,
    Boolean,
    Symbol,
    Bigint,
}

/// Object subtype hint. Specified for object type values only
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-RemoteObject
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RemoteObjectSubtype {
    Array,
    Null,
    Node,
    RegExp,
    Date,
    Map,
    Set,
    WeakMap,
    WeakSet,
    Iterator,
    Generator,
    Error,
    Proxy,
    Promise,
    TypedArray,
    ArrayBuffer,
    DataView,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    #[serde(rename = "type")]
    pub object_type: RemoteObjectType,
    pub subtype: Option<RemoteObjectSubtype>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    pub value: Option<serde_json::Value>,
    pub unserializable_value: Option<String>,
    pub preview: Option<ObjectPreview>,
}

/// Experimental
/// If debuggerId is set stack trace comes from another debugger and can be resolved there.
/// This allows to track cross-debugger calls. See Runtime.StackTrace and Debugger.paused for usages.
/// Experimental feature of DevTools
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-StackTraceId
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StackTraceId {
    pub id: String,
    pub debugger_id: UniqueDebuggerId,
}

pub type UniqueDebuggerId = String;
