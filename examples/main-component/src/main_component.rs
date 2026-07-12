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

impl Component for Main {
    fn connected(id: &str) {
        log!("Main connected:", id);
        start_background_task(&id);
    }

    fn update(id: &str) {
        log!("Main update:", id);

        let Some(element) = get_element_by_id(id) else {
            error!("Main element not found:", id);
            return;
        };

        let Some(handle) = Self::get_component(id) else {
            error!("Main component not found:", id);
            return;
        };

        let mut component = handle.borrow_mut();
        component.test = !component.test;

        component.render(&element);
    }
}

impl Component for Other {
    fn connected(id: &str) {
        log!("Other connected:", id);

        let Some(handle) = Self::get_component(&id) else {
            error!("Other component not found:", id);
            return;
        };

        let mut component = handle.borrow_mut();
        component.the_number += 1;
        component.render(id);
    }

    fn update(id: &str) {
        log!("Other update:", id);
    }
}

impl Other {
    fn render(&self, id: &str) {
        let Some(element) = get_element_by_id(id) else {
            error!("Other element not found:", id);
            return;
        };

        let content = html! {{
            <div>The number: $(self.the_number)</div>
        }};

        log!("Render Other");
        element.set_inner_html(&content);
    }
}

impl Main {
    fn render(&self, element: &HtmlElement) {
        let content = html! {{
            <div>Known: $(self.test)</div>
            <other-component></other-component>
        }};

        log!("Render Main", self.test);
        element.set_inner_html(&content);
    }
}

fn start_background_task(id: &str) {
    let component_id = id.to_string();

    log!("Starting background thread:", id);

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let component_exists = Main::component_exists(&component_id);

            if !component_exists {
                log!("Stopping background thread:", component_id);
                break;
            }

            Main::update(component_id.as_str());
            gloo_timers::future::TimeoutFuture::new(1000).await;
        }
    });
}
