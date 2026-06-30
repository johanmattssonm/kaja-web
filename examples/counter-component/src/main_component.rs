use kaja_web::prelude::*;

#[component("main-component")]
pub struct Main;

impl Component for Main {
    fn connected(&mut self, element: HtmlElement) {
        self.render(&element);
    }

    fn render(&self, element: &HtmlElement) {
        let content = html! {{
            <counter-component count="1"></counter-component>
            <counter-component count="42"></counter-component>
        }};

        element.set_inner_html(content.as_str());
    }
}
