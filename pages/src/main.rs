use gloo_render::{request_animation_frame, AnimationFrame};
use std::cell::RefCell;
use std::rc::Rc;
use yew::{function_component, hook, html, use_effect_with, use_state, Html};
use yew_flight_indicators::component::*;

/// Seconds since first rendered, updated every animation frame, for up to
/// `limit` seconds.
#[hook]
fn use_seconds(limit: f64) -> f32 {
    let seconds = use_state(|| 0.0f64);
    let setter = seconds.setter();
    use_effect_with((), move |_| {
        let frame: Rc<RefCell<Option<AnimationFrame>>> = Rc::default();
        fn next(
            frame: Rc<RefCell<Option<AnimationFrame>>>,
            start: f64,
            limit: f64,
            setter: yew::UseStateSetter<f64>,
        ) {
            let f = frame.clone();
            *frame.borrow_mut() = Some(request_animation_frame(move |now| {
                let elapsed = ((now - start) / 1000.0).min(limit);
                setter.set(elapsed);
                if elapsed < limit {
                    next(f, start, limit, setter);
                }
            }));
        }
        let start = web_sys::window().unwrap().performance().unwrap().now();
        next(frame.clone(), start, limit, setter);
        // Stops it (dropping the frame requested cancels it).
        move || drop(frame.take())
    });
    *seconds as f32
}

#[function_component(App)]
fn app() -> Html {
    let time = use_seconds(128.0);

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
