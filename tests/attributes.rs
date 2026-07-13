#[path = "common.rs"]
mod common;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

use std::str::FromStr;

fn parse_value<T>(s: &str) -> Option<T>
where
    T: FromStr,
{
    s.parse::<T>().ok()
}

fn set() {
    let n: u32 = parse_value("42").unwrap();
    let f: f64 = parse_value("3.14").unwrap();
    let b: bool = parse_value("true").unwrap();
}

#[wasm_bindgen_test]
fn test_getter_and_setter() {
    common::setup_test();

    let id = "test-get-set-id";
    let content = html! {{
        <test-component2 id="$id" test_attribute="initial-value-for-attribute"></test-component1>
    }};

    inner_html("#test-div", &content).unwrap();

    {
        // borrows first time and release
        let component = TestComponent2::get_component(id).unwrap();
        let test_component = component.borrow();
        assert_eq!(test_component.test_attribute, "initial-value-for-attribute");
    }

    {
        // borrows again
        let res = TestComponent2::set_test_attribute(id, "updated-value-in-component".to_string());
        assert!(res.is_ok());
    }

    let test_attribute_from_getter = TestComponent2::get_test_attribute(id).unwrap();
    assert_eq!(test_attribute_from_getter, "updated-value-in-component");
}

#[component("test-component2")]
struct TestComponent2 {
    attribute_updated: bool,

    //#[html_attribute]
    test_attribute: String,
}

impl TestComponent2 {
    fn set_test_attribute(id: &str, value: String) -> Result<()> {
        let attribute_name = "test_attribute";
        let attribute_string = value.clone();

        let element = get_element_by_id(id).ok_or(Error)?;

        return element
            .set_attribute(attribute_name, &attribute_string)
            .map_err(|_| Error);
    }

    fn get_test_attribute(id: &str) -> Option<String> {
        let component = Self::get_component(id)?;
        let test_attribute = component.borrow().test_attribute.clone();
        Some(test_attribute)
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["test_attribute"]
    }

    fn attribute_changed_update_struct(id: &str, name: &str, _old_value: &str, new_value: &str) {
        log!("Attribute changed internally", name, "to", new_value);
        let component = Self::get_component(id);

        if component.is_none() {
            error!("No component found for", id);
            return;
        }

        let component = component.unwrap();
        let component = component.try_borrow_mut();

        if component.is_err() {
            error!(
                "Failed to borrow component for",
                id, "in attribute_changed of ", name
            );
            return;
        }

        if name == "test_attribute" {
            component.unwrap().test_attribute = new_value.to_string();
        }
    }
}

impl Component for TestComponent2 {
    fn attribute_changed(id: &str, name: &str, old_value: &str, new_value: &str) {
        Self::attribute_changed_update_struct(id, name, old_value, new_value);
        log!("Attribute changed", name, "to", new_value);
    }
}
