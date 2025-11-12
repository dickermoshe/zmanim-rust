use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

use crate::{constants::*, geolocation::GeoLocation, noaa_calculator::NOAACalculator};
/// TODO ADD DOCS
pub struct AstronomicalCalendar<Tz: TimeZone> {
    /// TODO ADD DOCS
    pub date_time: DateTime<Tz>,
    /// TODO ADD DOCS
    pub geo_location: GeoLocation,
    /// TODO ADD DOCS
    pub noaa_calculator: NOAACalculator,
}

impl<Tz: TimeZone> AstronomicalCalendar<Tz> {
    pub fn new(
        date_time: DateTime<Tz>,
        geo_location: GeoLocation,
        noaa_calculator: NOAACalculator,
    ) -> Self {
        Self {
            date_time,
            geo_location,
            noaa_calculator,
        }
    }
}

pub trait AstronomicalCalendarTrait<Tz: TimeZone> {
    fn get_date_time(&self) -> &DateTime<Tz>;
    fn get_geo_location(&self) -> &GeoLocation;
    fn get_noaa_calculator(&self) -> &NOAACalculator;

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
        // println!("Rsunset: {:?}", result);
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
    fn get_adjusted_date_time(&self, date_time: &DateTime<Tz>) -> DateTime<Tz> {
        let offset = self
            .get_geo_location()
            .get_antimeridian_adjustment(&date_time);

        if offset == 0 {
            return date_time.clone();
        }
        let adjusted_date_time = if offset > 0 {
            date_time.clone() + Duration::days(offset as i64)
        } else {
            date_time.clone() - Duration::days(offset as i64)
        };
        adjusted_date_time
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

        let local_time_hours = (self.get_geo_location().longitude / 15.0) as i64;
        if solar_event == _SolarEvent::Sunrise && local_time_hours + hours > 18 {
            cal -= chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Sunset && local_time_hours + hours < 6 {
            cal += chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Midnight && local_time_hours + hours < 12 {
            cal += chrono::Duration::days(1);
        } else if solar_event == _SolarEvent::Noon && local_time_hours + hours > 24 {
            cal -= chrono::Duration::days(1);
        }

        cal = cal
            .with_hour(hours as u32)
            .and_then(|dt| dt.with_minute(minutes as u32))
            .and_then(|dt| dt.with_second(seconds as u32))
            .and_then(|dt| dt.with_nanosecond((calculated_time * 1_000_000_000.0) as u32))?;

        Some(adjusted_dt.timezone().from_utc_datetime(&cal.naive_utc()))
    }
}

impl<Tz: TimeZone> AstronomicalCalendarTrait<Tz> for AstronomicalCalendar<Tz> {
    fn get_date_time(&self) -> &DateTime<Tz> {
        &self.date_time
    }

    fn get_geo_location(&self) -> &GeoLocation {
        &self.geo_location
    }

    fn get_noaa_calculator(&self) -> &NOAACalculator {
        &self.noaa_calculator
    }
}

#[cfg(test)]
mod jni_tests {

    use crate::test_utils::jni::{
        DEFAULT_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, RandomGeoLocation,
        assert_almost_equal_f64_option, assert_almost_equal_i64_option, create_java_calendar,
        create_java_geo_location, init_jvm, random_date_time,
    };

    use super::*;

    use j4rs::{Instance, InvocationArg, Jvm};
    use rand::Rng;

    struct TestCase {
        java_calendar: Instance,
        calendar: AstronomicalCalendar<chrono_tz::Tz>,
        message: String,
        zenith: f64,
    }

    fn get_java_date_millis(jvm: &Jvm, date_instance: &Instance) -> Option<i64> {
        let millis_result = jvm.invoke(date_instance, "getTime", InvocationArg::empty());
        if millis_result.is_err() {
            return None;
        }
        let millis = jvm.to_rust::<i64>(millis_result.unwrap()).ok()?;
        Some(millis)
    }

    fn create_test_case(jvm: &Jvm) -> Option<TestCase> {
        let (date_time, timezone_id) = random_date_time();
        let random_geo_location = RandomGeoLocation::new();
        // let date_time = chrono_tz::Asia::Tehran
        //     .timestamp_millis_opt(283378756156)
        //     .unwrap();
        // let timezone_id = "Asia/Tehran";
        // let random_geo_location =
        //     GeoLocation::new(23.44664578436729, 23.44664578436729, 232.63640622976277).unwrap();

        let geo_location = GeoLocation::new(
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation,
        );

        let java_calendar = create_java_calendar(&jvm, date_time.timestamp_millis(), timezone_id)?;

        let java_geo_location = create_java_geo_location(
            &jvm,
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation,
            timezone_id,
        );
        let message = format!(
            "Latitude: {}, Longitude: {}, Elevation: {}, Timezone: {}, DateTime: {}",
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation,
            timezone_id,
            date_time.timestamp_millis()
        );

        assert_eq!(
            geo_location.is_some(),
            java_geo_location.is_some(),
            "Failed to create test case for {}",
            message
        );
        if geo_location.is_none() {
            return None;
        }
        let zenith = rand::thread_rng().gen_range(-180.0..=180.0);

        let astronomical_calendar = AstronomicalCalendar {
            date_time,
            geo_location: geo_location.unwrap(),
            noaa_calculator: NOAACalculator,
        };

        let java_astronomical_calendar = jvm
            .create_instance(
                "com.kosherjava.zmanim.AstronomicalCalendar",
                &[InvocationArg::from(java_geo_location.unwrap())],
            )
            .unwrap();
        jvm.invoke(
            &java_astronomical_calendar,
            "setCalendar",
            &[InvocationArg::from(java_calendar)],
        )
        .unwrap();

        Some(TestCase {
            calendar: astronomical_calendar,
            java_calendar: java_astronomical_calendar,
            message,
            zenith,
        })
    }

    #[test]
    fn test_get_sunrise_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;

            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sunrise()
                .map(|d| d.timestamp_millis());
            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSunrise",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sea_level_sunrise_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sea_level_sunrise()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSeaLevelSunrise",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_begin_civil_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;

            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_begin_civil_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getBeginCivilTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_begin_nautical_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_begin_nautical_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getBeginNauticalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_begin_astronomical_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_begin_astronomical_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getBeginAstronomicalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sunset_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sunset()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSunset",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sea_level_sunset_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sea_level_sunset()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSeaLevelSunset",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_end_civil_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_end_civil_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getEndCivilTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_end_nautical_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_end_nautical_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getEndNauticalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_end_astronomical_twilight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_end_astronomical_twilight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getEndAstronomicalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sunrise_offset_by_degrees_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sunrise_offset_by_degrees(test_case.zenith)
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSunriseOffsetByDegrees",
                    &[InvocationArg::try_from(test_case.zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sunset_offset_by_degrees_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sunset_offset_by_degrees(test_case.zenith)
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSunsetOffsetByDegrees",
                    &[InvocationArg::try_from(test_case.zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_utc_sunrise_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_utc_sunrise(test_case.zenith);

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getUTCSunrise",
                    &[InvocationArg::try_from(test_case.zenith)
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

            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_TEST_EPSILON,
                &test_case.message,
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_utc_sea_level_sunrise_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_utc_sea_level_sunrise(test_case.zenith);

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getUTCSeaLevelSunrise",
                    &[InvocationArg::try_from(test_case.zenith)
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

            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_TEST_EPSILON,
                &test_case.message,
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_utc_sunset_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_utc_sunset(test_case.zenith);

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getUTCSunset",
                    &[InvocationArg::try_from(test_case.zenith)
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

            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_TEST_EPSILON,
                &test_case.message,
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_utc_sea_level_sunset_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_utc_sea_level_sunset(test_case.zenith);

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getUTCSeaLevelSunset",
                    &[InvocationArg::try_from(test_case.zenith)
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

            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_TEST_EPSILON,
                &test_case.message,
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_temporal_hour_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_temporal_hour();

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getTemporalHour",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).ok();
            let java_result = if java_result == Some(-9223372036854775808i64) {
                None
            } else {
                java_result
            };

            let result_millis = result.map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result_millis, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_temporal_hour_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = test_case.calendar.get_sea_level_sunrise();
            let end_of_day = test_case.calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_temporal_hour_from_times(&start_of_day, &end_of_day);

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
                    &test_case.java_calendar,
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

            assert_almost_equal_i64_option(&result_millis, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sun_transit_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_sun_transit()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSunTransit",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_solar_midnight_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case
                .calendar
                .get_solar_midnight()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_calendar,
                    "getSolarMidnight",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sun_transit_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = test_case.calendar.get_sea_level_sunrise();
            let end_of_day = test_case.calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
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
                    &test_case.java_calendar,
                    "getSunTransit",
                    &[
                        InvocationArg::from(java_start),
                        InvocationArg::from(java_end),
                    ],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }
}
