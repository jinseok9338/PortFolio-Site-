use crate::{components, pages::global_style::global_style};
use stylist::yew::Global;

use yew::prelude::*;
use components::hero_section::hero::Hero;
use components::hero_section::hey::Hey;
use components::about::About;

#[derive(PartialEq, Properties)]
pub struct LandingPagesProps {
   
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
        let global_style_css = global_style();
        html! {
            <>
                <Global css={global_style_css}/>
                <div class="dummy"/>
                <Hey/>
                <Hero/>
                <About/>
            </>
        }
    }
}