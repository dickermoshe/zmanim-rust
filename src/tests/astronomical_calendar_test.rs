use chrono::{DateTime, Duration, TimeZone};
use j4rs::{Instance, InvocationArg, Jvm};

use crate::{
    astronomical_calculator::NOAACalculator,
    astronomical_calendar::AstronomicalCalendarTrait,
    constants::_SolarEvent,
    geolocation::GeoLocation,
    tests::test_utils::{dt_to_java_calendar, dt_to_java_date, geolocation_to_java_geolocation},
};

pub struct JavaAstronomicalCalendar<'a, Tz: TimeZone> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
    pub date_time: DateTime<Tz>,
}

impl<'a, Tz: TimeZone> JavaAstronomicalCalendar<'a, Tz> {
    pub fn new(jvm: &'a Jvm, date_time: DateTime<Tz>, geo_location: GeoLocation) -> Self {
        let java_geolocation = geolocation_to_java_geolocation(jvm, &geo_location, date_time.timezone()).unwrap();
        let java_date_time = dt_to_java_calendar(jvm, &date_time).unwrap();
        let java_astronomical_calendar = jvm
            .create_instance(
                "com.kosherjava.zmanim.AstronomicalCalendar",
                &[InvocationArg::from(java_geolocation)],
            )
            .unwrap();
        jvm.invoke(
            &java_astronomical_calendar,
            "setCalendar",
            &[InvocationArg::from(java_date_time)],
        )
        .unwrap();

        Self {
            jvm,
            instance: java_astronomical_calendar,
            date_time: date_time,
        }
    }

    fn get_java_date_millis(&self, date_instance: &Instance) -> Option<i64> {
        let millis_result = self.jvm.invoke(date_instance, "getTime", InvocationArg::empty());
        if millis_result.is_err() {
            return None;
        }
        let millis = self.jvm.to_rust::<i64>(millis_result.unwrap()).ok()?;
        Some(millis)
    }

    fn java_date_to_rust_datetime(&self, date_instance: &Instance) -> Option<DateTime<Tz>> {
        let millis = self.get_java_date_millis(date_instance)?;
        Some(self.date_time.timezone().timestamp_millis_opt(millis).unwrap())
    }
}

impl<'a, Tz: TimeZone> AstronomicalCalendarTrait<Tz, GeoLocation, NOAACalculator> for JavaAstronomicalCalendar<'a, Tz> {
    // These methods are not used in the tests, but we need to implement them for the trait.
    fn get_date_time(&self) -> &DateTime<Tz> {
        todo!()
    }
    // These methods are not used in the tests, but we need to implement them for the trait.
    fn get_geo_location(&self) -> &GeoLocation {
        todo!()
    }
    // These methods are not used in the tests, but we need to implement them for the trait.
    fn get_noaa_calculator(&self) -> &NOAACalculator {
        todo!()
    }
    // These methods are not used in the tests, but we need to implement them for the trait.
    fn get_date_from_time(&self, _calculated_time: f64, _solar_event: _SolarEvent) -> Option<DateTime<Tz>> {
        todo!()
    }

    fn get_sunrise(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSunrise", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSeaLevelSunrise", InvocationArg::empty())
            .ok()?;

        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_begin_civil_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getBeginCivilTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_begin_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getBeginNauticalTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_begin_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getBeginAstronomicalTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sunset(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSunset", InvocationArg::empty())
            .ok()?;

        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSeaLevelSunset", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_end_civil_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getEndCivilTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_end_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getEndNauticalTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_end_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getEndAstronomicalTwilight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sunrise_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSunriseOffsetByDegrees",
                &[InvocationArg::try_from(offset_zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSunsetOffsetByDegrees",
                &[InvocationArg::try_from(offset_zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunrise",
                &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
            )
            .ok()?;
        let result = self.jvm.to_rust::<f64>(java_result).ok()?;
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSeaLevelSunrise",
                &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
            )
            .ok()?;
        let result = self.jvm.to_rust::<f64>(java_result).ok()?;
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_utc_sunset(&self, zenith: f64) -> Option<f64> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunset",
                &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
            )
            .ok()?;
        let result = self.jvm.to_rust::<f64>(java_result).ok()?;
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSeaLevelSunset",
                &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
            )
            .ok()?;
        let result = self.jvm.to_rust::<f64>(java_result).ok()?;
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_temporal_hour(&self) -> Option<Duration> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getTemporalHour", InvocationArg::empty())
            .ok()?;
        let millis = self.jvm.to_rust::<i64>(java_result).ok()?;
        // DIFF: Java returns Long.MIN_VALUE (-9223372036854775808) to indicate null/None
        if millis == -9223372036854775808i64 {
            None
        } else {
            Some(Duration::milliseconds(millis))
        }
    }

    fn get_temporal_hour_from_times(&self, start_of_day: &DateTime<Tz>, end_of_day: &DateTime<Tz>) -> Option<Duration> {
        let java_start = self
            .jvm
            .create_instance(
                "java.util.Date",
                &[InvocationArg::try_from(start_of_day.timestamp_millis())
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .ok()?;
        let java_end = self
            .jvm
            .create_instance(
                "java.util.Date",
                &[InvocationArg::try_from(end_of_day.timestamp_millis())
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .ok()?;
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getTemporalHour",
                &[InvocationArg::from(java_start), InvocationArg::from(java_end)],
            )
            .ok()?;
        let millis = self.jvm.to_rust::<i64>(java_result).ok()?;
        // DIFF: Java returns Long.MIN_VALUE (-9223372036854775808) to indicate null/None
        if millis == -9223372036854775808i64 {
            None
        } else {
            Some(Duration::milliseconds(millis))
        }
    }

    fn get_sun_transit(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSunTransit", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_solar_midnight(&self) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSolarMidnight", InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sun_transit_from_times(&self, start_of_day: DateTime<Tz>, end_of_day: DateTime<Tz>) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, &start_of_day);
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSunTransit",
                &[InvocationArg::from(java_start), InvocationArg::from(java_end)],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_local_mean_time(&self, hours: f64) -> Option<DateTime<Tz>> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getLocalMeanTime",
                &[InvocationArg::try_from(hours).unwrap().into_primitive().unwrap()],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }
}

#[cfg(test)]
mod jni_tests {

    use rand::Rng;

    use crate::{
        astronomical_calendar::AstronomicalCalendar,
        tests::{
            geolocation_test::random_geolocations,
            test_utils::{
                DEFAULT_F64_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64_option,
                assert_almost_equal_i64_option, init_jvm, random_date_time, random_zenith,
            },
        },
    };

    use super::*;

    fn create_astronomical_calendars<'a>(
        jvm: &'a Jvm,
    ) -> Option<(
        AstronomicalCalendar<chrono_tz::Tz>,
        JavaAstronomicalCalendar<'a, chrono_tz::Tz>,
    )> {
        let mut rng = rand::thread_rng();
        let (geo_location, java_geo_location) = random_geolocations(jvm, &mut rng)?;

        let date_time = random_date_time(&mut rng, java_geo_location.timezone);

        let rust_calendar = AstronomicalCalendar::new(date_time, geo_location.clone());
        let java_calendar = JavaAstronomicalCalendar::new(jvm, date_time, geo_location);

        Some((rust_calendar, java_calendar))
    }

    fn get_java_date_millis(jvm: &Jvm, date_instance: &Instance) -> Option<i64> {
        let millis_result = jvm.invoke(date_instance, "getTime", InvocationArg::empty());
        if millis_result.is_err() {
            return None;
        }
        let millis = jvm.to_rust::<i64>(millis_result.unwrap()).ok()?;
        Some(millis)
    }

    // A helper function to test a function that returns a DateTime and takes no arguments
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

            let (calendar, java_calendar) = test_case.unwrap();

            let result = fn_to_test(&calendar).map(|d| d.timestamp_millis());
            let java_result = jvm
                .invoke(&java_calendar.instance, method, InvocationArg::empty())
                .ok()
                .and_then(|instance| get_java_date_millis(&jvm, &instance));
            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50, // Allow for a 50ms difference
                &format!("{} against java {:?}", method, calendar),
            );
        }
        assert!(ran, "No test cases were run");
    }

    // A helper function to test a function that returns a DateTime and takes a zenith as an argument
    fn i64_with_zenith_tester(
        fn_to_test: impl Fn(&AstronomicalCalendar<chrono_tz::Tz>, f64) -> Option<DateTime<chrono_tz::Tz>>,
        method: &str,
    ) {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let zenith = random_zenith(&mut rng);

            let (calendar, java_calendar) = test_case.unwrap();

            let result = fn_to_test(&calendar, zenith).map(|d| d.timestamp_millis());
            let java_result = jvm
                .invoke(
                    &java_calendar.instance,
                    method,
                    &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
                )
                .ok()
                .and_then(|instance| get_java_date_millis(&jvm, &instance));
            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50, // Allow for a 50ms difference
                &format!("{} against java {:?} with zenith {}", method, calendar, zenith),
            );
        }
        assert!(ran, "No test cases were run");
    }

    // A helper function to test a function that returns a f64 and takes a zenith as an argument
    fn f64_with_zenith_tester(
        fn_to_test: impl Fn(&AstronomicalCalendar<chrono_tz::Tz>, f64) -> Option<f64>,
        method: &str,
    ) {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let zenith = random_zenith(&mut rng);

            let (calendar, java_calendar) = test_case.unwrap();

            let result = fn_to_test(&calendar, zenith);
            let java_result = jvm
                .invoke(
                    &java_calendar.instance,
                    method,
                    &[InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap()],
                )
                .ok()
                .and_then(|instance| jvm.to_rust::<f64>(instance).ok())
                .map(|result| if result.is_nan() { None } else { Some(result) })
                .flatten();
            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_F64_TEST_EPSILON,
                &format!("{} against java {:?} with zenith {}", method, calendar, zenith),
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sunrise_against_java() {
        date_time_tester(|calendar| calendar.get_sunrise(), "getSunrise");
    }

    #[test]
    fn test_get_sea_level_sunrise_against_java() {
        date_time_tester(|calendar| calendar.get_sea_level_sunrise(), "getSeaLevelSunrise");
    }

    #[test]
    fn test_get_begin_civil_twilight_against_java() {
        date_time_tester(|calendar| calendar.get_begin_civil_twilight(), "getBeginCivilTwilight");
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
        date_time_tester(|calendar| calendar.get_sea_level_sunset(), "getSeaLevelSunset");
    }

    #[test]
    fn test_get_end_civil_twilight_against_java() {
        date_time_tester(|calendar| calendar.get_end_civil_twilight(), "getEndCivilTwilight");
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
        f64_with_zenith_tester(|calendar, zenith| calendar.get_utc_sunrise(zenith), "getUTCSunrise");
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
        f64_with_zenith_tester(|calendar, zenith| calendar.get_utc_sunset(zenith), "getUTCSunset");
    }

    #[test]
    fn test_get_utc_sea_level_sunset_against_java() {
        f64_with_zenith_tester(
            |calendar, zenith| calendar.get_utc_sea_level_sunset(zenith),
            "getUTCSeaLevelSunset",
        );
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
    fn test_get_temporal_hour_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar) = test_case.unwrap();

            let result = calendar.get_temporal_hour().map(|d| d.num_milliseconds());
            let java_result = java_calendar.get_temporal_hour().map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50, // Allow for a 50ms difference
                &format!("{} against java {:?}", "getTemporalHour", calendar),
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_temporal_hour_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar) = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = calendar.get_sea_level_sunrise();
            let end_of_day = calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            // Apply a random offset to the start and end of the day
            let start_of_day = start_of_day + Duration::seconds(rng.gen_range(-(24 * 3600)..=(12 * 3600))); // Random offset between -24 hours and +12 hours
            let end_of_day = end_of_day + Duration::seconds(rng.gen_range(-(12 * 3600)..=(24 * 3600))); // Random offset between -12 hours and +24 hours

            let result = calendar
                .get_temporal_hour_from_times(&start_of_day, &end_of_day)
                .map(|d| d.num_milliseconds());
            let java_result = java_calendar
                .get_temporal_hour_from_times(&start_of_day, &end_of_day)
                .map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50, // Allow for a 50ms difference
                &format!("{} against java {:?}", "getTemporalHourFromTimes", calendar),
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sun_transit_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        let mut rng = rand::thread_rng();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_astronomical_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (calendar, java_calendar) = test_case.unwrap();

            // Get sunrise and sunset for test
            let start_of_day = calendar.get_sea_level_sunrise();
            let end_of_day = calendar.get_sea_level_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            // Apply a random offset to the start and end of the day
            let start_of_day = start_of_day + Duration::seconds(rng.gen_range(-(24 * 3600)..=(12 * 3600))); // Random offset between -24 hours and +12 hours
            let end_of_day = end_of_day + Duration::seconds(rng.gen_range(-(12 * 3600)..=(24 * 3600))); // Random offset between -12 hours and +24 hours

            let result = calendar
                .get_sun_transit_from_times(start_of_day, end_of_day)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_sun_transit_from_times(start_of_day, end_of_day)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                &format!("{} against java {:?}", "getSunTransitFromTimes", calendar),
            );
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
            let (calendar, java_calendar) = test_case.unwrap();

            // Random number between -1.0 and 25.0
            let hours = rand::thread_rng().gen_range(-25.0..=25.0);

            let result = calendar.get_local_mean_time(hours).map(|d| d.timestamp_millis());

            let java_result = java_calendar.get_local_mean_time(hours).map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                &format!("{} against java {:?}", "getLocalMeanTime", calendar),
            );
        }
        assert!(ran, "No test cases were run");
    }
}
