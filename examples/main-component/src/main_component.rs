use crate::log;
use async_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, OnceLock, RwLock};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[component("main-component")]
struct Main {
    test: bool,
}

#[component("other-component")]
struct Other {
    the_number: i32,
}

impl Component for Main {
    fn connected(id: &str) {
        #[cfg(feature = "threads")]
        start_background(id);
    }

    fn update(id: &str) {
        log!("Main ui_thread_update:", id);
        let handle = Self::get_component_arc(id);
        let mut component = handle.lock().unwrap();

        component.test = !component.test;
        component.render(id);
    }
}

impl Main {
    fn render(&self, id: &str) {
        log!("Render Main", self.test);
        let element = get_element_by_id(id);
        element.inner_text(&self.test.to_string());
    }
}

impl Component for Other {
    fn ui_thread_update(id: &str) {
        log!("Other update:", id);
    }
}

#[cfg(not(feature = "threads"))]
fn start_background(id: &str) {
    let component_id = id.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            post_ui_thread_update(component_id.as_str()); // await?
            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}

#[cfg(feature = "threads")]
fn start_background(id: &str) {
    let component_id = id.to_string();

    std::thread::spawn(move || {
        loop {
            post_ui_thread_update(component_id.as_str());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
