use crate::hooks::theme_context::{use_theme, ThemeKind};
use stylist::css;
use yew::prelude::*;

#[function_component(ThemeButton)]
pub fn theme_button() -> HTML {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let background_color = other_theme
        .clone()
        .current()
        .primary_background_color
        .to_owned();
    let font_color = other_theme.clone().current().font_color.to_owned();
    let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));

    html! {
        <div id="theme-toggler" onclick={switch_theme} class={css!(
            r#"
            background: ${bg};"#
        ,bg=background_color.clone())}>
        if &theme_str == &"Dark Theme"{
            <i class={classes!("fa" ,"fa-moon-o",css!("color: ${font_color};",font_color=font_color.clone()))} aria-hidden="true"></i>
        } else{ <i class={classes!("fa" ,"fa-sun-o",css!("color: ${font_color};",font_color=font_color.clone()))} aria-hidden="true"></i>}
        </div>
    }
}
