use yew::prelude::*;

mod components;

use components::temperature::TemperatureComponent;
use components::wind_angle::WindAngleComponent;
use components::weather::WeatherComponent;

pub enum Msg {
    Update,
    Settings,
    Increment,
}

pub struct App {
    temperature: f32,
    weather: bool,
    settings: bool,
    wind_angle: i16,
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            weather: false,
            settings: false,
            temperature: 0.0,
            wind_angle: 0,
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
            }
            Msg::Increment => {
                self.temperature += 1.0;
                self.wind_angle += 5;
                true
            }
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

                            <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                                { "Increment"}
                            </button>
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
            }
            false => {
                html!(
                    <>
                        <div class="grid-wrapper">
                            <div>
                                <WindAngleComponent wind_angle={self.wind_angle} />
                            </div>
                            <div>
                                <TemperatureComponent temperature={self.temperature}/>
                            </div>
                            <div>
                                
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
                                <button class="button" onclick={ctx.link().callback(|_| Msg::Settings)}>
                                    { "Open Settings" }
                                </button>
                            </div>
                        </div>
                        <style>
                            { "body {"}
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
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
