use crate::{transition, unwrap_degrees};
use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeadingIndicatorProps {
    /// Compass heading in degrees.
    pub heading: f32,
    /// Optional autopilot heading in degrees.
    #[prop_or(None)]
    pub autopilot_heading: Option<f32>,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
    /// Seconds over which the card turns to each new heading. Zero, the
    /// default, turns it at once.
    #[prop_or(0.0)]
    pub transition: f32,
}

/// Indicates compass heading.
#[non_exhaustive]
pub struct HeadingIndicator {
    /// The heading shown, unwrapped across updates (see [`unwrap_degrees`]),
    /// so the card turns the short way round.
    heading: f32,
}

impl Component for HeadingIndicator {
    type Message = ();
    type Properties = HeadingIndicatorProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            heading: ctx.props().heading,
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        self.heading = unwrap_degrees(self.heading, props.heading);
        props != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let props = ctx.props();
        heading_indicator(&HeadingIndicatorProps {
            heading: self.heading,
            autopilot_heading: props.autopilot_heading,
            size: props.size.clone(),
            transition: props.transition,
        })
    }
}

/// Indicates compass heading.
///
/// With a `transition`, the card turns the long way round when the heading
/// wraps; the [`HeadingIndicator`] component doesn't.
pub fn heading_indicator(props: &HeadingIndicatorProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let heading_indicator_back = include_str!("./svg_part_data_uri/heading_indicator_back.svg");
    let heading_indicator_autopilot =
        include_str!("./svg_part_data_uri/heading_indicator_autopilot.svg");
    let heading_indicator_outside =
        include_str!("./svg_part_data_uri/heading_indicator_outside.svg");
    let transition = transition(props.transition, "transform");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img
                src={heading_indicator_back}
                class={box_style.clone()}
                alt=""
                style={format!("transform: rotate({}deg);{transition}", -props.heading)}
            />
            if let Some(autopilot_heading) = props.autopilot_heading {
                <img
                    src={heading_indicator_autopilot}
                    class={box_style.clone()}
                    alt=""
                    style={format!("transform: rotate({}deg);{transition}", -props.heading + autopilot_heading)}
                />
            }
            <img src={heading_indicator_outside} class={box_style.clone()} alt=""/>
        </div>
    }
}
