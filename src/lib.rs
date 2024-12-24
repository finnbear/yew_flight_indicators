mod air_speed;
mod altimeter;
mod attitude;
mod attitude2;
mod heading;
mod turn_coordinator;
mod variometer;

/// Properties, used for components and arguments to functions.
pub mod props {
    pub use crate::air_speed::AirspeedIndicatorProps;
    pub use crate::altimeter::AltimeterProps;
    pub use crate::attitude::AttitudeIndicatorProps;
    pub use crate::attitude2::AttitudeIndicatorProps2;
    pub use crate::heading::HeadingIndicatorProps;
    pub use crate::turn_coordinator::TurnCoordinatorProps;
    pub use crate::variometer::VariometerProps;
}

/// Yew components.
pub mod component {
    pub use crate::air_speed::AirspeedIndicator;
    pub use crate::altimeter::Altimeter;
    pub use crate::attitude::AttitudeIndicator;
    pub use crate::attitude2::AttitudeIndicator2;
    pub use crate::heading::HeadingIndicator;
    pub use crate::turn_coordinator::TurnCoordinator;
    pub use crate::variometer::Variometer;
}

/// Pure functions that return Yew `Html`.
pub mod function {
    pub use crate::air_speed::airspeed;
    pub use crate::altimeter::altimeter;
    pub use crate::attitude::attitude_indicator;
    pub use crate::attitude2::attitude_indicator2;
    pub use crate::heading::heading_indicator;
    pub use crate::turn_coordinator::turn_coordinator;
    pub use crate::variometer::variometer;
}
