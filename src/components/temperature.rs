use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html};

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
        div.set_inner_html(&format!(include_str!("temperature.html"), temperature=self.temperature));
        Html::VRef(div.into())
    }
}
