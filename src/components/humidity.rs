use csv::Reader;
use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use yew::{html, Component, Properties};

use crate::credentials::{INFLUX_ORG, INFLUX_TOKEN};

pub struct HumidityComponent {
    //_interval: Interval,
    cl: f32,
}

pub enum Msg {
    Update,
    Value(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub humidity: u8,
    pub pressure: f32
}

impl Component for HumidityComponent {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        //let clock_hanlde = {
        //    let link = ctx.link().clone();
        //    Interval::new(10000, move || link.send_message(Msg::Update))
        //};
        //ctx.link().send_message(Msg::Update);
        Self {
            //_interval: clock_hanlde,
            cl: 1.0,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                ctx.link().send_future(async move {
                    let response = Request::post(&format!(
                        "http://192.168.12.100:8086/api/v2/query?org={}",
                        INFLUX_ORG
                    ))
                    .header("Authorization", &format!("Token {}", INFLUX_TOKEN))
                    .header("accept", "application/csv")
                    .header("Content-type", "application/vnd.flux")
                    .body(
                        "from(bucket: \"mathome-sensors\")
                    |> range(start: -1d)
                    |> filter(fn: (r) => r[\"_measurement\"] == \"shellies\")
                    |> filter(fn: (r) => r[\"_field\"] == \"Cl\")
                    |> last()",
                    )
                    .send()
                    .await
                    .unwrap();
                    Msg::Value(response.text().await.unwrap())
                });
                false
            }
            Msg::Value(str) => {
                self.cl = Reader::from_reader(str.as_bytes())
                    .records()
                    .next()
                    .unwrap()
                    .unwrap()[6]
                    .parse()
                    .unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let percent = ctx.props().humidity;
        let pressure = ctx.props().pressure;
        html!(
            <svg width="200px" height="200px">
                <polygon points="20,80 80,80, 50,20" stroke="black" stroke-width="5" fill={get_color_from_percent(&percent)}/>
                <text x="55" y="73" style="font-size: 50px;">{ "!" }</text>
                <text x="90" y="40" style="font-size: 25px;">{ "Feuchte" }</text>
                <text x="95" y="90" style="font-size: 40px;">{ format!("{} %", &percent) }</text>
                <polygon points="20,160 80,160, 50,100" stroke="black" stroke-width="5" fill={get_color_from_pressure(&pressure)}/>
                <text x="55" y="153" style="font-size: 50px;">{ "!" }</text>
                <text x="90" y="140" style="font-size: 25px;">{ "Luftdruck" }</text>
                <text x="35" y="195" style="font-size: 25px;">{ format!("{} mbar", &pressure) }</text>
                //<rect x="0" y="155" width="160" height="45" fill={get_color_from_cl(self.cl)} fill-opacity={get_opacity_from_cl(self.cl)}/>
                //<text x="0" y="190" style="font-size: 38px;">{ format!("{:.2} mg/l", &self.cl)}</text>
            </svg>
        )
    }
}

fn get_color_from_percent(percent: &u8) -> String {
    match percent {
        71..=100 => "red".to_string(),
        61..=70 => "yellow".to_string(),
        41..=60 => "green".to_string(),
        21..=40 => "yellow".to_string(),
        11..=20 => "red".to_string(),
        0..=10 => "blue".to_string(),
        101..=u8::MAX => "blue".to_string(),
    }
}

fn get_color_from_pressure(pressure: &f32) -> String {
    "green".to_string()
}

fn get_color_from_cl(cl: f32) -> String {
    if !(0.25..=0.8).contains(&cl) {
        return "red".to_string();
    }
    if !(0.3..=0.7).contains(&cl) {
        return "yellow".to_string();
    }
    "black".to_string()
}

fn get_opacity_from_cl(cl: f32) -> String {
    if (0.3..=0.7).contains(&cl) {
        return "0%".to_string();
    }
    "100%".to_string()
}
