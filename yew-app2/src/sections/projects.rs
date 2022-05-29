use crate::hooks::theme_context::use_theme;
use stylist::css;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> HTML {
    let theme = use_theme();
    let background_color = &theme.clone().primary_background_color;
    let font_color = &theme.clone().font_color;

    html! {
       // <!-- **** Projects Section **** -->
        <section id="projects" class={css!("background: ${bg}; color: ${font_color};",bg=background_color.clone(),font_color=font_color)}>
          <div class="container">
            <div class="project-wrapper">
              <h2 class="section-title dark-blue-text">{"Projects"}</h2>

             // <!-- Notice: each .row is a project -->
              <div class="row">
                <div class="col-lg-4 col-sm-12">
                  <div class="project-wrapper__text load-hidden">
                    <h3 class="project-wrapper__text-title">{"Project Title 0"}</h3>
                    <div>
                      <p class="mb-4">
                {"        Lorem ipsum dolor sit, amet consectetur adipisicing elit.
                        Excepturi neque, ipsa animi maiores repellendus distinctio
                        aperiam earum dolor voluptatum consequatur blanditiis
                        inventore debitis fuga numquam voluptate ex architecto
                        itaque molestiae."}
                      </p>
                    </div>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn cta-btn--hero"
                      href="#!"
                    >
                     {" See Live"}
                    </a>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn text-color-main"
                      href="#!"
                    >
                     {" Source Code"}
                    </a>
                  </div>
                </div>
                <div class="col-lg-8 col-sm-12">
                  <div class="project-wrapper__image load-hidden">
                    <a rel="noreferrer" href="#!" target="_blank">
                      <div
                      data-tilt = "true"
                        data-tilt-max="4"
                        data-tilt-glare="true"
                        data-tilt-max-glare="0.5"
                        class="thumbnail rounded js-tilt"
                      >
                        <img
                          alt="Project Image"
                          class="img-fluid"
                          src="assets/project.jpg"
                        />
                      </div>
                    </a>
                  </div>
                </div>
              </div>
              //<!-- /END Project -->

              //<!-- Notice: each .row is a project -->
              <div class="row">
                <div class="col-lg-4 col-sm-12">
                  <div class="project-wrapper__text load-hidden">
                    <h3 class="project-wrapper__text-title">{"Project Title 1"}</h3>
                    <div>
                      <p class="mb-4">
                     {"   Lorem ipsum dolor sit, amet consectetur adipisicing elit.
                        Excepturi neque, ipsa animi maiores repellendus distinctio
                        aperiam earum dolor voluptatum consequatur blanditiis
                        inventore debitis fuga numquam voluptate ex architecto
                        itaque molestiae."}
                      </p>
                    </div>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn cta-btn--hero"
                      href="#!"
                    >
                    {"  See Live"}
                    </a>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn text-color-main"
                      href="#!"
                    >
                     {" Source Code"}
                    </a>
                  </div>
                </div>
                <div class="col-lg-8 col-sm-12">
                  <div class="project-wrapper__image load-hidden">
                    <a rel="noreferrer" href="#!" target="_blank">
                      <div
                      data-tilt = "true"
                        data-tilt-max="4"
                        data-tilt-glare="true"
                        data-tilt-max-glare="0.5"
                        class="thumbnail rounded js-tilt"
                      >
                        <img
                          alt="Project Image"
                          class="img-fluid"
                          src="assets/project.jpg"
                        />
                      </div>
                    </a>
                  </div>
                </div>
              </div>
              //<!-- /END Project -->

             // <!-- Notice: each .row is a project -->
              <div class="row">
                <div class="col-lg-4 col-sm-12">
                  <div class="project-wrapper__text load-hidden">
                    <h3 class="project-wrapper__text-title">{"Project Title 2"}</h3>
                    <div>
                      <p class="mb-4">
                    {"    Lorem ipsum dolor sit, amet consectetur adipisicing elit.
                        Excepturi neque, ipsa animi maiores repellendus distinctio
                        aperiam earum dolor voluptatum consequatur blanditiis
                        inventore debitis fuga numquam voluptate ex architecto
                        itaque molestiae."}
                      </p>
                    </div>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn cta-btn--hero"
                      href="#!"
                    >
                      {"See Live"}
                    </a>
                    <a
                      rel="noreferrer"
                      target="_blank"
                      class="cta-btn text-color-main"
                      href="#!"
                    >
                     {" Source Code"}
                    </a>
                  </div>
                </div>
                <div class="col-lg-8 col-sm-12">
                  <div class="project-wrapper__image load-hidden">
                    <a rel="noreferrer" href="#!" target="_blank">
                      <div
                      data-tilt = "true"
                        data-tilt-max="4"
                        data-tilt-glare="true"
                        data-tilt-max-glare="0.5"
                        class="thumbnail rounded js-tilt"
                      >
                        <img
                          alt="Project Image"
                          class="img-fluid"
                          src="assets/project.jpg"
                        />
                      </div>
                    </a>
                  </div>
                </div>
              </div>
             // <!-- /END Project -->
            </div>
          </div>
        </section>
        //<!-- /END Projects Section -->
    }
}
