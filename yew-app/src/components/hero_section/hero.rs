use yew::prelude::*;
use crate::theme::theme::{use_theme, ThemeKind};

use crate::components::hero_section::hero_style::hero_style;





//https://github.com/cobiwave/simplefolio/blob/master/src/sass/base/_typography.scss

#[function_component(Hero)]
pub fn hero() ->Html {
    let theme = use_theme();
    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme", 
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    // let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));
    let class = hero_style();


 html!{
<div {class}>
    // <!-- **** Hero Section **** --> 
    <div id="hero">
        <div>
            <h1 class="text-typing hero-title">
            {"My name is"} <span class="text-color-main">{" Jason"}</span>
            </h1>
            <h1 class="text-typing2 hero-title show">
            {" I'm the Web Developer."}
            </h1>   
        </div>
    </div> 
    // <!-- /END Hero Section -->
</div>
 }
}

//https://alvarotrigo.com/blog/css-text-animations/


// <button class={css!(r#"color: white;
// height: 50px;
// width: 300px;
// font-size: 20px;
// background-color: rgb(88, 164, 255);
// border-radius: 5px;
// border: none;
// "#)} onclick={switch_theme} id="yew-sample-button">{"Switch to "}{theme_str}</button>


