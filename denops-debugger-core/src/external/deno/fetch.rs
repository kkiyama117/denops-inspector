use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch, catch)]
    async fn js_fetch(s: &str) -> Result<JsValue, JsValue>;
}

pub async fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, anyhow::Error> {
    return match js_fetch(url).await {
        Ok(response) => {
            let resp: Response = response.into();
            match resp.json() {
                Ok(json) => {
                    let js_json = JsFuture::from(json);
                    match js_json.await {
                        Ok(result) => Ok(T::from(result.into_serde()?)),
                        Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
                    }
                }
                Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
            }
        }
        Err(e3) => Err(anyhow::Error::msg(format!("{:?}", e3.into_serde()?))),
    };
}
