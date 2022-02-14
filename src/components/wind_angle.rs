use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

pub struct WindAngleComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub wind_angle: i16,
}

impl Component for WindAngleComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        let wind_angle = ctx.props().wind_angle;
        div.set_inner_html(&format!(
            include_str!("sources/wind_angle.html"),
            deg = wind_angle
        ));
        Html::VRef(div.into())
    }
}
