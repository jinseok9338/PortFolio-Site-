use yew::prelude::*;

pub struct LandingPages;

pub enum Msg {
}

impl Component for LandingPages {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"This is landing Pages"}</h1>
        }
    }
}