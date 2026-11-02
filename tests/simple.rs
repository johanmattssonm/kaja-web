use kaja_web::{get_cookie_value, get_value, inner_html, set_cookie_value};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_dom() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    assert!(document.body().is_some());
}

#[wasm_bindgen_test]
fn test_get_value_from_textarea() {
    inner_html("body", r#"<textarea id="message">Hello</textarea>"#).unwrap();

    assert_eq!(get_value("#message"), Some("Hello".to_owned()));
}

#[wasm_bindgen_test]
fn test_get_value_from_select() {
    inner_html(
        "body",
        r#"<select id="country"><option value="se">Sweden</option><option value="no" selected>Norway</option></select>"#,
    )
    .unwrap();

    assert_eq!(get_value("#country"), Some("no".to_owned()));
}

#[wasm_bindgen_test]
fn test_cookie() {
    set_cookie_value("test-cookie-value", "some-value").unwrap();

    let val = get_cookie_value("test-cookie-value").unwrap();
    assert_eq!(val, "some-value");

    let not_in_cookie = get_cookie_value("not-in-cookie");
    assert!(not_in_cookie.is_none());
}
