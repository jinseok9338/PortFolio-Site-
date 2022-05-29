mod components;
mod global_style;
mod hooks;
mod sections;
mod util;

use components::theme_button::ThemeButton;
use global_style::GlobalStyle;
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
            // <GlobalStyle>
                <ThemeButton/>
                <Hero/>
                <About/>
                <Projects/>
                <Contacts/>
                <Footer/>
            // </GlobalStyle>
        </ThemeProvider>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
