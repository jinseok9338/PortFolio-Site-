use yew::prelude::*;

pub struct Card {
    name: String,
}

impl Component for Card {
    type Message = ();

    type Properties =();

    fn create(ctx: &Context<Self>) -> Self {
        let name ="We are back".to_owned();
        Self {name}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section>
            <h1>{&self.name}</h1>
            </section>
        }
    }
}