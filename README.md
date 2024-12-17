# yew_flight_indicators

[![Documentation](https://docs.rs/yew_confetti/badge.svg)](https://docs.rs/yew_flight_indicators)
[![crates.io](https://img.shields.io/crates/v/yew_confetti.svg)](https://crates.io/crates/yew_flight_indicators)
[![Build](https://github.com/finnbear/yew_flight_indicators/actions/workflows/build.yml/badge.svg)](https://github.com/finnbear/yew_flight_indicators/actions/workflows/build.yml) 
[![Test Page](https://img.shields.io/badge/Test-page-green)](https://finnbear.github.io/yew_flight_indicators/)

Like [react-flight-indicators](https://github.com/skyhop/react-flight-indicators) but for Yew.

![example](/example.png)

## Extra Features

- Configurable max air speed, instead of it being fixed to 160 knots.
- SVG's are optimized in advance (see `Makefile`).

## Removed Features

- Showing a box around an indicator is not supported.

## Authors

Originally, this project was based on work by igneosaur, which could be found [on Bitbucket](https://bitbucket.org/igneosaur/attitude-indicator).

Further work was done by [Sébastien Matton](seb_matton@hotmail.com), whom developed the jQuery plugin as part of his master's for showing realtime flight information from a quadcopter.

[Corstian Boerman](https://corstianboerman.com) has adapted the project by Sébastien for use with React.

[Finn Bear](https://finnbear.com) has adapted the project by Corstian for use with Yew.

## License

The project is published under the GPLv3 License. See LICENSE file for more information.