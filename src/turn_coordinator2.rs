use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TurnCoordinatorProps2 {
    /// Rate of change of heading, in degrees of turn.
    pub turn: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates the rate of change of heading.
///
/// The inlinometer is not yet supported.
#[non_exhaustive]
pub struct TurnCoordinator2;

impl Component for TurnCoordinator2 {
    type Message = ();
    type Properties = TurnCoordinatorProps2;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        turn_coordinator2(ctx.props())
    }
}

/// Indicates the rate of change of heading.
///
/// The inlinometer is not yet supported.
pub fn turn_coordinator2(props: &TurnCoordinatorProps2) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let turn_coordinator_outside = include_str!("./svg_data_uri/turn_coordinator_outside.svg");
    let turn_coordinator_plane = include_str!("./svg_data_uri/turn_coordinator_plane.svg");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={turn_coordinator_outside} class={box_style.clone()} alt=""/>
            <img
                src={turn_coordinator_plane}
                class={box_style.clone()}
                alt=""
                style={format!("transform: rotate({}deg);", props.turn)}
            />
        </div>
    }
}
