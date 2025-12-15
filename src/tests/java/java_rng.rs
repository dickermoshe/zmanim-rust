//! A set of functions which generate random pairs of Java and Rust objects.
//! This is used in our testing framework to create random test cases.
use std::str::FromStr;

use super::*;
use crate::prelude::*;
use chrono::{Datelike, Duration, Timelike, Utc};
use j4rs::Jvm;
use lazy_static::lazy_static;
use rand::Rng;
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}

/// Default number of years to test.
static DEFAULT_TEST_YEARS: i64 = 100;

/// Default number of milliseconds in the given number of years.
static DEFAULT_TEST_YEARS_IN_MILLISECONDS: i64 = 1000 * 3600 * 24 * 365 * DEFAULT_TEST_YEARS;

/// Generates a random DateTime in the range 1870-2070 with the given timezone.
fn random_date_time(rng: &mut impl Rng, tz: &chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    let milliseconds_since_epoch: i64 = rng.gen_range(
        -DEFAULT_TEST_YEARS_IN_MILLISECONDS..=DEFAULT_TEST_YEARS_IN_MILLISECONDS, // 1870 to 2070
    );
    tz.timestamp_millis_opt(milliseconds_since_epoch).unwrap()
}

pub fn random_time_and_place<'a, Rng: rand::Rng>(
    jvm: &'a Jvm,
    rng: &mut Rng,
) -> Option<(TimeAndPlace<chrono_tz::Tz>, JavaTimeAndPlace)> {
    // WWe are using a different algorithim to calculate sinrise and sunset.
    // The difference between these 2 algorithms are small under most cases. However as
    // you get closer to the poles, these results can vary signifigantly.
    // We are allowing for a n second difference between results. If we test for locations
    // too close to the poles, we would need to allow for a much larger room for error
    // which would start to affect the effectiveness of the tests.
    let latitude = rng.gen_range(-50.0..=50.0);
    let longitude = rng.gen_range(-180.0..=180.0);
    let elevation = rng.gen_range(-0.0..=1000.0);
    let timezone_id = FINDER.get_tz_name(longitude, latitude);
    let timezone = chrono_tz::Tz::from_str(timezone_id).ok()?;
    let date_time = random_date_time(rng, &timezone);
    let rust_time_and_place = TimeAndPlace::new(latitude, longitude, elevation, date_time.date_naive(), timezone)?;
    let java_time_and_place = JavaTimeAndPlace::new(jvm, &rust_time_and_place)?;
    Some((rust_time_and_place, java_time_and_place))
}

pub fn random_zmanim_calendars<'a>(
    jvm: &'a Jvm,
    rng: &mut impl Rng,
    time_and_places: Option<(TimeAndPlace<chrono_tz::Tz>, JavaTimeAndPlace)>,
) -> Option<(ZmanimCalendar<chrono_tz::Tz>, JavaZmanimCalendar<'a>)> {
    let (rust_time_and_place, java_time_and_place) = time_and_places.or_else(|| random_time_and_place(jvm, rng))?;

    let candle_lighting_offset = Duration::minutes(rng.gen_range(0..=60));
    let use_astronomical_chatzos = rng.gen_bool(0.5);
    let use_astronomical_chatzos_for_other_zmanim = rng.gen_bool(0.5);
    let ateret_torah_sunset_offset = Duration::minutes(rng.gen_range(0..=60));
    let rust_calendar = ZmanimCalendar::new(
        rust_time_and_place.clone(),
        use_astronomical_chatzos,
        use_astronomical_chatzos_for_other_zmanim,
        candle_lighting_offset,
        ateret_torah_sunset_offset,
    )?;
    let java_calendar = JavaZmanimCalendar::new(
        jvm,
        java_time_and_place,
        candle_lighting_offset,
        use_astronomical_chatzos,
        use_astronomical_chatzos_for_other_zmanim,
        ateret_torah_sunset_offset,
    )?;

    Some((rust_calendar, java_calendar))
}

// pub fn create_jewish_calendars<'a>(
//     jvm: &'a Jvm,
//     rng: &mut impl Rng,
// ) -> Option<(JewishCalendar, JavaJewishCalendar<'a>, String)> {
//     let use_gregorian_date = rng.gen_bool(0.5);
//     let in_israel = rng.gen_bool(0.5);
//     let is_mukaf_choma = rng.gen_bool(0.5);
//     let use_modern_holidays = rng.gen_bool(0.5);

//     if use_gregorian_date {
//         let date_time = random_date_time(rng, true, chrono_tz::Tz::UTC);
//         let message = format!(
//             "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
//             date_time.year(),
//             date_time.month(),
//             date_time.day(),
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays
//         );

//         let rust_calendar = JewishCalendar::from_gregorian_date(
//             date_time.year(),
//             date_time.month() as u8,
//             date_time.day() as u8,
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );
//         let java_calendar = JavaJewishCalendar::from_gregorian_date(
//             jvm,
//             date_time.year() as i32,
//             date_time.month() as i32,
//             date_time.day() as i32,
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );

//         assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
//         if rust_calendar.is_none() || java_calendar.is_none() {
//             return None;
//         }

//         Some((rust_calendar.unwrap(), java_calendar.unwrap(), message))
//     } else {
//         let (year, month, day) = random_hebrew_date(rng);
//         let message = format!(
//             "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
//             year, month as i32, day, in_israel, is_mukaf_choma, use_modern_holidays
//         );

//         let rust_calendar =
//             JewishCalendar::from_hebrew_date(year, month, day, in_israel, is_mukaf_choma, use_modern_holidays);
//         let java_calendar = JavaJewishCalendar::from_jewish_date(
//             jvm,
//             year,
//             month,
//             day as i32,
//             in_israel,
//             is_mukaf_choma,
//             use_modern_holidays,
//         );

//         assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
//         if rust_calendar.is_none() || java_calendar.is_none() {
//             return None;
//         }

//         let java_calendar = java_calendar.unwrap();

//         Some((rust_calendar.unwrap(), java_calendar, message))
//     }
// }

// pub fn create_teffila_rules<'a, Rng: rand::Rng>(jvm: &'a Jvm, rng: &mut Rng) -> (TefilaRules, JavaTefilaRules<'a>) {
//     let tefila_rules = TefilaRules::new(
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//         rng.gen_bool(0.5),
//     );
//     let java_tefila_rules = JavaTefilaRules::new(
//         jvm,
//         tefila_rules.tachanun_recited_end_of_tishrei,
//         tefila_rules.tachanun_recited_week_after_shavuos,
//         tefila_rules.tachanun_recited_13_sivan_out_of_israel,
//         tefila_rules.tachanun_recited_pesach_sheni,
//         tefila_rules.tachanun_recited_15_iyar_out_of_israel,
//         tefila_rules.tachanun_recited_mincha_erev_lag_baomer,
//         tefila_rules.tachanun_recited_shivas_yemei_hamiluim,
//         tefila_rules.tachanun_recited_week_of_hod,
//         tefila_rules.tachanun_recited_week_of_purim,
//         tefila_rules.tachanun_recited_fridays,
//         tefila_rules.tachanun_recited_sundays,
//         tefila_rules.tachanun_recited_mincha_all_year,
//         tefila_rules.mizmor_lesoda_recited_erev_yom_kippur_and_pesach,
//     );

//     (tefila_rules, java_tefila_rules)
// }
