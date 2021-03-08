#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use denops_debugger_core::external::fetch::*;
    use denops_debugger_core::v8_types::inspector::version::V8Version;
    use denops_debugger_core::v8_types::inspector::*;
    use denops_debugger_core::*;
    use std::error::Error;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn it_works() {
        // run_client("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").await;
        assert_eq!(
            fetch::<V8Version>("http://localhost:9229/json/version")
                .await
                .unwrap()
                .v8_version,
            "9.0.257.3"
        );
        assert_eq!(2 + 2, 4);
    }
}
