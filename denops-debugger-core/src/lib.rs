#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
#[macro_use]
pub(crate) mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/wasm/mod.rs"]
#[macro_use]
pub(crate) mod external;

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    // use crate::external::ws_cli::run_client;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn it_works() {
        // run_client("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").await;
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use crate::external::ws_cli::start_websocket;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn it_works() {
        // run_client("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").await;
        assert_eq!(2 + 2, 4);
    }
}
