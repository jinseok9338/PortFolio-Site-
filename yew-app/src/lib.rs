
mod routes;
mod components;
mod pages;
mod theme;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::{Route, switch};
use theme::theme::{COLOR_THEME,Theme};

enum Msg {
    ChangeTheme,
}


pub struct App {
    theme: Theme,
    current_theme:String
}



impl Component for App  {
    type Message =Msg;

    type Properties =();

    fn create(_ctx: &Context<Self>) -> Self {
      Self {
        theme:COLOR_THEME,
        current_theme:"LightMode".to_owned(),
      }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeTheme => {
                let new_theme  = if &self.current_theme == &"LightMode".to_owned() {"DarkMode".to_owned() } else { "LightMode".to_owned() };
                self.current_theme = new_theme;
                true
            }
        }
    }

    
    fn view(&self ,ctx: &Context<Self>) -> Html {
        html! {
        <ContextProvider<Theme> context={(*ctx).clone()}>
            <BrowserRouter>
                <div>        
                    <main>
                        <Switch<Route> render={Switch::render(switch)}  />
                    </main>
                </div>
            </BrowserRouter>
        </ContextProvider<Theme>>
         }
    }
}