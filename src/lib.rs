#![cfg_attr(not(feature = "std"), no_std)]

// Ensure exactly one of std or libm is enabled
#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("Features 'std' and 'libm' are mutually exclusive. Please enable only one.");

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either 'std' or 'libm' feature must be enabled.");

pub mod constants;
pub mod daf;
pub mod jewish_calendar;
pub mod parshas;
pub mod tefila_rules;
#[cfg(test)]
pub mod tests;
pub mod time_and_place;
pub mod zmanim_calendar;
pub mod hebrew_holiday_calendar;

pub mod prelude {
    pub use crate::{
        constants::*, daf::*, jewish_calendar::*, parshas::*, tefila_rules::*, time_and_place::*, zmanim_calendar::*,
    };
}
