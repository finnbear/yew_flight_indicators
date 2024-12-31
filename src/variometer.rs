use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct VariometerProps {
    /// Vertical speed in feet per minute. Max 20 feet per minute is visible.
    pub vertical_speed: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates vertical speed.
#[non_exhaustive]
pub struct Variometer;

impl Component for Variometer {
    type Message = ();
    type Properties = VariometerProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        variometer(ctx.props())
    }
}

/// Indicates vertical speed.
pub fn variometer(props: &VariometerProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let variometer_outside = include_str!("./svg_part_data_uri/variometer_outside.svg");
    let variometer_hand = include_str!("./svg_part_data_uri/variometer_hand.svg");

    let vario = props.vertical_speed * (1.0 / 100.0);
    fn positive_to_angle(vario: f32) -> f32 {
        if vario < 5.0 {
            vario * (35.0 / 5.0)
        } else if vario < 10.0 {
            35.0 + (vario - 5.0) * (45.0 / 5.0)
        } else if vario < 15.0 {
            35.0 + 45.0 + (vario - 10.0) * (50.0 / 5.0)
        } else {
            35.0 + 45.0 + 50.0 + (vario.min(20.0) - 15.0) * (42.5 / 5.0)
        }
    }
    let angle = positive_to_angle(vario.abs()).copysign(vario);

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={variometer_outside} class={box_style.clone()} alt=""/>
            <img
                src={variometer_hand}
                class={box_style.clone()}
                alt=""
                style={format!("transform: rotate({angle}deg);")}
            />
        </div>
    }
}
