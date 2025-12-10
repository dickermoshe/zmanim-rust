//! A set of functions which generate random pairs of Java and Rust objects.
//! This is used in our testing framework to create random test cases.
use chrono::{Datelike, Duration};
use chrono_tz::TZ_VARIANTS;
use j4rs::Jvm;
use rand::Rng;

use super::*;
use crate::{
    prelude::*,
    tests::{random_date_time, random_hebrew_date},
};

pub fn create_geolocations<'a, Rng: rand::Rng>(
    jvm: &'a Jvm,
    rng: &mut Rng,
) -> Option<(GeoLocation, JavaGeoLocation<'a>)> {
    let latitude = rng.gen_range(-91.0..=91.0);
    let longitude = rng.gen_range(-181.0..=181.0);
    let elevation = rng.gen_range(-1.0..=1000.0);
    let tz = TZ_VARIANTS[rng.gen_range(0..TZ_VARIANTS.len())];
    // DIFF: Java cannot handle the some timezones
    // Tehran at the time of the revolution had a unclear timezone which Java and Rust handle differently
    if tz.name() == "ROC" || tz.name() == "America/Coyhaique" || tz.name() == "GMT" || tz.name() == "Asia/Tehran" {
        return None;
    }

    let geo_location = GeoLocation::new(latitude, longitude, elevation);
    let java_geo_location = JavaGeoLocation::new(jvm, latitude, longitude, elevation, tz);
    // Ensure both are some or none
    assert_eq!(
        geo_location.is_some(),
        java_geo_location.is_some(),
        "Failed to create test case for latitude: {}, longitude: {}, elevation: {}, timezone: {}",
        latitude,
        longitude,
        elevation,
        tz.name()
    );
    if geo_location.is_none() || java_geo_location.is_none() {
        return None;
    }
    let geo_location = geo_location.unwrap();
    let java_geo_location = java_geo_location.unwrap();
    Some((geo_location, java_geo_location))
}

pub fn create_zmanim_calendars<'a>(
    jvm: &'a Jvm,
    rng: &mut impl Rng,
) -> Option<(
    ZmanimCalendar<chrono_tz::Tz, GeoLocation, NOAACalculator>,
    JavaZmanimCalendar<'a, chrono_tz::Tz>,
)> {
    let (geo_location, java_geo_location) = create_geolocations(jvm, rng)?;

    let date_time = random_date_time(rng, java_geo_location.timezone);
    let candle_lighting_offset = Duration::minutes(rng.gen_range(0..=60));
    let use_astronomical_chatzos = rng.gen_bool(0.5);
    let use_astronomical_chatzos_for_other_zmanim = rng.gen_bool(0.5);
    let ateret_torah_sunset_offset = Duration::minutes(rng.gen_range(0..=60));

    let rust_calendar = ZmanimCalendar::new(
        date_time,
        geo_location.clone(),
        NOAACalculator,
        use_astronomical_chatzos,
        use_astronomical_chatzos_for_other_zmanim,
        candle_lighting_offset,
        ateret_torah_sunset_offset,
    );
    let java_calendar = JavaZmanimCalendar::new(
        jvm,
        date_time,
        geo_location,
        candle_lighting_offset,
        use_astronomical_chatzos,
        use_astronomical_chatzos_for_other_zmanim,
        ateret_torah_sunset_offset,
    );

    Some((rust_calendar, java_calendar))
}

pub fn create_jewish_calendars<'a>(
    jvm: &'a Jvm,
    rng: &mut impl Rng,
) -> Option<(JewishCalendar<NOAACalculator>, JavaJewishCalendar<'a>, String)> {
    let use_gregorian_date = rng.gen_bool(0.5);
    let in_israel = rng.gen_bool(0.5);
    let is_mukaf_choma = rng.gen_bool(0.5);
    let use_modern_holidays = rng.gen_bool(0.5);

    if use_gregorian_date {
        let date_time = random_date_time(rng, chrono_tz::Tz::UTC);
        let message = format!(
            "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
            date_time.year(),
            date_time.month(),
            date_time.day(),
            in_israel,
            is_mukaf_choma,
            use_modern_holidays
        );

        let rust_calendar = JewishCalendar::from_gregorian_date(
            date_time.year(),
            date_time.month() as u8,
            date_time.day() as u8,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            NOAACalculator,
        );
        let java_calendar = JavaJewishCalendar::from_gregorian_date(
            jvm,
            date_time.year() as i32,
            date_time.month() as i32,
            date_time.day() as i32,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
        );

        assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
        if rust_calendar.is_none() || java_calendar.is_none() {
            return None;
        }

        Some((rust_calendar.unwrap(), java_calendar.unwrap(), message))
    } else {
        let (year, month, day) = random_hebrew_date(rng);
        let message = format!(
            "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
            year, month as i32, day, in_israel, is_mukaf_choma, use_modern_holidays
        );

        let rust_calendar = JewishCalendar::from_hebrew_date(
            year,
            month,
            day,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            NOAACalculator,
        );
        let java_calendar = JavaJewishCalendar::from_jewish_date(
            jvm,
            year,
            month,
            day as i32,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
        );

        assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
        if rust_calendar.is_none() || java_calendar.is_none() {
            return None;
        }

        let java_calendar = java_calendar.unwrap();

        Some((rust_calendar.unwrap(), java_calendar, message))
    }
}

pub fn create_teffila_rules<'a, Rng: rand::Rng>(jvm: &'a Jvm, rng: &mut Rng) -> (TefilaRules, JavaTefilaRules<'a>) {
    let tefila_rules = TefilaRules::new(
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
        rng.gen_bool(0.5),
    );
    let java_tefila_rules = JavaTefilaRules::new(
        jvm,
        tefila_rules.tachanun_recited_end_of_tishrei,
        tefila_rules.tachanun_recited_week_after_shavuos,
        tefila_rules.tachanun_recited_13_sivan_out_of_israel,
        tefila_rules.tachanun_recited_pesach_sheni,
        tefila_rules.tachanun_recited_15_iyar_out_of_israel,
        tefila_rules.tachanun_recited_mincha_erev_lag_baomer,
        tefila_rules.tachanun_recited_shivas_yemei_hamiluim,
        tefila_rules.tachanun_recited_week_of_hod,
        tefila_rules.tachanun_recited_week_of_purim,
        tefila_rules.tachanun_recited_fridays,
        tefila_rules.tachanun_recited_sundays,
        tefila_rules.tachanun_recited_mincha_all_year,
        tefila_rules.mizmor_lesoda_recited_erev_yom_kippur_and_pesach,
    );

    (tefila_rules, java_tefila_rules)
}
