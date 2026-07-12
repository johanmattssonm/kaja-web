use kaja_web::prelude::*;

#[derive(Clone)]
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
        log!("Main update:", id);

        let Some(element) = get_element_by_id(id) else {
            error!("Element not found:", id);
            return;
        };

        let Some(handle) = Self::get_component(id) else {
            error!("Main component not found:", id);
            return;
        };

        let component = update_shared_component(&handle, |component| {
            component.test = !component.test;
        });

        let comp = handle.borrow();
        comp.render(&element);
    }
}

impl Main {
    fn render(&self, element: &HtmlElement) {
        log!("Render Main", self.test);
        element.set_inner_text(&self.test.to_string());
    }
}

fn start_background(id: &str) {
    let component_id = id.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            Main::update(component_id.as_str());
            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}
