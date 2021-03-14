use headless_chrome::protocol::types::*;
use headless_chrome::protocol::CallId;
use serde::Deserialize;

use crate::errors::RemoteError;
use std::fmt::Debug;

// TODO: Add feature to use only specific mod.
pub mod browser;
pub mod errors;
mod websocket_target_list;
pub mod types;

pub mod prelude {
    pub use crate::browser::*;
    pub use crate::errors::*;
    pub use crate::websocket_target_list::*;
}


#[derive(Serialize, Debug)]
pub struct MethodCall<T>
    where
        T: Debug,
{
    #[serde(rename = "method")]
    method_name: &'static str,
    pub id: CallId,
    params: T,
}

impl<T> MethodCall<T>
    where
        T: Debug,
{
    pub fn get_params(&self) -> &T {
        &self.params
    }
}

pub trait Method: Debug {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug;

    fn to_method_call(self, call_id: CallId) -> Box<MethodCall<Self>>
        where
            Self: std::marker::Sized,
    {
        Box::new(MethodCall {
            id: call_id,
            params: self,
            method_name: Self::NAME,
        }
        )
    }
}


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
pub enum Event {/*
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
    RuntimeExceptionThrown(runtime::events::ExceptionThrownEvent),*/
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Event(Event),
    Response(Response),
    ConnectionShutdown,
}