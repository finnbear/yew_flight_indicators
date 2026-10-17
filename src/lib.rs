/// The CSS `transition` for an indicator's moving part, easing each of the
/// comma-separated `properties` over `seconds`, or nothing at all if
/// `seconds` is not positive.
///
/// Linear, so a part fed a steady stream of readings moves steadily between
/// them rather than lurching to each and settling.
fn transition(seconds: f32, properties: &str) -> String {
    if seconds > 0.0 {
        let transitions = properties
            .split(',')
            .map(|property| format!("{} {seconds}s linear", property.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        format!(" transition: {transitions};")
    } else {
        String::new()
    }
}

/// `angle` in degrees, moved by whole turns to within half a turn of
/// `previous`: so that a rotation transitions the short way round, from 359°
/// to 1° through 0° rather than back through 180°.
fn unwrap_degrees(previous: f32, angle: f32) -> f32 {
    previous + ((angle - previous + 180.0).rem_euclid(360.0) - 180.0)
}

mod airspeed_indicator;
mod altimeter;
mod attitude_indicator;
mod heading_indicator;
mod turn_coordinator;
mod variometer;

/// Properties, used for components and arguments to functions.
pub mod props {
    pub use crate::airspeed_indicator::AirspeedIndicatorProps;
    pub use crate::altimeter::AltimeterProps;
    pub use crate::attitude_indicator::AttitudeIndicatorProps;
    pub use crate::heading_indicator::HeadingIndicatorProps;
    pub use crate::turn_coordinator::TurnCoordinatorProps;
    pub use crate::variometer::VariometerProps;
}

/// Yew components.
pub mod component {
    pub use crate::airspeed_indicator::AirspeedIndicator;
    pub use crate::altimeter::Altimeter;
    pub use crate::attitude_indicator::AttitudeIndicator;
    pub use crate::heading_indicator::HeadingIndicator;
    pub use crate::turn_coordinator::TurnCoordinator;
    pub use crate::variometer::Variometer;
}

/// Pure functions that return Yew `Html`.
pub mod function {
    pub use crate::airspeed_indicator::airspeed_indicator;
    pub use crate::altimeter::altimeter;
    pub use crate::attitude_indicator::attitude_indicator;
    pub use crate::heading_indicator::heading_indicator;
    pub use crate::turn_coordinator::turn_coordinator;
    pub use crate::variometer::variometer;
}
