use gloo::{timers::callback::Interval, console::{__macro::JsValue, log, externs::log}};
use yew::{html, Callback, Component, Properties};

pub struct SettingsComponent {
    interval: Interval,
}

pub enum Msg {
    Update,
}



#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub settings_callback: Callback<crate::MouseEvent>,
    pub timestamp: f64,
}

impl Component for SettingsComponent {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let clock_hanlde = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::Update))
        };
        Self {
            interval: clock_hanlde,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"year".into(), &"numeric".into()).unwrap();
        js_sys::Reflect::set(&options, &"month".into(), &"numeric".into()).unwrap();
        js_sys::Reflect::set(&options, &"day".into(), &"numeric".into()).unwrap();
        html! {
            <>
                <p style="font-size: 60%; padding-left: 10px; margin-bottom:8px">
                    { js_sys::Date::new_0().to_locale_time_string("de-DE") } <br/>
                    { js_sys::Date::new_0().to_locale_date_string("de-DE", &options) } <br/>
                </p>
                <p style="font-size: 30%; padding-left:10px; margin-bottom:0px">
                    { "Letzte Aktualisierung:" } <br/>
                    {js_sys::Date::new(&JsValue::from_f64(ctx.props().timestamp * 1000.0)).to_locale_time_string("de-DE") }
                </p>
                <button class="button" onClick="window.location.reload(true);" style="padding-top:0px">
                    {"⟳"}
                </button>
            </>
        }
    }
}
