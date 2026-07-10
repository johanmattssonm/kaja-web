use kaja_web::prelude::*;
use std::time::Duration;

thread_local! {
    static BACKGROUND_POOL: std::cell::RefCell<Option<crate::pool::WorkerPool>> =
        const { std::cell::RefCell::new(None) };
}

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

        let Some(handle) = Self::get_component(id) else {
            error!("Main component not found:", id);
            return;
        };

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
        element.set_inner_text(&self.test.to_string());
    }
}

fn start_background_thread(id: &str) {
    let component_id = id.to_string();

    BACKGROUND_POOL.with(|pool_slot| {
        if pool_slot.borrow().is_none() {
            match crate::pool::WorkerPool::new(1) {
                Ok(pool) => {
                    *pool_slot.borrow_mut() = Some(pool);
                }
                Err(err) => {
                    error!("Failed to create worker pool for background thread");
                    crate::logv(&err);
                    return;
                }
            }
        }

        let pool_ref = pool_slot.borrow();
        let pool = pool_ref.as_ref().unwrap();

        if let Err(err) = pool.run(move || {
            loop {
                Main::post_ui_thread_update(component_id.as_str());
                std::thread::sleep(Duration::from_millis(1000));

                // this a background thread, so:
                // try_inner_html("#test", "<b>INNER_HTML</b>");
                // will not update the DOM. Only the UI thread can do that.
            }
        }) {
            error!("Failed to spawn background worker task");
            crate::logv(&err);
        }
    });
}
