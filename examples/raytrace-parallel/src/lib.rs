use kaja_web::prelude::*;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn logv(x: &JsValue);
}

mod main_component;
mod pool;

#[wasm_bindgen(start)]
pub fn init() {
    log!("Starting");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    log!("Init callbacks");
    init_callbacks();
    log!("Init complete");
}
