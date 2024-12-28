mod airspeed_indicator;
mod airspeed_indicator2;
mod altimeter;
mod altimeter2;
mod attitude_indicator;
mod attitude_indicator2;
mod heading_indicator;
mod turn_coordinator;
mod variometer;

/// Properties, used for components and arguments to functions.
pub mod props {
    pub use crate::airspeed_indicator::AirspeedIndicatorProps;
    pub use crate::airspeed_indicator2::AirspeedIndicatorProps2;
    pub use crate::altimeter::AltimeterProps;
    pub use crate::altimeter2::AltimeterProps2;
    pub use crate::attitude_indicator::AttitudeIndicatorProps;
    pub use crate::attitude_indicator2::AttitudeIndicatorProps2;
    pub use crate::heading_indicator::HeadingIndicatorProps;
    pub use crate::turn_coordinator::TurnCoordinatorProps;
    pub use crate::variometer::VariometerProps;
}

/// Yew components.
pub mod component {
    pub use crate::airspeed_indicator::AirspeedIndicator;
    pub use crate::airspeed_indicator2::AirspeedIndicator2;
    pub use crate::altimeter::Altimeter;
    pub use crate::altimeter2::Altimeter2;
    pub use crate::attitude_indicator::AttitudeIndicator;
    pub use crate::attitude_indicator2::AttitudeIndicator2;
    pub use crate::heading_indicator::HeadingIndicator;
    pub use crate::turn_coordinator::TurnCoordinator;
    pub use crate::variometer::Variometer;
}

/// Pure functions that return Yew `Html`.
pub mod function {
    pub use crate::airspeed_indicator::airspeed;
    pub use crate::airspeed_indicator2::airspeed_indicator2;
    pub use crate::altimeter::altimeter;
    pub use crate::altimeter2::altimeter2;
    pub use crate::attitude_indicator::attitude_indicator;
    pub use crate::attitude_indicator2::attitude_indicator2;
    pub use crate::heading_indicator::heading_indicator;
    pub use crate::turn_coordinator::turn_coordinator;
    pub use crate::variometer::variometer;
}
