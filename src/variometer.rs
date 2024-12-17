use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct VariometerProps {
    /// Vertical speed in feet per minute.
    pub vertical_speed: f32,
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates vertical speed.
#[non_exhaustive]
pub struct Variometer;

impl Component for Variometer {
    type Message = ();
    type Properties = VariometerProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        variometer(ctx.props())
    }
}

/// Indicates vertical speed.
pub fn variometer(props: &VariometerProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let vertical_mechanics = include_str!("./svg_data_uri/vertical_mechanics.svg");
    let fi_needle = include_str!("./svg_data_uri/fi_needle.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    let vario = props.vertical_speed * (1.0 / 1000.0);
    const LIMIT: f32 = 1.95;
    let vario = vario.clamp(-LIMIT, LIMIT);
    let vario = vario * 90.0;

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={vertical_mechanics} class={box_style.clone()} alt=""/>
            <div class={box_style.clone()} style={format!("transform: rotate({}deg);", vario)}>
                <img src={fi_needle} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
