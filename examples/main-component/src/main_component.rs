use crate::log;
use async_channel::{Receiver, Sender};
use kaja_web::update;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, OnceLock, RwLock};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct Main {
    test: bool,
}

#[derive(Default)]
struct Other {
    the_number: i32,
}

trait Component {
    fn update(id: &str)
    where
        Self: Sized;
}

impl Component for Main {
    fn update(id: &str) {
        log!("Main update:", id);
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

impl Main {
    fn get_component_arc(_id: &str) -> Arc<Mutex<Main>> {
        Arc::new(Mutex::new(Main::default()))
    }
}

impl Component for Other {
    fn update(id: &str) {
        log!("Other update:", id);
    }
}

static UPDATE_SENDER: OnceLock<Sender<String>> = OnceLock::new();

pub fn schedule_update(id: &str) {
    if let Some(sender) = UPDATE_SENDER.get() {
        match sender.try_send(id.to_string()) {
            Ok(_) => {
                log!("scheduled update for:", id);
            }
            Err(e) => {
                log!("could not schedule update for", id);
            }
        }
    }
}

async fn ui_update_loop(receiver: Receiver<String>) {
    while let Ok(id) = receiver.recv().await {
        update(&id);
    }
}

static MAIN_COMPONENT: LazyLock<RwLock<HashMap<String, Arc<Mutex<Main>>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

fn update_component_main(id: &str) -> bool {
    {
        let lock = MAIN_COMPONENT
            .read()
            .expect("Failed to lock MAIN_COMPONENT");

        if !lock.contains_key(id) {
            return false;
        }
    }

    Main::update(id);
    true
}

#[cfg(not(feature = "threads"))]
fn start_background(sender: Sender<String>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            schedule_update("main-component-1"); // await?
            schedule_update("other-component-1"); // await?

            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}

#[cfg(feature = "threads")]
fn start_background(sender: Sender<String>) {
    std::thread::spawn(move || {
        loop {
            schedule_update("main-component-1");
            schedule_update("other-component-1");

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

pub fn init_test_loop() {
    let (sender, receiver) = async_channel::unbounded::<String>();

    UPDATE_SENDER
        .set(sender.clone())
        .expect("UPDATE_SENDER already initialized");

    // This runs on the browser UI thread
    wasm_bindgen_futures::spawn_local(ui_update_loop(receiver));

    #[cfg(feature = "threads")]
    start_background(sender);
}
