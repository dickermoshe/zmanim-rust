use chrono::{DateTime, Datelike, Duration, Offset, TimeDelta, TimeZone, Utc};

use crate::{constants::*, geolocation::GeoLocation, noaa_calculator::NOAACalculator};
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AstronomicalCalendar<Tz: TimeZone> {
    pub date_time: DateTime<Tz>,
    pub geo_location: GeoLocation,
    pub noaa_calculator: NOAACalculator,
}

impl<Tz: TimeZone> AstronomicalCalendar<Tz> {
    pub fn new(date_time: DateTime<Tz>, geo_location: GeoLocation) -> Self {
        Self {
            date_time,
            geo_location,
            noaa_calculator: NOAACalculator,
        }
    }
    fn get_adjusted_date_time(&self, date_time: &DateTime<Tz>) -> DateTime<Tz> {
        let offset = self
            .get_geo_location()
            .get_antimeridian_adjustment(date_time);
        if offset >= 1 {
            return date_time.clone() + Duration::days(offset as i64);
        } else if offset < 0 {
            return date_time.clone() - Duration::days(offset as i64);
        } else {
            date_time.clone()
        }
    }
}

impl<Tz: TimeZone> AstronomicalCalendarTrait<Tz> for AstronomicalCalendar<Tz> {
    fn get_date_time(&self) -> &DateTime<Tz> {
        &self.date_time
    }

    fn get_geo_location(&self) -> &impl GeoLocationTrait {
        &self.geo_location
    }

    fn get_noaa_calculator(&self) -> &impl NOAACalculatorTrait {
        &self.noaa_calculator
    }
    fn get_sunrise(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunrise(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
    }

    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sea_level_sunrise(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
    }

    fn get_begin_civil_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_CIVIL_ZENITH)
    }

    fn get_begin_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_NAUTICAL_ZENITH)
    }

    fn get_begin_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_ASTRONOMICAL_ZENITH)
    }

    fn get_sunset(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunset(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
    }

    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sea_level_sunset(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
    }

    fn get_end_civil_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_CIVIL_ZENITH)
    }

    fn get_end_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_NAUTICAL_ZENITH)
    }

    fn get_end_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_ASTRONOMICAL_ZENITH)
    }

    fn get_sunrise_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunrise(offset_zenith)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
    }

    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunset(offset_zenith)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
    }

    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time());
        self.get_noaa_calculator().get_utc_sunrise(
            &adjusted_date_time,
            self.get_geo_location(),
            zenith,
            true,
        )
    }

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64> {
        self.get_noaa_calculator().get_utc_sunrise(
            &self.get_adjusted_date_time(self.get_date_time()),
            self.get_geo_location(),
            zenith,
            false,
        )
    }

    fn get_utc_sunset(&self, zenith: f64) -> Option<f64> {
        self.get_noaa_calculator().get_utc_sunset(
            &self.get_adjusted_date_time(self.get_date_time()),
            self.get_geo_location(),
            zenith,
            true,
        )
    }

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64> {
        self.get_noaa_calculator().get_utc_sunset(
            &self.get_adjusted_date_time(self.get_date_time()),
            self.get_geo_location(),
            zenith,
            false,
        )
    }

    fn get_temporal_hour(&self) -> Option<Duration> {
        let sea_level_sunrise = self.get_sea_level_sunrise()?;
        let sea_level_sunset = self.get_sea_level_sunset()?;
        self.get_temporal_hour_from_times(&sea_level_sunrise, &sea_level_sunset)
    }

    fn get_temporal_hour_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<Duration> {
        Some((end_of_day.clone() - start_of_day) / 12)
    }

    fn get_sun_transit(&self) -> Option<DateTime<Tz>> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time());
        let noon = self
            .get_noaa_calculator()
            .get_utc_noon(&adjusted_date_time, self.get_geo_location());
        if noon.is_nan() {
            return None;
        }
        self.get_date_from_time(noon, _SolarEvent::Noon)
    }

    fn get_solar_midnight(&self) -> Option<DateTime<Tz>> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time());
        let midnight = self
            .get_noaa_calculator()
            .get_utc_midnight(&adjusted_date_time, self.get_geo_location());
        if midnight.is_nan() {
            return None;
        }
        self.get_date_from_time(midnight, _SolarEvent::Midnight)
    }

    fn get_sun_transit_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        let temporal_hour = self.get_temporal_hour_from_times(&start_of_day, &end_of_day)?;
        Some(start_of_day + (temporal_hour * 6))
    }

    fn get_date_from_time(
        &self,
        mut calculated_time: f64,
        solar_event: _SolarEvent,
    ) -> Option<DateTime<Tz>> {
        let adjusted_dt = self.get_adjusted_date_time(self.get_date_time());

        let cal_result = Utc.with_ymd_and_hms(
            adjusted_dt.year(),
            adjusted_dt.month(),
            adjusted_dt.day(),
            0,
            0,
            0,
        );

        let mut cal = match cal_result {
            chrono::LocalResult::Single(dt) => dt,
            _ => return None,
        };

        let hours = calculated_time as i64;
        calculated_time -= hours as f64;

        calculated_time *= 60.0;
        let minutes = calculated_time as i64;
        calculated_time -= minutes as f64;

        calculated_time *= 60.0;
        let seconds = calculated_time as i64;
        calculated_time -= seconds as f64;

        let local_time_hours = (self.get_geo_location().get_longitude() / 15.0) as i64;
        if solar_event == _SolarEvent::Sunrise && local_time_hours + hours > 18 {
            cal -= chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Sunset && local_time_hours + hours < 6 {
            cal += chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Midnight && local_time_hours + hours < 12 {
            cal += chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Noon && local_time_hours + hours > 24 {
            cal -= chrono::Duration::days(1);
        }

        cal = cal.checked_add_signed(
            TimeDelta::hours(hours)
                + TimeDelta::minutes(minutes)
                + TimeDelta::seconds(seconds)
                + TimeDelta::nanoseconds((calculated_time * 1_000_000_000.0) as i64),
        )?;

        Some(adjusted_dt.timezone().from_utc_datetime(&cal.naive_utc()))
    }

    fn get_local_mean_time(&self, hours: f64) -> Option<DateTime<Tz>> {
        if hours < 0.0 || hours >= 24.0 {
            return None;
        }
        let timezone_offset_hours =
            self.date_time.offset().fix().local_minus_utc() as f64 / 60.0 / 60.0;
        let start = self.get_date_from_time(hours - timezone_offset_hours, _SolarEvent::Sunrise)?;
        let offset = self
            .get_geo_location()
            .get_local_mean_time_offset(&self.date_time);
        return Some(start - offset);
    }
}

#[cfg(test)]
mod jni_tests {

    use crate::test_utils::jni::{
        DEFAULT_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64_option,
        assert_almost_equal_i64_option, create_astronomical_calendars, init_jvm,
    };

    use super::*;

    use j4rs::{Instance, InvocationArg, Jvm};
    use rand::Rng;

    fn date_time_tester(
        fn_to_test: impl Fn(&AstronomicalCalendar<chrono_tz::Tz>) -> Option<DateTime<chrono_tz::Tz>>,
        method: &str,
    ) {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;

            let (calendar, java_calendar, message) = test_case.unwrap();

            let result = fn_to_test(&calendar).map(|d| d.timestamp_millis());
            let java_result = jvm
                .invoke(&java_calendar, method, InvocationArg::empty())
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);
            assert_almost_equal_i64_option(&result, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }
    fn i64_with_zenith_tester(
        fn_to_test: impl Fn(
            &AstronomicalCalendar<chrono_tz::Tz>,
            f64,
        ) -> Option<DateTime<chrono_tz::Tz>>,
        method: &str,
    ) {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let zenith = rand::thread_rng().gen_range(-180.0..=180.0);

            let (calendar, java_calendar, message) = test_case.unwrap();

            let result = fn_to_test(&calendar, zenith).map(|d| d.timestamp_millis());
            let java_result = jvm
                .invoke(
                    &java_calendar,
                    method,
                    &[InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);
            assert_almost_equal_i64_option(&result, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }
    fn f64_with_zenith_tester(
        fn_to_test: impl Fn(&AstronomicalCalendar<chrono_tz::Tz>, f64) -> Option<f64>,
        method: &str,
    ) {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let zenith = rand::thread_rng().gen_range(-180.0..=180.0);

            let (calendar, java_calendar, message) = test_case.unwrap();

            let result = fn_to_test(&calendar, zenith);
            let java_result = jvm
                .invoke(
                    &java_calendar,
                    method,
                    &[InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_result = jvm.to_rust::<f64>(java_result).unwrap();
            let java_result = if java_result.is_nan() {
                None
            } else {
                Some(java_result)
            };
            assert_almost_equal_f64_option(&result, &java_result, DEFAULT_TEST_EPSILON, &message);
        }
        assert!(ran, "No test cases were run");
    }
    fn get_java_date_millis(jvm: &Jvm, date_instance: &Instance) -> Option<i64> {
        let millis_result = jvm.invoke(date_instance, "getTime", InvocationArg::empty());
        if millis_result.is_err() {
            return None;
        }
        let millis = jvm.to_rust::<i64>(millis_result.unwrap()).ok()?;
        Some(millis)
    }

    #[test]
    fn test_get_sunrise_against_java() {
        date_time_tester(|calendar| calendar.get_sunrise(), "getSunrise");
    }

    #[test]
    fn test_get_sea_level_sunrise_against_java() {
        date_time_tester(
            |calendar| calendar.get_sea_level_sunrise(),
            "getSeaLevelSunrise",
        );
    }

    #[test]
    fn test_get_begin_civil_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_begin_civil_twilight(),
            "getBeginCivilTwilight",
        );
    }

    #[test]
    fn test_get_begin_nautical_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_begin_nautical_twilight(),
            "getBeginNauticalTwilight",
        );
    }

    #[test]
    fn test_get_begin_astronomical_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_begin_astronomical_twilight(),
            "getBeginAstronomicalTwilight",
        );
    }

    #[test]
    fn test_get_sunset_against_java() {
        date_time_tester(|calendar| calendar.get_sunset(), "getSunset");
    }

    #[test]
    fn test_get_sea_level_sunset_against_java() {
        date_time_tester(
            |calendar| calendar.get_sea_level_sunset(),
            "getSeaLevelSunset",
        );
    }
    #[test]
    fn test_get_end_civil_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_end_civil_twilight(),
            "getEndCivilTwilight",
        );
    }

    #[test]
    fn test_get_end_nautical_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_end_nautical_twilight(),
            "getEndNauticalTwilight",
        );
    }

    #[test]
    fn test_get_end_astronomical_twilight_against_java() {
        date_time_tester(
            |calendar| calendar.get_end_astronomical_twilight(),
            "getEndAstronomicalTwilight",
        );
    }

    #[test]
    fn test_get_sunrise_offset_by_degrees_against_java() {
        i64_with_zenith_tester(
            |calendar, zenith| calendar.get_sunrise_offset_by_degrees(zenith),
            "getSunriseOffsetByDegrees",
        );
    }

    #[test]
    fn test_get_sunset_offset_by_degrees_against_java() {
        i64_with_zenith_tester(
            |calendar, zenith| calendar.get_sunset_offset_by_degrees(zenith),
            "getSunsetOffsetByDegrees",
        );
    }

    #[test]
    fn test_get_utc_sunrise_against_java() {
        f64_with_zenith_tester(
            |calendar, zenith| calendar.get_utc_sunrise(zenith),
            "getUTCSunrise",
        );
    }

    #[test]
    fn test_get_utc_sea_level_sunrise_against_java() {
        f64_with_zenith_tester(
            |calendar, zenith| calendar.get_utc_sea_level_sunrise(zenith),
            "getUTCSeaLevelSunrise",
        );
    }

    #[test]
    fn test_get_utc_sunset_against_java() {
        f64_with_zenith_tester(
            |calendar, zenith| calendar.get_utc_sunset(zenith),
            "getUTCSunset",
        );
    }

    #[test]
    fn test_get_utc_sea_level_sunset_against_java() {
        f64_with_zenith_tester(
            |calendar, zenith| calendar.get_utc_sea_level_sunset(zenith),
            "getUTCSeaLevelSunset",
        );
    }

    #[test]
    fn test_get_temporal_hour_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar, message) = test_case.unwrap();

            let result = calendar.get_temporal_hour();

            let java_result = jvm
                .invoke(&java_calendar, "getTemporalHour", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).ok();
            let java_result = if java_result == Some(-9223372036854775808i64) {
                None
            } else {
                java_result
            };

            let result_millis = result.map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result_millis, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_temporal_hour_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar, message) = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = calendar.get_sea_level_sunrise();
            let end_of_day = calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = calendar.get_temporal_hour_from_times(&start_of_day, &end_of_day);

            // Create Java Date objects
            let java_start = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(start_of_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_end = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(end_of_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();

            let java_result = jvm
                .invoke(
                    &java_calendar,
                    "getTemporalHour",
                    &[
                        InvocationArg::from(java_start),
                        InvocationArg::from(java_end),
                    ],
                )
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).ok();
            let java_result = if java_result == Some(-9223372036854775808i64) {
                None
            } else {
                java_result
            };

            let result_millis = result.map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result_millis, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sun_transit_against_java() {
        date_time_tester(|calendar| calendar.get_sun_transit(), "getSunTransit");
    }

    #[test]
    fn test_get_solar_midnight_against_java() {
        date_time_tester(|calendar| calendar.get_solar_midnight(), "getSolarMidnight");
    }

    #[test]
    fn test_get_sun_transit_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar, message) = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = calendar.get_sea_level_sunrise();
            let end_of_day = calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = calendar
                .get_sun_transit_from_times(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

            // Create Java Date objects
            let java_start = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(start_of_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_end = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(end_of_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();

            let java_result = jvm
                .invoke(
                    &java_calendar,
                    "getSunTransit",
                    &[
                        InvocationArg::from(java_start),
                        InvocationArg::from(java_end),
                    ],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_local_mean_time_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar, message) = test_case.unwrap();

            // Random number between 0.0 and 23.9999...
            let hours = rand::thread_rng().gen_range(-1.0..=25.0);

            let result = calendar
                .get_local_mean_time(hours)
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &java_calendar,
                    "getLocalMeanTime",
                    &[InvocationArg::try_from(hours)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .ok()
                .map(|instance| get_java_date_millis(&jvm, &instance))
                .flatten();

            assert_almost_equal_i64_option(&result, &java_result, 50, &message);
        }
        assert!(ran, "No test cases were run");
    }
}
