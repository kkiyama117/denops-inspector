#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use denops_debugger_core::external::fetch::*;
    use std::error::Error;
    use v8_inspector_api_types::prelude::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch() {
        // run_client("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").await;
        assert_eq!(
            fetch::<V8Version>("http://localhost:9229/json/version")
                .await
                .unwrap()
                .v8_version,
            "9.0.257.3"
        );
        dbg!(fetch::<serde_json::Value>("http://localhost:9229/json")
            .await
            .unwrap());
        assert_eq!(2 + 2, 4);
    }
}
