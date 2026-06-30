use kaja_web::prelude::*;

#[component("counter-component")]
pub struct Counter {
    value: i32,
}

impl Counter {
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

        let component_id = self.get_component_id();
        let content = html! {{
            <p>
                Count: $(count.unwrap())
                <button onclick="increment('$component_id');">Increment</button>
            </p>
        }};

        element.set_inner_html(content.as_str());
    }
}

#[callback(increment)]
pub fn increment(id: String) {
    log!("increment");
    // id is a String from the callback; update_component accepts AsRef<str>.
    update_component(id, |component: &mut Counter, element: &HtmlElement| {
        log!("update_component", component.get_component_id());
        increment_component(component, element);
    });
}

fn increment_component(component: &mut Counter, element: &HtmlElement) {
    let count_attribute = element.get_attribute("count");

    if count_attribute.is_none() {
        error!("Attribute 'count' not found for element");
        return;
    }

    let count_res = count_attribute.unwrap().parse::<i32>();
    if count_res.is_err() {
        error!("Count is not an integer.");
        return;
    }

    let new_count = count_res.unwrap() + 1;

    component.value = new_count;
    element.set_attribute("count", new_count.to_string().as_str());

    log!("new count", new_count);

    component.render(element);
}
