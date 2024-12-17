mod air_speed_indicator;
mod altitude_indicator;
mod attitude_indicator;
mod heading_indicator;


pub mod props {
    pub use crate::air_speed_indicator::AirSpeedIndicatorProps;
    pub use crate::altitude_indicator::AltitudeIndicatorProps;
    pub use crate::attitude_indicator::AttitudeIndicatorProps;
    pub use crate::heading_indicator::HeadingIndicatorProps;
}

pub mod component {

}

pub mod function {
    pub use crate::air_speed_indicator::air_speed_indicator;
    pub use crate::altitude_indicator::altitude_indicator;
    pub use crate::attitude_indicator::attitude_indicator;
    pub use crate::heading_indicator::heading_indicator;
}
