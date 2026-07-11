use arc_swap::ArcSwap;
use kaja_web::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::time::Duration;

static THREAD_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static THREAD_COMPONENT_MAP: LazyLock<RwLock<HashMap<String, ThreadComponent>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

struct ThreadComponent {
    id: u32,
    ui: Arc<ArcSwap<Main>>, // read only for use in ui foreground thread
    bg: Arc<RwLock<Main>>,  // read/write in background thread with mutex
}

impl ThreadComponent {
    fn shallow_clone(&self) -> Self {
        Self {
            id: THREAD_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ui: self.ui.clone(),
            bg: self.bg.clone(),
        }
    }

    fn new(component: Main, component_id: &str) -> Self {
        let bg = component.clone();

        let thread_component = Self {
            id: THREAD_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ui: Arc::new(ArcSwap::from_pointee(component)),
            bg: Arc::new(RwLock::new(bg)),
        };

        THREAD_COMPONENT_MAP
            .write()
            .unwrap()
            .insert(component_id.to_string(), thread_component.shallow_clone());
        thread_component
    }

    fn lock_component<F>(&self, f: F)
    where
        F: FnOnce(&mut Main),
    {
        let mut bg = self.bg.write().unwrap();
        f(&mut bg);
        self.ui.store(Arc::new((*bg).clone()));
    }
}

fn start_background_threads(id: &str) {
    let handle = Main::get_component(id).expect("Component not found");
    let component = handle.lock().unwrap().clone();

    let thread_component = ThreadComponent::new(component, id);

    start_background_thread(thread_component.shallow_clone(), id.to_string());
    start_background_thread(thread_component.shallow_clone(), id.to_string());
}

fn start_background_thread(thread_component: ThreadComponent, component_id: String) {
    let _ = crate::thread::spawn(move || {
        loop {
            thread_component.lock_component(|component| {
                component.test = !component.test;
                log!("bg test:", component.test, thread_component.id);
            });

            let ui = thread_component.ui.load();
            log!("ui test:", ui.test, thread_component.id);

            Main::post_ui_thread_update(component_id.as_str());

            let sleep_ms = (js_sys::Math::random() * 1000.0) as u64;
            std::thread::sleep(Duration::from_millis(sleep_ms));
        }
    });
}

#[derive(Clone)]
#[component("main-component")]
struct Main {
    test: bool,
}

impl Component for Main {
    fn connected(id: &str) {
        start_background_threads(&id);
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

        let component = {
            let map = THREAD_COMPONENT_MAP.read().unwrap();
            let Some(thread_component) = map.get(id) else {
                error!("Thread component not found:", id);
                return;
            };

            thread_component.ui.load_full()
        };

        log!("Rendering component, loaded read only:", id);
        component.render(&element);
    }
}

impl Main {
    fn render(&self, element: &HtmlElement) {
        element.set_inner_text(&self.test.to_string());
    }
}
