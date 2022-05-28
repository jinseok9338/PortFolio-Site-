use yew::prelude::*;
use crate::components::hero_section::hey_style::hey_style;

#[function_component(Hey)]
pub fn hey() ->HTML {
    let class = hey_style();

    html!{
    <div {class}>    
        <div class="content">
            <h2 class="text_shadows">{"hey"}</h2>
        </div>
    </div> 
    }
}