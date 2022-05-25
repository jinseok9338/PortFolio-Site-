use crate::{components, theme::theme::Theme};

use yew::prelude::*;
use components::header::Header;

#[derive(PartialEq, Properties)]
pub struct LandingPagesProps {
    pub theme: Theme,
}

pub struct LandingPages {
    
}


pub enum Msg {
}

impl Component for LandingPages {
    type Message = Msg;
    type Properties =();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Header/>
        }
    }
}