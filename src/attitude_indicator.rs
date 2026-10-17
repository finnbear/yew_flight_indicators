use crate::{transition, unwrap_degrees};
use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AttitudeIndicatorProps {
    /// Aircraft pitch in degrees. Max 40 degree deflection is visible.
    pub pitch: f32,
    /// Aircraft roll in degrees.
    pub roll: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
    /// Seconds over which the horizon moves to each new attitude. Zero, the
    /// default, moves it at once.
    #[prop_or(0.0)]
    pub transition: f32,
}

/// Indicates pitch and roll.
#[non_exhaustive]
pub struct AttitudeIndicator {
    /// The roll shown, unwrapped across updates (see [`unwrap_degrees`]), so
    /// the horizon rolls the short way round.
    roll: f32,
}

impl Component for AttitudeIndicator {
    type Message = ();
    type Properties = AttitudeIndicatorProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            roll: ctx.props().roll,
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        self.roll = unwrap_degrees(self.roll, props.roll);
        props != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let props = ctx.props();
        attitude_indicator(&AttitudeIndicatorProps {
            pitch: props.pitch,
            roll: self.roll,
            size: props.size.clone(),
            transition: props.transition,
        })
    }
}

/// Indicates pitch and roll.
///
/// With a `transition`, the horizon rolls the long way round when the roll
/// wraps; the [`AttitudeIndicator`] component doesn't.
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
    let roll_transition = transition(props.transition, "transform");
    let pitch_transition = transition(props.transition, "top, clip-path");
    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div
                class={box_style.clone()}
                style={format!("top: 0%; transform: rotate({}deg);{roll_transition}", props.roll)}
            >
                <div
                    class={box_style.clone()}
                    style={format!("top: {}%; clip-path: circle(36% at 50% {}%);{pitch_transition}", pitch_percent, 50.0 - pitch_percent)}
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
