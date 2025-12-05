#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub mod astronomical_calculator;
pub mod astronomical_calendar;
pub mod constants;
pub mod daf;
pub mod geolocation;
// pub mod jewish_calendar;
mod add_date;
pub mod jewish_date;
pub mod math;
// pub mod parshas;
// pub mod tefila_rules;
#[cfg(test)]
pub mod tests;
// pub mod zmanim_calendar;
