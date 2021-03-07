use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = global, js_name = readFile, catch)]
    async fn js_read_file(path: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = writeFile, catch)]
    async fn js_write_file(path: &str, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = fileExists)]
    fn js_file_exists(path: &str) -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = mkdir, catch)]
    fn js_mkdir(path: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = isAbsolutePath)]
    fn js_is_absolute_path(path: &str) -> bool;
}

pub(crate) fn is_absolute_path(s: &str) -> bool {
    js_is_absolute_path(s)
}

pub(crate) async fn read_file(p: &str) -> Result<Vec<u8>, anyhow::Error> {
    let res: JsValue = js_read_file(p).await.map_err(|e| anyhow!("{:?}", e))?;
    Ok(Uint8Array::from(res).to_vec())
}

pub(crate) async fn write_file(p: &str, data: &[u8]) -> Result<(), anyhow::Error> {
    js_write_file(p, Uint8Array::from(data).into())
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}

pub(crate) fn file_exists(p: &str) -> bool {
    js_file_exists(p)
}

pub(crate) fn mkdir(p: &str) -> Result<(), anyhow::Error> {
    js_mkdir(p).map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}
