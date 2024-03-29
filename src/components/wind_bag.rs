use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

pub struct WindBagComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub speed: i16,
}

impl Component for WindBagComponent {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        let speed = ctx.props().speed;
        div.set_inner_html(&format!(
            include_str!("sources/wind_bag.html"),
            speed = speed,
            deg = (75 - speed * 2)
        ));
        Html::VRef(div.into())
    }
}
