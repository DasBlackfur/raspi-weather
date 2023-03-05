use csv::Reader;
use gloo::{timers::callback::Interval, console::externs::log};
use reqwasm::http::Request;
use yew::{html, Component, Properties};

use crate::credentials::{INFLUX_TOKEN, INFLUX_ORG};

pub struct CO2Component {
    interval: Interval,
    watt: u16
}

pub enum Msg {
    Update,
    Value(String)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub co2level: u16,
}

impl Component for CO2Component {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let clock_hanlde = {
            let link = ctx.link().clone();
            Interval::new(10000, move || link.send_message(Msg::Update))
        };
        ctx.link().send_message(Msg::Update);
        Self {
            interval: clock_hanlde,
            watt: 1,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                ctx.link().send_future(async move {
                    let response = Request::post(
                        &format!("http://192.168.12.100:8086/api/v2/query?org={}", INFLUX_ORG),
                    )
                    .header("Authorization", &format!("Token {}", INFLUX_TOKEN))
                    .header("accept", "application/csv")
                    .header("Content-type", "application/vnd.flux")
                    .body("from(bucket: \"mathome-sensors\")
                    |> range(start: -1d)
                    |> filter(fn: (r) => r[\"_measurement\"] == \"shellies\")
                    |> filter(fn: (r) => r[\"_field\"] == \"apower\")
                    |> last()")
                    .send()
                    .await
                    .unwrap();

                    Msg::Value(response.text().await.unwrap())
                });
                false
            },
            Msg::Value(str) => {
                self.watt = Reader::from_reader(str.as_bytes()).records().next().unwrap().unwrap()[6].parse().unwrap();
                true
            },
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let level = ctx.props().co2level;
        html!(
            <svg width="200px" height="200px">
                <polygon points="20,80 80,80, 50,20" stroke="black" stroke-width="5" fill={get_color_from_level(&level)}/>
                <text x="40" y="73" style="font-size: 50px;">{ "!" }</text>
                <text x="85" y="60" style="font-size: 30px;">{ "CO2" }</text>
                <text x="0" y="130" style="font-size: 38px;">{ format!("{} ppm", &level) }</text>
                <text x="0" y="180" style="font-sime: 38px;">{ format!("{} W", &self.watt)}</text>
            </svg>
        )
    }
}

fn get_color_from_level(level: &u16) -> String {
    match level {
        0..=600 => "green".to_string(),
        601..=900 => "GreenYellow".to_string(),
        901..=1200 => "yellow".to_string(),
        1201..=1500 => "orange".to_string(),
        1501..=u16::MAX => "red".to_string(),
    }
}

fn get_text_from_level(level: &u16) -> String {
    match level {
        0..=900 => "Gut".to_string(),
        901..=1000 => "Mittel".to_string(),
        1001..=1100 => "Schlecht".to_string(),
        1101..=u16::MAX => "BROKEN".to_string(),
    }
}
