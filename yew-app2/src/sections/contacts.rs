use yew::prelude::*;

#[function_component(Contacts)]
pub fn contacts() -> HTML {
    html! {
      // <!-- **** Contact Section **** -->
       <section id="contact">
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
