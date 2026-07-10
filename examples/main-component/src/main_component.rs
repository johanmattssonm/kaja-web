use kaja_web::prelude::*;

#[component("main-component")]
struct Main {
    test: bool,
}

#[component("other-component")]
struct Other {
    the_number: i32,
}

impl Component for Other {}

impl Component for Main {
    fn connected(id: &str) {
        log!("Main connected:", id);
        start_background(&id);
    }

    fn update(id: &str) {
        log!("Main ui_thread_update:", id);

        let Some(handle) = Self::get_component(id) else {
            error!("Main component not found:", id);
            return;
        };

        let Some(element) = get_element_by_id("2") else {
            error!("Element 2 not found:");
            return;
        };

        log!("Element 2 found");

        let Some(element) = get_element_by_id(id) else {
            error!("Element not found:", id);
            return;
        };

        let mut component = handle.lock().unwrap();
        component.test = !component.test;
        component.render(&element);
    }
}

impl Main {
    fn render(&self, element: &HtmlElement) {
        log!("Render Main", self.test);
        element.set_inner_text(&self.test.to_string());
    }
}

#[cfg(not(feature = "threads"))]
fn start_background(id: &str) {
    let component_id = id.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            Main::post_ui_thread_update(component_id.as_str());
            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}

#[cfg(feature = "threads")]
fn start_background(id: &str) {
    let component_id = id.to_string();

    std::thread::spawn(move || {
        loop {
            Main::post_ui_thread_update(component_id.as_str());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
