#![cfg(target_arch = "wasm32")]
use gloo::console::log;
use kaja_callback_macro::callback;
use kaja_html_macro::html;
use kaja_web::*;
use serde::Deserialize;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;

pub struct AppData {
    current_count: isize,
}

#[derive(Deserialize)]
pub struct Test1Data {
    test_parameter: String,
}

pub static APP_DATA: RwLock<AppData> = RwLock::new(AppData { current_count: 0 });

fn update(app_data: &AppData) {
    let count = app_data.current_count;

    let counter = html! {{
        <button onclick="test0()">
            Test 0
        </button>

        <button onclick="test1({test_parameter: 'just a test'})">
            Test 1
        </button>

        <button onclick="test2({test_parameter: 'just a test'}, 'string_param')">
            Test 2
        </button>

        <button onclick="test3(1, 2)">
            Test 3
        </button>
    }};

    let result = inner_html("#quotes", counter.as_str());

    if result.is_err() {
        log!("Failed to set inner HTML for '#quotes'.");
    }
}

#[callback(test0)]
pub fn test0() {
    log!("test0 called");
}

#[callback(test1)]
pub fn test1(arg0_rust: Test1Data) {
    log!("test1 called with parameter: {}", arg0_rust.test_parameter);
}

#[callback(test2)]
pub fn test2(arg0_rust: Test1Data, arg1_rust: String) {
    log!(
        "test2 called with parameter: {} and string param: {}",
        arg0_rust.test_parameter,
        arg1_rust
    );
}

#[callback(test3)]
pub fn test3(a: u32, b: u32, c: u32) {
    log!("test3 {}, {} and {}", a, b, c);
}

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    let mut app_data = APP_DATA.read().unwrap();
    update(&app_data);

    init_callbacks();
}
