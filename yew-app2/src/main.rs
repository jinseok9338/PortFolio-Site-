mod sections;

use sections::about::About;
use sections::contacts::Contacts;
use sections::footer::Footer;
use sections::hero::Hero;
use sections::projects::Projects;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Hero/>
            <About/>
            <Projects/>
            <Contacts/>
            <Footer/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
