#![cfg(target_arch = "wasm32")]
use kaja_web::prelude::*;
use std::sync::RwLock;

struct AppData {
    current_count: isize,
}

pub static APP_DATA: RwLock<AppData> = RwLock::new(AppData { current_count: 0 });

fn update_counter_display(app_data: &AppData) {
    let count = app_data.current_count;

    let counter = html! {{
        <div class="click-counter-display">$count</div>

        <button class="click-counter-increment-button"
            onclick="incrementCounter()">
            Increment
        </button>

        <button class="click-counter-decrement-button"
            onclick="decrementCounter()">
            Decrement
        </button>
    }};

    let result = inner_html("#click-counter", counter.as_str());

    if result.is_err() {
        log!("Failed to set inner HTML for '#click-counter'.");
    }
}

#[callback(incrementCounter)]
pub fn increment_counter() {
    let mut app_data = APP_DATA.write().unwrap();
    app_data.current_count = app_data.current_count.saturating_add(1);
    update_counter_display(&app_data);
}

#[callback(decrementCounter)]
pub fn decrement_counter() {
    let mut app_data = APP_DATA.write().unwrap();
    app_data.current_count = app_data.current_count.saturating_sub(1);
    update_counter_display(&app_data);
}

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_error_panic_hook::set_once();

    let mut app_data = APP_DATA.read().unwrap();
    update_counter_display(&app_data);

    init_callbacks();
}
