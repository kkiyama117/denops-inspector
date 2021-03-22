use crate::external::ws::WebSocket;
use async_trait::async_trait;
use v8_inspector_api_types::prelude::*;

pub struct DebuggerClientInfo {
    base_url: String,
}

impl Default for DebuggerClientInfo {
    fn default() -> Self {
        DebuggerClientInfo {
            base_url: "http://localhost:9229/".into(),
        }
    }
}

pub struct DebuggerClient {
    info: DebuggerClientInfo,
    ws_connection: Option<WebSocket>,
}

impl Default for DebuggerClient {
    fn default() -> Self {
        DebuggerClient::new(DebuggerClientInfo::default())
    }
}

impl DebuggerClient {
    pub fn new(info: DebuggerClientInfo) -> Self {
        DebuggerClient {
            info,
            ws_connection: None,
        }
    }

    fn get_base_url(&self) -> String {
        self.info.base_url.clone()
    }
}

#[async_trait]
pub trait DebuggerClientTrait {
    async fn check_version(&self) -> Version;
    fn open(&self);
    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo>;
    // async fn send_method<T>(&self, method: Box<dyn Method<ReturnObject = T>>) -> Vec<Response>;
}

#[async_trait]
impl DebuggerClientTrait for DebuggerClient {
    async fn check_version(&self) -> Version {
        let mut url = self.get_base_url();
        url.push_str("json/version");
        crate::external::fetch::fetch(url.as_str()).await.unwrap()
    }

    fn open(&self) {
        unimplemented!()
    }

    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo> {
        let mut url = self.get_base_url();
        url.push_str("json");
        crate::external::fetch::fetch::<Vec<WebSocketConnectionInfo>>(url.as_str())
            .await
            .unwrap()
    }

    // async fn send_method<T>(&self, method: Box<dyn Method<ReturnObject = T>>) -> Vec<Response> {
    //     unimplemented!()
    // }
}
