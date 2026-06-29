use kaja_web::prelude::*;

#[component("counter-component")]
pub struct Counter {
    value: i32,
}

impl Component for Counter {
    fn connected(&mut self, element: HtmlElement) {
        self.render(&element);
    }

    fn disconnected(&mut self, element: HtmlElement) {
        log!("Element removed");
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }

    fn attribute_changed(
        &mut self,
        element: HtmlElement,
        name: &str,
        old_value: &str,
        new_value: &str,
    ) {
        self.render(&element);
    }
}

impl Counter {
    fn render(&self, element: &HtmlElement) {
        let count = element.get_attribute("count");

        if count.is_none() {
            log!("No count to render.");
            return;
        }

        let component_id = element.get_attribute("componentid");

        if component_id.is_none() {
            log!("No component_id.");
            return;
        }

        let id = component_id.unwrap();

        let content = html! {{
            <p>
                Count: $(count.unwrap())
                <button onclick="increment('$id');">Increment</button>
            </p>
        }};

        element.set_inner_html(content.as_str());
    }
}

#[callback(increment)]
pub fn increment(id: String) {
    let element = get_component_element(id.as_str());

    if element.is_none() {
        error!("Element not found for id", id);
        return;
    }

    let html_element = element.unwrap();
    let count_attribute = html_element.get_attribute("count");

    if count_attribute.is_none() {
        error!("Attribute count not found for id", id);
        return;
    }

    let count = count_attribute
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(-1);

    let new_count = count + 1;

    if let Some(counter_arc) = get_component::<Counter>(id.as_str()) {
        let mut counter = counter_arc.lock().unwrap();
        counter.value = new_count;
        counter.render(&html_element);
    } else {
        error!("Component not found for id", id);
        return;
    }

    let _ = html_element.set_attribute("count", new_count.to_string().as_str());
}
