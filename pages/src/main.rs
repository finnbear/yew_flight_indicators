use yew::{
    function_component, html, Html,
};
use yew_flight_indicators::{props::*, function::*};

#[function_component(App)]
fn app() -> Html {
    html! {<>
        <h2 style="margin-top: 0;">{"yew_flight_indicators"}</h2>
        <div style="display: flex; flex-direction: column; gap: 0.5rem; width: min-content;">
            {heading_indicator(&yew::props!{HeadingIndicatorProps{
                heading: 10.0,
            }})}
            {altitude_indicator(&yew::props!{AltitudeIndicatorProps{
                altitude: 1250.0,
            }})}
            {attitude_indicator(&yew::props!{AttitudeIndicatorProps{
                pitch: 20.0,
                roll: 8.0,
            }})}
            {air_speed_indicator(&yew::props!{AirSpeedIndicatorProps{
                air_speed: 80.0,
            }})}
        </div>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
