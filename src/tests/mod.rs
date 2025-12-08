#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
mod astronomical_calculator_test;
mod astronomical_calendar_test;
mod geolocation_test;
mod jewish_calendar_test;
mod jewish_date_test;
mod tefila_rules_test;
mod zmanim_calendar_test;

use std::sync::{Mutex, Once};

use chrono::{DateTime, Datelike, Duration, TimeZone};
use chrono_tz::Tz;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
use rand::{Rng, distributions::WeightedIndex, prelude::Distribution};

use crate::{constants::JewishMonth, geolocation::GeoLocationTrait, tests::geolocation_test::JavaGeoLocation};

/// Default number of iterations for randomized tests.
pub static DEFAULT_TEST_ITERATIONS: i32 = 10000;

/// Default epsilon for floating-point comparisons in tests.
pub static DEFAULT_F64_TEST_EPSILON: f64 = 0.02;

/// Tracks cases where Rust returns None but Java returns Some.
/// Allows a small number of such cases (0.05% of test iterations) to accommodate edge cases
/// while still catching legitimate bugs where None is returned incorrectly.
static RUST_NONE_COUNT: Mutex<u64> = Mutex::new(0);
static MAX_RUST_NONE_COUNT: u64 = (DEFAULT_TEST_ITERATIONS as f64 * 0.0005) as u64;

/// Increments the Rust None counter and panics if the threshold is exceeded.
fn add_rust_none_count() {
    let mut rust_none_count = RUST_NONE_COUNT.lock().unwrap();
    if *rust_none_count > MAX_RUST_NONE_COUNT {
        panic!(
            "Error: Rust None count ({}) exceeded max allowed ({})",
            *rust_none_count, MAX_RUST_NONE_COUNT
        );
    }
    *rust_none_count += 1;
}
/// Guards the one-time JVM creation. Runs exactly once, even across threads.
static JVM_INIT: Once = Once::new();

/// Initializes or attaches to the shared JVM instance for testing against KosherJava.
///
/// The JVM is created once on first call, then subsequent calls attach the current thread.
/// This allows multi-threaded tests to share a single JVM instance.
pub fn init_jvm() -> Jvm {
    JVM_INIT.call_once(|| {
        let _ = JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new("./kosher-java/target/zmanim-2.6.0-SNAPSHOT.jar"))
            .build()
            .unwrap();
    });

    // Attach the current thread to the existing shared JVM (returns a local handle).
    // This works on any thread; JNI allows re-attach on the same thread.
    Jvm::attach_thread().unwrap()
}

/// Generates a random DateTime in the range 1870-2070 with the given timezone.
pub fn random_date_time(rng: &mut impl Rng, tz: Tz) -> DateTime<chrono_tz::Tz> {
    let years_in_milliseconds = 1000 * 3600 * 24 * 365;
    let milliseconds_since_epoch: i64 = rng.gen_range(
        -years_in_milliseconds..=years_in_milliseconds * 100, // 1870 to 2070
    );
    tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap()
}

/// Converts a generic TimeZone to chrono_tz::Tz.
///
/// # Safety
/// This function uses unsafe transmute and is only safe when `Tz` is actually `chrono_tz::Tz`.
/// This function should only be called in test contexts where we know the timezone type.
///
/// # Panics
/// This function may cause undefined behavior if `Tz` is not `chrono_tz::Tz`.
pub fn tz_to_chrono_tz<Tz: chrono::TimeZone>(timezone: Tz) -> chrono_tz::Tz {
    // SAFETY: This is only safe when Tz is chrono_tz::Tz. In test contexts, we ensure
    // that all timezones are chrono_tz::Tz before calling this function.
    unsafe { std::mem::transmute_copy::<Tz, chrono_tz::Tz>(&timezone) }
}

/// Generates a random zenith angle for testing edge cases.
///
/// Most commonly returns a valid zenith in [-180, 180], but occasionally returns
/// out-of-range values, infinity, or NaN to test error handling.
pub fn random_zenith(rng: &mut impl Rng) -> f64 {
    let random_value = rng_double_type(rng);
    match random_value {
        DoubleType::Finite => rng.gen_range(-180.0..=180.0),
        DoubleType::OutOfRange => {
            if rng.gen_bool(0.5) {
                -181.0
            } else {
                181.0
            }
        }
        DoubleType::Infinite => f64::INFINITY,
        DoubleType::Nan => f64::NAN,
    }
}

/// Converts a Rust timezone to a Java TimeZone instance.
///
/// Returns None if Java cannot find a matching timezone (falls back to GMT).
pub fn tz_to_java_timezone<Tz: chrono::TimeZone>(jvm: &Jvm, timezone: Tz) -> Option<Instance> {
    // unsafe cast to chrono_tz::Tz - assumes Tz is chrono_tz::Tz
    let timezone = tz_to_chrono_tz(timezone);
    let timezone_id = timezone.name();
    let tz = jvm
        .invoke_static(
            "java.util.TimeZone",
            "getTimeZone",
            &[InvocationArg::try_from(timezone_id).unwrap()],
        )
        .unwrap();
    // Get the id and ensure that it created a valid timezone
    let id = jvm.invoke(&tz, "getID", InvocationArg::empty()).unwrap();

    // If java is unable to find a timezone it will return GMT
    // We should return None if this is the case
    if jvm.to_rust::<String>(id).unwrap() == "GMT" && timezone_id != "UTC" {
        return None;
    }
    Some(tz)
}
/// Converts a Rust DateTime to a Java Calendar instance.
///
/// Returns None if the timezone cannot be converted to Java.
pub fn dt_to_java_calendar<Tz: chrono::TimeZone>(jvm: &Jvm, date: &DateTime<Tz>) -> Option<Instance> {
    let timezone = date.timezone();
    let java_timezone = tz_to_java_timezone(jvm, timezone)?;
    let java_calendar = jvm
        .invoke_static("java.util.Calendar", "getInstance", InvocationArg::empty())
        .unwrap();
    jvm.invoke(&java_calendar, "setTimeZone", &[InvocationArg::from(java_timezone)])
        .unwrap();
    jvm.invoke(
        &java_calendar,
        "setTimeInMillis",
        &[InvocationArg::try_from(date.timestamp_millis())
            .unwrap()
            .into_primitive()
            .unwrap()],
    )
    .unwrap();

    Some(java_calendar)
}

/// Converts a Rust DateTime to a Java Date instance.
pub fn dt_to_java_date<Tz: chrono::TimeZone>(jvm: &Jvm, date: &DateTime<Tz>) -> Instance {
    jvm.create_instance(
        "java.util.Date",
        &[InvocationArg::try_from(date.timestamp_millis())
            .unwrap()
            .into_primitive()
            .unwrap()],
    )
    .unwrap()
}

/// Converts a Rust GeoLocation to a Java GeoLocation instance.
///
/// Returns None if the timezone cannot be converted to Java.
pub fn geolocation_to_java_geolocation<Tz: chrono::TimeZone>(
    jvm: &Jvm,
    geolocation: &impl GeoLocationTrait,
    timezone: Tz,
) -> Option<Instance> {
    JavaGeoLocation::new(
        jvm,
        geolocation.get_latitude(),
        geolocation.get_longitude(),
        geolocation.get_elevation(),
        tz_to_chrono_tz(timezone),
    )
    .map(|java_geo_location| java_geo_location.instance)
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
pub fn assert_almost_equal_datetime<Tz: TimeZone>(a: &DateTime<Tz>, b: &DateTime<Tz>, message: &str) {
    let result = (a.timestamp_millis() - b.timestamp_millis()).abs() < 50;
    let distance = (a.timestamp_millis() - b.timestamp_millis()).abs();
    assert!(result, "Error: {:?} vs {:?}, distance: {}, {}", a, b, distance, message);
}
pub fn assert_almost_equal_datetime_option(
    a: &Option<DateTime<chrono_tz::Tz>>,
    b: &Option<DateTime<chrono_tz::Tz>>,
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
/// Asserts that two Duration values are approximately equal within the given millisecond tolerance.
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

/// An enum to represent the types of a double value.
#[derive(Clone, Copy)]
pub enum DoubleType {
    Infinite,
    OutOfRange,
    Nan,
    Finite,
}
/// Generates a random double type.
/// This is used in testing code to test methods which have double parameters
/// with different types of values.
pub fn rng_double_type(rng: &mut impl Rng) -> DoubleType {
    let random_values = [
        DoubleType::Finite,
        DoubleType::Infinite,
        DoubleType::OutOfRange,
        DoubleType::Nan,
    ];
    random_values[WeightedIndex::new([999, 1, 1, 1]).unwrap().sample(rng)]
}

/// Generates a random Hebrew date.
///
/// # Examples
///
/// ```
/// use kosher_java::tests::random_hebrew_date;
/// use kosher_java::constants::JewishMonth;
/// let mut rng = rand::thread_rng();
/// let (year, month, day) = random_hebrew_date(&mut rng);
/// ```
pub fn random_hebrew_date(rng: &mut impl Rng) -> (i32, JewishMonth, u8) {
    let dt = random_date_time(rng, chrono_tz::Tz::UTC);
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
