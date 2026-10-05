pub use crate::{
    CallbackRegistration, Component, Error, InitComponentFn, InitFn, Result, SharedComponent,
    UpdateComponentFn, add_css_class, dispatch_component_update, get_callback, get_cookie_value,
    get_element, get_element_by_id, get_elements, get_value, init_callbacks, inner_html,
    inner_text, remove_css_class, try_get_value, try_handle_component_update_message,
    try_inner_html, try_inner_text,
};

pub use gloo::console::error;
pub use gloo::console::log;
pub use serde_wasm_bindgen;

pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;
pub use std::sync::{MutexGuard, RwLock};

pub use wasm_bindgen::JsCast;
pub use wasm_bindgen::JsValue;
pub use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen_futures;

pub use kaja_callback_macro::callback;
pub use kaja_html_macro::html;
pub use kaja_web_component::component;

pub use js_sys::Function;
pub use web_sys::HtmlElement;
