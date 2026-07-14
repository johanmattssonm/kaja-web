#[path = "common.rs"]
mod common;

use kaja_web::prelude::*;
use kaja_web_component::component;
use wasm_bindgen_test::*;

use std::rc::Rc;
use std::str::FromStr;

thread_local! {
    static TEST2_COMPONENTS: RefCell<HashMap<String, Test2Handle>> =
        RefCell::new(HashMap::new());
}

fn parse_value<T>(s: &str) -> Option<T>
where
    T: FromStr,
{
    s.parse::<T>().ok()
}

#[wasm_bindgen_test]
fn test_new_component() {
    common::setup_test();

    create_test2();

    let id = "test-get-set-id";

    let content = html! {{
        <test-component2 id="$id" test_attribute="initial-value-for-attribute"></test-component1>
    }};

    inner_html("#test-div", &content).unwrap();
    let component = Test2Component::get_component(id); // ref
}

fn create_test2() {
    let handle = Rc::new(RefCell::new(Test2ActualContainer {
        id: "test2".to_string(),
        attribute_values: Rc::new(RefCell::new(Test2::default())),
    }));

    register_component(handle.clone());
    Test2ActualContainer::connect(handle);
}

fn register_component(component: Test2Handle) {
    let id = component.borrow().id.clone();

    TEST2_COMPONENTS.with(|components| {
        components.borrow_mut().insert(id, component);
    });
}

#[derive(Default)]
struct Test2 {
    test: bool,
}

struct Test2ActualContainer {
    id: String,
    attribute_values: Rc<RefCell<Test2>>,
}

type Test2Handle = Rc<RefCell<Test2ActualContainer>>;

impl Test2ActualContainer {
    fn connect(handle: Test2Handle) {}
    fn attributes_update(&self, name: &str, old_value: &str, new_value: &str) {}
    fn set_test_attribute() {}
    fn get_test_attribute(&self) -> String {
        String::from("test ...")
    }
}

struct Test2Component {
    handle: Test2Handle,
}

impl Test2Component {
    fn connect(&self) {}
    fn get_component(id: &str) -> Option<Test2Component> {
        let handle = TEST2_COMPONENTS.with(|components| components.borrow().get(id).cloned());

        if !handle.is_some() {
            return None;
        }

        Some(Test2Component {
            handle: handle.unwrap(),
        })
    }

    fn attributes_update(&self, name: &str, old_value: &str, new_value: &str) {
        self.handle
            .borrow()
            .attributes_update(name, old_value, new_value);
    }

    fn set_test_attribute(&self, name: &str, value: &str) {}

    fn get_test_attribute(&self) -> String {
        String::from("test ...")
    }

    fn get_attributes(&self) -> Rc<RefCell<Test2>> {
        self.handle.borrow().attribute_values.clone()
    }

    fn with_attributes_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut Test2),
    {
        let handle = self.handle.borrow();
        let mut attributes = handle.attribute_values.borrow_mut();
        f(&mut attributes);
    }

    fn with_attributes<F>(&self, f: F)
    where
        F: FnOnce(&Test2),
    {
        let handle = self.handle.borrow();
        let attributes = handle.attribute_values.borrow();
        f(&attributes);
    }

    fn get_id(&self) -> String {
        self.handle.borrow().id.clone()
    }
}
