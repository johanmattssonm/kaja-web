use kaja_web::prelude::*;

mod counter;
mod main_component;

use counter::*;
use main_component::*;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    init_callbacks();
}
