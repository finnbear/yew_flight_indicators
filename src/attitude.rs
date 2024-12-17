use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AttitudeIndicatorProps {
    /// Aircraft pitch in degrees.
    pub pitch: f32,
    /// Aircraft roll in degrees.
    pub roll: f32,
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

    let horizon_back = include_str!("./svg_data_uri/horizon_back.svg");
    let horizon_ball = include_str!("./svg_data_uri/horizon_ball.svg");
    let horizon_circle = include_str!("./svg_data_uri/horizon_circle.svg");
    let horizon_mechanics = include_str!("./svg_data_uri/horizon_mechanics.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <div
                class={box_style.clone()}
                style={format!("top: 0%; transform: rotate({}deg);", props.roll)}
            >
                <img src={horizon_back} alt="" class={box_style.clone()}/>
                <div
                    class={box_style.clone()}
                    style={format!("top: {}%;", props.pitch.clamp(-30.0, 30.0) * 0.7)}
                >
                    <img src={horizon_ball} class={box_style.clone()} alt=""/>
                </div>
                <img src={horizon_circle} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={horizon_mechanics} class={box_style.clone()} alt=""/>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
