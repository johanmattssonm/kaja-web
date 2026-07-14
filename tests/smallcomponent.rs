#![allow(unreachable_code, dead_code)]

#[path = "common.rs"]
mod common;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_small_component() {
    common::setup_test();

    let content = html! {{
        <test-component></test-component>
    }};

    inner_html("#test-div", &content).unwrap();

    let element = get_element("test-component").unwrap();
    let id = element.id();
    assert_ne!(id, "");

    let component =
        Test1Component::get_component(id.as_str()).expect("Component not found in global store");

    assert_ne!(component.get_id(), "");
}

#[component("test-component")]
struct Test1 {}

impl Component for Test1Component {}
