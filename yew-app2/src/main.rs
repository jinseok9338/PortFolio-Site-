mod sections;

use sections::hero::Hero;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Hero/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
