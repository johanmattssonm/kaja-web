#[path = "common.rs"]
mod common;

use std::borrow;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_new_component() {
    common::setup_test();

    let id = "test-id-for-component";
    let content = html! {{
        <test-get-set-id id="$id" test_attribute="initial-value-for-attribute"></test-get-set-id>
    }};

    inner_html("#test-div", &content).unwrap();

    let component = Test2Component::get_component(id).expect("Component not found in global store");
    let attr = component.get_attribute("number").unwrap();
    assert_eq!(attr, "42");

    {
        component.set_attribute("number", "100");
        let attr = component.get_attribute("number").unwrap();
        assert_eq!(attr, "100");

        let test2_inner = component.get_attributes();
        let test2 = test2_inner.borrow();
        log!("test2 number:", test2.number);
    }

    component.set_attribute("number", "SOMETHING_NOT_i32"); // NAN

    {
        let attr = component.get_attribute("number").unwrap();
        assert_eq!(attr, "100");

        let test2_inner = component.get_attributes();
        let test2 = test2_inner.borrow();
        log!("test2 number:", test2.number);
    }
}

#[component("test-get-set-id")]
struct Test2 {
    test: bool,
    number: i32,
}

impl Component for Test2Component {
    fn connected(&self) {
        log!("Connected:", self.get_id());
        log!("Set number");

        let id = self.get_id();
        let element = get_element_by_id(&id).unwrap();
        let res = element.set_attribute("number", "42");
        assert!(res.is_ok());
    }

    fn disconnected(&self) {
        log!("Disconnected:", self.get_id());
    }

    fn attribute_changed(&self, name: &str, old_value: &str, new_value: &str) {
        log!(
            "Attribute changed:",
            name,
            "old:",
            old_value,
            "new:",
            new_value
        );
    }
}
