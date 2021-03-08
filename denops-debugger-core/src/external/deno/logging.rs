use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
         $crate::external::logging::log_info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
         $crate::external::logging::log_warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
         $crate::external::logging::log_error(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {  $crate::external::logging::log_info(&format!($($arg)*)) }
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js_log_info(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn js_log_warn(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn js_log_error(s: &str);

    #[wasm_bindgen(js_namespace = global, js_name = sendMessage)]
    fn js_send_message(message: JsValue);
}

pub(crate) fn log_info(s: &str) {
    js_log_info(s)
}

pub(crate) fn log_warn(s: &str) {
    js_log_warn(s)
}

pub(crate) fn log_error(s: &str) {
    js_log_error(s)
}
