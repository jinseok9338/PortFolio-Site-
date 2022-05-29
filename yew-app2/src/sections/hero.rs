use crate::hooks::theme_context::use_theme;
use stylist::css;
use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> HTML {
    let theme = use_theme();
    let background_color = &theme.clone().primary_background_color;
    let font_color = &theme.clone().font_color;

    html! {
       //<!-- **** Hero Section **** -->

       <section id="hero" class= {css!("background: ${bg}; color: ${font_color};",bg=background_color.clone(),font_color=font_color)} >
         <div class="container">
           <h1 class="hero-title load-hidden">
            {" Hi, my name is"} <span class="text-color-main">{"Your Name"}</span>
             <br />
            {" I'm the Unknown Developer."}
           </h1>
           <p class="hero-cta load-hidden">
             <a rel="noreferrer" class="cta-btn cta-btn--hero" href="#about"
               >{"Know more"}</a
             >
           </p>
         </div>
       </section>

       //<!-- /END Hero Section -->
    }
}
