use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

fn setup_test() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let test_div = document.get_element_by_id("test-div");

    if test_div.is_some() {
        let test_div = test_div.unwrap().unchecked_into::<HtmlElement>();
        test_div.set_inner_html("");
        return;
    }

    let test_div = document.create_element("div").unwrap();
    test_div.set_id("test-div");
    body.append_child(&test_div).unwrap();

    init_callbacks();
}

#[wasm_bindgen_test]
fn test_component() {
    setup_test();

    let content = html! {{
        <test-component1></test-component1>
    }};

    try_inner_html("#test-div", &content);

    let element = get_element("test-component1").unwrap();
    let id = element.id();
    log!("id", &id);
    assert!(id.starts_with("test-component1-"));

    let component = TestComponent1::get_component(&id).unwrap();
    let test_component = component.borrow();
    assert!(test_component.tested);
}

#[wasm_bindgen_test]
fn test_custom_id() {
    setup_test();

    let content = html! {{
        <test-component1 id="custom-id"></test-component1>
    }};

    try_inner_html("#test-div", &content);

    let element = get_element("test-component1").unwrap();
    assert_eq!(element.id(), "custom-id");

    let component = TestComponent1::get_component("custom-id").unwrap();
    let test_component = component.borrow();
    assert!(test_component.tested);
}

#[wasm_bindgen_test]
fn test_attribute_updated() {
    setup_test();

    let content = html! {{
        <test-component1 id="custom-id" test-attribute="initial-value"></test-component1>
    }};

    try_inner_html("#test-div", &content);

    let element = get_element("test-component1").unwrap();

    {
        let component = TestComponent1::get_component("custom-id").unwrap();
        let test_component = component.borrow();

        assert!(test_component.tested);
        assert!(test_component.attribute_updated);
        assert_eq!(test_component.test_attribute, "initial-value"); // value on struct not on element
    }

    element
        .set_attribute("test-attribute", "updated-value")
        .unwrap();

    {
        let component = TestComponent1::get_component("custom-id").unwrap();
        let test_component = component.borrow();

        assert_eq!(test_component.test_attribute, "updated-value");
        assert!(test_component.attribute_updated);
    }
}

#[component("test-component1")]
struct TestComponent1 {
    tested: bool,
    attribute_updated: bool,
    test_attribute: String,
}

impl Component for TestComponent1 {
    fn connected(id: &str) {
        let component = TestComponent1::get_component(id).unwrap();
        let mut test_component = component.borrow_mut();
        test_component.tested = true;
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["test-attribute"]
    }

    fn attribute_changed(id: &str, name: &str, _old_value: &str, new_value: &str) {
        log!(
            "running attribute_changed for ",
            name,
            " new_value: ",
            new_value
        );

        let component = TestComponent1::get_component(id).unwrap();
        let mut test_component = component.borrow_mut();
        test_component.attribute_updated = true;
        test_component.test_attribute = new_value.to_string();
    }
}
