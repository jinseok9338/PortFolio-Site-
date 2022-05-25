mod components;
mod pages;
mod theme;

use yew::prelude::*;


use pages::landing_pages::LandingPages;
use theme::ThemeProvider; 



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