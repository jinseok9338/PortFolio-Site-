use yew::prelude::*;
use crate::theme::theme::{use_theme, ThemeKind};

use crate::components::hero_style::hero_style;







#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
    let theme = use_theme();
    let active = use_state(|| "active");
    let page_names = vec!["Home","Contact","About"];

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let switch_theme = Callback::from(move |_:String| theme.set(other_theme.clone()));
    let class = hero_style();


 html!{
<div {class}>
    // <!-- **** Hero Section **** -->
    <section id="hero" class="jumbotron">
        <div class="container">
            <h1 class="hero-title load-hidden">
            {"Hi, my name is"} <span class="text-color-main">{"Your Name"}</span>
            <br />
            {" I'm the Unknown Developer."}
            </h1>
            <p class="hero-cta load-hidden">
            <a rel="noreferrer" class="cta-btn cta-btn--hero" href="#about">
            {"  Know more"}
            </a>
            </p>
        </div>
    </section>
    // <!-- /END Hero Section -->
</div>
 }
}




// <button class={css!(r#"color: white;
// height: 50px;
// width: 300px;
// font-size: 20px;
// background-color: rgb(88, 164, 255);
// border-radius: 5px;
// border: none;
// "#)} onclick={switch_theme} id="yew-sample-button">{"Switch to "}{theme_str}</button>


pub struct Header {
   
}



impl Component for Header {
    type Message = ();
    type Properties = ();

    

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

  

    fn view(&self, _ctx: &Context<Self>) -> Html {


     
        html! {
            <nav class={classes!("header-container","w-screen","md:block","hidden","h-[100px]")}>
            <HeaderContents/>
         </nav >
        }
    }
}
