use crate::main_component::init_test_loop;
use kaja_web::prelude::*;
mod main_component;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    init_callbacks();
    init_test_loop();
}
