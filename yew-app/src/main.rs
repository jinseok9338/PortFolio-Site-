mod components;
mod pages;
mod theme;

use yew::prelude::*;
use yew::ContextProvider;

use pages::landing_pages::LandingPages;
use crate::theme::theme::ThemeProvider;



/// Main component
#[function_component(App)]
pub fn app() -> Html {
 
    html! {
        <ThemeProvider>
            <LandingPages />  
        </ThemeProvider>
     }
}


fn main() {
    yew::start_app::<App>();
}