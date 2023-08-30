
use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use serde_json::Value;

use yew::prelude::*;

mod components;

mod credentials;
use credentials::*;

use components::co2::CO2Component;
use components::humidity::HumidityComponent;
use components::rain::RainComponent;
use components::settings::SettingsComponent;
use components::temperature::TemperatureComponent;
use components::temperature_out::TemperatureComponentOut;
use components::wind_angle::WindAngleComponent;
use components::wind_bag::WindBagComponent;

pub enum Msg {
    Update,
    Settings,
    Increment,
    Refresh,
    Token(String, String),
    Value(String),
}

pub struct App {
    temperature: f32,
    temperature_out: f32,
    weather: bool,
    settings: bool,
    wind_angle: i16,
    co2: u16,
    humidity: u8,
    wind_speed: i16,
    rain: f32,
    interval: Interval,
    token: String,
    refresh_token: String,
    got_token: bool,
    timestamp: f64,
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let data_handle = {
            let link = ctx.link().clone();
            Interval::new(60000, move || link.send_message(Msg::Update))
        };

        ctx.link().send_future(async {
            //let response = Request::post(&"https://api.netatmo.com/oauth2/token?grant_type=password&client_id=".to_string() + CLIENT_ID.to + "&client_secret=" + &CLIENT_SECRET + "&username=" + &USERNAME + "&password=" + &PASSWORD).send().await.unwrap();
            let response = Request::get("http://192.168.12.75:8081/get/Wetterstation")
                .send()
                .await
                .unwrap();
            let code = response.text().await.unwrap().as_str().to_string();
            if code == "NONE" {
                return Msg::Update;
            }
            let response = Request::post("https://api.netatmo.com/oauth2/token")
                .header(
                    "Content-Type",
                    "application/x-www-form-urlencoded;charset=UTF-8",
                )
                .body(
                    "grant_type=authorization_code&client_id=".to_string()
                        + CLIENT_ID
                        + "&client_secret="
                        + CLIENT_SECRET
                        + "&code="
                        + &code
                        + "&scope=read_station&redirect_uri=http://192.168.12.75:8081/auth",
                )
                .send()
                .await
                .unwrap();
            let thingy: serde_json::Value =
                serde_json::from_str(&response.text().await.unwrap()).unwrap();
            Msg::Token(
                thingy
                    .pointer("/access_token")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                thingy
                    .pointer("/refresh_token")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        });
        Self {
            weather: false,
            settings: false,
            temperature: 0.0,
            wind_angle: 0,
            co2: 0,
            humidity: 0,
            wind_speed: 0,
            interval: data_handle,
            token: "".to_string(),
            refresh_token: "".to_string(),
            got_token: false,
            temperature_out: 0.0,
            rain: 0.0,
            timestamp: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                if self.got_token {
                    let token = self.token.clone();

                    ctx.link().send_future(async move {
                        let response = Request::get(
                            "https://api.netatmo.com/api/getstationsdata?get_favorites=false",
                        )
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("accept", "application/json")
                        .send()
                        .await
                        .unwrap();

                        if response.status() != 200 {
                            return Msg::Refresh;
                        }

                        Msg::Value(response.text().await.unwrap())
                    });
                }
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
                self.temperature = thingy
                    .pointer("/body/devices/0/dashboard_data/Temperature")
                    .unwrap_or(&Value::from(0.0))
                    .as_f64()
                    .unwrap() as f32;
                self.humidity = thingy
                    .pointer("/body/devices/0/dashboard_data/Humidity")
                    .unwrap_or(&Value::from(0))
                    .as_u64()
                    .unwrap() as u8;
                self.co2 = thingy
                    .pointer("/body/devices/0/dashboard_data/CO2")
                    .unwrap_or(&Value::from(0))
                    .as_i64()
                    .unwrap() as u16;
                self.wind_angle = thingy
                    .pointer("/body/devices/0/modules/2/dashboard_data/WindAngle")
                    .unwrap_or(&Value::from(0))
                    .as_i64()
                    .unwrap() as i16;
                self.wind_speed = thingy
                    .pointer("/body/devices/0/modules/2/dashboard_data/GustStrength")
                    .unwrap_or(&Value::from(0))
                    .as_i64()
                    .unwrap() as i16;
                self.weather = match (thingy
                    .pointer("/body/devices/0/modules/1/dashboard_data/sum_rain_1")
                    .unwrap_or(&Value::from(0))
                    .as_f64()
                    .unwrap()
                    * 10.0) as u32
                {
                    0 => false,
                    _ => false,
                };
                self.rain = thingy
                    .pointer("/body/devices/0/modules/1/dashboard_data/sum_rain_24")
                    .unwrap_or(&Value::from(0.0))
                    .as_f64()
                    .unwrap() as f32;
                self.temperature_out = thingy
                    .pointer("/body/devices/0/modules/0/dashboard_data/Temperature")
                    .unwrap_or(&Value::from(0.0))
                    .as_f64()
                    .unwrap() as f32;
                self.timestamp = thingy
                    .pointer("/body/devices/0/dashboard_data/time_utc")
                    .unwrap_or(&Value::from(0))
                    .as_f64()
                    .unwrap();
                true
            }
            Msg::Token(token, refresh_token) => {
                self.token = token;
                self.refresh_token = refresh_token;
                self.got_token = true;
                ctx.link().send_message(Msg::Update);
                true
            }
            Msg::Refresh => {
                self.got_token = false;
                let body = "grant_type=refresh_token&client_id=".to_string()
                    + CLIENT_ID
                    + "&client_secret="
                    + CLIENT_SECRET
                    + "&refresh_token="
                    + &self.refresh_token;
                ctx.link().send_future(async {
                    //let response = Request::post(&"https://api.netatmo.com/oauth2/token?grant_type=password&client_id=".to_string() + CLIENT_ID.to + "&client_secret=" + &CLIENT_SECRET + "&username=" + &USERNAME + "&password=" + &PASSWORD).send().await.unwrap();
                    let response = Request::post("https://api.netatmo.com/oauth2/token")
                        .header(
                            "Content-Type",
                            "application/x-www-form-urlencoded;charset=UTF-8",
                        )
                        .body(body)
                        .send()
                        .await
                        .unwrap();
                    let thingy: serde_json::Value =
                        serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    Msg::Token(
                        thingy
                            .pointer("/access_token")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        thingy
                            .pointer("/refresh_token")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    )
                });
                false
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
                                false => "bg_sunny.jpg",
                                true => "bg_thunder.jpg",
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
                                <TemperatureComponentOut temperature={self.temperature_out}/>
                            </div>
                            <div>
                                <TemperatureComponent temperature={self.temperature}/>
                            </div>
                            <div>
                                <CO2Component co2level={self.co2}/>
                            </div>
                            <div>
                                <WindBagComponent speed={self.wind_speed}/>
                            </div>
                            <div>
                                <RainComponent rain_level={self.rain}/>
                            </div>
                            <div>
                                <HumidityComponent humidity={self.humidity} />
                            </div>
                            <div>
                                <SettingsComponent settings_callback={ctx.link().callback(|_| Msg::Refresh)} timestamp={self.timestamp}/>
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
                                false => "bg_sunny.jpg",
                                true => "bg_thunder.jpg",
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
    yew::Renderer::<App>::new().render();
}
