use denops_debugger_core::external::*;
use v8_inspector_api_types::prelude::{V8Version, WebSocketConnectionInfo};

#[tokio::main]
async fn main() {
    dbg!(
        fetch::fetch::<V8Version>("http://localhost:9229/json/version")
            .await
            .unwrap()
    );
    ws_cli::ws_connection(
        fetch::fetch::<Vec<WebSocketConnectionInfo>>("http://localhost:9229/json")
            .await
            .unwrap()
            .get(0)
            .unwrap()
            .clone()
            .web_socket_debugger_url,
    );
}
