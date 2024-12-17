use stylist::css;
use yew::{html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeadingIndicatorProps {
    /// Compass heading in degrees.
    pub heading: f32,
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

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
                style={format!("transform: rotate({}deg)", -props.heading)}
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
