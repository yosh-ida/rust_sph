use wasm_bindgen::JsValue;
use web_sys::console;

pub mod kernel;
pub mod particle;
pub mod vector;

pub fn console_log(s: &str) {
    // console::log_1(&JsValue::from_str(s));
}
