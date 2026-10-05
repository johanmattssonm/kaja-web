#[path = "common.rs"]
mod common;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_attributes_in_component() {
    common::setup_test();

    let id = "test-id-for-component";
    let content = html! {{
        <test-get-set-id
            id="$id"
            number="1"
            test_attribute="initial-value-for-attribute">
        </test-get-set-id>
    }};

    inner_html("#test-div", &content).unwrap();

    let component = Test2Component::get_component(id).expect("Component not found in global store");

    let attr = component.get_attribute_string("number").unwrap();
    assert_eq!(attr, "1");

    let typed_attr: i32 = component.get_attribute("number").unwrap();
    assert_eq!(typed_attr, 1);

    let _ = component.set_attribute("number", "100");
    let attr = component.get_attribute_string("number").unwrap();
    assert_eq!(attr, "100");

    let typed_attr = component.get_attribute::<i32>("number").unwrap();
    assert_eq!(typed_attr, 100);

    let _ = component.set_attribute("number", "SOMETHING_NOT_i32"); // NAN

    let attr = component.get_attribute_string("number").unwrap();
    assert_eq!(attr, "100");

    let typed_attr: i32 = component.get_attribute("number").unwrap();
    assert_eq!(typed_attr, 100);

    {
        let test2_inner = component.get_attributes();
        let test2 = test2_inner.borrow();
        log!("test2 number:", test2.number);
    }

    let element = get_element_by_id(id);
    assert!(element.is_some());

    let element = element.unwrap();
    let err = element.set_attribute("number", "SOMETHING_NOT_i32_in_JS");
    assert!(err.is_err());
}

#[component("test-get-set-id")]
struct Test2 {
    #[attribute]
    test: bool,

    #[attribute]
    number: i32,
}

impl Component for Test2Component {
    fn connected(&self) {
        log!("Connected:", self.get_id());
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
