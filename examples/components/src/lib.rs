use js_sys::Function;
use kaja_web::prelude::*;
use std::sync::RwLock;
use web_sys::HtmlElement;

pub static COUNTER_COMPONENTS: RwLock<Option<Counter>> = RwLock::new(None);

#[callback(counterComponentConnect)]
fn counter_connect(element: HtmlElement, wasm_id: u32) {
    log!("counterComponentConnect");
    let mut counter = COUNTER_COMPONENTS.write().unwrap();

    if counter.is_none() {
        error!("Component not found in connect.");
        return;
    }

    let mut counter_info: &mut Counter = counter.as_mut().unwrap();
    counter_info.connected(element);
}

#[callback(counterComponentDisconnect)]
fn counter_disconnect(element: HtmlElement, wasm_id: u32) {
    log!("counterComponentDisconnect");
    let mut counter = COUNTER_COMPONENTS.write().unwrap();

    if counter.is_none() {
        error!("Component not found in connect.");
        return;
    }

    let mut counter_info: &mut Counter = counter.as_mut().unwrap();
    counter_info.disconnected(element);
}

#[callback(counterInit)]
fn counter_init() {
    log!("counterInit");
    let counter = Counter::default();
    let mut counter_store = COUNTER_COMPONENTS.write().unwrap();
    *counter_store = Some(counter);
}

fn create_component(tag_name: &str) {
    let wasm_id = 0;

    let component = Function::new_no_args(
        r#"
        return class extends HTMLElement {
            let component_id = -1;

            connectedCallback() {
                if (component_id == -1) {
                    component_id = counter_init();
                }

                counterComponentConnect(this, component_id);
            }

            disconnectedCallback() {
                counterComponentDisconnect(this, component_id);
            }

            attributeChangedCallback(name, oldValue, newValue) {
                //callbacks.attributeChanged(this, name, oldValue, newValue);
            }

            static get observedAttributes() {
                return [];
            }
        }
    "#,
    );

    let class = component.call0(&JsValue::NULL);

    if class.is_err() {
        error!("Failed to register class for", tag_name);
        return;
    }

    let class_fn = class.unwrap();
    let registry = web_sys::window().unwrap().custom_elements();

    let result = registry.define(tag_name, &class_fn.unchecked_into());

    if result.is_err() {
        error!("Failed to register component", tag_name);
    }
}

trait Component {
    fn connected(&mut self, element: HtmlElement);
    fn disconnected(&mut self, element: HtmlElement);
    fn observed_attributes() -> &'static [&'static str] {
        &[]
    }

    fn attribute_changed(
        &mut self,
        element: HtmlElement,
        name: &str,
        old_value: &JsValue,
        new_value: &JsValue,
    ) {
        log!("Attribute changed but no update handler for ", name);
    }
}

//#[component]
#[derive(Default)]
struct Counter {
    value: usize,
}

impl Component for Counter {
    fn connected(&mut self, element: HtmlElement) {
        element.set_inner_html("Hello!");
    }

    fn disconnected(&mut self, element: HtmlElement) {
        element.set_inner_html("Bye");
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }

    fn attribute_changed(
        &mut self,
        element: HtmlElement,
        name: &str,
        old_value: &JsValue,
        new_value: &JsValue,
    ) {
        log!("change", name, old_value, new_value);
    }
}

fn init_components() {
    let _component = create_component("my-button");
}

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();
    init_components();
    init_callbacks();

    create_component("example-component")
}
