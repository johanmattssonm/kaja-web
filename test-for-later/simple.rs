use kaja_web::{get_cookie_value, set_cookie_value};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_dom() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    assert!(document.body().is_some());
}

#[wasm_bindgen_test]
fn test_cookie() {
    set_cookie_value("test-cookie-value", "some-value").unwrap();

    let val = get_cookie_value("test-cookie-value").unwrap();
    assert_eq!(val, "some-value");

    let not_in_cookie = get_cookie_value("not-in-cookie");
    assert!(not_in_cookie.is_none());
}
