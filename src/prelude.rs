pub use crate::{
    get_element, get_elements, init_callbacks, inner_html, inner_text, try_inner_html,
    try_inner_text, CallbackRegistration, Error, InitFn, Result,
};

pub use gloo::console::error;
pub use gloo::console::log;
pub use serde_wasm_bindgen;

pub use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen::JsCast;
pub use wasm_bindgen::JsValue;

pub use kaja_callback_macro::callback;
pub use kaja_html_macro::html;
