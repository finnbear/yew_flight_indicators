use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TurnCoordinatorProps {
    /// Rate of change of heading, in degrees of turn.
    pub turn: f32,
    /// Slip angle in degrees. Use negative values for skid.
    #[prop_or(0.0)]
    pub slip: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates the rate of change of heading and the slip/skid.
#[non_exhaustive]
pub struct TurnCoordinator;

impl Component for TurnCoordinator {
    type Message = ();
    type Properties = TurnCoordinatorProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        turn_coordinator(ctx.props())
    }
}

/// Indicates the rate of change of heading and the slip/skid.
pub fn turn_coordinator(props: &TurnCoordinatorProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let turn_coordinator_outside = include_str!("./svg_part_data_uri/turn_coordinator_outside.svg");
    let turn_coordinator_ball = include_str!("./svg_part_data_uri/turn_coordinator_ball.svg");
    let turn_coordinator_plane = include_str!("./svg_part_data_uri/turn_coordinator_plane.svg");

    let slip = props.slip.clamp(-45.0, 45.0).to_radians().sin() * 20.0;
    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={turn_coordinator_outside} class={box_style.clone()} alt=""/>
            <img
                src={turn_coordinator_ball}
                class={box_style.clone()}
                alt=""
                style={format!("transform: translate({}%, {}%);", slip, slip.abs() * -0.05)}
            />
            <img
                src={turn_coordinator_plane}
                class={box_style.clone()}
                alt=""
                style={format!("transform: rotate({}deg);", props.turn)}
            />
        </div>
    }
}
