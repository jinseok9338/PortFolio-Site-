use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub level: u8,
   
}


#[function_component(Header)]
pub fn header(props: &HeaderProps) ->Html {
 html!{
     <div>

     </div>
 }
}
