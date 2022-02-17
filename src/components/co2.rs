use yew::{Component, Properties, html};

pub struct CO2Component;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub co2level: u16
}

impl Component for CO2Component {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let level = ctx.props().co2level;
        html!(
            <svg width="200px" height="200px">
                <polygon points="20,80 80,80, 50,20" stroke="black" stroke-width="5" fill={get_color_from_level(&level)}/>
                <text x="40" y="73" style="font-size: 50px;">{ "!" }</text>
                <text x="80" y="40" style="font-size: 25px;">{get_text_from_level(&level)}</text>
                <text x="80" y="70" style="font-size: 25px;">{ "CO2" }</text>
                <text x="0" y="130" style="font-size: 38px;">{ format!("{} ppo", &level) }</text>
            </svg>
        )
    }
}

fn get_color_from_level(level: &u16) -> String {
    match level {
        0..=900 => "green".to_string(),
        901..=1000 => "yellow".to_string(),
        1001..=1100 => "red".to_string(),
        1101..=u16::MAX => "blue".to_string()
    }
}

fn get_text_from_level(level: &u16) -> String {
    match level {
        0..=900 => "Nominal".to_string(),
        901..=1000 => "Medium".to_string(),
        1001..=1100 => "High".to_string(),
        1101..=u16::MAX => "BROKEN".to_string()
    }
}