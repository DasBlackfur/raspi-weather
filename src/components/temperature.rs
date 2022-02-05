use yew::{Component, html};

pub struct TemperatureComponent {
    temperature: f32,
}

impl Component for TemperatureComponent {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            temperature: 0.0,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!{
            <svg width="200px" height="200px">
                <defs>
                    <linearGradient id="grad" x1="0" x2="0" y1="0" y2="1">
                        <stop offset="0%" stop-color="rgba(0, 0, 0, 0)" />
                        <stop offset="25%" stop-color="rgba(0, 0, 0, 0)" />
                        <stop offset="25%" stop-color="rgba(calc(2.55*75), 0, 0, 255)"/>
                        <stop offset="100%" stop-color="rgba(70, 70, 255, 255"/>
                    </linearGradient>
                </defs>
                <path d="M 40 150 A 20 20 0 1 0 70 150 L 70 50 A 10 10 0 1 0 40 50 L 40 150 Z" stroke="black" stroke-width="3" fill="url(#grad)"/>
                <text x="100" y="125" font-size="50px">{ format!("{}°C", self.temperature) }</text>
            </svg>
        }
    }
}