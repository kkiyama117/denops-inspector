use serde::de::DeserializeOwned;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch, catch)]
    async fn js_fetch(s: &str) -> Result<JsValue, JsValue>;
}

pub async fn fetch<T: DeserializeOwned>(url: Url) -> Result<T, anyhow::Error> {
    return match js_fetch(url.as_str()).await {
        Ok(response) => {
            let resp: Response = response.into();
            match resp.json() {
                Ok(json) => {
                    let js_json = JsFuture::from(json);
                    match js_json.await {
                        Ok(result) => Ok(T::from(result.into_serde()?)),
                        // TODO: use macro for convert
                        Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
                    }
                }
                Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
            }
        }
        Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
    };
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use crate::external::fetch::fetch;
    use url::Url;
    use v8_inspector_api_types::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn it_works() {
        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        assert_eq!(
            fetch::<Version>(Url::parse("http://localhost:9229/json/version").unwrap())
                .await
                .unwrap()
                .v8_version,
            String::from(
                fetch::<serde_json::Value>(
                    Url::parse("http://localhost:9229/json/version").unwrap()
                )
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
