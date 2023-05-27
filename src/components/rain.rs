use csv::Reader;
use gloo::timers::callback::Interval;
use gloo_utils::document;
use reqwasm::http::Request;
use web_sys::Element;
use yew::{Component, Html, Properties};

use crate::credentials::{INFLUX_TOKEN, INFLUX_ORG};

pub struct RainComponent {
    interval: Interval,
    ph: f32
}

pub enum Msg {
    Update,
    Value(String)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub rain_level: f32,
}

impl Component for RainComponent {
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
            ph: 1.0,
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
                    |> filter(fn: (r) => r[\"_field\"] == \"pH\")
                    |> last()")
                    .send()
                    .await
                    .unwrap();

                    Msg::Value(response.text().await.unwrap())
                });
                false
            },
            Msg::Value(str) => {
                self.ph = Reader::from_reader(str.as_bytes()).records().next().unwrap().unwrap()[6].parse().unwrap_or(1.0);
                true
            },
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        let level = ctx.props().rain_level;
        div.set_inner_html(&format!(
            include_str!("sources/rain.html"),
            liter = level,
            percent = 90 - get_percent_from_level(level),
            ph_color = get_color_from_ph(self.ph),
            ph_opacity = get_opacity_from_ph(self.ph),
            ph = format!("{:.2} pH", self.ph)
        ));
        Html::VRef(div.into())
    }
}

fn get_percent_from_level(level: f32) -> u8 {
    let percent = level * 5.0;
    match percent {
        percent if percent > 90.0 => 90,
        percent if percent < 0.0 => 0,
        _ => percent as u8,
    }
}

fn get_color_from_ph(ph: f32) -> String {
    if ph < 6.9 || ph > 7.5 {
        return "red".to_string();
    }
    if ph < 7.0 || ph > 7.3 {
        return "yellow".to_string();
    }
    "black".to_string()
}

fn get_opacity_from_ph(ph: f32) -> String {
    if ph >= 7.0 && ph <= 7.3 {
        return "0%".to_string();
    }
    "100%".to_string()
}
