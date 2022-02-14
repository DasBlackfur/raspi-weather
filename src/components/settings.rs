use yew::{Component, html};

pub struct SettingsComponent;

impl Component for SettingsComponent {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"weekday".into(), &"short".into()).unwrap();
        js_sys::Reflect::set(&options, &"year".into(), &"numeric".into()).unwrap();
        js_sys::Reflect::set(&options, &"month".into(), &"short".into()).unwrap();
        js_sys::Reflect::set(&options, &"day".into(), &"numeric".into()).unwrap();
        html!{
            <p style="font-size: 70%">
                { js_sys::Date::new_0().to_locale_time_string("de-DE").to_string() } <br/>
                { js_sys::Date::new_0().to_locale_date_string("de-DE", &options).to_string() }
            </p>
        }
    }
}
