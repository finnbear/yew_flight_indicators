use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AttitudeIndicatorProps {
    /// Aircraft pitch in degrees.
    pub pitch: f32,
    /// Aircraft roll in degrees.
    pub roll: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates pitch and roll.
#[non_exhaustive]
pub struct AttitudeIndicator;

impl Component for AttitudeIndicator {
    type Message = ();
    type Properties = AttitudeIndicatorProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        attitude_indicator(ctx.props())
    }
}

/// Indicates pitch and roll.
pub fn attitude_indicator(props: &AttitudeIndicatorProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let altitude_indicator_outside =
        include_str!("./svg_part_data_uri/attitude_indicator_outside.svg");
    let altitude_indicator_roll = include_str!("./svg_part_data_uri/attitude_indicator_roll.svg");

    let pitch_percent = props.pitch.clamp(-40.0, 40.0);
    let attitude_indicator_pitch = Html::from_html_unchecked(
        include_str!("./svg_part_data_uri/attitude_indicator_pitch.svg.raw").into(),
    );
    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div
                class={box_style.clone()}
                style={format!("top: 0%; transform: rotate({}deg);", props.roll)}
            >
                <div
                    class={box_style.clone()}
                    style={format!("top: {}%; clip-path: circle(36% at 50% {}%);", pitch_percent, 50.0 - pitch_percent)}
                >
                    // Include as `Html` to allow overflowing untransformed image border.
                    {attitude_indicator_pitch}
                </div>
                <img src={altitude_indicator_roll} class={box_style.clone()} alt=""/>
            </div>
            <img src={altitude_indicator_outside} class={box_style.clone()} alt=""/>
        </div>
    }
}
