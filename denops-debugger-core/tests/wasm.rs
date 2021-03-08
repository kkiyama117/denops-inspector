#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use denops_debugger_core::external::fetch::*;
    use v8_inspector_api_types::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn it_works() {
        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        assert_eq!(
            fetch::<V8Version>("http://localhost:9229/json/version")
                .await
                .unwrap()
                .v8_version,
            String::from(
                fetch::<serde_json::Value>("http://localhost:9229/json/version")
                    .await
                    .unwrap()
                    .get("V8-Version")
                    .unwrap()
                    .as_str()
                    .unwrap()
            )
        );
        assert_eq!(2 + 2, 4);
    }
}
