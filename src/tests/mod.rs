#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
#[cfg(feature = "std")]
mod java;
use crate::prelude::JewishMonth;
use chrono::{DateTime, Datelike, Duration, TimeZone};
use chrono_tz::Tz;
use rand::Rng;

pub static STATIC_OFFSET_TIMEZONES: &[Tz] = &[
    Tz::Etc__GMT,
    Tz::Etc__GMTPlus0,
    Tz::Etc__GMTPlus1,
    Tz::Etc__GMTPlus10,
    Tz::Etc__GMTPlus11,
    Tz::Etc__GMTPlus12,
    Tz::Etc__GMTPlus2,
    Tz::Etc__GMTPlus3,
    Tz::Etc__GMTPlus4,
    Tz::Etc__GMTPlus5,
    Tz::Etc__GMTPlus6,
    Tz::Etc__GMTPlus7,
    Tz::Etc__GMTPlus8,
    Tz::Etc__GMTPlus9,
    Tz::Etc__GMTMinus0,
    Tz::Etc__GMTMinus1,
    Tz::Etc__GMTMinus10,
    Tz::Etc__GMTMinus11,
    Tz::Etc__GMTMinus12,
    Tz::Etc__GMTMinus13,
    Tz::Etc__GMTMinus14,
    Tz::Etc__GMTMinus2,
    Tz::Etc__GMTMinus3,
    Tz::Etc__GMTMinus4,
    Tz::Etc__GMTMinus5,
    Tz::Etc__GMTMinus6,
    Tz::Etc__GMTMinus7,
    Tz::Etc__GMTMinus8,
    Tz::Etc__GMTMinus9,
];

static DEFAULT_TEST_YEARS: i64 = 100;
static DEFAULT_TEST_YEARS_IN_MILLISECONDS: i64 = 1000 * 3600 * 24 * 365 * DEFAULT_TEST_YEARS;

/// Generates a random DateTime in the range 1870-2070 with the given timezone.
pub fn random_date_time<Tz: TimeZone>(rng: &mut impl Rng, is_static_offset: bool, tz: Tz) -> DateTime<Tz> {
    // Java and Rust handle historical timezones very differently.
    // Therefor, if the timezone is not a static offset, we will only generate dates after 1970.
    let start = if is_static_offset {
        -DEFAULT_TEST_YEARS_IN_MILLISECONDS
    } else {
        0
    };

    let milliseconds_since_epoch: i64 = rng.gen_range(
        start..=DEFAULT_TEST_YEARS_IN_MILLISECONDS, // 1870 to 2070
    );
    tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap()
}

/// Generates a random zenith angle for testing edge cases.
pub fn random_zenith(rng: &mut impl Rng) -> f64 {
    rng.gen_range(-180.0..=180.0)
}
/// Generates a random DateTime before and after the given datetime.
/// This can occasionally return a datetime that is before the given datetime.
pub fn random_before_after_datetime<Tz: TimeZone>(
    rng: &mut impl Rng,
    datetime: &DateTime<Tz>,
) -> (DateTime<Tz>, DateTime<Tz>) {
    let before = datetime.clone() + Duration::seconds(rng.gen_range(-(24 * 3600)..=(12 * 3600))); // Random offset between -24 hours and +12 hours
    let after = datetime.clone() + Duration::seconds(rng.gen_range(-(12 * 3600)..=(24 * 3600))); // Random offset between -12 hours and +24 hours
    (before, after)
}

/// Asserts that two f64 values are approximately equal within the given tolerance.
pub fn assert_almost_equal_f64(a: f64, b: f64, diff: f64, message: &str) {
    let result = (a - b).abs() < diff;
    let distance = (a - b).abs();
    assert!(result, "Error: {:?}, {:?}, distance: {}, {}", a, b, distance, message);
}
/// Asserts that two i64 values are approximately equal within the given tolerance.
pub fn assert_almost_equal_i64(a: i64, b: i64, diff: i64, message: &str) {
    let result = (a - b).abs() < diff;
    let distance = (a - b).abs();
    assert!(result, "Error: {:?}, {:?}, distance: {}, {}", a, b, distance, message);
}

/// Asserts that two optional f64 values are approximately equal.
///
/// Allows a limited number of cases where Rust returns None but Java returns Some.
pub fn assert_almost_equal_f64_option(a: &Option<f64>, b: &Option<f64>, diff: f64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

/// Asserts that two optional i64 values are approximately equal.
///
/// Allows a limited number of cases where Rust returns None but Java returns Some.
pub fn assert_almost_equal_i64_option(a: &Option<i64>, b: &Option<i64>, diff: i64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_i64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
/// Asserts that two DateTime values are approximately equal within a 50 millisecond tolerance.
pub fn assert_almost_equal_datetime<Tz: TimeZone, Tz2: TimeZone>(a: &DateTime<Tz>, b: &DateTime<Tz2>, message: &str) {
    let result = (a.timestamp_millis() - b.timestamp_millis()).abs() < 50;
    let distance = (a.timestamp_millis() - b.timestamp_millis()).abs();
    assert!(result, "Error: {:?} vs {:?}, distance: {}, {}", a, b, distance, message);
}
/// Asserts that two optional DateTime values are approximately equal.
pub fn assert_almost_equal_datetime_option<Tz1: TimeZone, Tz2: TimeZone>(
    a: &Option<DateTime<Tz1>>,
    b: &Option<DateTime<Tz2>>,
    message: &str,
) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_datetime(a, b, message),
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
/// Asserts that two Duration values are approximately equal within a 10 millisecond tolerance.
pub fn assert_almost_equal_duration(a: &Duration, b: &Duration, message: &str) {
    let result = (a.num_milliseconds() - b.num_milliseconds()).abs() < 10;
    let distance = (a.num_milliseconds() - b.num_milliseconds()).abs();
    assert!(result, "Error: {:?} vs {:?}, distance: {}, {}", a, b, distance, message);
}

pub fn assert_almost_equal_duration_option(a: &Option<Duration>, b: &Option<Duration>, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_duration(a, b, message),
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

/// Generates a random Hebrew date in the range 1870-2070.
pub fn random_hebrew_date(rng: &mut impl Rng) -> (i32, JewishMonth, u8) {
    let dt = random_date_time(rng, true, chrono_tz::Tz::UTC);
    let year = dt.year() + 3760; // 3760 is the difference between the Gregorian and Hebrew years

    let month = match rng.gen_range(1..=13) {
        1 => JewishMonth::Nissan,
        2 => JewishMonth::Iyar,
        3 => JewishMonth::Sivan,
        4 => JewishMonth::Tammuz,
        5 => JewishMonth::Av,
        6 => JewishMonth::Elul,
        7 => JewishMonth::Tishrei,
        8 => JewishMonth::Cheshvan,
        9 => JewishMonth::Kislev,
        10 => JewishMonth::Teves,
        11 => JewishMonth::Shevat,
        12 => JewishMonth::Adar,
        13 => JewishMonth::AdarII,
        _ => unreachable!(),
    };
    let day = rng.gen_range(1..=30);
    (year, month, day as u8)
}
/// Helper function for randomly returning a Some or None.
/// This is used to test the edge cases of the software.
pub fn to_random_option<T, R: rand::Rng>(rng: &mut R, d: T) -> Option<T> {
    if rng.gen_bool(0.5) { Some(d) } else { None }
}
