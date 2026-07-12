use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

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
        <test-component2 id="custom-id"></test-component2>
    }};

    try_inner_html("#test-div", &content);

    let element = get_element("test-component2").unwrap();
    assert_eq!(element.id(), "custom-id");

    let component = TestComponent2::get_component("custom-id").unwrap();
    let test_component = component.borrow();
    assert!(test_component.tested);
}

#[component("test-component1")]
struct TestComponent1 {
    tested: bool,
}

impl Component for TestComponent1 {
    fn connected(id: &str) {
        let component = TestComponent1::get_component(id).unwrap();
        let mut test_component = component.borrow_mut();
        test_component.tested = true;
    }
}

#[component("test-component2")]
struct TestComponent2 {
    tested: bool,
}

impl Component for TestComponent2 {
    fn connected(id: &str) {
        log!("connected TestComponent2:", id);
        let component = TestComponent2::get_component(id).unwrap();
        let mut test_component = component.borrow_mut();
        test_component.tested = true;
    }
}
