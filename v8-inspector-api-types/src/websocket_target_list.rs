use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct WebSocketConnectionInfo {
    pub description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    pub devtools_frontend_url: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}
