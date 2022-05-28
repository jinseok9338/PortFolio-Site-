use yew::prelude::*;
use crate::components::about_section::about_style::about_style;
#[function_component(About)]
pub fn about() ->Html {

    let class = about_style();
    html!{
        // <!-- **** About Section **** -->
        <div {class}>
        <section id="about">
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
                    src="assets/profile.jpg"
                  />
                </div>
              </div>
              <div class="col-md-6 col-sm-12">
                <div class="about-wrapper__info load-hidden">
                  <p class="about-wrapper__info-text">
                    {"This is where you can describe about yourself. The more you
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
    </div>
        // <!-- /END About Section -->
    }
}