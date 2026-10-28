use kaja_web::prelude::*;

#[component("main-component")]
struct Main {
    #[attribute]
    test: bool,
}

impl Component for MainComponent {
    fn connected(&self) {
        log!("Connected:", self.get_id());
    }

    fn disconnected(&self) {
        log!("Disconnected:", self.get_id());
    }

    fn attribute_changed(&self, name: &str, _old_value: &str, _new_value: &str) {
        if name == "test" {
            self.render();
        }
    }
}

impl MainComponent {
    fn render(&self) {
        let test: bool = self.get_attribute("test").unwrap_or_default();

        let content = html! {{
            <p>Test: $test</p>
        }};

        let id = self.get_id();

        if let Some(element) = get_element_by_id(&id) {
            element.set_inner_html(content.as_str());
        } else {
            error!("Element not found for component id:", id);
        }
    }
}

pub(crate) fn start_background_task() {
    log!("Starting background task");

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let elements = get_elements("main-component");

            for element in elements {
                let id = element.id();

                if id.is_empty() {
                    continue;
                }

                if let Some(element) = get_element_by_id(&id) {
                    let test = element
                        .get_attribute("test")
                        .unwrap_or_default()
                        .parse::<bool>()
                        .unwrap_or_default();
                    let test = !test;
                    let _ = element.set_attribute("test", test.to_string().as_str());
                } else {
                    error!("Element not found for component id:", id);
                }
            }

            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}
