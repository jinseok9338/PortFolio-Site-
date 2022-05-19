
mod routes;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::{Route, switch};


pub struct App {

}



impl Component for App  {
    type Message =();

    type Properties =();

    fn create(_ctx: &Context<Self>) -> Self {
      Self {}
    }

    
    fn view(&self ,_ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)}  />
                </main>
            </BrowserRouter>
         }
    }
}