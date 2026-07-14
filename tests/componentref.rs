#[path = "common.rs"]
mod common;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_new_component() {
    common::setup_test();

    Test2RefContainer::create_component("test-get-set-id");

    let id = "test-get-set-id";

    let content = html! {{
        <test-component2 id="$id" test_attribute="initial-value-for-attribute"></test-component1>
    }};

    inner_html("#test-div", &content).unwrap();
    let component = Test2Component::get_component(id); // ref
}

#[derive(Default)]
struct Test2 {
    test: bool,
    number: i32,
}

impl Component for Test2Component {
    fn connected(&self) {
        log!("Connected:", self.get_id());
    }

    fn disconnected(&self) {
        log!("Disconnected:", self.get_id());
    }

    fn attribute_changed(&self, name: &str, old_value: &str, new_value: &str) {
        log!(
            "Attribute changed:",
            name,
            "old:",
            old_value,
            "new:",
            new_value
        );
    }
}

thread_local! {
    static TEST2_COMPONENTS: RefCell<HashMap<String, std::rc::Rc<std::cell::RefCell<Test2RefContainer>>>> =
        RefCell::new(HashMap::new());
}

impl Test2 {
    fn set_test_attribute(&mut self, name: &str, value: &str) {
        let _ = self.try_set_test_attribute(name, value);
    }

    fn try_set_test_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        if name == "test" {
            self.test = value.parse().map_err(|_| Error)?;
        } else if name == "number" {
            self.number = value.parse().map_err(|_| Error)?;
        } else {
            error!("No field", name, "in Test2 can't set it to", value);
            return Err(Error);
        }

        Ok(())
    }
}

#[derive(Default)]
struct Test2RefContainer {
    id: String,
    attribute_values: Rc<RefCell<Test2>>,
}

impl Test2RefContainer {
    fn create_component(id: &str) {
        let handle = Rc::new(RefCell::new(Test2RefContainer {
            id: id.to_string(),
            attribute_values: std::rc::Rc::new(std::cell::RefCell::new(Test2::default())),
        }));

        Self::register_component(handle.clone());
        let component = Test2Component::get_component(&id);

        if component.is_none() {
            error!("Failed to register compoent.");
            return;
        }

        let component_impl = component.unwrap();
        component_impl.connected();
    }

    fn register_component(component: Rc<RefCell<Test2RefContainer>>) {
        let id = component.borrow().id.clone();

        TEST2_COMPONENTS.with(|components| {
            components.borrow_mut().insert(id, component);
        });
    }
}

#[derive(Default)]
struct Test2Component {
    handle: Rc<RefCell<Test2RefContainer>>,
}

impl Test2Component {
    fn get_component(id: &str) -> Option<Test2Component> {
        let handle = TEST2_COMPONENTS.with(|components| components.borrow().get(id).cloned());

        if !handle.is_some() {
            return None;
        }

        Some(Test2Component {
            handle: handle.unwrap(),
        })
    }

    fn internal_observed_attributes() -> &'static [&'static str] {
        &[]
    }

    fn internal_attributes_update(&self, name: &str, old_value: &str, new_value: &str) {
        let handle = self.handle.borrow();
        let mut attr = handle.attribute_values.borrow_mut();
        attr.set_test_attribute(name, new_value);
    }

    fn get_attributes(&self) -> Rc<RefCell<Test2>> {
        self.handle.borrow().attribute_values.clone()
    }

    fn get_id(&self) -> String {
        self.handle.borrow().id.clone()
    }
}
