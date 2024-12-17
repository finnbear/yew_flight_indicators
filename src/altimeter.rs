use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AltimeterProps {
    /// Aircraft altitude in feet.
    pub altitude: f32,
    /// Air pressure calibration in mmHg.
    #[prop_or(1013.25)]
    pub pressure: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates altitude. The pressure calibration is configurable.
#[non_exhaustive]
pub struct Altimeter;

impl Component for Altimeter {
    type Message = ();
    type Properties = AltimeterProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        altimeter(ctx.props())
    }
}

/// Indicates altitude. The pressure calibration is configurable.
pub fn altimeter(props: &AltimeterProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let fi_needle = include_str!("./svg_data_uri/fi_needle.svg");
    let fi_needle_small = include_str!("./svg_data_uri/fi_needle_small.svg");
    let altitude_pressure = include_str!("./svg_data_uri/altitude_pressure.svg");
    let altitude_ticks = include_str!("./svg_data_uri/altitude_ticks.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    let needle = 90.0 + (props.altitude as u32 % 1000) as f32 * 360.0 / 1000.0;
    let needle_small = props.altitude as f32 / 10000.0 * 360.0;
    let pressure = 2.0 * props.pressure - 1980.0;

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div class={box_style.clone()} style={format!("transform: rotate({pressure}deg);")}>
                <img src={altitude_pressure} class={box_style.clone()} alt=""/>
            </div>
            <img src={altitude_ticks} class={box_style.clone()} alt=""/>
            <div class={box_style.clone()} style={format!("transform: rotate({needle_small}deg);")}>
                <img src={fi_needle_small} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()} style={format!("transform: rotate({needle}deg);")}>
                <img src={fi_needle} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
