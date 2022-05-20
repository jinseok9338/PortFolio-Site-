use yew::prelude::*;



#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
 html!{
        <div class={classes!("flex","w-full", "h-full", "bg-[#ff00e1]")}>{"This is header"}</div>
 }
}



pub struct Header {
   
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

  

    fn view(&self, _ctx: &Context<Self>) -> Html {
       
        html! {
            <div class={classes!("header-container","w-screen","bg-[black]","h-[30px]")}>
            <HeaderContents/>
         </div>
        }
    }
}
