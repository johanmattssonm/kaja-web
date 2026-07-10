use kaja_web::prelude::*;

mod main_component;

#[wasm_bindgen(start)]
pub fn init() {
    log!("Starting");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    log!("Init callbacks");
    init_callbacks();
    log!("Init complete");
}
