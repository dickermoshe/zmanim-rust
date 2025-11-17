#[cfg(test)]
pub mod jni {
    use std::sync::Once;

    use chrono::{DateTime, Datelike, Duration, TimeZone};
    use chrono_tz::{TZ_VARIANTS, Tz};
    use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder, errors::J4RsError};
    use rand::Rng;

    use crate::{
        astronomical_calendar::AstronomicalCalendar,
        constants::{JewishDateTrait, JewishMonth},
        geolocation::GeoLocation,
        jewish_calendar::JewishCalendar,
        jewish_date::JewishDate,
        zmanim_calendar::ZmanimCalendar,
    };
    pub static DEFAULT_TEST_ITERATIONS: i32 = 1000;
    pub static DEFAULT_TEST_EPSILON: f64 = 0.2;

    // Guards the one-time JVM creation. Runs exactly once, even across threads.
    static JVM_INIT: Once = Once::new();

    pub fn init_jvm() -> Jvm {
        JVM_INIT.call_once(|| {
            let _ = JvmBuilder::new()
                .classpath_entry(ClasspathEntry::new(
                    "./kosher-java/target/zmanim-2.6.0-SNAPSHOT.jar",
                ))
                .build()
                .unwrap();
        });

        // Attach the current thread to the existing shared JVM (returns a local handle).
        // This works on any thread; JNI allows re-attach on the same thread.
        Jvm::attach_thread().unwrap()
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

    pub fn create_random_geolocations(
        jvm: &Jvm,
        timezone_id: &str,
    ) -> Option<(GeoLocation, Instance, String)> {
        let mut rng = rand::thread_rng();
        let latitude = rng.gen_range(-91.0..=91.0);
        let longitude = rng.gen_range(-181.0..=181.0);
        let elevation = rng.gen_range(-1.0..=1000.0);
        let message = format!(
            "Latitude: {}, Longitude: {}, Elevation: {}",
            latitude, longitude, elevation
        );
        // Being unable to find a timezone that is available in the JVM
        // is not a valid test case, so we return None
        let tz = create_java_timezone(jvm, timezone_id)?;
        let java_geo_location = jvm
            .create_instance(
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
                    InvocationArg::from(tz),
                ],
            )
            .ok();
        let geo_location = GeoLocation::new(latitude, longitude, elevation);
        assert_eq!(
            geo_location.is_some(),
            java_geo_location.is_some(),
            "Failed to create test case for {}",
            message
        );
        if geo_location.is_none() {
            return None;
        }
        let geo_location = geo_location.unwrap();
        let java_geo_location = java_geo_location.unwrap();
        Some((geo_location, java_geo_location, message))
    }

    fn random_date_time() -> (DateTime<chrono_tz::Tz>, Tz, i64, String) {
        let mut rng = rand::thread_rng();
        let tz = random_timezone();
        let years_in_milliseconds = 1000 * 3600 * 24 * 365;
        let milliseconds_since_epoch: i64 = rng.gen_range(
            -years_in_milliseconds..=years_in_milliseconds * 100, // 1870 to 2070
        );
        let dt = tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap();
        let timezone_id = tz.name();
        let message = format!("DateTime: {}, Timezone: {}", dt, timezone_id);
        (dt, tz, milliseconds_since_epoch, message)
    }
    fn random_jewish_year() -> i32 {
        let epoch_year = 5730;
        rand::thread_rng().gen_range((epoch_year - 100)..=(epoch_year + 100)) // 100 years before and after the epoch
    }

    pub fn create_date_times(jvm: &Jvm) -> Option<(DateTime<chrono_tz::Tz>, Instance, Tz, String)> {
        let (date_time, timezone, milliseconds_since_epoch, message) = random_date_time();
        let timezone_id = timezone.name();
        let java_timezone = create_java_timezone(jvm, timezone_id)?;
        let java_calendar = jvm
            .invoke_static("java.util.Calendar", "getInstance", InvocationArg::empty())
            .unwrap();
        jvm.invoke(
            &java_calendar,
            "setTimeZone",
            &[InvocationArg::from(java_timezone)],
        )
        .unwrap();
        jvm.invoke(
            &java_calendar,
            "setTimeInMillis",
            &[InvocationArg::try_from(milliseconds_since_epoch)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();

        Some((date_time, java_calendar, timezone, message))
    }

    pub fn create_date_times_with_geolocation(
        jvm: &Jvm,
    ) -> Option<(
        DateTime<chrono_tz::Tz>,
        Instance,
        GeoLocation,
        Instance,
        String,
    )> {
        let (date_time, java_calendar, timezone, date_time_message) = create_date_times(jvm)?;
        let (geolocation, java_geo_location, geolocation_message) =
            create_random_geolocations(jvm, timezone.name())?;
        let new_message = format!("{}, {}", date_time_message, geolocation_message);
        Some((
            date_time,
            java_calendar,
            geolocation,
            java_geo_location,
            new_message,
        ))
    }
    pub fn create_astronomical_calendars(
        jvm: &Jvm,
    ) -> Option<(AstronomicalCalendar<chrono_tz::Tz>, Instance, String)> {
        let (date_time, java_calendar, geo_location, java_geo_location, message) =
            create_date_times_with_geolocation(jvm)?;
        let astronomical_calendar = AstronomicalCalendar::new(date_time, geo_location);
        let java_astronomical_calendar = jvm
            .create_instance(
                "com.kosherjava.zmanim.AstronomicalCalendar",
                &[InvocationArg::from(java_geo_location)],
            )
            .unwrap();
        jvm.invoke(
            &java_astronomical_calendar,
            "setCalendar",
            &[InvocationArg::from(java_calendar)],
        )
        .unwrap();
        Some((astronomical_calendar, java_astronomical_calendar, message))
    }

    pub fn create_zmanim_calendars(
        jvm: &Jvm,
    ) -> Option<(ZmanimCalendar<chrono_tz::Tz>, Instance, String)> {
        let (date_time, java_calendar, geo_location, java_geo_location, message) =
            create_date_times_with_geolocation(jvm)?;
        let candle_lighting_offset = Duration::minutes(rand::thread_rng().gen_range(0..=60));
        let use_astronomical_chatzos = rand::thread_rng().gen_bool(0.5);
        let use_astronomical_chatzos_for_other_zmanim = rand::thread_rng().gen_bool(0.5);
        let zmanim_calendar = ZmanimCalendar::new(
            AstronomicalCalendar::new(date_time, geo_location),
            candle_lighting_offset,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
        );
        let message = format!(
            "{}, {}, {}, {}, {}",
            message,
            candle_lighting_offset,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset
        );
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
        // TODO: Document that we also factor in elevation for zmanim calculations
        jvm.invoke(
            &instance,
            "setUseElevation",
            &[InvocationArg::try_from(true)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        Some((zmanim_calendar, instance, message))
    }

    pub fn create_jewish_dates(jvm: &Jvm) -> Option<(JewishDate, Instance, String)> {
        let use_gregorian_date = rand::thread_rng().gen_bool(0.5);
        if use_gregorian_date {
            let (date_time, _, _, _) = random_date_time();

            let message = format!(
                "year: {}, month: {}, day: {}",
                date_time.year(),
                date_time.month(),
                date_time.day(),
            );
            let year_arg = InvocationArg::try_from(date_time.year() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let month_arg = InvocationArg::try_from(date_time.month() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let day_arg = InvocationArg::try_from(date_time.day() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let jewish_date_instance = jvm
                .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
                .unwrap();
            let jewish_date = JewishDate::from_gregorian_date(
                date_time.year(),
                date_time.month() as u8,
                date_time.day() as u8,
            );
            let java_jewish_date = jvm
                .create_instance(
                    "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                    &[InvocationArg::from(jewish_date_instance)],
                )
                .ok();

            assert_eq!(
                jewish_date.is_some(),
                java_jewish_date.is_some(),
                "{}",
                message
            );
            if jewish_date.is_none() {
                return None;
            }
            let jewish_date = jewish_date.unwrap();
            let java_jewish_date = java_jewish_date.unwrap();
            return Some((jewish_date, java_jewish_date, message));
        } else {
            let (year, month, day) = random_hebrew_date();
            let message = format!("year: {}, month: {}, day: {}", year, month, day,);
            let jewish_date =
                JewishDate::from_hebrew_date(year, JewishMonth::try_from(month).unwrap(), day);
            let year_arg = InvocationArg::try_from(year as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let month_arg = InvocationArg::try_from(month as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let day_arg = InvocationArg::try_from(day as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let instance = jvm.create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                &[year_arg, month_arg, day_arg],
            );

            let java_jewish_date = match instance {
                Ok(instance) => Some(instance),
                Err(err) => {
                    if let J4RsError::JavaError(message) = &err {
                        // We will ignore the error if it is because the month is not between 1 and 12
                        if message.contains("The Jewish month has to be between 1 and 12") {
                            return None;
                        } else {
                            panic!("{}", err);
                        }
                    }
                    panic!("{}", err);
                }
            };
            // Java will gracefully handle the case of a day of 30 for a month that only has 29 days,
            // and will set it to the last day of the month, whereas Rust will not.
            if jewish_date.is_none() && java_jewish_date.is_some() && day == 30 {
                return None;
            }
            assert_eq!(
                jewish_date.is_some(),
                java_jewish_date.is_some(),
                "{}",
                message
            );
            if jewish_date.is_none() {
                return None;
            }
            let jewish_date = jewish_date.unwrap();
            let java_jewish_date = java_jewish_date.unwrap();
            return Some((jewish_date, java_jewish_date, message));
        }
    }

    pub fn create_jewish_calendars(jvm: &Jvm) -> Option<(JewishCalendar, Instance, String)> {
        let use_gregorian_date = rand::thread_rng().gen_bool(0.5);
        let in_israel = rand::thread_rng().gen_bool(0.5);
        let is_mukaf_choma = rand::thread_rng().gen_bool(0.5);
        let use_modern_holidays = rand::thread_rng().gen_bool(0.5);
        if use_gregorian_date {
            let (date_time, _, _, _) = random_date_time();

            let message = format!(
                "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
                date_time.year(),
                date_time.month(),
                date_time.day(),
                in_israel,
                is_mukaf_choma,
                use_modern_holidays,
            );
            let year_arg = InvocationArg::try_from(date_time.year() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let month_arg = InvocationArg::try_from(date_time.month() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let day_arg = InvocationArg::try_from(date_time.day() as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let jewish_date_instance = jvm
                .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
                .unwrap();
            let jewish_calendar = JewishCalendar::from_gregorian_date(
                date_time.year(),
                date_time.month() as u8,
                date_time.day() as u8,
                in_israel,
                is_mukaf_choma,
                use_modern_holidays,
            );
            let java_jewish_calendar = jvm
                .create_instance(
                    "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
                    &[InvocationArg::from(jewish_date_instance)],
                )
                .ok();

            assert_eq!(
                jewish_calendar.is_some(),
                java_jewish_calendar.is_some(),
                "{}",
                message
            );
            if jewish_calendar.is_none() {
                return None;
            }
            let jewish_calendar = jewish_calendar.unwrap();
            let java_jewish_calendar = java_jewish_calendar.unwrap();
            let in_israel_arg = InvocationArg::try_from(in_israel)
                .unwrap()
                .into_primitive()
                .unwrap();
            let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
                .unwrap()
                .into_primitive()
                .unwrap();
            let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
                .unwrap()
                .into_primitive()
                .unwrap();
            jvm.invoke(&java_jewish_calendar, "setInIsrael", &[in_israel_arg])
                .unwrap();
            jvm.invoke(
                &java_jewish_calendar,
                "setIsMukafChoma",
                &[is_mukaf_choma_arg],
            )
            .unwrap();
            jvm.invoke(
                &java_jewish_calendar,
                "setUseModernHolidays",
                &[use_modern_holidays_arg],
            )
            .unwrap();
            return Some((jewish_calendar, java_jewish_calendar, message));
        } else {
            let (year, month, day) = random_hebrew_date();
            let message = format!(
                "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
                year, month, day, in_israel, is_mukaf_choma, use_modern_holidays,
            );
            let jewish_calendar = JewishCalendar::from_hebrew_date(
                year,
                JewishMonth::try_from(month).unwrap(),
                day,
                in_israel,
                is_mukaf_choma,
                use_modern_holidays,
            );
            let year_arg = InvocationArg::try_from(year as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let month_arg = InvocationArg::try_from(month as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let day_arg = InvocationArg::try_from(day as i32)
                .unwrap()
                .into_primitive()
                .unwrap();
            let instance = jvm.create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
                &[year_arg, month_arg, day_arg],
            );

            let java_jewish_calendar = match instance {
                Ok(instance) => Some(instance),
                Err(err) => {
                    if let J4RsError::JavaError(message) = &err {
                        // We will ignore the error if it is because the month is not between 1 and 12
                        if message.contains("The Jewish month has to be between 1 and 12") {
                            return None;
                        } else {
                            panic!("{}", err);
                        }
                    }
                    panic!("{}", err);
                }
            };
            // Java will gracefully handle the case of a day of 30 for a month that only has 29 days,
            // and will set it to the last day of the month, whereas Rust will not.
            if jewish_calendar.is_none() && java_jewish_calendar.is_some() && day == 30 {
                return None;
            }
            assert_eq!(
                jewish_calendar.is_some(),
                java_jewish_calendar.is_some(),
                "{}",
                message
            );
            if jewish_calendar.is_none() {
                return None;
            }
            let jewish_calendar = jewish_calendar.unwrap();
            let java_jewish_calendar = java_jewish_calendar.unwrap();
            let in_israel_arg = InvocationArg::try_from(in_israel)
                .unwrap()
                .into_primitive()
                .unwrap();
            let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
                .unwrap()
                .into_primitive()
                .unwrap();
            let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
                .unwrap()
                .into_primitive()
                .unwrap();
            jvm.invoke(&java_jewish_calendar, "setInIsrael", &[in_israel_arg])
                .unwrap();
            jvm.invoke(
                &java_jewish_calendar,
                "setIsMukafChoma",
                &[is_mukaf_choma_arg],
            )
            .unwrap();
            jvm.invoke(
                &java_jewish_calendar,
                "setUseModernHolidays",
                &[use_modern_holidays_arg],
            )
            .unwrap();
            return Some((jewish_calendar, java_jewish_calendar, message));
        }
    }

    pub fn create_java_noaa_calculator(jvm: &Jvm) -> Instance {
        jvm.create_instance(
            "com.kosherjava.zmanim.util.NOAACalculator",
            InvocationArg::empty(),
        )
        .unwrap()
    }

    pub fn random_timezone() -> Tz {
        TZ_VARIANTS[rand::thread_rng().gen_range(0..TZ_VARIANTS.len())]
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

    pub fn random_hebrew_date() -> (i32, u8, u8) {
        use crate::constants::JewishMonth;
        let mut rng = rand::thread_rng();
        let year = random_jewish_year();

        // Convert to Rust JewishMonth enum
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
        (year, month as u8, day as u8)
    }
    pub fn random_gregorian_date() -> (i64, i64, i64) {
        let (date_time, _, _, _) = random_date_time();
        (
            date_time.year() as i64,
            date_time.month() as i64,
            date_time.day() as i64,
        )
    }
}
