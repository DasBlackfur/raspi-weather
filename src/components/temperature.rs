use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

pub struct TemperatureComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub temperature: f32,
}

impl Component for TemperatureComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        div.set_inner_html(&format!(include_str!("temperature.html"), temperature=&ctx.props().temperature));
        Html::VRef(div.into())
    }
}