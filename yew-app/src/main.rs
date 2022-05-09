use gloo_utils::document;
use yew::prelude::*;


struct Display {
  
}

impl Component for Display {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          
           <canvas id ="card">

           </canvas>
           
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let canvas = document().query_selector("#card").unwrap().unwrap();
        let canvas_context: () = canvas.set_inner_html("hello");
    }
}

fn main() {
    yew::start_app::<Display>();
}