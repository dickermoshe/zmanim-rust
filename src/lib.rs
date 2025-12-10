#![cfg_attr(not(feature = "std"), no_std)]

pub mod astronomical_calculator;
pub mod constants;
pub mod daf;
pub mod geolocation;
pub mod jewish_calendar;
pub mod parshas;
pub mod tefila_rules;
#[cfg(test)]
pub mod tests;
pub mod zmanim_calendar;

pub mod prelude {
    pub use crate::{
        astronomical_calculator::*, constants::*, daf::*, geolocation::*, jewish_calendar::*, parshas::*,
        tefila_rules::*, zmanim_calendar::*,
    };
}
