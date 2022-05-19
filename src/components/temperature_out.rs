use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

pub struct TemperatureComponentOut;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub temperature: f32,
}

impl Component for TemperatureComponentOut {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        let temperature = ctx.props().temperature;
        div.set_inner_html(&format!(
            include_str!("sources/temperature_out.html"),
            temperature = temperature,
            percent = (100 - get_percent_from_temperature(temperature)),
            inv_percent = get_percent_from_temperature(temperature)
        ));
        Html::VRef(div.into())
    }
}

fn get_percent_from_temperature(temperature: f32) -> u8 {
    let percent = (temperature +10.0) * 2.0;
    match percent {
        percent if percent > 100.0 => 100,
        percent if percent < 0.0 => 0,
        _ => percent as u8,
    }
}
