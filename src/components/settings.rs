use yew::{Component, html};

pub struct SettingsComponent;

impl Component for SettingsComponent {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!{
            { js_sys::Date::new_0().to_locale_time_string("de-DE").to_string() }
        }
    }
}
