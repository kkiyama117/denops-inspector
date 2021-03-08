use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch, catch)]
    async fn js_fetch(s: &str) -> Result<JsValue, JsValue>;
}

pub async fn run(url: &str) -> Result<JsValue, JsValue> {
    let resp_value = js_fetch(url).await?;
    let resp: Response = resp_value.into();
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}
