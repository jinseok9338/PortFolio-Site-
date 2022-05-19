use yew::prelude::*;
use yew_router::{prelude::RouterScopeExt, history::Location};
use serde::{ Deserialize};

#[derive(Debug, Deserialize)]
struct QueryParams {
    page:String
}

pub struct Card {
    query_params: QueryParams,
}

impl Component for Card {
    type Message = ();

    type Properties =();

    fn create(ctx: &Context<Self>) -> Self {
        
        let query_params :QueryParams = ctx.link().location().unwrap().query().unwrap();
        Self {query_params}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        
        html! {
            <section class={classes!("flex", "justify-center")} >
                <h1 class={classes!("underline", "text-blue-600")}>{&self.query_params.page}</h1>
            </section>
        }
    }
}