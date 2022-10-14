use yew::prelude::*;

mod components;
use components::CounterComponent;
use components::Navbar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Navbar />
            <CounterComponent />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
