use yew::{Component, Properties, html};

pub struct HumidityComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub humidity: u8
}

impl Component for HumidityComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let percent = ctx.props().humidity;
        html!(
            <svg width="200px" height="200px">
                <polygon points="20,80 80,80, 50,20" stroke="black" stroke-width="5" fill={get_color_from_percent(&percent)}/>
                <text x="40" y="73" style="font-size: 50px;">{ "!" }</text>
                <text x="80" y="40" style="font-size: 25px;">{get_text_from_percent(&percent)}</text>
                <text x="80" y="70" style="font-size: 25px;">{ "Feuchtigkeit" }</text>
                <text x="0" y="130" style="font-size: 38px;">{ format!("{} %", &percent) }</text>
            </svg>
        )
    }
}

fn get_color_from_percent(percent: &u8) -> String {
    match percent {
        91..=100 => "green".to_string(),
        81..=90 => "yellow".to_string(),
        61..=80 => "red".to_string(),
        0..=60 => "blue".to_string(),
        101..=u8::MAX => "blue".to_string()
    }
}

fn get_text_from_percent(percent: &u8) -> String {
    match percent {
        91..=100 => "Good".to_string(),
        81..=90 => "Medium".to_string(),
        61..=80 => "Low".to_string(),
        0..=60 => "BROKEN".to_string(),
        101..=u8::MAX => "BROKEN".to_string()
    }
}