
mod routes;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::{Route, switch};


pub struct App {
 name: String,
}






impl Component for App  {
    type Message =();

    type Properties =();

    fn create(_ctx: &Context<Self>) -> Self {
        let name = "Hello world".to_owned();
        
        Self {name}
    }

    
    fn view(&self ,_ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
            <main>
                <Switch<Route> render={Switch::render(switch)}  />
            </main>

            // <div class ={classes!("flex", "align-center", "justify-center")} >
            //     <h1 class={classes!("bg-red-100")} >{&self.name}</h1>
            // </div>
            </BrowserRouter>
         }
    }
}