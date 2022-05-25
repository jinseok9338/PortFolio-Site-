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
    let ctx = use_state(|| color_theme());

    html! {
        <ContextProvider<Theme> context={(*ctx).clone()}>
            <LandingPages />
            </ContextProvider<Theme>>
     }
}


fn main() {
    yew::start_app::<App>();
}