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
            </p>
        }};

        element.set_inner_html(content.as_str());
    }
}

#[callback(increment)]
pub fn increment(id: String) {
    let html_element = get_component_element(id.as_str());

    if html_element.is_none() {
        error!("No html_element in increment callback.");
        return;
    }

    let element = html_element.unwrap();
    let mut new_count = increment_component(&element);

    if new_count == -1 {
        error!("Increment failed.");
        return;
    }

    set_value!(&id, Counter.value, new_count);
    element.set_attribute("count", new_count.to_string().as_str());
    rerender::<Counter>(&id);
}

fn increment_component(html_element: &HtmlElement) -> i32 {
    let count_attribute = html_element.get_attribute("count");

    if count_attribute.is_none() {
        error!("Attribute 'count' not found for element");
        return -1;
    }

    let count_res = count_attribute.unwrap().parse::<i32>();

    if count_res.is_err() {
        error!("Count is not an integer.");
        return -1;
    }

    let new_count = count_res.unwrap() + 1;
    new_count
}
