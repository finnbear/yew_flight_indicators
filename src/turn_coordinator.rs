use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TurnCoordinatorProps {
    /// Rate of change of heading, in degrees of turn.
    pub turn: f32,
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates the rate of change of heading.
///
/// The inlinometer is not supported.
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

/// Indicates the rate of change of heading.
///
/// The inlinometer is not supported.
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

    let turn_coordinator = include_str!("./svg_data_uri/turn_coordinator.svg");
    let fi_tc_airplane = include_str!("./svg_data_uri/fi_tc_airplane.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={turn_coordinator} class={box_style.clone()} alt=""/>
                <div class={box_style.clone()} style={format!("transform: rotate({}deg);", props.turn)}>
            <img src={fi_tc_airplane} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
