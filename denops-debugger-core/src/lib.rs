use async_trait::async_trait;
use v8_inspector_api_types::browser::Version;
use v8_inspector_api_types::prelude::WebSocketConnectionInfo;

#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
pub mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
pub mod external;

pub struct DebuggerClient {
    base_url: String,
}

impl DebuggerClient {
    pub fn new() -> DebuggerClient {
        DebuggerClient {
            base_url: "http://localhost:9229/".into(),
        }
    }
}

#[async_trait]

pub trait DebuggerClientTrait {
    async fn check_version(&self) -> Version;
    fn open(&self);
    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo>;
}

#[async_trait]
impl DebuggerClientTrait for DebuggerClient {
    async fn check_version(&self) -> Version {
        let mut url = self.base_url.clone();
        url.push_str("json/version");
        crate::external::fetch::fetch(url.as_str()).await.unwrap()
    }

    fn open(&self) {
        unimplemented!()
    }

    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo> {
        let mut url = self.base_url.clone();
        url.push_str("json");
        crate::external::fetch::fetch::<Vec<WebSocketConnectionInfo>>(url.as_str())
            .await
            .unwrap()
    }
}
