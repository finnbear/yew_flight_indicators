use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeadingIndicatorProps2 {
    /// Compass heading in degrees.
    pub heading: f32,
    /// Optional autopilot heading in degrees.
    #[prop_or(None)]
    pub autopilot_heading: Option<f32>,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates compass heading.
#[non_exhaustive]
pub struct HeadingIndicator2;

impl Component for HeadingIndicator2 {
    type Message = ();
    type Properties = HeadingIndicatorProps2;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        heading_indicator2(ctx.props())
    }
}

/// Indicates compass heading.
pub fn heading_indicator2(props: &HeadingIndicatorProps2) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    
    let heading_indicator_back = include_str!("./svg_data_uri/heading_indicator_back.svg");
    let heading_indicator_autopilot = include_str!("./svg_data_uri/heading_indicator_autopilot.svg");
    let heading_indicator_outside = include_str!("./svg_data_uri/heading_indicator_outside.svg");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img
                src={heading_indicator_back}
                class={box_style.clone()}
                alt=""
                style={format!("transform: rotate({}deg);", -props.heading)}
            />
            if let Some(autopilot_heading) = props.autopilot_heading {
                <img
                    src={heading_indicator_autopilot}
                    class={box_style.clone()}
                    alt=""
                    style={format!("transform: rotate({}deg);", -props.heading + autopilot_heading)}
                />
            }
            <img src={heading_indicator_outside} class={box_style.clone()} alt=""/>
        </div>
    }
}
