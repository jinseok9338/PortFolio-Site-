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
use stylist::css;
use stylist::yew::Global;
use yew::{function_component, html};

use crate::hooks::theme_context::use_theme;

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();
    let background_color = &theme.clone().primary_background_color;

    html! {
    <>
        <Global css={css!("background-color:${bg};",bg=background_color)}/>
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
