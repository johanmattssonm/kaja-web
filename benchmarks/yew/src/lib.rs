use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct CounterState(u32);

impl Reducible for CounterState {
    type Action = ();

    fn reduce(self: std::rc::Rc<Self>, _action: Self::Action) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self(self.0.saturating_add(1)))
    }
}

#[function_component(Counter)]
fn counter() -> Html {
    let count = use_reducer(|| CounterState(0));
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.dispatch(()))
    };

    html! {
        <button id="counter" {onclick}>{format!("Clicks: {}", count.0)}</button>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<Counter>::new().render();
}
