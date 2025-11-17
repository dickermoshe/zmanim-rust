#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub mod astronomical_calendar;
pub mod constants;
pub mod daf;
pub mod geolocation;
pub mod jewish_calendar;
pub mod jewish_date;
pub mod math;
pub mod noaa_calculator;
pub mod parshas;
pub mod tefila_rules;
#[cfg(test)]
pub mod test_utils;
pub mod zmanim_calendar;
