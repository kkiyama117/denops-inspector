use denops_debugger_core::external::fetch::fetch;
use v8_inspector_api_types::prelude::{V8Version, WebSocketConnectionInfo};

#[tokio::main]
async fn main() {
    dbg!(
        fetch::<V8Version>("http://localhost:9229/json/version")
            .await
            .unwrap()
            .v8_version,
    );
    dbg!(
        fetch::<Vec<WebSocketConnectionInfo>>("http://localhost:9229/json")
            .await
            .unwrap()
    );
}
