use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

#[derive(Clone, PartialEq)]
pub enum Weather {
    Rainy,
    Cloudy,
    Mixed,
    Sunny,
    April,
}

pub struct WeatherComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub weather: Weather,
}

impl Component for WeatherComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        match ctx.props().weather {
            Weather::Rainy => {
                div.set_inner_html(include_str!("sources/rainy.html"));
            }
            Weather::Cloudy => {
                div.set_inner_html(include_str!("sources/cloudy.html"));
            }
            Weather::Mixed => {
                div.set_inner_html(include_str!("sources/mixed.html"));
            }
            Weather::Sunny => {
                div.set_inner_html(include_str!("sources/sunny.html"));
            }
            Weather::April => {
                div.set_inner_html(include_str!("sources/april.html"));
            }
        }
        Html::VRef(div.into())
    }
}
