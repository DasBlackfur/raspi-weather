use yew::{html, Component, Properties};

pub struct HumidityComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub humidity: u8,
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
                <text x="90" y="60" style="font-size: 25px;">{ "Feuchte" }</text>
                <text x="55" y="130" style="font-size: 40px;">{ format!("{} %", &percent) }</text>
            </svg>
        )
    }
}

fn get_color_from_percent(percent: &u8) -> String {
    match percent {
        71..=100 => "red".to_string(),
        61..=70 => "yellow".to_string(),
        41..=60 => "green".to_string(),
        21..=40 => "yellow".to_string(),
        11..=20 => "red".to_string(),
        0..=10 => "blue".to_string(),
        101..=u8::MAX => "blue".to_string(),
    }
}

fn get_text_from_percent(percent: &u8) -> String {
    match percent {
        61..=100 => "Hohe".to_string(),
        41..=60 => "Mittlere".to_string(),
        11..=40 => "Niedrige".to_string(),
        0..=10 => "BROKEN".to_string(),
        101..=u8::MAX => "BROKEN".to_string(),
    }
}
