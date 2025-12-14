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
            .classpath_entry(ClasspathEntry::new("./kosher-java/target/dependency/icu4j-78.1.jar"))
            .build()
            .unwrap();
    });

    // Attach the current thread to the existing shared JVM (returns a local handle).
    // This works on any thread; JNI allows re-attach on the same thread.
    Jvm::attach_thread().unwrap()
}
/// Create a random timezone that is compatible with Java's Timezone class
pub fn random_timezone(rng: &mut impl Rng) -> Tz {
    let tz = TZ_VARIANTS[rng.gen_range(0..TZ_VARIANTS.len())];
    // DIFF: Java cannot handle the some timezones
    // Tehran at the time of the revolution had a unclear timezone which Java and Rust handle differently
    if tz.name() == "ROC"
        || tz.name() == "America/Coyhaique"
        || tz.name() == "GMT"
        || tz.name() == "Asia/Tehran"
        || tz.name() == "Iran"
    {
        return random_timezone(rng);
    }
    tz
}

/// Converts a Rust timezone to a Java TimeZone instance.
///
/// Returns None if Java cannot find a matching timezone (falls back to GMT).
pub fn tz_to_java_timezone(jvm: &Jvm, timezone_id: &str) -> Instance {
    jvm.invoke_static(
        "com.ibm.icu.util.TimeZone",
        "getTimeZone",
        &[InvocationArg::try_from(timezone_id).unwrap()],
    )
    .unwrap()
}
/// Converts a Rust DateTime to a Java Calendar instance.
///
/// Returns None if the timezone cannot be converted to Java.
pub fn dt_to_java_calendar<Tz: TimeZone>(jvm: &Jvm, date: &DateTime<Tz>, timezone_id: &str) -> Option<Instance> {
    let java_timezone = tz_to_java_timezone(jvm, timezone_id);
    let java_calendar = jvm
        .invoke_static("com.ibm.icu.util.Calendar", "getInstance", InvocationArg::empty())
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
pub fn geolocation_to_java_geolocation(
    jvm: &Jvm,
    geolocation: &impl GeoLocationTrait,
    timezone_id: &str,
) -> Option<Instance> {
    JavaGeoLocation::new(
        jvm,
        geolocation.get_latitude(),
        geolocation.get_longitude(),
        geolocation.get_elevation(),
        timezone_id,
    )
    .map(|java_geo_location| java_geo_location.instance)
}
#[cfg(test)]
mod java_tests {
    use chrono::Timelike;

    use crate::tests::java::*;

    #[test]
    fn test_geolocation_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran_once = false;
        for _ in 0..get_test_iterations() {
            let tz = random_timezone(&mut rng);
            let timezone_id = tz.name();
            if let Some((geo_location, java_geo_location)) = create_geolocations(&jvm, &mut rng, timezone_id)
                && let Some((other_geo_location, other_java_geo_location)) =
                    create_geolocations(&jvm, &mut rng, timezone_id)
            {
                let date = random_date_time(&mut rng, STATIC_OFFSET_TIMEZONES.contains(&tz), tz);
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

        for _ in 0..get_test_iterations() {
            let tz = random_timezone(&mut rng);
            let timezone_id = tz.name();
            let java_calculator = JavaAstronomicalCalculator::new(&jvm, timezone_id);

            if let Some((geo_location, java_geo_location)) = create_geolocations(&jvm, &mut rng, timezone_id) {
                let date = random_date_time(&mut rng, STATIC_OFFSET_TIMEZONES.contains(&tz), tz);
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
    fn test_zmanim_calendar_tz_aware_against_java() {
        use std::panic;

        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        let mut iteration = 0;
        for _ in 0..get_test_iterations() {
            let tz = random_timezone(&mut rng);
            let timezone_id = tz.name();
            let test_case = create_zmanim_calendars(&jvm, &mut rng, tz, timezone_id);
            if test_case.is_none() {
                continue;
            }

            ran = true;
            iteration += 1;
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

            let hours = rng.gen_range(-1.0..=25.0);

            // Capture all test parameters for reproduction
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
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
            }));

            if result.is_err() {
                eprintln!(
                    "\n========== TEST FAILURE REPRODUCTION INFO (Iteration {}) ==========",
                    iteration
                );
                eprintln!("Calendar configuration:");
                eprintln!("  date_time: {:?}", calendar.date_time);
                eprintln!("  timezone: {}", timezone_id);
                eprintln!("  geo_location.latitude: {}", calendar.geo_location.get_latitude());
                eprintln!("  geo_location.longitude: {}", calendar.geo_location.get_longitude());
                eprintln!("  geo_location.elevation: {}", calendar.geo_location.get_elevation());
                eprintln!("  use_astronomical_chatzos: {}", calendar.use_astronomical_chatzos);
                eprintln!(
                    "  use_astronomical_chatzos_for_other_zmanim: {}",
                    calendar.use_astronomical_chatzos_for_other_zmanim
                );
                eprintln!("  candle_lighting_offset: {:?}", calendar.candle_lighting_offset);
                eprintln!(
                    "  ateret_torah_sunset_offset: {:?}",
                    calendar.ateret_torah_sunset_offset
                );
                eprintln!("\nTest parameters:");
                eprintln!("  offset_zenith: {}", offset_zenith);
                eprintln!("  zenith: {}", zenith);
                eprintln!("  hours: {}", hours);
                eprintln!("  start_of_day: {:?}", start_of_day);
                eprintln!("  end_of_day: {:?}", end_of_day);
                eprintln!("  degrees: {}", degrees);
                eprintln!("  sunset: {}", sunset);
                eprintln!("  start_of_half_day: {:?}", start_of_half_day);
                eprintln!("  end_of_half_day: {:?}", end_of_half_day);
                eprintln!("  start_of_day_option: {:?}", start_of_day_option);
                eprintln!("  end_of_day_option: {:?}", end_of_day_option);
                eprintln!("  synchronous: {}", synchronous);
                eprintln!("  alos: {:?}", alos);
                eprintln!("  tzais: {:?}", tzais);

                eprintln!("\n========== COPY-PASTE REPRODUCTION TEST CODE ==========");
                eprintln!("#[test]");
                eprintln!("fn test_failure_reproduction_iteration_{}() {{", iteration);
                eprintln!("    use chrono::NaiveDate;");
                eprintln!("    let jvm = init_jvm();");
                eprintln!(
                    "    let tz = Tz::{};",
                    timezone_id
                        .replace('/', "__")
                        .replace("+", "Plus")
                        .replace("-", "Minus")
                );
                eprintln!("    let timezone_id = \"{}\";", timezone_id);
                eprintln!(
                    "    let geo_location = GeoLocation::new({}, {}, {}).unwrap();",
                    calendar.geo_location.get_latitude(),
                    calendar.geo_location.get_longitude(),
                    calendar.geo_location.get_elevation()
                );
                eprintln!(
                    "    let date = NaiveDate::from_ymd_opt({}, {}, {}).unwrap();",
                    calendar.date_time.year(),
                    calendar.date_time.month(),
                    calendar.date_time.day()
                );
                eprintln!("    let date_time = tz.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap()).unwrap();");
                eprintln!(
                    "    let calendar = ZmanimCalendar::new(date, tz, geo_location.clone(), NOAACalculator, {}, {}, Duration::seconds({}), Duration::seconds({})).unwrap();",
                    calendar.use_astronomical_chatzos,
                    calendar.use_astronomical_chatzos_for_other_zmanim,
                    calendar.candle_lighting_offset.num_seconds(),
                    calendar.ateret_torah_sunset_offset.num_seconds()
                );
                eprintln!(
                    "    let java_calendar = JavaZmanimCalendar::new(&jvm, date_time, timezone_id, geo_location, Duration::seconds({}), {}, {}, Duration::seconds({}));",
                    calendar.candle_lighting_offset.num_seconds(),
                    calendar.use_astronomical_chatzos,
                    calendar.use_astronomical_chatzos_for_other_zmanim,
                    calendar.ateret_torah_sunset_offset.num_seconds()
                );

                // Generate code for all the test parameters
                eprintln!("    // Test parameters");
                if let Some(ref a) = alos {
                    eprintln!(
                        "    let alos = Some(tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap());",
                        a.year(),
                        a.month(),
                        a.day(),
                        a.hour(),
                        a.minute(),
                        a.second()
                    );
                } else {
                    eprintln!("    let alos: Option<DateTime<Tz>> = None;");
                }
                if let Some(ref t) = tzais {
                    eprintln!(
                        "    let tzais = Some(tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap());",
                        t.year(),
                        t.month(),
                        t.day(),
                        t.hour(),
                        t.minute(),
                        t.second()
                    );
                } else {
                    eprintln!("    let tzais: Option<DateTime<Tz>> = None;");
                }

                eprintln!("    // Add the specific comparison that failed here");
                eprintln!(
                    "    compare_zmanim_calendars(&calendar, &java_calendar, {}, {}, {}, &tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap(), &tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap(), {}, {}, &tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap(), &tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap(), {:?}, {:?}, {}, alos.as_ref(), tzais.as_ref());",
                    offset_zenith,
                    zenith,
                    hours,
                    start_of_day.year(),
                    start_of_day.month(),
                    start_of_day.day(),
                    start_of_day.hour(),
                    start_of_day.minute(),
                    start_of_day.second(),
                    end_of_day.year(),
                    end_of_day.month(),
                    end_of_day.day(),
                    end_of_day.hour(),
                    end_of_day.minute(),
                    end_of_day.second(),
                    degrees,
                    sunset,
                    start_of_half_day.year(),
                    start_of_half_day.month(),
                    start_of_half_day.day(),
                    start_of_half_day.hour(),
                    start_of_half_day.minute(),
                    start_of_half_day.second(),
                    end_of_half_day.year(),
                    end_of_half_day.month(),
                    end_of_half_day.day(),
                    end_of_half_day.hour(),
                    end_of_half_day.minute(),
                    end_of_half_day.second(),
                    start_of_day_option.map(|dt| format!(
                        "Some(tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap())",
                        dt.year(),
                        dt.month(),
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second()
                    )),
                    end_of_day_option.map(|dt| format!(
                        "Some(tz.with_ymd_and_hms({}, {}, {}, {}, {}, {}).unwrap())",
                        dt.year(),
                        dt.month(),
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second()
                    )),
                    synchronous
                );
                eprintln!("}}");
                eprintln!("================================================================\n");

                // Re-panic to fail the test
                panic::resume_unwind(result.unwrap_err());
            }
        }
        assert!(ran, "No test cases were run");
    }
    #[test]
    fn test_zmanim_calendar_naive_against_java() {
        use std::panic;

        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        let mut iteration = 0;
        for _ in 0..get_test_iterations() {
            let test_case = create_zmanim_calendars_naive(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }

            ran = true;
            iteration += 1;
            let (calendar_naive, java_calendar_naive) = test_case.unwrap();
            let date_time = calendar_naive.get_date_time();
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

            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                compare_zmanim_calendars(
                    &calendar_naive,
                    &java_calendar_naive,
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
            }));

            if result.is_err() {
                eprintln!(
                    "\n========== TEST FAILURE REPRODUCTION INFO (NAIVE, Iteration {}) ==========",
                    iteration
                );
                eprintln!("Calendar configuration:");
                eprintln!("  date_time: {:?}", calendar_naive.date_time);
                eprintln!("  timezone: UTC (naive)");
                eprintln!(
                    "  geo_location.latitude: {}",
                    calendar_naive.geo_location.get_latitude()
                );
                eprintln!(
                    "  geo_location.longitude: {}",
                    calendar_naive.geo_location.get_longitude()
                );
                eprintln!(
                    "  geo_location.elevation: {}",
                    calendar_naive.geo_location.get_elevation()
                );
                eprintln!(
                    "  use_astronomical_chatzos: {}",
                    calendar_naive.use_astronomical_chatzos
                );
                eprintln!(
                    "  use_astronomical_chatzos_for_other_zmanim: {}",
                    calendar_naive.use_astronomical_chatzos_for_other_zmanim
                );
                eprintln!("  candle_lighting_offset: {:?}", calendar_naive.candle_lighting_offset);
                eprintln!(
                    "  ateret_torah_sunset_offset: {:?}",
                    calendar_naive.ateret_torah_sunset_offset
                );
                eprintln!("\nTest parameters:");
                eprintln!("  offset_zenith: {}", offset_zenith);
                eprintln!("  zenith: {}", zenith);
                eprintln!("  hours: {}", hours);
                eprintln!("  start_of_day: {:?}", start_of_day);
                eprintln!("  end_of_day: {:?}", end_of_day);
                eprintln!("  degrees: {}", degrees);
                eprintln!("  sunset: {}", sunset);
                eprintln!("  start_of_half_day: {:?}", start_of_half_day);
                eprintln!("  end_of_half_day: {:?}", end_of_half_day);
                eprintln!("  start_of_day_option: {:?}", start_of_day_option);
                eprintln!("  end_of_day_option: {:?}", end_of_day_option);
                eprintln!("  synchronous: {}", synchronous);
                eprintln!("  alos: {:?}", alos);
                eprintln!("  tzais: {:?}", tzais);
                eprintln!("================================================================\n");

                // Re-panic to fail the test
                panic::resume_unwind(result.unwrap_err());
            }
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
