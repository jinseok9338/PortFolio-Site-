use crate::hooks::theme_context::use_theme;
use stylist::css;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> HTML {
    let theme = use_theme();
    let background_colors = &theme.clone();
    let font_color = &theme.clone().font_color;

    html! {
        // <!-- **** About Section **** -->
        <section id="about" class={classes!(css!("background-image: linear-gradient(
          135deg,
          ${primary_color} 0%,
          ${secondary_color} 100%
        );
       
        color: ${font_color};"
        ,primary_color=background_colors.primary_background_color.clone(),
        secondary_color=background_colors.secondary_background_color.clone(),
        font_color=font_color.clone())
       )}>
          <div class="container">
            <h2 class="section-title load-hidden">{"About me"}</h2>
            <div class="row about-wrapper">
              <div class="col-md-6 col-sm-12">
                <div class="about-wrapper__image load-hidden">
                  <img
                    alt="Profile Image"
                    class="img-fluid rounded shadow-lg"
                    height="auto"
                    width="300px"
                    src="https://picsum.photos/200/300"
                  />
                </div>
              </div>
              <div class="col-md-6 col-sm-12">
                <div class="about-wrapper__info load-hidden">
                  <p class="about-wrapper__info-text">
                  {"  This is where you can describe about yourself. The more you
                    describe about yourself, the more chances you can!"}
                  </p>
                  <p class="about-wrapper__info-text">
                    {"Extra Information about you! like hobbies and your goals."}
                  </p>
                  <span class="d-flex mt-3">
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn cta-btn--resume"
                      href="assets/resume.pdf"
                    >
                      {"View Resume"}
                    </a>
                  </span>
                </div>
              </div>
            </div>
          </div>
        </section>
       // <!-- /END About Section -->
    }
}
