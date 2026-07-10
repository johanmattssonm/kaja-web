euse kaja_web::prelude::*;

#[component("counter-component")]
pub struct Counter {
    total_num_clicks: i32,
}

impl Component for Arc<Mutex<Counter>> {
    fn connected(&mut self, element: HtmlElement) {
        log!(
            "Connected component",
            self.get_component_name(),
            self.get_element_id()
        );

        self.render(&element);
    }

    fn disconnected(&mut self, _element: HtmlElement) {
        log!("Element removed");
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }

    fn attribute_changed(
        &mut self,
        element: HtmlElement,
        name: &str,
        _old_value: &str,
        new_value: &str,
    ) {
        match name {
            "count" => {
                let count_value = new_value.parse::<i32>().expect("Not an integer.");
                log!("count updated: ", count_value);
            }
            _ => {
                log!("attribute_changed not implementd for", name);
            }
        }

        self.render(&element);
    }

    fn render(&self, element: &HtmlElement) {
        let count = element.get_attribute("count");

        if count.is_none() {
            log!("No count to render.");
            return;
        }

        let component_id = get_component_id(element);
        let content = html! {{
            <p>
                Count: $(count.unwrap())
                <button onclick="increment('$component_id');">Increment</button>
                <button onclick="decrement('$component_id');">Decrement</button>
            </p>
        }};

        element.set_inner_html(content.as_str());
    }
}

#[callback(increment)]
pub fn increment(id: String) {
    increment_total(&id);
    update_count(&id, 1);
    rerender::<Counter>(&id);
}

#[callback(decrement)]
pub fn decrement(id: String) {
    increment_total(&id);
    update_count(&id, -1);
    rerender::<Counter>(&id);
}

fn increment_total(id: &str) {
    let component_arc = get_component_arc(id);
    let mut component = component_arc.lock().unwrap();
    component.total_num_clicks += 1;

    log!("total clicks: ", total_clicks);
}

fn update_count(id: &str, change: i32) {
    let html_element = get_component_element(id);

    if html_element.is_none() {
        error!("No html_element in update_count.");
        return;
    }

    let element = html_element.unwrap();
    let new_count = update_count_in_element(&element, change);

    if let Some(count) = new_count {
        if let Err(e) = element.set_attribute("count", count.to_string().as_str()) {
            error!("Failed to set attribute 'count'.", e);
        }
    } else {
        error!("Click counter failed.");
    }
}

fn update_count_in_element(html_element: &HtmlElement, change: i32) -> Option<i32> {
    let count_attribute = html_element.get_attribute("count");

    if count_attribute.is_none() {
        error!("Attribute 'count' not found for element");
        return None;
    }

    let count_res = count_attribute.unwrap().parse::<i32>();

    if count_res.is_err() {
        error!("Count is not an integer.");
        return None;
    }

    let new_count = count_res.unwrap() + change;
    Some(new_count)
}
