//! A set of functions which compare the Java and Rust implementations of the same trait.
//! This is used in our testing framework to ensure that the Java and Rust implementations are equivalent.
use crate::{
    astronomical_calculator::*,
    constants::*,
    geolocation::*,
    jewish_calendar::*,
    tefila_rules::{TefilaRules, TefilaRulesTrait},
    tests::*,
    zmanim_calendar::*,
};

use super::*;
pub fn compare_astronomical_calculators<'a>(
    rust_calculator: &NOAACalculator,
    java_calculator: &JavaAstronomicalCalculator<'a>,
    date: &chrono::DateTime<chrono_tz::Tz>,
    rust_geo_location: &GeoLocation,
    java_geo_location: &JavaGeoLocation<'a>,
    zenith: f64,
    adjust_for_elevation: bool,
) {
    // Test get_utc_noon
    let rust_noon = rust_calculator.get_utc_noon(date, rust_geo_location);
    let java_noon = java_calculator.get_utc_noon(date, java_geo_location);
    assert_almost_equal_f64(
        rust_noon,
        java_noon,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUtcNoon of {:?} at {:?}", rust_geo_location, date),
    );
    // Test get_utc_midnight
    let rust_midnight = rust_calculator.get_utc_midnight(date, rust_geo_location);
    let java_midnight = java_calculator.get_utc_midnight(date, java_geo_location);
    assert_almost_equal_f64(
        rust_midnight,
        java_midnight,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUtcMidnight of {:?} at {:?}", rust_geo_location, date),
    );
    // Test get_utc_sunrise
    let rust_sunrise = rust_calculator.get_utc_sunrise(date, rust_geo_location, zenith, adjust_for_elevation);
    let java_sunrise = java_calculator.get_utc_sunrise(date, java_geo_location, zenith, adjust_for_elevation);
    assert_almost_equal_f64_option(
        &rust_sunrise,
        &java_sunrise,
        DEFAULT_F64_TEST_EPSILON,
        &format!(
            "getUtcSunrise of {:?} at {:?} with zenith {} and adjust_for_elevation {}",
            rust_geo_location, date, zenith, adjust_for_elevation
        ),
    );
    // Test get_utc_sunset
    let rust_sunset = rust_calculator.get_utc_sunset(date, rust_geo_location, zenith, adjust_for_elevation);
    let java_sunset = java_calculator.get_utc_sunset(date, java_geo_location, zenith, adjust_for_elevation);
    assert_almost_equal_f64_option(
        &rust_sunset,
        &java_sunset,
        DEFAULT_F64_TEST_EPSILON,
        &format!(
            "getUtcSunset of {:?} at {:?} with zenith {} and adjust_for_elevation {}",
            rust_geo_location, date, zenith, adjust_for_elevation
        ),
    );
    // Test get_solar_elevation
    let rust_elevation = rust_calculator.get_solar_elevation(date, rust_geo_location);
    let java_elevation = java_calculator.get_solar_elevation(date, java_geo_location);
    assert_almost_equal_f64(
        rust_elevation,
        java_elevation,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getSolarElevation of {:?} at {:?}", rust_geo_location, date),
    );
    // Test get_solar_azimuth
    let rust_azimuth = rust_calculator.get_solar_azimuth(date, rust_geo_location);
    let java_azimuth = java_calculator.get_solar_azimuth(date, java_geo_location);
    assert_almost_equal_f64(
        rust_azimuth,
        java_azimuth,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getSolarAzimuth of {:?} at {:?}", rust_geo_location, date),
    );
    // Test get_solar_elevation
    let rust_elevation = rust_calculator.get_solar_elevation(date, rust_geo_location);
    let java_elevation = java_calculator.get_solar_elevation(date, java_geo_location);
    assert_almost_equal_f64(
        rust_elevation,
        java_elevation,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getSolarElevation of {:?} at {:?}", rust_geo_location, date),
    );
}
pub fn compare_geolocations<'a>(
    rust_geolocation: &GeoLocation,
    java_geolocation: &JavaGeoLocation<'a>,
    other_rust_geolocation: &GeoLocation,
    other_java_geolocation: &JavaGeoLocation<'a>,
    date: &chrono::DateTime<chrono_tz::Tz>,
) {
    assert_almost_equal_f64(
        rust_geolocation.get_rhumb_line_distance(other_rust_geolocation),
        java_geolocation.get_rhumb_line_distance(other_java_geolocation),
        0.02,
        &format!(
            "getRhumbLineDistance of {:?} against {:?}",
            rust_geolocation, other_rust_geolocation
        ),
    );
    assert_almost_equal_f64(
        rust_geolocation.get_rhumb_line_bearing(other_rust_geolocation),
        java_geolocation.get_rhumb_line_bearing(other_java_geolocation),
        0.02,
        &format!(
            "getRhumbLineBearing of {:?} against {:?}",
            rust_geolocation, other_rust_geolocation
        ),
    );
    assert_almost_equal_f64_option(
        &rust_geolocation.get_geodesic_initial_bearing(other_rust_geolocation),
        &java_geolocation.get_geodesic_initial_bearing(other_java_geolocation),
        0.02,
        &format!(
            "getGeodesicInitialBearing of {:?} against {:?}",
            rust_geolocation, other_rust_geolocation
        ),
    );
    assert_almost_equal_f64_option(
        &rust_geolocation.get_geodesic_final_bearing(other_rust_geolocation),
        &java_geolocation.get_geodesic_final_bearing(other_java_geolocation),
        0.02,
        &format!(
            "getGeodesicFinalBearing of {:?} against {:?}",
            rust_geolocation, other_rust_geolocation
        ),
    );
    assert_almost_equal_f64_option(
        &rust_geolocation.get_geodesic_distance(other_rust_geolocation),
        &java_geolocation.get_geodesic_distance(other_java_geolocation),
        0.02,
        &format!(
            "getGeodesicDistance of {:?} against {:?}",
            rust_geolocation, other_rust_geolocation
        ),
    );
    assert_almost_equal_duration(
        &rust_geolocation.get_local_mean_time_offset(date),
        &java_geolocation.get_local_mean_time_offset(date),
        &format!(
            "getLocalMeanTimeOffset of {:?} against {:?} at {:?} with timezone {:?}",
            rust_geolocation,
            other_rust_geolocation,
            date,
            date.timezone()
        ),
    );
    assert_eq!(
        rust_geolocation.get_antimeridian_adjustment(date),
        java_geolocation.get_antimeridian_adjustment(date),
        "getAntimeridianAdjustment of {:?} against {:?} at {:?}",
        rust_geolocation,
        other_rust_geolocation,
        date
    );
}
#[allow(clippy::too_many_arguments)]
pub fn compare_zmanim_calendars<'a>(
    rust_calendar: &ZmanimCalendar<chrono_tz::Tz, GeoLocation, NOAACalculator>,
    java_calendar: &JavaZmanimCalendar<'a, chrono_tz::Tz>,
    offset_zenith: f64,
    zenith: f64,
    hours: f64,
    start_of_day: &DateTime<chrono_tz::Tz>,
    end_of_day: &DateTime<chrono_tz::Tz>,
    degrees: f64,
    sunset: bool,
    start_of_half_day: &DateTime<chrono_tz::Tz>,
    end_of_half_day: &DateTime<chrono_tz::Tz>,
    start_of_day_option: Option<&DateTime<chrono_tz::Tz>>,
    end_of_day_option: Option<&DateTime<chrono_tz::Tz>>,
    synchronous: bool,
    alos: Option<&DateTime<chrono_tz::Tz>>,
    tzais: Option<&DateTime<chrono_tz::Tz>>,
) {
    let rust_sunrise = rust_calendar.get_sunrise();
    let java_sunrise = java_calendar.get_sunrise();
    assert_almost_equal_datetime_option(
        &rust_sunrise,
        &java_sunrise,
        &format!("getSunrise using {:?} ", rust_calendar),
    );

    let rust_sea_level_sunrise = rust_calendar.get_sea_level_sunrise();
    let java_sea_level_sunrise = java_calendar.get_sea_level_sunrise();
    assert_almost_equal_datetime_option(
        &rust_sea_level_sunrise,
        &java_sea_level_sunrise,
        &format!("getSeaLevelSunrise using {:?} ", rust_calendar),
    );

    let rust_begin_civil_twilight = rust_calendar.get_begin_civil_twilight();
    let java_begin_civil_twilight = java_calendar.get_begin_civil_twilight();
    assert_almost_equal_datetime_option(
        &rust_begin_civil_twilight,
        &java_begin_civil_twilight,
        &format!("getBeginCivilTwilight using {:?} ", rust_calendar),
    );

    let rust_begin_nautical_twilight = rust_calendar.get_begin_nautical_twilight();
    let java_begin_nautical_twilight = java_calendar.get_begin_nautical_twilight();
    assert_almost_equal_datetime_option(
        &rust_begin_nautical_twilight,
        &java_begin_nautical_twilight,
        &format!("getBeginNauticalTwilight using {:?} ", rust_calendar),
    );

    let rust_begin_astronomical_twilight = rust_calendar.get_begin_astronomical_twilight();
    let java_begin_astronomical_twilight = java_calendar.get_begin_astronomical_twilight();
    assert_almost_equal_datetime_option(
        &rust_begin_astronomical_twilight,
        &java_begin_astronomical_twilight,
        &format!("getBeginAstronomicalTwilight using {:?} ", rust_calendar),
    );

    let rust_sunset = rust_calendar.get_sunset();
    let java_sunset = java_calendar.get_sunset();
    assert_almost_equal_datetime_option(
        &rust_sunset,
        &java_sunset,
        &format!("getSunset using {:?} ", rust_calendar),
    );

    let rust_sea_level_sunset = rust_calendar.get_sea_level_sunset();
    let java_sea_level_sunset = java_calendar.get_sea_level_sunset();
    assert_almost_equal_datetime_option(
        &rust_sea_level_sunset,
        &java_sea_level_sunset,
        &format!("getSeaLevelSunset using {:?} ", rust_calendar),
    );

    let rust_end_civil_twilight = rust_calendar.get_end_civil_twilight();
    let java_end_civil_twilight = java_calendar.get_end_civil_twilight();
    assert_almost_equal_datetime_option(
        &rust_end_civil_twilight,
        &java_end_civil_twilight,
        &format!("getEndCivilTwilight using {:?} ", rust_calendar),
    );

    let rust_end_nautical_twilight = rust_calendar.get_end_nautical_twilight();
    let java_end_nautical_twilight = java_calendar.get_end_nautical_twilight();
    assert_almost_equal_datetime_option(
        &rust_end_nautical_twilight,
        &java_end_nautical_twilight,
        &format!("getEndNauticalTwilight using {:?} ", rust_calendar),
    );

    let rust_end_astronomical_twilight = rust_calendar.get_end_astronomical_twilight();
    let java_end_astronomical_twilight = java_calendar.get_end_astronomical_twilight();
    assert_almost_equal_datetime_option(
        &rust_end_astronomical_twilight,
        &java_end_astronomical_twilight,
        &format!("getEndAstronomicalTwilight using {:?} ", rust_calendar),
    );

    let rust_sunrise_offset_by_degrees = rust_calendar.get_sunrise_offset_by_degrees(offset_zenith);
    let java_sunrise_offset_by_degrees = java_calendar.get_sunrise_offset_by_degrees(offset_zenith);
    assert_almost_equal_datetime_option(
        &rust_sunrise_offset_by_degrees,
        &java_sunrise_offset_by_degrees,
        &format!("getSunriseOffsetByDegrees using {:?} ", rust_calendar),
    );

    let rust_sunset_offset_by_degrees = rust_calendar.get_sunset_offset_by_degrees(offset_zenith);
    let java_sunset_offset_by_degrees = java_calendar.get_sunset_offset_by_degrees(offset_zenith);
    assert_almost_equal_datetime_option(
        &rust_sunset_offset_by_degrees,
        &java_sunset_offset_by_degrees,
        &format!("getSunsetOffsetByDegrees using {:?} ", rust_calendar),
    );

    let rust_utc_sunrise = rust_calendar.get_utc_sunrise(zenith);
    let java_utc_sunrise = java_calendar.get_utc_sunrise(zenith);
    assert_almost_equal_f64_option(
        &rust_utc_sunrise,
        &java_utc_sunrise,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUTCSunrise using {:?} ", rust_calendar),
    );

    let rust_utc_sea_level_sunrise = rust_calendar.get_utc_sea_level_sunrise(zenith);
    let java_utc_sea_level_sunrise = java_calendar.get_utc_sea_level_sunrise(zenith);
    assert_almost_equal_f64_option(
        &rust_utc_sea_level_sunrise,
        &java_utc_sea_level_sunrise,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUTCSeaLevelSunrise using {:?} ", rust_calendar),
    );

    let rust_utc_sunset = rust_calendar.get_utc_sunset(zenith);
    let java_utc_sunset = java_calendar.get_utc_sunset(zenith);
    assert_almost_equal_f64_option(
        &rust_utc_sunset,
        &java_utc_sunset,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUTCSunset using {:?} ", rust_calendar),
    );

    let rust_utc_sea_level_sunset = rust_calendar.get_utc_sea_level_sunset(zenith);
    let java_utc_sea_level_sunset = java_calendar.get_utc_sea_level_sunset(zenith);
    assert_almost_equal_f64_option(
        &rust_utc_sea_level_sunset,
        &java_utc_sea_level_sunset,
        DEFAULT_F64_TEST_EPSILON,
        &format!("getUTCSeaLevelSunset using {:?} ", rust_calendar),
    );

    let rust_temporal_hour = rust_calendar.get_temporal_hour();
    let java_temporal_hour = java_calendar.get_temporal_hour();
    assert_almost_equal_duration_option(
        &rust_temporal_hour,
        &java_temporal_hour,
        &format!("getTemporalHour using {:?} ", rust_calendar),
    );

    let rust_temporal_hour_from_times = rust_calendar.get_temporal_hour_from_times(start_of_day, end_of_day);
    let java_temporal_hour_from_times = java_calendar.get_temporal_hour_from_times(start_of_day, end_of_day);
    assert_almost_equal_duration_option(
        &rust_temporal_hour_from_times,
        &java_temporal_hour_from_times,
        &format!("getTemporalHourFromTimes using {:?} ", rust_calendar),
    );

    let rust_sun_transit = rust_calendar.get_sun_transit();
    let java_sun_transit = java_calendar.get_sun_transit();
    assert_almost_equal_datetime_option(
        &rust_sun_transit,
        &java_sun_transit,
        &format!("getSunTransit using {:?} ", rust_calendar),
    );

    let rust_sun_transit_from_times = rust_calendar.get_sun_transit_from_times(start_of_day, end_of_day);
    let java_sun_transit_from_times = java_calendar.get_sun_transit_from_times(start_of_day, end_of_day);
    assert_almost_equal_datetime_option(
        &rust_sun_transit_from_times,
        &java_sun_transit_from_times,
        &format!("getSunTransitFromTimes using {:?} ", rust_calendar),
    );

    let rust_solar_midnight = rust_calendar.get_solar_midnight();
    let java_solar_midnight = java_calendar.get_solar_midnight();
    assert_almost_equal_datetime_option(
        &rust_solar_midnight,
        &java_solar_midnight,
        &format!("getSolarMidnight using {:?} ", rust_calendar),
    );

    let rust_local_mean_time = rust_calendar.get_local_mean_time(hours);
    let java_local_mean_time = java_calendar.get_local_mean_time(hours);
    assert_almost_equal_datetime_option(
        &rust_local_mean_time,
        &java_local_mean_time,
        &format!("getLocalMeanTime using {:?} ", rust_calendar),
    );
    for zman in Zman::values() {
        let result = rust_calendar.get_zman(&zman);
        let java_result = java_calendar.get_zman(&zman);
        assert_almost_equal_datetime_option(
            &result,
            &java_result,
            &format!("get_zman({:?}) against java with calendar {:?}", zman, rust_calendar),
        );
    }
    let result = rust_calendar.get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);
    let java_result = java_calendar.get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);
    assert_almost_equal_f64_option(
        &result,
        &java_result,
        DEFAULT_F64_TEST_EPSILON,
        &format!("get_percent_of_shaah_zmanis_from_degrees({}, {})", degrees, sunset),
    );

    let result = rust_calendar.get_shaah_zmanis_gra();
    let java_result = java_calendar.get_shaah_zmanis_gra();
    assert_almost_equal_duration_option(
        &result,
        &java_result,
        &format!("get_shaah_zmanis_gra against java with calendar {:?}", rust_calendar),
    );

    let result = rust_calendar.get_shaah_zmanis_mga();
    let java_result = java_calendar.get_shaah_zmanis_mga();
    assert_almost_equal_duration_option(
        &result,
        &java_result,
        &format!("get_shaah_zmanis_mga against java with calendar {:?}", rust_calendar),
    );

    let result = rust_calendar.get_half_day_based_zman_from_times(start_of_half_day, end_of_half_day, hours);
    let java_result = java_calendar.get_half_day_based_zman_from_times(start_of_half_day, end_of_half_day, hours);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_half_day_based_zman_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_half_day, end_of_half_day, hours,
        ),
    );
    let result = rust_calendar.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day, hours);
    let java_result = java_calendar.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day, hours);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_shaah_zmanis_based_zman_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day, end_of_day, hours,
        ),
    );

    let result = rust_calendar.get_half_day_based_shaah_zmanis_from_times(start_of_half_day, end_of_half_day);
    let java_result = java_calendar.get_half_day_based_shaah_zmanis_from_times(start_of_half_day, end_of_half_day);
    assert_almost_equal_duration_option(
        &result,
        &java_result,
        &format!(
            "get_half_day_based_shaah_zmanis_from_times against java with calendar {:?} with args {:?}, {:?}",
            rust_calendar, start_of_half_day, end_of_half_day,
        ),
    );

    let result = rust_calendar.get_sof_zman_shma_from_times(start_of_day, end_of_day_option, synchronous);
    let java_result = java_calendar.get_sof_zman_shma_from_times(start_of_day, end_of_day_option, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_sof_zman_shma_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day, end_of_day_option, synchronous,
        ),
    );

    let result = rust_calendar.get_mincha_ketana_from_times(start_of_day_option, end_of_day, synchronous);
    let java_result = java_calendar.get_mincha_ketana_from_times(start_of_day_option, end_of_day, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_mincha_ketana_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day_option, end_of_day, synchronous,
        ),
    );

    let result = rust_calendar.get_sof_zman_tfila_from_times(start_of_day, end_of_day_option, synchronous);
    let java_result = java_calendar.get_sof_zman_tfila_from_times(start_of_day, end_of_day_option, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_sof_zman_tfila_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day, end_of_day_option, synchronous,
        ),
    );

    let result = rust_calendar.get_mincha_gedola_from_times(start_of_day_option, end_of_day, synchronous);
    let java_result = java_calendar.get_mincha_gedola_from_times(start_of_day_option, end_of_day, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_mincha_gedola_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day_option, end_of_day, synchronous,
        ),
    );
    let result = rust_calendar.get_plag_hamincha_from_times(start_of_day_option, end_of_day, synchronous);
    let java_result = java_calendar.get_plag_hamincha_from_times(start_of_day_option, end_of_day, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_plag_hamincha_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day_option, end_of_day, synchronous,
        ),
    );

    let result = rust_calendar.get_samuch_le_mincha_ketana_from_times(start_of_day_option, end_of_day, synchronous);
    let java_result =
        java_calendar.get_samuch_le_mincha_ketana_from_times(start_of_day_option, end_of_day, synchronous);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_samuch_le_mincha_ketana_from_times against java with calendar {:?} with args {:?}, {:?}, {:?}",
            rust_calendar, start_of_day_option, end_of_day, synchronous,
        ),
    );

    let result = rust_calendar.get_sof_zman_kidush_levana_15_days_from_times(alos, tzais);
    let java_result = java_calendar.get_sof_zman_kidush_levana_15_days_from_times(alos, tzais);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_sof_zman_kidush_levana_15_days_from_times against java with calendar {:?} with args {:?}, {:?}",
            rust_calendar, alos, tzais,
        ),
    );

    let result = rust_calendar.get_sof_zman_kidush_levana_between_moldos_from_times(alos, tzais);
    let java_result = java_calendar.get_sof_zman_kidush_levana_between_moldos_from_times(alos, tzais);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_sof_zman_kidush_levana_between_moldos_from_times against java with calendar {:?} with args {:?}, {:?}",
            rust_calendar, alos, tzais,
        ),
    );

    let result = rust_calendar.get_tchilas_zman_kidush_levana_3_days_from_times(alos, tzais);
    let java_result = java_calendar.get_tchilas_zman_kidush_levana_3_days_from_times(alos, tzais);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_tchilas_zman_kidush_levana_3_days_from_times against java with calendar {:?} with args {:?}, {:?}",
            rust_calendar, alos, tzais,
        ),
    );

    let result = rust_calendar.get_tchilas_zman_kidush_levana_7_days_from_times(alos, tzais);
    let java_result = java_calendar.get_tchilas_zman_kidush_levana_7_days_from_times(alos, tzais);
    assert_almost_equal_datetime_option(
        &result,
        &java_result,
        &format!(
            "get_tchilas_zman_kidush_levana_7_days_from_times against java with calendar {:?} with args {:?}, {:?}",
            rust_calendar, alos, tzais,
        ),
    );
}
pub fn compare_jewish_calendars(
    rust_calendar: &impl JewishCalendarTrait,
    java_calendar: &impl JewishCalendarTrait,
    message: &str,
    is_recursive: bool,
) {
    assert_eq!(
        rust_calendar.get_yom_tov_index(),
        java_calendar.get_yom_tov_index(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_yom_tov(), java_calendar.is_yom_tov(), "{}", message);
    assert_eq!(
        rust_calendar.is_yom_tov_assur_bemelacha(),
        java_calendar.is_yom_tov_assur_bemelacha(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_assur_bemelacha(), java_calendar.is_assur_bemelacha());
    assert_eq!(
        rust_calendar.has_candle_lighting(),
        java_calendar.has_candle_lighting(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_tomorrow_shabbos_or_yom_tov(),
        java_calendar.is_tomorrow_shabbos_or_yom_tov(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_erev_yom_tov_sheni(),
        java_calendar.is_erev_yom_tov_sheni(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_aseres_yemei_teshuva(),
        java_calendar.is_aseres_yemei_teshuva(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_pesach(), java_calendar.is_pesach());
    assert_eq!(
        rust_calendar.is_chol_hamoed_pesach(),
        java_calendar.is_chol_hamoed_pesach(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_shavuos(), java_calendar.is_shavuos(), "{}", message);
    assert_eq!(rust_calendar.is_rosh_hashana(), java_calendar.is_rosh_hashana());
    assert_eq!(
        rust_calendar.is_yom_kippur(),
        java_calendar.is_yom_kippur(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_succos(), java_calendar.is_succos());
    assert_eq!(
        rust_calendar.is_hoshana_rabba(),
        java_calendar.is_hoshana_rabba(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_shemini_atzeres(), java_calendar.is_shemini_atzeres());
    assert_eq!(
        rust_calendar.is_simchas_torah(),
        java_calendar.is_simchas_torah(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_chol_hamoed_succos(),
        java_calendar.is_chol_hamoed_succos(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_chol_hamoed(),
        java_calendar.is_chol_hamoed(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_erev_yom_tov(),
        java_calendar.is_erev_yom_tov(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_rosh_chodesh(),
        java_calendar.is_rosh_chodesh(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_isru_chag(),
        java_calendar.is_isru_chag(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_taanis(), java_calendar.is_taanis(), "{}", message);
    assert_eq!(
        rust_calendar.is_taanis_bechoros(),
        java_calendar.is_taanis_bechoros(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.get_day_of_chanukah(), java_calendar.get_day_of_chanukah());
    assert_eq!(rust_calendar.is_chanukah(), java_calendar.is_chanukah(), "{}", message);
    assert_eq!(rust_calendar.is_purim(), java_calendar.is_purim());
    assert_eq!(
        rust_calendar.get_day_of_omer(),
        java_calendar.get_day_of_omer(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_tisha_beav(), java_calendar.is_tisha_beav());
    assert_eq!(rust_calendar.get_parshah(), java_calendar.get_parshah(), "{}", message);
    assert_eq!(
        rust_calendar.get_daf_yomi_bavli(),
        java_calendar.get_daf_yomi_bavli(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_daf_yomi_yerushalmi(),
        java_calendar.get_daf_yomi_yerushalmi(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_birkas_hachamah(),
        java_calendar.is_birkas_hachamah(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_erev_rosh_chodesh(),
        java_calendar.is_erev_rosh_chodesh(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_yom_kippur_katan(),
        java_calendar.is_yom_kippur_katan(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.is_be_hab(), java_calendar.is_be_hab(), "{}", message);
    assert_eq!(rust_calendar.is_machar_chodesh(), java_calendar.is_machar_chodesh());
    assert_eq!(
        rust_calendar.is_shabbos_mevorchim(),
        java_calendar.is_shabbos_mevorchim(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_upcoming_parshah(),
        java_calendar.get_upcoming_parshah(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.get_special_shabbos(), java_calendar.get_special_shabbos());
    assert_eq!(
        rust_calendar.get_molad_as_date(),
        java_calendar.get_molad_as_date(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_tchilaszman_kidush_levana_3_days(),
        java_calendar.get_tchilaszman_kidush_levana_3_days(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_tchilaszman_kidush_levana_7_days(),
        java_calendar.get_tchilaszman_kidush_levana_7_days(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_sof_zman_kidush_levana_between_moldos(),
        java_calendar.get_sof_zman_kidush_levana_between_moldos(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_sof_zman_kidush_levana_15_days(),
        java_calendar.get_sof_zman_kidush_levana_15_days(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.get_tekufas_tishrei_elapsed_days(),
        java_calendar.get_tekufas_tishrei_elapsed_days(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_vesein_tal_umatar_start_date(),
        java_calendar.is_vesein_tal_umatar_start_date(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_vesein_tal_umatar_starting_tonight(),
        java_calendar.is_vesein_tal_umatar_starting_tonight(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_vesein_tal_umatar_recited(),
        java_calendar.is_vesein_tal_umatar_recited(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_vesein_beracha_recited(),
        java_calendar.is_vesein_beracha_recited(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_mashiv_haruach_start_date(),
        java_calendar.is_mashiv_haruach_start_date(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_mashiv_haruach_end_date(),
        java_calendar.is_mashiv_haruach_end_date(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_mashiv_haruach_recited(),
        java_calendar.is_mashiv_haruach_recited(),
        "{}",
        message
    );
    assert_eq!(
        rust_calendar.is_morid_hatal_recited(),
        java_calendar.is_morid_hatal_recited(),
        "{}",
        message
    );
    assert_eq!(rust_calendar.get_jewish_year(), java_calendar.get_jewish_year());
    assert_eq!(rust_calendar.get_jewish_month(), java_calendar.get_jewish_month());
    assert_eq!(
        rust_calendar.get_jewish_day_of_month(),
        java_calendar.get_jewish_day_of_month()
    );
    assert_eq!(rust_calendar.get_gregorian_year(), java_calendar.get_gregorian_year());
    assert_eq!(rust_calendar.get_gregorian_month(), java_calendar.get_gregorian_month());
    assert_eq!(
        rust_calendar.get_gregorian_day_of_month(),
        java_calendar.get_gregorian_day_of_month()
    );
    assert_eq!(rust_calendar.get_day_of_week(), java_calendar.get_day_of_week());
    assert_eq!(rust_calendar.is_jewish_leap_year(), java_calendar.is_jewish_leap_year());
    assert_eq!(
        rust_calendar.get_days_in_jewish_year(),
        java_calendar.get_days_in_jewish_year()
    );
    assert_eq!(
        rust_calendar.get_days_in_jewish_month(),
        java_calendar.get_days_in_jewish_month()
    );
    assert_eq!(rust_calendar.is_cheshvan_long(), java_calendar.is_cheshvan_long());
    assert_eq!(rust_calendar.is_kislev_short(), java_calendar.is_kislev_short());
    assert_eq!(
        rust_calendar.get_cheshvan_kislev_kviah(),
        java_calendar.get_cheshvan_kislev_kviah()
    );
    assert_eq!(
        rust_calendar.get_days_since_start_of_jewish_year(),
        java_calendar.get_days_since_start_of_jewish_year()
    );
    assert_eq!(
        rust_calendar.get_chalakim_since_molad_tohu(),
        java_calendar.get_chalakim_since_molad_tohu()
    );

    assert_eq!(rust_calendar.get_molad(), java_calendar.get_molad());
    assert_eq!(rust_calendar.get_molad_as_date(), java_calendar.get_molad_as_date());

    if !is_recursive
        && let (Some(jewish_molad), Some(java_molad)) = (
            rust_calendar.get_molad_as_calendar(),
            java_calendar.get_molad_as_calendar(),
        )
    {
        compare_jewish_calendars(&jewish_molad, &java_molad, message, true);
    }
}
pub fn compare_tefila_rules(
    rust_tefila_rules: &TefilaRules,
    java_tefila_rules: &JavaTefilaRules,
    rust_jewish_calendar: &JewishCalendar<NOAACalculator>,
    java_jewish_calendar: &JavaJewishCalendar,
    message: &str,
) {
    assert_eq!(
        rust_tefila_rules.is_tachanun_recited_shacharis(rust_jewish_calendar),
        java_tefila_rules.is_tachanun_recited_shacharis(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_tachanun_recited_mincha(rust_jewish_calendar),
        java_tefila_rules.is_tachanun_recited_mincha(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_hallel_recited(rust_jewish_calendar),
        java_tefila_rules.is_hallel_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_hallel_shalem_recited(rust_jewish_calendar),
        java_tefila_rules.is_hallel_shalem_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_al_hanissim_recited(rust_jewish_calendar),
        java_tefila_rules.is_al_hanissim_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_yaaleh_veyavo_recited(rust_jewish_calendar),
        java_tefila_rules.is_yaaleh_veyavo_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_mizmor_lesoda_recited(rust_jewish_calendar),
        java_tefila_rules.is_mizmor_lesoda_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_vesein_tal_umatar_start_date(rust_jewish_calendar),
        java_tefila_rules.is_vesein_tal_umatar_start_date(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_vesein_tal_umatar_starting_tonight(rust_jewish_calendar),
        java_tefila_rules.is_vesein_tal_umatar_starting_tonight(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_vesein_tal_umatar_recited(rust_jewish_calendar),
        java_tefila_rules.is_vesein_tal_umatar_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_vesein_beracha_recited(rust_jewish_calendar),
        java_tefila_rules.is_vesein_beracha_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_mashiv_haruach_start_date(rust_jewish_calendar),
        java_tefila_rules.is_mashiv_haruach_start_date(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_mashiv_haruach_end_date(rust_jewish_calendar),
        java_tefila_rules.is_mashiv_haruach_end_date(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_mashiv_haruach_recited(rust_jewish_calendar),
        java_tefila_rules.is_mashiv_haruach_recited(java_jewish_calendar),
        "{}",
        message
    );
    assert_eq!(
        rust_tefila_rules.is_morid_hatal_recited(rust_jewish_calendar),
        java_tefila_rules.is_morid_hatal_recited(java_jewish_calendar),
        "{}",
        message
    );
}
