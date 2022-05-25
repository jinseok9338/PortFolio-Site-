mod components;
mod pages;
mod theme;

use yew::prelude::*;
use yew::ContextProvider;

use pages::landing_pages::LandingPages;
use crate::{theme::theme::{color_theme,Theme}};



/// Main component
#[function_component(App)]
pub fn app() -> Html {
    let theme_ctx = use_state(|| color_theme());
    let current_color_ctx = use_state(||"LightTheme".to_owned());
    
    html! {
        <ContextProvider<Theme> context={(*theme_ctx).clone()}>
            <ContextProvider<UseStateHandle<String>> context={(current_color_ctx).clone()}>
                <LandingPages />
            </ContextProvider<UseStateHandle<String>>>
        </ContextProvider<Theme>>
     }
}


fn main() {
    yew::start_app::<App>();
}