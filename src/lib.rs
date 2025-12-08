#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub mod astronomical_calculator;
pub mod astronomical_calendar;
pub mod constants;
pub mod daf;
pub mod defmt;
pub mod geolocation;
pub mod jewish_calendar;
pub mod jewish_date;
pub mod math;
pub mod parshas;
pub mod tefila_rules;
#[cfg(all(test, not(feature = "defmt")))]
pub mod tests;

pub mod zmanim_calendar;
