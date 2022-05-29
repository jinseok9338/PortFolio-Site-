use crate::hooks::theme_context::use_theme;
use stylist::css;
use yew::prelude::*;

#[function_component(Contacts)]
pub fn contacts() -> HTML {
    let theme = use_theme();
    let background_colors = &theme.clone();
    let font_color = &theme.clone().font_color;

    html! {
      // <!-- **** Contact Section **** -->
       <section id="contact" class={css!("background-image: linear-gradient(
        135deg,
        ${primary_color} 0%,
        ${secondary_color} 100%
      );
      color:${font_color};",
      primary_color=background_colors.primary_background_color.clone(),
      secondary_color=background_colors.secondary_background_color.clone(),
      font_color=font_color.clone())}>
         <div class="container">
           <h2 class="section-title">{"Contact"}</h2>
           <div class="contact-wrapper load-hidden">
             <p class="contact-wrapper__text">{"[Put your call to action here]"}</p>
             <a
               rel="noreferrer"
               target="_blank"
               class="cta-btn cta-btn--resume"
               href="mailto:example@email.com"
               >{"Call to Action"}</a
             >
           </div>
         </div>
       </section>
      // <!-- /END Contact Section -->
    }
}
