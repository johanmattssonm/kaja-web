use kaja_web::prelude::*;

#[component("main-component")]
pub struct Main {
    message: Option<String>,
}

impl Main {
    fn connected(&mut self, element: HtmlElement) {
        self.render(element);
    }
}

impl Main {
    fn render(&self, element: HtmlElement) {
        let content = html! {{
            <counter-component count="1"></counter-component>
            <counter-component count="42"></counter-component>
        }};

        element.set_inner_html(content.as_str());
    }
}

impl Main {
    fn disconnected(&mut self, _element: HtmlElement) {}

    fn observed_attributes() -> &'static [&'static str] {
        &[]
    }

    fn attribute_changed(
        &mut self,
        _parent: HtmlElement,
        _name: &str,
        _old_value: &str,
        _new_value: &str,
    ) {
    }
}
