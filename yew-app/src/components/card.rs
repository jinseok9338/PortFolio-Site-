use gloo::history::History;
use yew::prelude::*;
use yew_router::{prelude::RouterScopeExt, history::Location};
use serde::{ Deserialize};

#[derive(Debug, Deserialize)]
struct QueryParams {
    page:String
}

pub struct Card {
    name: String,
}

impl Component for Card {
    type Message = ();

    type Properties =();

    fn create(_ctx: &Context<Self>) -> Self {
        
        let name ="this is awesome".to_owned();
        Self {name}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let query_params :QueryParams = ctx.link().location().unwrap().query().unwrap();
        html! {
            <section>
            <h1>{&query_params.page}</h1>
            </section>
        }
    }
}