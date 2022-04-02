use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use yew::prelude::*;

mod components;

use components::temperature::TemperatureComponent;
use components::wind_angle::WindAngleComponent;
use components::settings::SettingsComponent;
use components::co2::CO2Component;
use components::humidity::HumidityComponent;
use components::wind_bag::WindBagComponent;

pub enum Msg {
    Update,
    Settings,
    Increment,
    Value(String),
}

pub struct App {
    temperature: f32,
    weather: bool,
    settings: bool,
    wind_angle: i16,
    co2: u16,
    humidity: u8,
    wind_speed: i16,
    interval: Interval
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let data_handle = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::Update))
        };
        Self {
            weather: false,
            settings: false,
            temperature: 0.0,
            wind_angle: 0,
            co2: 800,
            humidity: 50,
            wind_speed: 0,
            interval: data_handle
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                ctx.link().send_future(async {
                    let response = Request::get("https://api.netatmo.com/api/getstationsdata?get_favorites=false")
                        .header("Authorization", "Bearer TOKEN")
                        .header("accept", "application/json")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    
                    Msg::Value(response)
                });
                false
            }
            Msg::Settings => {
                self.settings = !self.settings;
                true
            }
            Msg::Increment => {
                self.temperature += 1.0;
                self.wind_angle += 5;
                self.humidity += 10;
                self.co2 += 100;
                self.wind_speed += 10;
                true
            }
            Msg::Value(val) => {
                let thingy: serde_json::Value = serde_json::from_str(&val).unwrap();
                self.temperature = thingy.pointer("/body/devices/0/dashboard_data/Temperature").unwrap().as_f64().unwrap() as f32;
                self.humidity = thingy.pointer("/body/devices/0/dashboard_data/Humidity").unwrap().as_u64().unwrap() as u8;
                self.co2 = thingy.pointer("/body/devices/0/dashboard_data/CO2").unwrap().as_u64().unwrap() as u16;
                self.wind_angle = thingy.pointer("/body/devices/0/modules/2/dashboard_data/WindAngle").unwrap().as_i64().unwrap() as i16;
                self.wind_speed = thingy.pointer("/body/devices/0/modules/2/dashboard_data/WindStrength").unwrap().as_i64().unwrap() as i16;
                self.weather = match thingy.pointer("/body/devices/0/modules/1/dashboard_data/Rain").unwrap().as_i64().unwrap() {
                    0 => false,
                    _ => true,
                };
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
                                { "Update Data" }
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
                                { "C" }
                            </div>
                            <div>
                                <CO2Component co2level={self.co2} />
                            </div>
                            <div>
                                <WindBagComponent speed={self.wind_speed}/>
                            </div>
                            <div>
                                <HumidityComponent humidity={self.humidity} />
                            </div>
                            <div>
                                { "G" }
                            </div>
                            <div>
                                <SettingsComponent settings_callback={ctx.link().callback(|_| Msg::Settings)}/>
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
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
