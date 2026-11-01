use kaja_web::prelude::*;
use std::cell::Cell;

thread_local! {
    static CLICK_COUNT: Cell<u32> = const { Cell::new(0) };
}

#[component("kaja-counter")]
struct KajaCounter;

impl Component for KajaCounterComponent {
    fn connected(&self) {
        let content = html! {{
            <button id="counter" onclick="incrementCounter()">Clicks: 0</button>
        }};

        if let Some(element) = get_element_by_id(&self.get_id()) {
            element.set_inner_html(content.as_str());
        }
    }
}

#[callback(incrementCounter)]
fn increment_counter() {
    CLICK_COUNT.with(|count| {
        let next = count.get().saturating_add(1);
        count.set(next);

        if let Some(element) = get_element_by_id("counter") {
            element.set_text_content(Some(&format!("Clicks: {next}")));
        }
    });
}

#[wasm_bindgen(start)]
pub fn main() {
    init_callbacks();
}
