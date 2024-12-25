use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AltimeterProps2 {
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
pub struct Altimeter2;

impl Component for Altimeter2 {
    type Message = ();
    type Properties = AltimeterProps2;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        altimeter2(ctx.props())
    }
}

/// Indicates altitude. The pressure calibration is configurable.
pub fn altimeter2(props: &AltimeterProps2) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let altimeter_pressure =  include_str!("./svg_data_uri/altimeter_pressure.svg");
    let altimeter_outside =  include_str!("./svg_data_uri/altimeter_outside.svg");
    let altimeter_face =  include_str!("./svg_data_uri/altimeter_face.svg");
    let altimeter_small_hand =  include_str!("./svg_data_uri/altimeter_small_hand.svg");
    let altimeter_large_hand =  include_str!("./svg_data_uri/altimeter_large_hand.svg");

    let needle = 90.0 + (props.altitude as u32 % 1000) as f32 * 360.0 / 1000.0;
    let needle_small = props.altitude as f32 / 10000.0 * 360.0;
    let pressure = 2.0 * props.pressure - 1980.0;

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div class={box_style.clone()} style={format!("transform: rotate({pressure}deg);")}>
                <img src={altimeter_pressure} class={box_style.clone()} alt=""/>
            </div>
            <img src={altimeter_outside} class={box_style.clone()} alt=""/>
            <div class={box_style.clone()}>
                <img src={altimeter_face} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()} style={format!("transform: rotate({needle_small}deg);")}>
                <img src={altimeter_small_hand} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()} style={format!("transform: rotate({needle}deg);")}>
                <img src={altimeter_large_hand} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
