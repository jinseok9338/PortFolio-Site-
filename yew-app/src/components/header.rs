use yew::prelude::*;



#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
    let color_theme = use_state(|| "LightMode".to_owned());
    let toggle_theme = Callback::from(move |_:&str| 
        {   
            let cloned_color_theme = color_theme.clone();
            let new_theme = if *cloned_color_theme == "LightMode" { "DarkMode" } else { "LightMode" };
            cloned_color_theme.set(new_theme.to_owned())
        
        });
        
 html!{
        <div class={classes!("flex", "h-full","w-full","pt-4","pb-4","pl-16","pr-16")}>
            <div class={classes!("left-container","h-full","w-16","bg-[#eb4034]","flex","items-center","justify-center")}>
            
                <span class={classes!("font-bold","text-lg")}>{"J.J.J"}</span>
            </div>
            <div class={classes!("right-container", "h-full","bg-[#32a852]","items-center","flex","justify-around","grow")}>
                <div>
                    <ul>
                        <li class={classes!("inline-block")}><a href="#" >{"About"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Projects"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Resume"}</a></li>
                        <li class={classes!("inline-block","ml-[1rem]")}><a href="#">{"Contact"}</a></li>
                        <li class={classes!("inline-block","ml-auto")}>
                        if *color_theme == "Light Theme" {
                            <i class="uil uil-sunset"></i>
                        }else{
                            <i class="uil uil-moon"></i>
                        }
                        <button {toggle_theme}>{&*color_theme}</button>
                        </li>
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
