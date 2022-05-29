mod components;
mod hooks;
mod sections;

use components::theme_button::ThemeButton;
use hooks::theme_context::ThemeProvider;
use sections::about::About;
use sections::contacts::Contacts;
use sections::footer::Footer;
use sections::hero::Hero;
use sections::projects::Projects;
use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <ThemeProvider>
            <ThemeButton/>
            <Hero/>
            <About/>
            <Projects/>
            <Contacts/>
            <Footer/>
        </ThemeProvider>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
