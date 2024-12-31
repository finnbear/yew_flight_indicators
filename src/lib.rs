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
