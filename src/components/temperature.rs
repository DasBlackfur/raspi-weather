use gloo_utils::document;
use web_sys::Element;
use yew::{html, Component, Html};

const HTML: &str = include_str!("temperature.html");

pub struct TemperatureComponent {
    temperature: f32,
}

impl Component for TemperatureComponent {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self { temperature: 0.0 }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        div.set_inner_html(HTML);
        Html::VRef(div.into())
    }
}
