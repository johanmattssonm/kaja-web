use wasm_bindgen_test::*;

pub mod common;
pub use common::setup_test;

wasm_bindgen_test_configure!(run_in_browser);

mod componentref;
mod simple;
mod smallcomponent;
