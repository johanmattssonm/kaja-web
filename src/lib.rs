// Copyright (c) 2026 Johan Mattsson
// License: MIT

#![doc = include_str!("../README.md")]

use gloo_console::{error, log};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDocument, HtmlElement, HtmlInputElement, NodeList};

pub mod prelude;

pub struct InitFn(pub fn());
inventory::collect!(InitFn);

#[derive(Debug, Clone, Copy)]
pub struct Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct CallbackRegistration {
    pub register: fn(),
}

inventory::collect!(CallbackRegistration);

/// Register callbacks that can be used in JS to run Rust functions in the WASM
/// module. This needs to run when the WASM bundle is loaded.
///
/// ```rust
/// #[wasm_bindgen(start)]
/// pub fn init() {
///     init_callbacks();
/// }
/// ```
///
/// The Rust functions as marked with the `#[callback]` attribute in order to
/// be callable from JS. The JS function in this example has the name
/// `testCallback`. Note that camel case is used in JS and regular snake case
/// is used in Rust for the same function.
///
/// ```rust
/// #[callback(testCallback)]
/// pub fn test_callback() {
/// }
/// ```
///
/// ```rust
/// let html = html!{{
///     <button onclick="testCallback()">Test</button>
/// }};
/// ```
pub fn init_callbacks() {
    for init in inventory::iter::<InitFn> {
        (init.0)();
    }
}

/// Queries the document for a single element matching the given selector.
/// Returns `None` if no element is found. Returns first matching element if multiple are found.
pub fn get_element(selector: &str) -> Option<HtmlElement> {
    let window = web_sys::window().ok_or(Error).ok()?;
    let document = window.document().ok_or(Error).ok()?;
    let elements = document
        .query_selector_all(selector)
        .map_err(|_| Error)
        .ok()?;

    if elements.length() == 0 {
        return None;
    }

    if elements.length() > 1 {
        let msg = format!(
            "Expecting a single element, found {} instead.",
            elements.length()
        );

        error!(msg);
    }

    if let Some(node) = elements.item(0) {
        if let Ok(element) = node.dyn_into::<HtmlElement>() {
            return Some(element);
        }
    }

    None
}

/// Returns all elements matching the given selector.
/// For example, `get_elements(".test")` would return all elements with the class `test`.
pub fn get_elements(selector: &str) -> Vec<HtmlElement> {
    let window = web_sys::window();

    if window.is_none() {
        return Vec::new();
    }

    let document = window.unwrap().document();

    if document.is_none() {
        return Vec::new();
    }

    let elements = document.unwrap().query_selector_all(selector);

    if elements.is_err() {
        let msg = format!("Failed to query selector all: {:?}", selector);
        error!(msg);
        return Vec::new();
    }

    return node_list_to_vec(elements.unwrap());
}

/// Sets the inner HTML of the elements matching the given selector.
/// Returns an error if no element is found.
pub fn inner_html(selector: &str, html: &str) -> Result<()> {
    let window = web_sys::window().ok_or(Error)?;
    let document = window.document().ok_or(Error)?;

    let elements = document.query_selector_all(selector).map_err(|_| Error)?;

    if elements.length() == 0 {
        return Err(Error);
    }

    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_inner_html(html);
            }
        }
    }

    Ok(())
}

/// Sets the inner text of the elements matching the given selector.
/// Returns an error if no element are found.
pub fn inner_text(selector: &str, text: &str) -> Result<()> {
    let window = web_sys::window().ok_or(Error)?;
    let document = window.document().ok_or(Error)?;

    let elements = document.query_selector_all(selector).map_err(|_| Error)?;

    if elements.length() == 0 {
        return Err(Error);
    }

    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_text_content(Some(text));
            }
        }
    }

    Ok(())
}

/// Get value from an input tag.
/// ```html
/// <input type="text" id="test" value="test value" />
/// ```
///
/// ```rust
/// let value = get_value("#test");
/// assert_eq!(value, Some("test value"));
/// ```
pub fn get_value(selector: &str) -> Option<String> {
    let element = get_element(selector)?;

    if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
        return Some(input.value());
    }

    None
}

/// Get value from input tag, or return an empty string if no value is found.
pub fn try_get_value(selector: &str) -> String {
    let value = get_value(selector);
    return value.unwrap_or_default();
}

/// Sets the inner HTML of elements matching the given selector, if at least one exists.
pub fn try_inner_html(selector: &str, html: &str) {
    let _ = inner_html(selector, html);
}

/// Sets the inner text of elements matching the given selector, if at least one exists.
pub fn try_inner_text(selector: &str, text: &str) {
    let _ = inner_text(selector, text);
}

fn node_list_to_vec(list: NodeList) -> Vec<HtmlElement> {
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect()
}

/// Get a value set in any cookie.
pub fn get_cookie_value(name: &str) -> Option<String> {
    let window = web_sys::window().ok_or(Error).ok()?;
    let document = window.document().ok_or(Error).ok()?;

    let cookies = if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
        html_doc.cookie().unwrap_or_default()
    } else {
        String::new()
    };

    for cookie in cookies.split(';') {
        let cookie = cookie.trim();
        if let Some((key, value)) = cookie.split_once('=') {
            if key == name {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Set a cookie value.
pub fn set_cookie_value(name: &str, value: &str) -> Result<()> {
    let window = web_sys::window().ok_or(Error)?;
    let document = window.document().ok_or(Error)?;
    const MAX_AGE: u64 = 60 * 60 * 24 * 365 * 10; // in seconds

    if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
        let cookie = format!("{}={}; path=/; Max-Age={}", name, value, MAX_AGE);
        html_doc.set_cookie(&cookie).map_err(|_| Error)?;
        return Ok(());
    }

    Err(Error)
}

/// Set a session cookie, it will be deleted when the browser is closed.
pub fn set_session_cookie_value(name: &str, value: &str) -> Result<()> {
    let window = web_sys::window().ok_or(Error)?;
    let document = window.document().ok_or(Error)?;

    if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
        let cookie = format!("{}={}; path=/", name, value);
        html_doc.set_cookie(&cookie).map_err(|_| Error)?;
        return Ok(());
    }

    Err(Error)
}

/// A callback registerd with `#[callback(someFoo)]` can be retrieved via
/// `get_callback("someFoo")`. This is usefull for assigning the callback
/// to HTML elements without generating an HTML string.
///
/// Example:
/// ```rust
/// use web_sys::HtmlImageElement;
/// use gloo_console::log;
///
/// #[callback(imageLoaded)]
/// pub fn image_loaded_callback(event: web_sys::Event) {
///     log!("Image loaded. Processing it ...");
/// }
///
/// fn create_image() {
///     let window = web_sys::window().unwrap();
///     let document = window.document().unwrap();
///
///     let img = document
///         .create_element("img")
///         .unwrap()
///         .dyn_into::<HtmlImageElement>()
///         .unwrap();
///
///     let onload_callback = get_callback("imageLoaded");
///     img.set_onload(onload_callback.as_ref());
///
///     img.set_src(&blob_url);
/// }
/// ```
pub fn get_callback(name: &str) -> Option<js_sys::Function> {
    let window = web_sys::window()?;

    if let Ok(val) = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str(name)) {
        if let Ok(func) = val.dyn_into::<js_sys::Function>() {
            return Some(func);
        } else {
            log!(
                "Not a callback function: ",
                name,
                "Did you run init_callbacks()?"
            );
        }
    }
    None
}
