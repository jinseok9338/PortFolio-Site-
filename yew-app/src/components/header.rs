use yew::prelude::*;



#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
 html!{
        <div class={classes!("flex", "h-full")}>
            <div class={classes!("left-container","h-full","w-16","bg-[#eb4034]","flex","items-center","justify-center")}>
                <span>{"J.J.J"}</span>
            </div>
            <div class={classes!("right-container", "w-auto","h-full","bg-[#32a852]","items-center","flex","justify-center")}>
                <div>
                    <ul>
                        <li class={classes!("inline-block")}><a href="#" >{"Home"} <span class="sr-only">{"(current)"}</span></a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Home"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Home"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Home"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Home"}</a></li>
                    </ul>
                </div>
            </div>
        </div>
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
            <nav class={classes!("header-container","w-screen","md:block","hidden","h-[100px]")}>
            <HeaderContents/>
         </nav >
        }
    }
}
