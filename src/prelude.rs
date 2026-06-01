pub use crate::{
    get_element, get_elements, init_callbacks, inner_html, inner_text, node_list_to_vec,
    try_inner_html, try_inner_text, App, CallbackRegistration, Error, InitFn, JsCallItem, Result,
};

pub use gloo::console::log;
pub use serde_wasm_bindgen;

pub use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen::JsCast;
pub use wasm_bindgen::JsValue;

pub use kaja_callback_macro::callback;
pub use kaja_html_macro::html;
