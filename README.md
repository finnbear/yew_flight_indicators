# yew_flight_indicators

[![Documentation](https://docs.rs/yew_confetti/badge.svg)](https://docs.rs/yew_flight_indicators)
[![crates.io](https://img.shields.io/crates/v/yew_flight_indicators.svg)](https://crates.io/crates/yew_flight_indicators)
[![Build](https://github.com/finnbear/yew_flight_indicators/actions/workflows/build.yml/badge.svg)](https://github.com/finnbear/yew_flight_indicators/actions/workflows/build.yml) 
[![Test Page](https://img.shields.io/badge/Test-page-green)](https://finnbear.github.io/yew_flight_indicators/)

Like [jQuery Flight Indicators](https://github.com/sebmatton/jQuery-Flight-Indicators) and [react-flight-indicators](https://github.com/skyhop/react-flight-indicators) but for Yew.

![example](/example.png)

## Usage

```rust
use yew_flight_indicators::component::*;

yew::html!{
    <div style="display: grid; grid-template-columns: repeat(3,1fr); width: min-content;">
        <AirspeedIndicator
            airspeed={80.0} // Knots.
            max_airspeed={160.0} // Knots; default can be omitted.
        />
        <Altimeter
            altitude={50.0} // Feet.
            pressure={1013.25} // mmHg; default can be omitted.
        />
        <AttitudeIndicator
            pitch={50.0} // Degrees.
            roll={30.0} // Degrees.
        />
        <TurnCoordinator
            turn={30.0} // Degrees.
        />
        <HeadingIndicator
            heading={200.0} // Degrees.
        />
        <Variometer
            vertical_speed={500.0} // Feet per minute.
        />
    </div>
}
```

## Changes

- Configurable max air speed, instead of it being fixed to 160 knots.
- SVG's are optimized in advance (see `Makefile`).
- Remove extraneous `filter:url(#filter7320)` from `fi_tc_airplane.svg`.
- Showing a box around an indicator is not supported.

## Authors

Danny Edwards created the original [attitude-indicator](https://gitlab.com/DannyEdwards/attitude-indicator) in HTML.

Sébastien Matton added SVG's and adapted it into a [jQuery plugin](https://github.com/sebmatton/jQuery-Flight-Indicators), as part of his master's for showing realtime flight information from a quadcopter.

Corstian Boerman adapted the project by Sébastien into a [React library](https://github.com/skyhop/react-flight-indicators).

Finn Bear has adapted the project by Corstian into a Yew library.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * [CC BY 4.0](svg_src/LICENSE-CC-BY)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.