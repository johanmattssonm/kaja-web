use kaja_web::prelude::*;

#[component("main-component")]
pub struct Main {
    message: Option<String>,
}

impl Component for Main {
    fn connected(&mut self, element: HtmlElement) {
        self.render(element);
    }
}

impl Main {
    fn render(&self, element: HtmlElement) {
        let content = html! {{
            <div><counter-component count="1" /></div>
            <div><counter-component count="42" /></div>
        }};

        element.set_inner_html(content.as_str());
    }
}
