use crate::hooks::theme_context::use_theme;
use stylist::css;
use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct GlobalStyleProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GlobalStyle)]
pub fn global_style(props: &GlobalStyleProps) -> Html {
    let props = &props.children;
    let theme = use_theme();
    let background_color = &theme.primary_background_color.clone();

    html! {
        <div class={css!("background: ${bg}; width:100vw; height:100vh;",bg=background_color)} >
            {props.clone()}
        </div>
    }
}
