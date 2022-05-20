use crate::components;

use yew::prelude::*;
use components::header::Header;

pub struct LandingPages;

pub enum Msg {
}

impl Component for LandingPages {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Header/>
        }
    }
}