use yew::{html, Component, Properties};

pub struct CO2Component;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub co2level: u16,
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
                <text x="0" y="130" style="font-size: 38px;">{ format!("{} ppm", &level) }</text>
            </svg>
        )
    }
}

fn get_color_from_level(level: &u16) -> String {
    match level {
        0..=600 => "green".to_string(),
        601..=900 => "GreenYellow".to_string(),
        901..=1200 => "yellow".to_string(),
        1201..=1500 => "orange".to_string(),
        1501..=u16::MAX => "red".to_string(),
    }
}

fn get_text_from_level(level: &u16) -> String {
    match level {
        0..=900 => "Gut".to_string(),
        901..=1000 => "Mittel".to_string(),
        1001..=1100 => "Schlecht".to_string(),
        1101..=u16::MAX => "BROKEN".to_string(),
    }
}
