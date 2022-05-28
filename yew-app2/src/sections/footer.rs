use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> HTML {
    html! {
    <footer class="footer navbar-static-bottom">
        <div class="container">
          <a rel="noreferrer" href="#top" class="back-to-top">
            <i class="fa fa-angle-up fa-2x" aria-hidden="true"></i>
          </a>
          <div class="social-links">
            <a rel="noreferrer" href="#!" target="_blank">
              <i class="fa fa-twitter fa-inverse"></i>
            </a>
            <a rel="noreferrer" href="#!" target="_blank">
              <i class="fa fa-linkedin fa-inverse"></i>
            </a>
            <a rel="noreferrer" href="#!" target="_blank">
              <i class="fa fa-github fa-inverse"></i>
            </a>
          </div>
        </div>
    </footer>
        }
}
