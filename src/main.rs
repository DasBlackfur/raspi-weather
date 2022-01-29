use yew::prelude::*;

pub enum Msg {
    Update,
}

pub struct App {
    weather: bool,
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            weather: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                self.weather = !self.weather;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div style={
                match self.weather {
                    true => "background-color: #00ff00;",
                    false => "background-color: #ff0000;",
                }
            }>
                <p>{ self.weather }</p>

                <button class="button" onclick={ctx.link().callback(|_| Msg::Update)}>
                    { "Toggle" }
                </button>

                <button class="button" onClick="window.location.reload();">
                    { "Reload" }
                </button>
            </div>
        )
    }
}

fn main(){
    yew::start_app::<App>();
}