use kaja_web::prelude::*;

#[component("counter-component")]
pub struct Counter {
    value: i32,
}

impl Counter {
    fn connected(&mut self, element: HtmlElement) {
        log!("Element connected");
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
        log!("Attribute changed", name, new_value);

        match name {
            "count" => {
                self.value = new_value.parse::<i32>().expect("Not an integer.");
            }
            _ => {
                log!("attribute_changed not implementd for", name);
            }
        }

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
    let mut new_count = -1;

    update_component(&id, |component: &mut Counter, element: &HtmlElement| {
        log!("update_component", component.get_component_id());
        increment_component(component, element);
    });

    let html_element = get_component_element(id.as_str());

    if html_element.is_none() {
        error!("No html_element in increment callback.");
        return;
    }

    let count = get_value(&id);

    let element = html_element.unwrap();
    element.set_attribute("count", count.to_string().as_str());

    rerender(&id);
}

fn rerender(id: &str) {
    let component = get_component::<Counter>(id);
    let binding = component.unwrap();
    let c = binding.lock().unwrap();

    log!("new count", c.value);

    let html_element = get_component_element(id);

    if html_element.is_none() {
        error!("No html_element in rerender.");
        return;
    }

    let element = html_element.unwrap();
    c.render(&element);
}

fn get_value(id: &str) -> i32 {
    let component = get_component::<Counter>(id);
    let binding = component.unwrap();
    let c = binding.lock().unwrap();
    c.value
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
}
