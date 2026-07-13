use kaja_web::prelude::*;
use web_sys::HtmlElement;

pub fn setup_test() {
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
