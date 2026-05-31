use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};

pub struct InitFn(pub fn());
inventory::collect!(InitFn);

#[derive(Debug, Clone, Copy)]
pub struct Error;

type Result<T> = std::result::Result<T, Error>;

pub struct JsCallItem {
    pub name: &'static str,
    pub install: fn(),
}

pub struct App {}

pub struct CallbackRegistration {
    pub register: fn(),
}

unsafe impl Sync for CallbackRegistration {}

inventory::collect!(CallbackRegistration);

pub fn init_callbacks() {
    for init in inventory::iter::<InitFn> {
        (init.0)();
    }
}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn set_callbacks(&self) {
        for reg in inventory::iter::<CallbackRegistration> {
            (reg.register)();
        }
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
        log!(
            "Expecting a single element, found {} instead.",
            elements.length()
        );
    }

    if let Some(node) = elements.item(0) {
        if let Ok(element) = node.dyn_into::<HtmlElement>() {
            return Some(element);
        }
    }

    None
}

pub fn node_list_to_vec(list: NodeList) -> Vec<HtmlElement> {
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect()
}

/// Queries the document for all elements matching the given selector.
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
        log!("Failed to query selector all: {:?}", selector);
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
/// Returns an error if no element is found.
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

/// Sets the inner HTML of the first element matching the given selector, if one exists.
pub fn try_inner_html(selector: &str, html: &str) {
    let _ = inner_html(selector, html);
}

/// Sets the inner text of the first element matching the given selector, if one exists.
pub fn try_inner_text(selector: &str, text: &str) {
    let _ = inner_text(selector, text);
}

#[macro_export]
macro_rules! callback {
    ($js_name:ident, $rust_fn:path) => {{
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use wasm_bindgen::JsValue;

        let window = web_sys::window().unwrap();

        let cb = Closure::<dyn FnMut(JsValue)>::new(|val: JsValue| {
            let event: _ =
                serde_wasm_bindgen::from_value(val).expect("Failed to deserialize event");

            $rust_fn(event);
        });

        ::js_sys::Reflect::set(
            &window,
            &stringify!($js_name).into(),
            cb.as_ref().unchecked_ref(),
        )
        .unwrap();

        cb.forget();
    }};
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use crate::App;
    use crate::CallbackRegistration;
    use gloo_console::log;
    use js_sys;
    #[derive(Debug, serde::Deserialize)]
    struct SomeClickEvent {
        índex: usize,
    }

    #[kaja_callback_macro::callback("someClickCallback")]
    fn some_on_click_function(event: SomeClickEvent) {
        log!("event.índex: {:?}", event.índex);
    }

    #[test]
    fn test_compile() {
        let app = App::new();
        app.set_callbacks(); // 👈 registers ALL #[callback] functions automatically
    }
}
