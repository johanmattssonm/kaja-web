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
        <test-component1 id="custom-id" test_attribute="initial-value"></test-component1>
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
        .set_attribute("test_attribute", "updated-value")
        .unwrap();

    {
        let component = TestComponent1::get_component("custom-id").unwrap();
        let test_component = component.borrow();

        assert_eq!(test_component.test_attribute, "updated-value");
        assert!(test_component.attribute_updated);
    }
}

#[wasm_bindgen_test]
fn test_getter_and_setter() {
    setup_test();

    let content = html! {{
        <test-component1 id="test-get-set-id" test_attribute="initial-value-for-attribute"></test-component1>
    }};

    try_inner_html("#test-div", &content);

    let id = "test-get-set-id";

    {
        let component = TestComponent1::get_component(id).unwrap();
        let test_component = component.borrow();

        assert_eq!(test_component.test_attribute, "initial-value-for-attribute");
    }

    {
        // borrows again
        let res = TestComponent1::set_test_attribute(id, "updated-value-in-component".to_string());
        assert!(res.is_ok());
    }

    let test_attribute_from_getter = TestComponent1::get_test_attribute(id).unwrap();
    assert_eq!(test_attribute_from_getter, "updated-value-in-component");
}

#[component("test-component1")]
struct TestComponent1 {
    tested: bool,
    attribute_updated: bool,

    //#[attribute]
    test_attribute: String,
}

impl TestComponent1 {
    fn set_test_attribute(id: &str, value: String) -> Result<()> {
        let attribute_name = "test_attribute";
        let attribute_string = value.clone();

        let element = get_element_by_id(id).ok_or(Error)?;

        {
            let component = Self::get_component(id).ok_or(Error)?;
            let mut component = component.try_borrow_mut().map_err(|_| Error)?;

            component.test_attribute = value;
        }

        element
            .set_attribute(attribute_name, &attribute_string)
            .map_err(|_| Error)
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["test_attribute"]
    }

    fn get_test_attribute(id: &str) -> Option<String> {
        let component = Self::get_component(id)?;
        let test_attribute = component.borrow().test_attribute.clone();
        Some(test_attribute)
    }
}

impl Component for TestComponent1 {
    fn connected(id: &str) {
        let component = TestComponent1::get_component(id).unwrap();
        let mut test_component = component.borrow_mut();
        test_component.tested = true;
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["test_attribute"]
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
