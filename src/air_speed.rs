use std::array;
use std::fmt::Write;
use stylist::css;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AirspeedIndicatorProps {
    /// Airspeed, in knots.
    pub airspeed: f32,
    /// Max airspeed, in knots, representing the end of the dial. The max air speed
    /// may be rounded up so that the tick marks have labels that are nice round numbers.
    #[prop_or(160.0)]
    pub max_airspeed: f32,
    /// Width and height in any CSS unit.
    #[prop_or("16rem".into())]
    pub size: AttrValue,
}

/// Indicates airspeed. The max airspeed is configurable.
#[non_exhaustive]
pub struct AirspeedIndicator;

impl Component for AirspeedIndicator {
    type Message = ();
    type Properties = AirspeedIndicatorProps;

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        airspeed(ctx.props())
    }
}

/// Indicates airspeed. The max airspeed is configurable.
pub fn airspeed(props: &AirspeedIndicatorProps) -> Html {
    let box_style = css!(
        r#"
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    "#
    );

    let speed = props.airspeed;
    let max_speed = (props.max_airspeed as u32).next_multiple_of(80) as f32;

    let mut speed_mechanics = include_str!("./svg_data_uri/speed_mechanics.svg").to_owned();

    let mut search = String::new();
    let mut ranges: [(usize, _); 9] = array::from_fn(|i| (i, 0usize..0usize));
    for (i, s) in (0..=160).step_by(20).enumerate() {
        write!(&mut search, "%3E{s}%3C").unwrap();

        let index = speed_mechanics.find(&search).unwrap();
        ranges[i].1 = index + 3..index + 3 + search.len() - 6;

        search.clear();
    }

    ranges.sort_by_key(|(_, r)| r.start);

    for (i, range) in ranges.into_iter().rev() {
        write!(&mut search, "{}", i * max_speed as usize / 8).unwrap();
        speed_mechanics.replace_range(range.clone(), &search);
        search.clear();
    }

    let fi_needle = include_str!("./svg_data_uri/fi_needle.svg");
    let fi_circle = include_str!("./svg_data_uri/fi_circle.svg");

    let speed = 90.0 + (speed / max_speed * 160.0) * 2.0;

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={speed_mechanics} class={box_style.clone()} alt=""/>
            <div class={box_style.clone()} style={format!("transform: rotate({speed}deg);")}>
                <img src={fi_needle} class={box_style.clone()} alt=""/>
            </div>
            <div class={box_style.clone()}>
                <img src={fi_circle} class={box_style.clone()} alt=""/>
            </div>
        </div>
    }
}
