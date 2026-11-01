use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Counter)]
fn counter() -> Html {
    let count = use_state(|| 0_u32);
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set((*count).saturating_add(1)))
    };

    html! {
        <button id="counter" {onclick}>{format!("Clicks: {}", *count)}</button>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Renderer::<Counter>::new().render();
}
