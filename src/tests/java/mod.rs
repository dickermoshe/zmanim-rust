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

use chrono::{DateTime, NaiveDate};
use chrono_tz::TZ_VARIANTS;
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

// /// Default Epsilon for floating point comparisons.
// pub static DEFAULT_F64_TEST_EPSILON: f64 = 1e-4;

static JVM_INIT: Once = Once::new();
/// Initializes or attaches to the shared JVM instance for testing against KosherJava.
///
/// The JVM is created once on first call, then subsequent calls attach the current thread.
/// This allows multi-threaded tests to share a single JVM instance.
pub fn init_jvm() -> Jvm {
    JVM_INIT.call_once(|| {
        let _ = JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new("./kosher-java/target/zmanim-2.6.0-SNAPSHOT.jar"))
            .classpath_entry(ClasspathEntry::new("./kosher-java/target/dependency/icu4j-78.1.jar"))
            .build()
            .unwrap();
    });

    // Attach the current thread to the existing shared JVM (returns a local handle).
    // This works on any thread; JNI allows re-attach on the same thread.
    Jvm::attach_thread().unwrap()
}
// /// Create a random timezone that is compatible with Java's Timezone class
// pub fn random_timezone(rng: &mut impl Rng) -> Tz {
//     let tz = TZ_VARIANTS[rng.gen_range(0..TZ_VARIANTS.len())];
//     // DIFF: Java cannot handle the some timezones
//     // Tehran at the time of the revolution had a unclear timezone which Java and Rust handle differently
//     if tz.name() == "ROC"
//         || tz.name() == "America/Coyhaique"
//         || tz.name() == "GMT"
//         || tz.name() == "Asia/Tehran"
//         || tz.name() == "Iran"
//     {
//         return random_timezone(rng);
//     }
//     tz
// }

#[cfg(test)]
mod java_tests {

    use time::Time;

    use crate::tests::java::*;

    #[test]
    fn test_zmanim_calendar_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for i in 0..get_test_iterations() {
            println!("Iteration {}", i);
            let test_case = random_zmanim_calendars(&jvm, &mut rng, None);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar) = test_case.unwrap();
            ran = true;
            let date_time = rust_calendar.get_date_time();
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

            let hours = rng.gen_range(-1.0..=25.0);
            // Capture all test parameters for reproduction
            compare_zmanim_calendars(
                &rust_calendar,
                java_calendar,
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
    fn test_sunrise_before_day_start() {
        // This date has it's sunrise before the day starts. We need to ensure that we are not getting the sunrise for the next day.
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();

        for day in [1, 2, 3, 4] {
            let time_and_place = TimeAndPlace::new(
                49.60139790853522,
                171.01752655220554,
                0.0,
                NaiveDate::from_ymd_opt(1986, 6, day).unwrap(),
                Tz::Etc__GMTMinus11,
            )
            .unwrap();

            let rust_calendar = ZmanimCalendar::new(
                time_and_place.clone(),
                false,
                false,
                Duration::minutes(60),
                Duration::minutes(60),
            )
            .unwrap();
            let java_time_and_place = JavaTimeAndPlace::new(&jvm, &time_and_place).unwrap();
            let java_calendar = JavaZmanimCalendar::new(
                &jvm,
                java_time_and_place,
                Duration::minutes(60),
                false,
                false,
                Duration::minutes(60),
            )
            .unwrap();
        let hours = rng.gen_range(-1.0..=25.0);

        let date_time = rust_calendar.get_date_time();
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
        compare_zmanim_calendars(&rust_calendar,
            java_calendar,
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
            tzais.as_ref(),);
    

        }
    }
    #[test]
    fn test_known_location() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();

        // for day in [1, 2, 3, 4,25] {
            let time_and_place = TimeAndPlace::new(
                -10.08085961488009,
                 58.94180768818123,
                0.0,
                NaiveDate::from_ymd_opt(2043, 9, 3).unwrap(),
                Tz::Etc__GMTMinus4,
            )
            .unwrap();

            let rust_calendar = ZmanimCalendar::new(
                time_and_place.clone(),
                false,
                false,
                Duration::minutes(60),
                Duration::minutes(60),
            )
            .unwrap();
            let java_time_and_place = JavaTimeAndPlace::new(&jvm, &time_and_place).unwrap();
            let java_calendar = JavaZmanimCalendar::new(
                &jvm,
                java_time_and_place,
                Duration::minutes(60),
                false,
                false,
                Duration::minutes(60),
            )
            .unwrap();
        let hours = rng.gen_range(-1.0..=25.0);

        let date_time = rust_calendar.get_date_time();
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
        compare_zmanim_calendars(&rust_calendar,
            java_calendar,
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
            tzais.as_ref(),);
    

     
    }
}
// #[test]
// fn test_tefila_rules_against_java() {
//     let jvm = init_jvm();
//     let mut rng = rand::thread_rng();
//     let mut ran = false;
//     for _ in 0..get_test_iterations() {
//         let test_case = create_jewish_calendars(&jvm, &mut rng);
//         if test_case.is_none() {
//             continue;
//         }
//         let (rust_calendar, java_calendar, message) = test_case.unwrap();
//         let (rust_tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm, &mut rng);
//         compare_tefila_rules(
//             &rust_tefila_rules,
//             &java_tefila_rules,
//             &rust_calendar,
//             &java_calendar,
//             &message,
//         );
//         ran = true;
//     }
//     assert!(ran, "No test cases were run");
// }

// #[test]
// fn test_jewish_calendar_against_java() {
//     let jvm = init_jvm();
//     let mut ran = false;
//     let mut rng = rand::thread_rng();
//     for _ in 0..get_test_iterations() {
//         let test_case = create_jewish_calendars(&jvm, &mut rng);
//         if test_case.is_none() {
//             continue;
//         }
//         let (rust_calendar, java_calendar, message) = test_case.unwrap();
//         compare_jewish_calendars(&rust_calendar, &java_calendar, &message, false);
//         ran = true;
//     }
//     assert!(ran, "No test cases were run");
// }
