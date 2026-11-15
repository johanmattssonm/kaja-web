use std::sync::Once;

use kaja_web::prelude::*;
use web_sys::HtmlElement;

static INIT_CALLBACKS_ONCE: Once = Once::new();

pub fn setup_test() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    INIT_CALLBACKS_ONCE.call_once(init_callbacks);

    let test_div = document.get_element_by_id("test-div");

    if let Some(test_div) = test_div {
        let test_div = test_div.unchecked_into::<HtmlElement>();
        test_div.set_inner_html("");
        return;
    }

    let test_div = document.create_element("div").unwrap();
    test_div.set_id("test-div");
    body.append_child(&test_div).unwrap();
}
