use yew::{function_component, html, Html};
use yew_flight_indicators::component::*;
use yew_hooks::use_raf;

#[function_component(App)]
fn app() -> Html {
    const SECONDS: u32 = 128;
    let time = use_raf(SECONDS * 1000, 0) as f32 * SECONDS as f32;

    let airspeed = 80.0 + 80.0 * (time * 0.5).sin();
    let size = "14rem";

    html! {<>
        <h2 style="margin-top: 0;">{"yew_flight_indicators"}</h2>
        <div style="display: grid; grid-template-columns: repeat(3,1fr); width: min-content;">
            <AirspeedIndicator2
                {airspeed}
                max_airspeed={airspeed + 50.0}
                stall_airspeed={airspeed * 0.5}
                caution_airspeed={airspeed + 10.0}
                never_exceed_airspeed={airspeed + 30.0}
                {size}
            />
            <Altimeter2
                altitude={10000.0 + 5000.0 * (time * 0.4).sin()}
                pressure={29.92 + 0.52 * (time * 0.3).sin()}
                {size}
            />
            <AttitudeIndicator2
                pitch={50.0 * (time * 0.25).sin()}
                roll={30.0 * (time * 0.5).sin()}
                {size}
            />
            <TurnCoordinator2
                turn={30.0 * (time * 0.5).sin()}
                slip={50.0 * (time * 0.6).sin()}
                {size}
            />
            <HeadingIndicator2
                heading={time * 10.0}
                autopilot_heading={time * 23.0}
                {size}
            />
            <Variometer2
                vertical_speed={2000.0 * (time * 0.5).sin()}
                {size}
            />
            <AirspeedIndicator
                airspeed={80.0 + 80.0 * (time * 0.5).sin()}
                max_airspeed={120.0 + 80.0 * (time * 0.5).sin()}
            />
            <Altimeter
                altitude={50.0 * time}
                pressure={1013.25 + 10.0 * (time * 0.3).sin()}
            />
            <AttitudeIndicator
                pitch={50.0 * (time * 0.25).sin()}
                roll={30.0 * (time * 0.5).sin()}
            />
            <TurnCoordinator
                turn={30.0 * (time * 0.5).sin()}
            />
            <HeadingIndicator heading={time * 10.0}/>
            <Variometer vertical_speed={2000.0 * (time * 0.5).sin()}/>
        </div>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
