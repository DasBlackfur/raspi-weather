use yew::prelude::*;

pub enum Msg {
    Update,
    Settings,
}

pub struct App {
    weather: bool,
    settings: bool,
    temprature: f32,
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            weather: false,
            temprature: 0.0,
            settings: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                self.weather = !self.weather;
                true
            }
            Msg::Settings => {
                self.settings = !self.settings;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.settings {
            true => {
                html!(
                    <>
                        <div>
                            <button class="button" onclick={ctx.link().callback(|_| Msg::Update)}>
                                { "Toggle Weather" }
                            </button>

                            <button class="button" onClick="window.location.reload();">
                                { "Reload" }
                            </button>

                            <button class="button" onclick={ctx.link().callback(|_| Msg::Settings)}>
                                { "Exit Settings" }
                            </button>
                            <a href="/test.html">
                                { "Go to test.html" }
                            </a>
                        </div>
                        <style>
                            { "body {" }
                            { "background-image: url(" }
                            { match self.weather {
                                false => "sunny.jpg",
                                true => "thunder.jpg",
                            } }
                            { ");" }
                            { "}" }
                        </style>
                    </>
                )
            },
            false => {
                html!(
                    <>
                        <div class="grid-wrapper">
                            <div>
                                <p>{ self.temprature }</p>
                            </div>
                            <div>
                                <button class="button" onclick={ctx.link().callback(|_| Msg::Settings)}>
                                    { "Open Settings" }
                                </button>
                            </div>
                            <div>
                                { "C" }
                            </div>
                            <div>
                                { "D" }
                            </div>
                            <div>
                                { "E" }
                            </div>
                            <div>
                                { "F" }
                            </div>
                            <div>
                                { "G" }
                            </div>
                            <div>
                                { "H" }
                            </div>
                        </div>
                        <style>
                            { ".grid-wrapper {" }
                            { "display: grid;" }
                            { "grid-template-columns: repeat(4, 1fr);" }
                            { "grid-template-rows: repeat(2, 1fr);" }
                            { "height: 100vh;" }
                            { "}" }
                            { "body {"}
                            { "background-image: url(" }
                            { match self.weather {
                                false => "sunny.jpg",
                                true => "thunder.jpg",
                            } }
                            { ");" }
                            { "margin: 0;"}
                            { "}" }
                        </style>
                    </>
                )
            },
        }
    }
}

fn main(){
    yew::start_app::<App>();
}
