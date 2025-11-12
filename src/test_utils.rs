#[cfg(test)]
pub mod jni {
    use std::sync::Once;

    use chrono::{DateTime, Duration, TimeZone};
    use chrono_tz::TZ_VARIANTS;
    use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
    use rand::Rng;
    pub static DEFAULT_TEST_ITERATIONS: i32 = 10000;
    pub static DEFAULT_TEST_EPSILON: f64 = 0.2;

    // Guards the one-time JVM creation. Runs exactly once, even across threads.
    static JVM_INIT: Once = Once::new();

    pub fn init_jvm() -> Jvm {
        // Ensure the shared JVM exists (creates it on first call).
        JVM_INIT.call_once(|| {
            let _ = JvmBuilder::new()
                .classpath_entry(ClasspathEntry::new("zmanim-2.6.0-SNAPSHOT.jar"))
                .build()
                .unwrap();
        });

        // Attach the current thread to the existing shared JVM (returns a local handle).
        // This works on any thread; JNI allows re-attach on the same thread.
        Jvm::attach_thread().unwrap()
    }

    pub struct RandomGeoLocation {
        pub latitude: f64,
        pub longitude: f64,
        pub elevation: f64,
    }

    impl RandomGeoLocation {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            Self {
                latitude: rng.gen_range(-91.0..=91.0),
                longitude: rng.gen_range(-181.0..=181.0),
                elevation: rng.gen_range(-1.0..=1000.0), // Java GeoLocation requires non-negative elevation
            }
        }
    }

    pub fn random_date_time() -> (DateTime<chrono_tz::Tz>, &'static str) {
        let mut rng = rand::thread_rng();
        let tz = TZ_VARIANTS[rng.gen_range(0..TZ_VARIANTS.len())];
        let years_in_milliseconds = 1000 * 3600 * 24 * 365;
        let milliseconds_since_epoch: i64 = rng.gen_range(
            -years_in_milliseconds..=years_in_milliseconds * 100, // 1870 to 2070
        );
        (
            tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap(),
            tz.name(),
        )
    }
    pub fn create_java_timezone(jvm: &Jvm, timezone_id: &str) -> Option<Instance> {
        let tz = jvm
            .invoke_static(
                "java.util.TimeZone",
                "getTimeZone",
                &[InvocationArg::try_from(timezone_id).unwrap()],
            )
            .unwrap();
        // Get the id and ensure that it created a valid timezone
        let id = jvm.invoke(&tz, "getID", InvocationArg::empty()).unwrap();

        if jvm.to_rust::<String>(id).unwrap() == "GMT" && timezone_id != "UTC" {
            return None;
        }
        Some(tz)
    }

    pub fn create_java_geo_location(
        jvm: &Jvm,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        timezone_id: &str,
    ) -> Option<Instance> {
        let utc = create_java_timezone(jvm, timezone_id)?;
        Some(
            jvm.create_instance(
                "com.kosherjava.zmanim.util.GeoLocation",
                &[
                    InvocationArg::try_from("Name").unwrap(),
                    InvocationArg::try_from(latitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(longitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::from(utc),
                ],
            )
            .ok()?,
        )
    }
    pub fn create_java_calendar(
        jvm: &Jvm,
        milliseconds_since_epoch: i64,
        timezone_id: &str,
    ) -> Option<Instance> {
        let timezone = create_java_timezone(jvm, timezone_id)?;
        let calendar_instance = jvm
            .invoke_static("java.util.Calendar", "getInstance", InvocationArg::empty())
            .unwrap();
        jvm.invoke(
            &calendar_instance,
            "setTimeZone",
            &[InvocationArg::from(timezone)],
        )
        .unwrap();
        jvm.invoke(
            &calendar_instance,
            "setTimeInMillis",
            &[InvocationArg::try_from(milliseconds_since_epoch)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        Some(calendar_instance)
    }
    pub fn create_java_noaa_calculator(jvm: &Jvm) -> Instance {
        jvm.create_instance(
            "com.kosherjava.zmanim.util.NOAACalculator",
            InvocationArg::empty(),
        )
        .unwrap()
    }

    pub fn create_java_zmanim_calendar(
        jvm: &Jvm,
        java_geo_location: Instance,
        java_calendar: Instance,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: Duration,
    ) -> Instance {
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.ZmanimCalendar",
                &[InvocationArg::try_from(java_geo_location).unwrap()],
            )
            .unwrap();
        jvm.invoke(
            &instance,
            "setCalendar",
            &[InvocationArg::try_from(java_calendar).unwrap()],
        )
        .unwrap();
        let noaa_calculator_instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.util.NOAACalculator",
                InvocationArg::empty(),
            )
            .unwrap();
        jvm.invoke(
            &instance,
            "setAstronomicalCalculator",
            &[InvocationArg::try_from(noaa_calculator_instance).unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setUseAstronomicalChatzos",
            &[InvocationArg::try_from(use_astronomical_chatzos)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setUseAstronomicalChatzosForOtherZmanim",
            &[
                InvocationArg::try_from(use_astronomical_chatzos_for_other_zmanim)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            ],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setCandleLightingOffset",
            &[
                InvocationArg::try_from(candle_lighting_offset.as_seconds_f64() / 60.0)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            ],
        )
        .unwrap();

        jvm.invoke(
            &instance,
            "setUseElevation",
            &[InvocationArg::try_from(true)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        instance
    }
    pub fn assert_almost_equal_f64(a: f64, b: f64, diff: f64, message: &str) {
        let result = (a - b).abs() < diff;
        let distance = (a - b).abs();
        assert!(
            result,
            "Error: {:?}, {:?}, distance: {}, {}",
            a, b, distance, message
        );
    }
    pub fn assert_almost_equal_i64(a: i64, b: i64, diff: i64, message: &str) {
        let result = (a - b).abs() < diff;
        let distance = (a - b).abs();
        assert!(
            result,
            "Error: {:?}, {:?}, distance: {}, {}",
            a, b, distance, message
        );
    }

    pub fn assert_almost_equal_f64_option(
        a: &Option<f64>,
        b: &Option<f64>,
        diff: f64,
        message: &str,
    ) {
        match (a, b) {
            (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
            (None, None) => (),
            _ => {
                assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
            }
        }
    }

    pub fn assert_almost_equal_i64_option(
        a: &Option<i64>,
        b: &Option<i64>,
        diff: i64,
        message: &str,
    ) {
        match (a, b) {
            (Some(a), Some(b)) => assert_almost_equal_i64(*a, *b, diff, message),
            (None, None) => (),
            _ => {
                assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
            }
        }
    }
    pub fn assert_almost_equal_datetime(
        a: &DateTime<chrono_tz::Tz>,
        b: &DateTime<chrono_tz::Tz>,
        diff: i64,
        message: &str,
    ) {
        let result = (a.timestamp_millis() - b.timestamp_millis()).abs() < diff;
        let distance = (a.timestamp_millis() - b.timestamp_millis()).abs();
        assert!(
            result,
            "Error: {:?} vs {:?}, distance: {}, {}",
            a, b, distance, message
        );
    }
    pub fn assert_almost_equal_datetime_option(
        a: &Option<DateTime<chrono_tz::Tz>>,
        b: &Option<DateTime<chrono_tz::Tz>>,
        diff: i64,
        message: &str,
    ) {
        match (a, b) {
            (Some(a), Some(b)) => assert_almost_equal_datetime(a, b, diff, message),
            (None, None) => (),
            _ => {
                assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
            }
        }
    }
}
