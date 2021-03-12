use headless_chrome::protocol::types::*;
use headless_chrome::protocol::CallId;
pub use headless_chrome::protocol::{Method, MethodCall};
use serde::Deserialize;

use crate::errors::RemoteError;

// TODO: Add feature to use only specific mod.
pub mod browser;
pub mod errors;
mod websocket_target_list;

pub mod prelude {
    pub use crate::browser::*;
    pub use crate::errors::*;
    pub use crate::websocket_target_list::*;
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
