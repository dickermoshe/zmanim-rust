TODO: If you are reading this it means that we forgot to do something before we published this library
TODO: Get changes merged into kosher jav
TODO: Add no_std tests
TODO: document that users shold create with datetimes that have 0:00 oclock
ensure we can compile no_Std

- Include a changelog
- restrict the submodules to a specific tag
- Update github description

# Jew-SDK

A comprehensive Rust SDK that packages together all the programming utilities useful for developing applications in the Jewish world. This library provides essential tools for working with Jewish calendars, calculating religious times (zmanim), and performing various Jewish-related computations.

## Overview

Jew-SDK aims to be the go-to library for developers building applications that need to work with Jewish time, dates, and religious calculations. The package is built with Rust for performance and reliability, and is designed to be accessible through language bindings for broader adoption across different programming ecosystems.

The library is built upon the foundation of KosherJava, a well-established and trusted library for Jewish calendar and zmanim calculations. We've ported and adapted this functionality to Rust, providing a modern, performant, and `no_std`-compatible implementation that can be used in a wide variety of environments, from embedded systems to web applications.

## Current Status

### Completed Features

- **Jewish Calendar**: Full implementation of the Hebrew calendar system, including date conversions, holiday calculations, and calendar operations
- **Zmanim Calculator**: Comprehensive calculation of Jewish religious times including sunrise, sunset, and various prayer times based on halachic requirements
- **Astronomical Calculations**: Support for astronomical calendar calculations and solar time computations
- **Daf Yomi**: Calculation and tracking of daily Talmud study cycles
- **Parsha Calculations**: Weekly Torah portion (parsha) calculations
- **Tefila Rules**: Rules and calculations related to Jewish prayer times
- **Geolocation Support**: Location-based calculations for accurate zmanim based on geographic coordinates
- **NOAA Calculator**: Integration with NOAA (National Oceanic and Atmospheric Administration) algorithms for precise astronomical calculations

### Planned Features

- **Limudim Calendar**: Calendar system for tracking daily learning schedules and study cycles. We already have support for Daf Yomi and Yerushalmi Yomi, however we need to add support for many more.
- **Gematria Calculator**: Tools for calculating and working with Hebrew gematria (numerical values of Hebrew letters)
- **Conversion Utilities**: Unit conversion tools for traditional Jewish measurements (e.g., Amos to Feet, Tefachim to Inches)
- **Localization Utilities**: Tools for converting between Hebrew and English text, including transliteration and translation helpers
- **Additional Language Bindings**: Extend support to other programming languages to maximize accessibility

## Technical Details

The library is designed with modularity and flexibility in mind. It supports both `std` and `no_std` environments, making it suitable for embedded systems, web assembly, and traditional applications. The core implementation is written in Rust, providing memory safety, performance, and cross-platform compatibility.

The project structure includes comprehensive modules for different aspects of Jewish time and calendar calculations, each carefully implemented to maintain accuracy and follow established halachic principles.

### Zmanim Calculator

We strive to follow KosherJava's APIs as closely as possible to maintain compatibility and familiarity for developers who have used the Java library. However, our documentation is not as comprehensive as KosherJava's. For detailed API documentation and usage examples, please refer to the [KosherJava documentation](https://kosherjava.com/zmanim-project/how-to-use-the-zmanim-api/).

#### API Differences from KosherJava

While we maintain API compatibility where possible, there are some important differences due to Rust's type system and design philosophy:

- **Null Handling**: In places where KosherJava returns `NaN` or `Long.MIN_VALUE` to denote null or invalid values, we use Rust's `Option<T>` type and return `None` instead. This provides type safety and makes null handling explicit in the API.

- **Elevation Handling**: We always factor elevation into calculations if it is provided when creating the calendar. If you want to calculate zmanim without elevation adjustments, provide `0` as the elevation value when creating the calendar instance.

## License

This project is based on KosherJava, which is released under the GNU Lesser General Public License version 2.1 (LGPL 2.1). This license allows the library to be used in both free and proprietary software while ensuring that modifications to the library itself remain open source.

For the full license text, please refer to the LICENSE file in the kosher-java subdirectory.

## Acknowledgments

This project would not be possible without the foundational work of the KosherJava project. We extend our sincere gratitude to:

- **KosherJava**: The original Java implementation by Eliyahu Hershfeld and contributors, which provides the core algorithms and calculations that this library is based upon. KosherJava has been a trusted resource in the Jewish programming community for many years, and we are grateful for their excellent work and the open-source license that makes projects like this possible.

## Roadmap

### Short-term Goals

- Add comprehensive documentation and usage examples
- Expand test coverage to ensure accuracy and reliability
- Create a changelog to track project evolution

### Medium-term Goals

- Implement Limudim Calendar functionality for tracking study schedules
- Develop Gematria Calculator tools
- Create conversion utilities for traditional Jewish measurements
- Build localization utilities for Hebrew-English text handling

### Long-term Vision

- Expand language bindings to support additional programming languages (JavaScript/TypeScript, Go, C/C++, etc.)
- Enhance the library with additional Jewish computational utilities as needs arise
- Build a community around the project to support ongoing development and improvements
- Ensure the library remains accurate and up-to-date with halachic requirements

## Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, improving documentation, or creating language bindings, your help is appreciated. Please ensure that any contributions maintain the accuracy and reliability standards required for halachic calculations.

## Disclaimer

While we have done our best to ensure accurate results using standardized astronomical calculations and established halachic principles, please exercise care when using this library for halacha lemaaseh (practical Jewish law). It is always recommended to verify critical calculations with qualified halachic authorities when necessary.
