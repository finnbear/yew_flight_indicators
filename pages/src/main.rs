use yew::{function_component, html, Html};
use yew_flight_indicators::component::*;
use yew_hooks::use_raf;

#[function_component(App)]
fn app() -> Html {
    const SECONDS: u32 = 128;
    let time = use_raf(SECONDS * 1000, 0) as f32 * SECONDS as f32;

    let airspeed = 200.0 + 200.0 * (time * 0.5).sin();
    let size = "14rem";

    html! {<>
        <h2 style="margin-top: 0;">{"yew_flight_indicators"}</h2>
        <div style="display: grid; grid-template-columns: repeat(3,1fr); gap: 0.5rem; width: min-content;">
            <AirspeedIndicator
                {airspeed}
                max_airspeed={airspeed + 50.0}
                stall_airspeed={(airspeed * 0.5).min(50.0)}
                caution_airspeed={airspeed + 10.0}
                never_exceed_airspeed={airspeed + 30.0}
                {size}
            />
            <Altimeter
                altitude={10000.0 + 5000.0 * (time * 0.4).sin()}
                pressure={29.92 + 0.52 * (time * 0.3).sin()}
                {size}
            />
            <AttitudeIndicator
                pitch={50.0 * (time * 0.25).sin()}
                roll={30.0 * (time * 0.5).sin()}
                {size}
            />
            <TurnCoordinator
                turn={30.0 * (time * 0.5).sin()}
                slip={50.0 * (time * 0.6).sin()}
                {size}
            />
            <HeadingIndicator
                heading={time * 10.0}
                autopilot_heading={time * 23.0}
                {size}
            />
            <Variometer
                vertical_speed={2000.0 * (time * 0.5).sin()}
                {size}
            />
        </div>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
