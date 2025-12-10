//! Module containing the Java bindings and comparisons for the KosherJava library.
//! This module is used to test the Java bindings and comparisons for the KosherJava library.
//!
//! This library is tested against the Java library KosherJava.
//!
//! Dates from the years 1870 to 2070 are tested.
//!
//! We allows for 50ms difference between the Java and Rust implementations.
//!
//! For other calculations, such as solar elevation and azimuth, we allow for a .02  difference.
//!
//! There are many instances where the Java library would return Nan or Long.MIN_VALUE to indicate null or invalid values.
//! We use the `Option<T>` type in Rust to handle these cases and return None instead.
//!
//! This library also handles invalid timezones differently than Java. Creating a DateTime in a rust with an invalid timezone will return None, whereas Java will return a GMT timezone. This is acounted for in testing.
//!
//! In all places where java would throw an exception, we return None instead. We never throw under any circumstances
//!
//! There are some timezones which are not supported by Java. These are not tested.
//!
//! Java's datetime library are more flexible in how they deal with DST transitions, while we are very strict. Any computation that can result in an ambiguous time, or a time which is invalid for the given timezone, will return None. Becuase of this we when comparing testing options, we allow the rust one to be None, and the java one to be Some. We limit this to .05% of all iterations to ensure we arent missing any valid bugs in the software
mod java_bindings;
mod java_compare;
mod java_rng;

use chrono::DateTime;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
use java_bindings::*;
use java_compare::*;
use java_rng::*;
use std::env;
use std::sync::Once;

use super::*;
use crate::prelude::*;

/// Default number of iterations for randomized tests.
pub fn get_test_iterations() -> i64 {
    env::var("TEST_ITERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

/// Default Epsilon for floating point comparisons.
pub static DEFAULT_F64_TEST_EPSILON: f64 = 1e-4;

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
#[cfg(test)]
mod java_tests {
    use crate::tests::java::*;

    #[test]
    fn test_geolocation_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran_once = false;
        for _ in 0..get_test_iterations() {
            if let Some((geo_location, java_geo_location)) = create_geolocations(&jvm, &mut rng)
                && let Some((other_geo_location, other_java_geo_location)) = create_geolocations(&jvm, &mut rng)
            {
                let date = random_date_time(&mut rng, &other_java_geo_location.timezone);
                compare_geolocations(
                    &geo_location,
                    &java_geo_location,
                    &other_geo_location,
                    &other_java_geo_location,
                    &date,
                );
                ran_once = true;
            }
        }
        assert!(ran_once, "No test cases were run");
    }
    #[test]
    fn test_astronomical_calculator_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran_once = false;
        let calculator = NOAACalculator;
        let java_calculator = JavaAstronomicalCalculator::new(&jvm);

        for _ in 0..get_test_iterations() {
            if let Some((geo_location, java_geo_location)) = create_geolocations(&jvm, &mut rng) {
                let date = random_date_time(&mut rng, &java_geo_location.timezone);
                let zenith = random_zenith(&mut rng);
                let adjust_for_elevation = rng.gen_bool(0.5);
                compare_astronomical_calculators(
                    &calculator,
                    &java_calculator,
                    &date,
                    &geo_location,
                    &java_geo_location,
                    zenith,
                    adjust_for_elevation,
                );
                ran_once = true;
            }
        }
        assert!(ran_once, "No test cases were run");
    }
    #[test]
    fn test_zmanim_calendar_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..get_test_iterations() {
            let test_case = create_zmanim_calendars(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar) = test_case.unwrap();
            let date_time = calendar.get_date_time();
            let offset_zenith = random_zenith(&mut rng);
            let zenith = random_zenith(&mut rng);

            let (start_of_day, end_of_day) = random_before_after_datetime(&mut rng, date_time);

            let degrees = rng.gen_range(-100.0..=100.0);
            let sunset = rng.gen_bool(0.5);

            let (start_of_half_day, end_of_half_day) = random_before_after_datetime(&mut rng, date_time);
            let (start_of_day_option, end_of_day_option) = random_before_after_datetime(&mut rng, date_time);
            let start_of_day_option = to_random_option(&mut rng, start_of_day_option);
            let end_of_day_option = to_random_option(&mut rng, end_of_day_option);
            let (alos, tzais) = random_before_after_datetime(&mut rng, date_time);
            let alos = to_random_option(&mut rng, alos);
            let tzais = to_random_option(&mut rng, tzais);
            let synchronous = rng.gen_bool(0.5);

            let hours = rng.gen_range(-25.0..=25.0);

            compare_zmanim_calendars(
                &calendar,
                &java_calendar,
                offset_zenith,
                zenith,
                hours,
                &start_of_day,
                &end_of_day,
                degrees,
                sunset,
                &start_of_half_day,
                &end_of_half_day,
                start_of_day_option.as_ref(),
                end_of_day_option.as_ref(),
                synchronous,
                alos.as_ref(),
                tzais.as_ref(),
            );
        }
        assert!(ran, "No test cases were run");
    }
    #[test]
    fn test_tefila_rules_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran = false;
        for _ in 0..get_test_iterations() {
            let test_case = create_jewish_calendars(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (rust_tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm, &mut rng);
            compare_tefila_rules(
                &rust_tefila_rules,
                &java_tefila_rules,
                &rust_calendar,
                &java_calendar,
                &message,
            );
            ran = true;
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_jewish_calendar_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..get_test_iterations() {
            let test_case = create_jewish_calendars(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            compare_jewish_calendars(&rust_calendar, &java_calendar, &message, false);
            ran = true;
        }
        assert!(ran, "No test cases were run");
    }
}
