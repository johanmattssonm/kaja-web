use kaja_web::prelude::*;

#[component("hello-card")]
struct HelloCard;

impl Component for HelloCardComponent {
    fn connected(&self) {
        let content = html! {{
            <strong>Hello from Rust!</strong>
        }};

        if let Some(element) = get_element_by_id(&self.get_id()) {
            element.set_inner_html(content.as_str());
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    log!("Kaja Web template started.");
    init_callbacks();
}
