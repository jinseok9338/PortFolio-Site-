use yew::prelude::*;
use crate::theme::theme::{use_theme, ThemeKind};
use stylist::Style;






const STYLE_FILE :&str = include_str!("header.css");

#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));
    
    let current_color_theme =  use_context::<UseStateHandle<String>>().expect("no ctx found");
    
    let cloned_current_color_theme = current_color_theme.clone();

    let toggle_theme = Callback::from(move |_:String| 
        {   
            let new_theme  = if &*cloned_current_color_theme == &"LightMode".to_owned() {"DarkMode".to_owned() } else { "LightMode".to_owned() };
            cloned_current_color_theme.set(new_theme);
        });

   
 html!{
    <div >
    <a href="#default" class="logo">{"CompanyLogo"}</a>
    <div class="header-right">
      <a class="active" href="#">{"Home"}</a>
      <a href="#">{"Contact"}</a>
      <a href="#">{"About"}</a>
      <button onclick={switch_theme} id="yew-sample-button">{"Switch to "}{theme_str}</button>
    </div>
  </div>
 }
}




// if  &*current_color_theme == &"LightMode".to_owned()  {
//     <i class="uil uil-sunset text-[2rem]"></i>
// }else{
//     <i class="uil uil-moon"></i>
// }


pub struct Header {
   
}



impl Component for Header {
    type Message = ();
    type Properties = ();

    

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

  

    fn view(&self, ctx: &Context<Self>) -> Html {


     
        html! {
            <nav class={classes!("header-container","w-screen","md:block","hidden","h-[100px]")}>
            <HeaderContents/>
         </nav >
        }
    }
}
