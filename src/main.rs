use yew::prelude::*;

pub enum Msg {
    Stormy,
    Sunny,
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
            Msg::Stormy => self.weather = true,
            Msg::Sunny => self.weather = false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <p>{ self.weather}</p>
        )
    }
}

fn main(){
    yew::start_app::<App>();
}