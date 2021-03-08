use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketConnectionInfo {
    description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    devtools_frontend_url: String,
    id: String,
    title: String,
    #[serde(rename = "type")]
    _type: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: String,
}
