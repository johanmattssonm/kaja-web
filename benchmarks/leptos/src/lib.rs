use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0_u32);

    view! {
        <button id="counter" on:click=move |_| set_count.update(|count| *count = count.saturating_add(1))>
            "Clicks: "
            {move || count.get()}
        </button>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(Counter);
}
