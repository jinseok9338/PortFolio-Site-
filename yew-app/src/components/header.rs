use yew::prelude::*;



#[function_component(HeaderContents)]
pub fn header_contents() ->Html {
 html!{
     <div class={classes!("header-container","w-screen","md:hidden","bg-[black]","h-[30px]")}>
        <h1>{"This is header"}</h1>
     </div>
 }
}
