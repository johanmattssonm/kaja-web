pub use crate::{
    CallbackRegistration, Error, InitFn, Result, get_callback, get_cookie_value, get_element,
    get_elements, get_value, init_callbacks, inner_html, inner_text, try_get_value, try_inner_html,
    try_inner_text,
};

pub use gloo::console::error;
pub use gloo::console::log;
pub use serde_wasm_bindgen;

pub use wasm_bindgen::JsCast;
pub use wasm_bindgen::JsValue;
pub use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen_futures;

pub use kaja_callback_macro::callback;
pub use kaja_html_macro::html;
