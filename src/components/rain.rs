use gloo_utils::document;
use web_sys::Element;
use yew::{Component, Html, Properties};

pub struct RainComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub rain_level: u32,
}

impl Component for RainComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let div: Element = document().create_element("div").unwrap();
        let level = ctx.props().rain_level;
        div.set_inner_html(&format!(
            include_str!("sources/rain.html"),
            liter = level,
            percent = 90 - get_percent_from_level(level)
        ));
        Html::VRef(div.into())
    }
}

fn get_percent_from_level(level: u32) -> u8 {
    let percent = level as f64 * 5.0;
    match percent {
        percent if percent > 90.0 => 90,
        percent if percent < 0.0 => 0,
        _ => percent as u8,
    }
}
