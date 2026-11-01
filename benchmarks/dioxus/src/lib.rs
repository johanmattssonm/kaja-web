use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0_u32);

    rsx! {
        button {
            id: "counter",
            onclick: move |_| count += 1,
            "Clicks: {count}"
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    dioxus::launch(Counter);
}
