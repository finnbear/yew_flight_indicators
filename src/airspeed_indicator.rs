use stylist::css;
use yew::{html, html_nested, AttrValue, Component, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AirspeedIndicatorProps {
    /// Current airspeed, in knots.
    pub airspeed: f32,
    /// Max airspeed, in knots, to label. The max airspeed
    /// may be rounded up so that the tick marks have labels that are nice round numbers.
    #[prop_or(160.0)]
    pub max_airspeed: f32,
    /// Stall airspeed, in knots, depicted as a green arc which ends at the `caution_airspeed`,
    /// `never_exceed_speed`, or `max_airspeed`.
    #[prop_or(None)]
    pub stall_airspeed: Option<f32>,
    #[prop_or(None)]
    /// Depicted as a yellow arc which ends at `never_exceed_speed` or the end of the dial.
    pub caution_airspeed: Option<f32>,
    /// Depicted as a red line.
    #[prop_or(None)]
    pub never_exceed_airspeed: Option<f32>,
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
        airspeed_indicator(ctx.props())
    }
}

/// Indicates airspeed. The max airspeed is configurable.
pub fn airspeed_indicator(props: &AirspeedIndicatorProps) -> Html {
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

    let airspeed_indicator_outside =
        include_str!("./svg_part_data_uri/airspeed_indicator_outside.svg");
    let airspeed_indicator_hand = include_str!("./svg_part_data_uri/airspeed_indicator_hand.svg");

    let total_angle = std::f32::consts::TAU - 40f32.to_radians();
    let speed_to_angle = |speed| -> f32 { speed / max_speed * total_angle };
    let speed_angle = speed_to_angle(speed).to_degrees();

    struct ArcRange {
        start: f32,
        end: f32,
        color: &'static str,
    }

    let mut ranges = Vec::<ArcRange>::new();

    if let Some(stall_speed) = props.stall_airspeed {
        let end = props
            .caution_airspeed
            .or(props.never_exceed_airspeed)
            .unwrap_or(props.max_airspeed);
        if end > stall_speed {
            ranges.push(ArcRange {
                start: stall_speed,
                end,
                color: "green",
            });
        }
    }
    if let Some(caution_speed) = props.caution_airspeed {
        let end = props.never_exceed_airspeed.unwrap_or(max_speed);
        if end > caution_speed {
            ranges.push(ArcRange {
                start: caution_speed,
                end,
                color: "yellow",
            });
        }
    }

    html! {
        <div
            style={format!("height: {}; width: {}; position: relative; display: inline-block; overflow: hidden;", props.size, props.size)}
        >
            <img src={airspeed_indicator_outside} class={box_style.clone()} alt=""/>
            <svg viewBox="0 0 100 100" class={box_style.clone()}>
                {(0..9).map(|i| {
                    let label = i * max_speed as usize / 8;
                    let angle = i as f32 / 8.0 * total_angle - std::f32::consts::FRAC_PI_2;
                    let (sin, cos) = angle.sin_cos();
                    let radius = 32.0;
                    html_nested!{
                        <text
                            x={(50.0 + cos * radius).to_string()}
                            y={(50.0 + sin * radius).to_string()}
                            fill="white"
                            style="font-size: 5.5px;"
                            text-anchor={if cos > 0.6 {
                                "end"
                            } else if cos.abs() <= 0.6 {
                                "middle"
                            } else {
                                "start"
                            }}
                            alignment-baseline={if sin < -0.6 {
                                "hanging"
                            } else if sin.abs() <= 0.6 {
                                "middle"
                            } else {
                                "baseline"
                            }}
                        >{label.to_string()}</text>
                    }
                }).collect::<Html>()}
                {ranges.into_iter().map(|range| {
                    let start_angle = speed_to_angle(range.start) - std::f32::consts::FRAC_PI_2;
                    let end_angle = speed_to_angle(range.end) - std::f32::consts::FRAC_PI_2;

                    let radius = 42.0;
                    let start_x = 50.0 + start_angle.cos() * radius;
                    let start_y = 50.0 + start_angle.sin() * radius;
                    let end_x = 50.0 + end_angle.cos() * radius;
                    let end_y = 50.0 + end_angle.sin() * radius;
                    let large = if end_angle - start_angle > std::f32::consts::PI {
                        "1"
                    } else {
                        "0"
                    };
                    html_nested!{
                        <path
                            stroke={range.color}
                            fill="none"
                            stroke-width="2"
                            d={format!(
                                "M {start_x} {start_y} A {radius} {radius} 0 {large} 1 {end_x} {end_y}"
                            )}
                        />
                    }
                }).collect::<Html>()}
                if let Some(never_exceed_angle) = props.never_exceed_airspeed.map(|s| speed_to_angle(s) - std::f32::consts::FRAC_PI_2) {
                    <line
                        x1={(50.0 + never_exceed_angle.cos() * 39.0).to_string()}
                        y1={(50.0 + never_exceed_angle.sin() * 39.0).to_string()}
                        x2={(50.0 + never_exceed_angle.cos() * 43.0).to_string()}
                        y2={(50.0 + never_exceed_angle.sin() * 43.0).to_string()}
                        stroke={"red"}
                        stroke-width="2"
                    />
                }
            </svg>
            <img
                src={airspeed_indicator_hand}
                class={box_style.clone()}
                style={format!("transform: rotate({speed_angle}deg);")}
                alt=""
            />
        </div>
    }
}
