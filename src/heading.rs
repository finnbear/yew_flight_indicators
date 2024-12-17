use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeadingIndicatorProps {
    /// Compass heading in degrees.
    pub heading: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates compass heading.
#[non_exhaustive]
pub struct HeadingIndicator;

impl Component for HeadingIndicator {
    type Message = ();
    type Properties = HeadingIndicatorProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        heading_indicator(ctx.props())
    }
}

/// Indicates compass heading.
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

    let heading_mechanics = include_str!("./svg_data_uri/heading_mechanics.svg");
    let heading_yaw = include_str!("./svg_data_uri/heading_yaw.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div
                class={box_style.clone()}
                style={format!("transform: rotate({}deg);", -props.heading)}
            >
                <img src={heading_yaw} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={heading_mechanics} class={box_style.clone()} alt=""/>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
