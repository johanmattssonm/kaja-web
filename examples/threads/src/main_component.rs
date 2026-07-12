use kaja_web::prelude::*;
use std::time::Duration;

#[derive(Clone)]
#[component("main-component")]
struct Main {
    test: bool,
}

impl Component for Main {
    fn connected(id: &str) {
        start_background_thread(&id);
    }

    fn update(id: &str) {
        log!(
            "Update from background thread, running on the UI thread:",
            id
        );

        let Some(element) = get_element_by_id(id) else {
            error!("Element not found:", id);
            return;
        };

        let Some(component) = Self::read_component(id) else {
            error!("Main component not found:", id);
            return;
        };

        component.render(&element);
    }
}

impl Main {
    fn render(&self, element: &HtmlElement) {
        element.set_inner_text(&self.test.to_string());
    }
}

fn start_background_thread(id: &str) {
    let component_id = id.to_string();
    let Some(handle) = Main::get_component(id) else {
        error!("Main component not found:", id);
        return;
    };

    let _ = crate::thread::spawn(move || {
        loop {
            update_shared_component(&handle, |component| {
                component.test = !component.test;
            });

            Main::post_ui_thread_update(component_id.as_str());
            std::thread::sleep(Duration::from_millis(1000));

            // this a background thread, so:
            // try_inner_html("#test", "<b>INNER_HTML</b>");
            // will not update the DOM. Only the UI thread can do that.
            //
            // Run post_ui_thread_update to schedule a UI thread update
            // using the Component::update method.
        }
    });
}
