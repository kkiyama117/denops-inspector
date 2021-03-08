#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use denops_debugger_core::external::http::run;
    use denops_debugger_core::external::ws_cli::start_websocket;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn it_works() {
        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        assert_eq!(
            start_websocket("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").unwrap(),
            ()
        );
        run("http://localhost:9229".into()).await.unwrap();
        assert_eq!(2 + 2, 4);
    }
}
