use serde::Deserialize;

use crate::errors::RemoteError;
use crate::types::JsUInt;

// TODO: Add feature to use only specific mod.
pub mod commands;
pub mod errors;
mod http_methods;
pub mod types;

pub mod prelude {
    pub use crate::commands::*;
    pub use crate::errors::*;
    pub use crate::http_methods::*;
}

type CallId = JsUInt;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    #[serde(rename(deserialize = "id"))]
    pub call_id: CallId,
    pub result: Option<serde_json::Value>,
    pub error: Option<RemoteError>,
}

pub fn parse_response<T>(response: Response) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    if let Some(error) = response.error {
        return Err(error.into());
    }

    let result: T = serde_json::from_value(response.result.unwrap()).unwrap();

    Ok(result)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /*
#[serde(rename = "Target.attachedToTarget")]
AttachedToTarget(target::events::AttachedToTargetEvent),
#[serde(rename = "Target.receivedMessageFromTarget")]
ReceivedMessageFromTarget(target::events::ReceivedMessageFromTargetEvent),
#[serde(rename = "Target.targetInfoChanged")]
TargetInfoChanged(target::events::TargetInfoChangedEvent),
#[serde(rename = "Target.targetCreated")]
TargetCreated(target::events::TargetCreatedEvent),
#[serde(rename = "Target.targetDestroyed")]
TargetDestroyed(target::events::TargetDestroyedEvent),
#[serde(rename = "Network.requestIntercepted")]
RequestIntercepted(network::events::RequestInterceptedEvent),
#[serde(rename = "Network.responseReceived")]
ResponseReceived(network::events::ResponseReceivedEvent),
#[serde(rename = "Log.entryAdded")]
LogEntryAdded(logs::events::EntryAddedEvent),
#[serde(rename = "Runtime.exceptionThrown")]
RuntimeExceptionThrown(runtime::events::ExceptionThrownEvent),*/}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Event(Event),
    Response(Response),
    ConnectionShutdown,
}
