use std::sync::{Mutex, Once};

use chrono::{DateTime, Datelike, Duration, TimeZone};
use chrono_tz::Tz;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
use rand::{Rng, distributions::WeightedIndex, prelude::Distribution};

use crate::{constants::JewishMonth, geolocation::GeoLocationTrait, tests::geolocation_test::JavaGeoLocation};

pub static DEFAULT_TEST_ITERATIONS: i32 = 10000;
pub static DEFAULT_F64_TEST_EPSILON: f64 = 0.02;
// We allow Rust to return None when java returns Some.
// We only allow this very few times, so that any legitimate None cases are not masked by this.
static RUST_NONE_COUNT: Mutex<u64> = Mutex::new(0);
static MAX_RUST_NONE_COUNT: u64 = (DEFAULT_TEST_ITERATIONS as f64 * 0.0005) as u64;
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
// Guards the one-time JVM creation. Runs exactly once, even across threads.
static JVM_INIT: Once = Once::new();

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

pub fn random_zenith(rng: &mut impl Rng) -> f64 {
    let random_value = random_random_value(rng);
    match random_value {
        RandomValue::Normal => rng.gen_range(-180.0..=180.0),
        RandomValue::OutOfRange => {
            if rng.gen_bool(0.5) {
                -181.0
            } else {
                181.0
            }
        }
        RandomValue::Infinite => f64::INFINITY,
        RandomValue::Nan => f64::NAN,
    }
}

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

// pub fn create_java_timezone(jvm: &Jvm, timezone_id: &str) -> Option<Instance> {
//     let tz = jvm
//         .invoke_static(
//             "java.util.TimeZone",
//             "getTimeZone",
//             &[InvocationArg::try_from(timezone_id).unwrap()],
//         )
//         .unwrap();
//     // Get the id and ensure that it created a valid timezone
//     let id = jvm.invoke(&tz, "getID", InvocationArg::empty()).unwrap();

//     if jvm.to_rust::<String>(id).unwrap() == "GMT" && timezone_id != "UTC" {
//         return None;
//     }
//     Some(tz)
// }

// pub fn create_random_geolocations(
//     jvm: &Jvm,
//     timezone_id: &str,
// ) -> Option<(GeoLocation, Instance, String)> {
//     let mut rng = rand::thread_rng();
//     let latitude = rng.gen_range(-91.0..=91.0);
//     let longitude = rng.gen_range(-181.0..=181.0);
//     let elevation = rng.gen_range(-1.0..=1000.0);
//     let message = format!(
//         "Latitude: {}, Longitude: {}, Elevation: {}",
//         latitude, longitude, elevation
//     );
//     // Being unable to find a timezone that is available in the JVM
//     // is not a valid test case, so we return None
//     let tz = create_java_timezone(jvm, timezone_id)?;
//     let java_geo_location = jvm
//         .create_instance(
//             "com.kosherjava.zmanim.util.GeoLocation",
//             &[
//                 InvocationArg::try_from("Name").unwrap(),
//                 InvocationArg::try_from(latitude)
//                     .unwrap()
//                     .into_primitive()
//                     .unwrap(),
//                 InvocationArg::try_from(longitude)
//                     .unwrap()
//                     .into_primitive()
//                     .unwrap(),
//                 InvocationArg::try_from(elevation)
//                     .unwrap()
//                     .into_primitive()
//                     .unwrap(),
//                 InvocationArg::from(tz),
//             ],
//         )
//         .ok();
//     let geo_location = GeoLocation::new(latitude, longitude, elevation);
//     assert_eq!(
//         geo_location.is_some(),
//         java_geo_location.is_some(),
//         "Failed to create test case for {}",
//         message
//     );
//     if geo_location.is_none() {
//         return None;
//     }
//     let geo_location = geo_location.unwrap();
//     let java_geo_location = java_geo_location.unwrap();
//     Some((geo_location, java_geo_location, message))
// }

// fn random_date_time() -> (DateTime<chrono_tz::Tz>, Tz, i64, String) {
//     let mut rng = rand::thread_rng();
//     let tz = random_timezone();
//     let years_in_milliseconds = 1000 * 3600 * 24 * 365;
//     let milliseconds_since_epoch: i64 = rng.gen_range(
//         -years_in_milliseconds..=years_in_milliseconds * 100, // 1870 to 2070
//     );
//     let dt = tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap();
//     let timezone_id = tz.name();
//     let message = format!("DateTime: {}, Timezone: {}", dt, timezone_id);
//     (dt, tz, milliseconds_since_epoch, message)
// }

// pub fn create_date_times(jvm: &Jvm) -> Option<(DateTime<chrono_tz::Tz>, Instance, Tz, String)> {
//     let (date_time, timezone, milliseconds_since_epoch, message) = random_date_time();
//     let timezone_id = timezone.name();
//     let java_timezone = create_java_timezone(jvm, timezone_id)?;
//     let java_calendar = jvm
//         .invoke_static("java.util.Calendar", "getInstance", InvocationArg::empty())
//         .unwrap();
//     jvm.invoke(
//         &java_calendar,
//         "setTimeZone",
//         &[InvocationArg::from(java_timezone)],
//     )
//     .unwrap();
//     jvm.invoke(
//         &java_calendar,
//         "setTimeInMillis",
//         &[InvocationArg::try_from(milliseconds_since_epoch)
//             .unwrap()
//             .into_primitive()
//             .unwrap()],
//     )
//     .unwrap();

//     Some((date_time, java_calendar, timezone, message))
// }

// pub fn create_date_times_with_geolocation(
//     jvm: &Jvm,
// ) -> Option<(
//     DateTime<chrono_tz::Tz>,
//     Instance,
//     GeoLocation,
//     Instance,
//     String,
// )> {
//     let (date_time, java_calendar, timezone, date_time_message) = create_date_times(jvm)?;
//     let (geolocation, java_geo_location, geolocation_message) =
//         create_random_geolocations(jvm, timezone.name())?;
//     let new_message = format!("{}, {}", date_time_message, geolocation_message);
//     Some((
//         date_time,
//         java_calendar,
//         geolocation,
//         java_geo_location,
//         new_message,
//     ))
// }
// pub fn create_astronomical_calendars(
//     jvm: &Jvm,
// ) -> Option<(AstronomicalCalendar<chrono_tz::Tz>, Instance, String)> {
//     let (date_time, java_calendar, geo_location, java_geo_location, message) =
//         create_date_times_with_geolocation(jvm)?;
//     let astronomical_calendar = AstronomicalCalendar::new(date_time, geo_location);
//     let java_astronomical_calendar = jvm
//         .create_instance(
//             "com.kosherjava.zmanim.AstronomicalCalendar",
//             &[InvocationArg::from(java_geo_location)],
//         )
//         .unwrap();
//     jvm.invoke(
//         &java_astronomical_calendar,
//         "setCalendar",
//         &[InvocationArg::from(java_calendar)],
//     )
//     .unwrap();
//     Some((astronomical_calendar, java_astronomical_calendar, message))
// }

// pub fn create_zmanim_calendars(
//     jvm: &Jvm,
// ) -> Option<(ZmanimCalendar<chrono_tz::Tz>, Instance, String)> {
//     let (date_time, java_calendar, geo_location, java_geo_location, message) =
//         create_date_times_with_geolocation(jvm)?;
//     let candle_lighting_offset = Duration::minutes(rand::thread_rng().gen_range(0..=60));
//     let use_astronomical_chatzos = rand::thread_rng().gen_bool(0.5);
//     let use_astronomical_chatzos_for_other_zmanim = rand::thread_rng().gen_bool(0.5);
//     let ateret_torah_sunset_offset_minutes = rand::thread_rng().gen_range(0..=60);
//     let ateret_torah_sunset_offset = Duration::minutes(ateret_torah_sunset_offset_minutes);
//     let zmanim_calendar = ZmanimCalendar::new(
//         AstronomicalCalendar::new(date_time, geo_location),
//         candle_lighting_offset,
//         use_astronomical_chatzos,
//         use_astronomical_chatzos_for_other_zmanim,
//         ateret_torah_sunset_offset,
//     );
//     let message = format!(
//         "{}, {}, {}, {}, {}",
//         message,
//         candle_lighting_offset,
//         use_astronomical_chatzos,
//         use_astronomical_chatzos_for_other_zmanim,
//         candle_lighting_offset
//     );
//     let instance = jvm
//         .create_instance(
//             "com.kosherjava.zmanim.ComplexZmanimCalendar",
//             &[InvocationArg::try_from(java_geo_location).unwrap()],
//         )
//         .unwrap();
//     jvm.invoke(
//         &instance,
//         "setCalendar",
//         &[InvocationArg::try_from(java_calendar).unwrap()],
//     )
//     .unwrap();
//     let noaa_calculator_instance = jvm
//         .create_instance(
//             "com.kosherjava.zmanim.util.NOAACalculator",
//             InvocationArg::empty(),
//         )
//         .unwrap();
//     jvm.invoke(
//         &instance,
//         "setAstronomicalCalculator",
//         &[InvocationArg::try_from(noaa_calculator_instance).unwrap()],
//     )
//     .unwrap();
//     jvm.invoke(
//         &instance,
//         "setUseAstronomicalChatzos",
//         &[InvocationArg::try_from(use_astronomical_chatzos)
//             .unwrap()
//             .into_primitive()
//             .unwrap()],
//     )
//     .unwrap();
//     jvm.invoke(
//         &instance,
//         "setUseAstronomicalChatzosForOtherZmanim",
//         &[
//             InvocationArg::try_from(use_astronomical_chatzos_for_other_zmanim)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap(),
//         ],
//     )
//     .unwrap();
//     jvm.invoke(
//         &instance,
//         "setCandleLightingOffset",
//         &[
//             InvocationArg::try_from(candle_lighting_offset.as_seconds_f64() / 60.0)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap(),
//         ],
//     )
//     .unwrap();
//     jvm.invoke(
//         &instance,
//         "setUseElevation",
//         &[InvocationArg::try_from(true)
//             .unwrap()
//             .into_primitive()
//             .unwrap()],
//     )
//     .unwrap();
//     jvm.invoke(
//         &instance,
//         "setAteretTorahSunsetOffset",
//         &[
//             InvocationArg::try_from(ateret_torah_sunset_offset_minutes as f64)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap(),
//         ],
//     )
//     .unwrap();
//     Some((zmanim_calendar, instance, message))
// }

// pub fn create_jewish_dates(jvm: &Jvm) -> Option<(JewishDate, Instance, String)> {
//     let use_gregorian_date = rand::thread_rng().gen_bool(0.5);
//     if use_gregorian_date {
//         let (date_time, _, _, _) = random_date_time();

//         let message = format!(
//             "year: {}, month: {}, day: {}",
//             date_time.year(),
//             date_time.month(),
//             date_time.day(),
//         );
//         let year_arg = InvocationArg::try_from(date_time.year() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let month_arg = InvocationArg::try_from(date_time.month() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let day_arg = InvocationArg::try_from(date_time.day() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let jewish_date_instance = jvm
//             .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
//             .unwrap();
//         let jewish_date = JewishDate::from_gregorian_date(
//             date_time.year(),
//             date_time.month() as u8,
//             date_time.day() as u8,
//         );
//         let java_jewish_date = jvm
//             .create_instance(
//                 "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
//                 &[InvocationArg::from(jewish_date_instance)],
//             )
//             .ok();

//         assert_eq!(
//             jewish_date.is_some(),
//             java_jewish_date.is_some(),
//             "{}",
//             message
//         );
//         if jewish_date.is_none() {
//             return None;
//         }
//         let jewish_date = jewish_date.unwrap();
//         let java_jewish_date = java_jewish_date.unwrap();
//         return Some((jewish_date, java_jewish_date, message));
//     } else {
//         let (year, month, day) = random_hebrew_date();
//         let message = format!("year: {}, month: {}, day: {}", year, month, day,);
//         let jewish_date =
//             JewishDate::from_hebrew_date(year, JewishMonth::try_from(month).unwrap(), day);
//         let year_arg = InvocationArg::try_from(year as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let month_arg = InvocationArg::try_from(month as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let day_arg = InvocationArg::try_from(day as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let instance = jvm.create_instance(
//             "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
//             &[year_arg, month_arg, day_arg],
//         );

//         let java_jewish_date = match instance {
//             Ok(instance) => Some(instance),
//             Err(err) => {
//                 if let J4RsError::JavaError(message) = &err {
//                     // We will ignore the error if it is because the month is not between 1 and 12
//                     if message.contains("The Jewish month has to be between 1 and 12") {
//                         return None;
//                     } else {
//                         panic!("{}", err);
//                     }
//                 }
//                 panic!("{}", err);
//             }
//         };
//         // Java will gracefully handle the case of a day of 30 for a month that only has 29 days,
//         // and will set it to the last day of the month, whereas Rust will not.
//         if jewish_date.is_none() && java_jewish_date.is_some() && day == 30 {
//             return None;
//         }
//         assert_eq!(
//             jewish_date.is_some(),
//             java_jewish_date.is_some(),
//             "{}",
//             message
//         );
//         if jewish_date.is_none() {
//             return None;
//         }
//         let jewish_date = jewish_date.unwrap();
//         let java_jewish_date = java_jewish_date.unwrap();
//         return Some((jewish_date, java_jewish_date, message));
//     }
// }

// pub fn create_jewish_calendars(jvm: &Jvm) -> Option<(JewishCalendar, Instance, String)> {
//     let use_gregorian_date = rand::thread_rng().gen_bool(0.5);
//     let in_israel = rand::thread_rng().gen_bool(0.5);
//     let is_mukaf_choma = rand::thread_rng().gen_bool(0.5);
//     let use_modern_holidays = rand::thread_rng().gen_bool(0.5);
//     if use_gregorian_date {
//         let (date_time, _, _, _) = random_date_time();

//         let message = format!(
//             "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
//             date_time.year(),
//             date_time.month(),
//             date_time.day(),
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );
//         let year_arg = InvocationArg::try_from(date_time.year() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let month_arg = InvocationArg::try_from(date_time.month() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let day_arg = InvocationArg::try_from(date_time.day() as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let jewish_date_instance = jvm
//             .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
//             .unwrap();
//         let jewish_calendar = JewishCalendar::from_gregorian_date(
//             date_time.year(),
//             date_time.month() as u8,
//             date_time.day() as u8,
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );
//         let java_jewish_calendar = jvm
//             .create_instance(
//                 "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
//                 &[InvocationArg::from(jewish_date_instance)],
//             )
//             .ok();

//         assert_eq!(
//             jewish_calendar.is_some(),
//             java_jewish_calendar.is_some(),
//             "{}",
//             message
//         );
//         if jewish_calendar.is_none() {
//             return None;
//         }
//         let jewish_calendar = jewish_calendar.unwrap();
//         let java_jewish_calendar = java_jewish_calendar.unwrap();
//         let in_israel_arg = InvocationArg::try_from(in_israel)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         jvm.invoke(&java_jewish_calendar, "setInIsrael", &[in_israel_arg])
//             .unwrap();
//         jvm.invoke(
//             &java_jewish_calendar,
//             "setIsMukafChoma",
//             &[is_mukaf_choma_arg],
//         )
//         .unwrap();
//         jvm.invoke(
//             &java_jewish_calendar,
//             "setUseModernHolidays",
//             &[use_modern_holidays_arg],
//         )
//         .unwrap();
//         return Some((jewish_calendar, java_jewish_calendar, message));
//     } else {
//         let (year, month, day) = random_hebrew_date();
//         let message = format!(
//             "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
//             year, month, day, in_israel, is_mukaf_choma, use_modern_holidays,
//         );
//         let jewish_calendar = JewishCalendar::from_hebrew_date(
//             year,
//             JewishMonth::try_from(month).unwrap(),
//             day,
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );
//         let year_arg = InvocationArg::try_from(year as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let month_arg = InvocationArg::try_from(month as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let day_arg = InvocationArg::try_from(day as i32)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let instance = jvm.create_instance(
//             "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
//             &[year_arg, month_arg, day_arg],
//         );

//         let java_jewish_calendar = match instance {
//             Ok(instance) => Some(instance),
//             Err(err) => {
//                 if let J4RsError::JavaError(message) = &err {
//                     // We will ignore the error if it is because the month is not between 1 and 12
//                     if message.contains("The Jewish month has to be between 1 and 12") {
//                         return None;
//                     } else {
//                         panic!("{}", err);
//                     }
//                 }
//                 panic!("{}", err);
//             }
//         };
//         // Java will gracefully handle the case of a day of 30 for a month that only has 29 days,
//         // and will set it to the last day of the month, whereas Rust will not.
//         if jewish_calendar.is_none() && java_jewish_calendar.is_some() && day == 30 {
//             return None;
//         }
//         assert_eq!(
//             jewish_calendar.is_some(),
//             java_jewish_calendar.is_some(),
//             "{}",
//             message
//         );
//         if jewish_calendar.is_none() {
//             return None;
//         }
//         let jewish_calendar = jewish_calendar.unwrap();
//         let java_jewish_calendar = java_jewish_calendar.unwrap();
//         let in_israel_arg = InvocationArg::try_from(in_israel)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
//             .unwrap()
//             .into_primitive()
//             .unwrap();
//         jvm.invoke(&java_jewish_calendar, "setInIsrael", &[in_israel_arg])
//             .unwrap();
//         jvm.invoke(
//             &java_jewish_calendar,
//             "setIsMukafChoma",
//             &[is_mukaf_choma_arg],
//         )
//         .unwrap();
//         jvm.invoke(
//             &java_jewish_calendar,
//             "setUseModernHolidays",
//             &[use_modern_holidays_arg],
//         )
//         .unwrap();
//         return Some((jewish_calendar, java_jewish_calendar, message));
//     }
// }

// pub fn create_java_noaa_calculator(jvm: &Jvm) -> Instance {
//     jvm.create_instance(
//         "com.kosherjava.zmanim.util.NOAACalculator",
//         InvocationArg::empty(),
//     )
//     .unwrap()
// }

pub fn assert_almost_equal_f64(a: f64, b: f64, diff: f64, message: &str) {
    let result = (a - b).abs() < diff;
    let distance = (a - b).abs();
    assert!(result, "Error: {:?}, {:?}, distance: {}, {}", a, b, distance, message);
}
pub fn assert_almost_equal_i64(a: i64, b: i64, diff: i64, message: &str) {
    let result = (a - b).abs() < diff;
    let distance = (a - b).abs();
    assert!(result, "Error: {:?}, {:?}, distance: {}, {}", a, b, distance, message);
}

pub fn assert_almost_equal_f64_option(a: &Option<f64>, b: &Option<f64>, diff: f64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (None, Some(_)) => {
            add_rust_none_count();
        }
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

pub fn assert_almost_equal_i64_option(a: &Option<i64>, b: &Option<i64>, diff: i64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_i64(*a, *b, diff, message),
        (None, Some(_)) => {
            add_rust_none_count();
        }
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
#[allow(dead_code)]
pub fn assert_almost_equal_datetime(
    a: &DateTime<chrono_tz::Tz>,
    b: &DateTime<chrono_tz::Tz>,
    diff: i64,
    message: &str,
) {
    let result = (a.timestamp_millis() - b.timestamp_millis()).abs() < diff;
    let distance = (a.timestamp_millis() - b.timestamp_millis()).abs();
    assert!(result, "Error: {:?} vs {:?}, distance: {}, {}", a, b, distance, message);
}
#[allow(dead_code)]
pub fn assert_almost_equal_datetime_option(
    a: &Option<DateTime<chrono_tz::Tz>>,
    b: &Option<DateTime<chrono_tz::Tz>>,
    diff: i64,
    message: &str,
) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_datetime(a, b, diff, message),
        (None, Some(_)) => {
            add_rust_none_count();
        }
        (None, None) => (),
        _ => {
            panic!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
pub fn assert_almost_equal_duration(a: &Duration, b: &Duration, diff: i64, message: &str) {
    let result = (a.num_milliseconds() - b.num_milliseconds()).abs() < diff;
    let distance = (a.num_milliseconds() - b.num_milliseconds()).abs();
    assert!(result, "Error: {:?} vs {:?}, distance: {}, {}", a, b, distance, message);
}
#[derive(Clone, Copy)]
pub enum RandomValue {
    Infinite,
    OutOfRange,
    Nan,
    Normal,
}

pub fn random_random_value(rng: &mut impl Rng) -> RandomValue {
    let random_values = [
        RandomValue::Normal,
        RandomValue::Infinite,
        RandomValue::OutOfRange,
        RandomValue::Nan,
    ];
    random_values[WeightedIndex::new([999, 1, 1, 1]).unwrap().sample(rng)]
}

pub fn random_hebrew_date(rng: &mut impl Rng) -> (i32, JewishMonth, u8) {
    let dt = random_date_time(rng, chrono_tz::Tz::UTC);
    let year = dt.year() + 3760; // 3760 is the difference between the Gregorian and Hebrew years

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
    (year, month, day as u8)
}
