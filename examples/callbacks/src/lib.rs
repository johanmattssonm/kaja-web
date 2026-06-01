use kaja_web::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Test1Data {
    test_parameter: String,
}

fn update() {
    let counter = html! {{
        <button onclick="test0()">
            Test 0
        </button>

        <button onclick="test1({test_parameter: 'just a test'})">
            Test 1
        </button>

        <button onclick="test2({test_parameter: 'just a test'}, 'string_param'">
            Test 2
        </button>

        <button onclick="test3(1, 2, 3)">
            Test 3
        </button>
    }};

    let result = inner_html("#main-view", counter.as_str());

    if result.is_err() {
        log!("Failed to set inner HTML for '#main-view'.");
    }
}

fn set_color(color: &str) {
    let main = get_element("#main-view");

    if let Some(main) = main {
        let _ = main.style().set_property("background-color", color);
    }
}

#[callback(test0)]
pub fn test0() {
    log!("test0 called");
    set_color("red");
}

#[callback(test1)]
pub fn test1(arg0_rust: Test1Data) {
    log!("test1 called with parameter: {}", arg0_rust.test_parameter);
    set_color("blue");
}

#[callback(test2)]
pub fn test2(arg0_rust: Test1Data, arg1_rust: String) {
    log!(
        "test2 called with parameter: {} and string param: {}",
        arg0_rust.test_parameter,
        arg1_rust
    );
    set_color("green");
}

#[callback(test3)]
pub fn test3(a: u32, b: u32, c: u32) {
    log!("test3 {}, {} and {}", a, b, c);
    set_color("pink");
}

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    update();
    init_callbacks();
}
