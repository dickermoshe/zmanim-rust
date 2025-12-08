use chrono::{DateTime, Duration, TimeZone};
use j4rs::{Instance, InvocationArg, Jvm};

use crate::{
    astronomical_calculator::NOAACalculator,
    astronomical_calendar::AstronomicalCalendarTrait,
    constants::_SolarEvent,
    defmt::DefmtFormatTrait,
    geolocation::GeoLocation,
    tests::{dt_to_java_calendar, dt_to_java_date, geolocation_to_java_geolocation},
};

pub struct JavaAstronomicalCalendar<'a, Tz: TimeZone> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
    pub date_time: DateTime<Tz>,
}

impl<'a, Tz: TimeZone> DefmtFormatTrait for JavaAstronomicalCalendar<'a, Tz> {}

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
            date_time,
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
    fn get_calculator(&self) -> &NOAACalculator {
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

    fn get_sun_transit_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, start_of_day);
        let java_end = dt_to_java_date(self.jvm, end_of_day);
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
            DEFAULT_TEST_ITERATIONS, geolocation_test::random_geolocations, init_jvm, random_date_time, random_zenith,
        },
    };

    use super::*;

    fn create_astronomical_calendars<'a>(
        jvm: &'a Jvm,
    ) -> Option<(
        AstronomicalCalendar<chrono_tz::Tz, GeoLocation, NOAACalculator>,
        JavaAstronomicalCalendar<'a, chrono_tz::Tz>,
    )> {
        let mut rng = rand::thread_rng();
        let (geo_location, java_geo_location) = random_geolocations(jvm, &mut rng)?;

        let date_time = random_date_time(&mut rng, java_geo_location.timezone);

        let rust_calendar = AstronomicalCalendar::new(date_time, geo_location.clone(), NOAACalculator);
        let java_calendar = JavaAstronomicalCalendar::new(jvm, date_time, geo_location);

        Some((rust_calendar, java_calendar))
    }

    fn compare_astronomical_calendars<'a>(
        rust_calendar: &AstronomicalCalendar<chrono_tz::Tz, GeoLocation, NOAACalculator>,
        java_calendar: &JavaAstronomicalCalendar<'a, chrono_tz::Tz>,
        offset_zenith: f64,
        zenith: f64,
        hours: f64,
        start_of_day: &DateTime<chrono_tz::Tz>,
        end_of_day: &DateTime<chrono_tz::Tz>,
    ) {
        let rust_sunrise = rust_calendar.get_sunrise();
        let java_sunrise = java_calendar.get_sunrise();
        assert_eq!(rust_sunrise, java_sunrise, "getSunrise using {:?} ", rust_calendar);

        let rust_sea_level_sunrise = rust_calendar.get_sea_level_sunrise();
        let java_sea_level_sunrise = java_calendar.get_sea_level_sunrise();
        assert_eq!(
            rust_sea_level_sunrise, java_sea_level_sunrise,
            "getSeaLevelSunrise using {:?} ",
            rust_calendar
        );

        let rust_begin_civil_twilight = rust_calendar.get_begin_civil_twilight();
        let java_begin_civil_twilight = java_calendar.get_begin_civil_twilight();
        assert_eq!(
            rust_begin_civil_twilight, java_begin_civil_twilight,
            "getBeginCivilTwilight using {:?} ",
            rust_calendar
        );

        let rust_begin_nautical_twilight = rust_calendar.get_begin_nautical_twilight();
        let java_begin_nautical_twilight = java_calendar.get_begin_nautical_twilight();
        assert_eq!(
            rust_begin_nautical_twilight, java_begin_nautical_twilight,
            "getBeginNauticalTwilight using {:?} ",
            rust_calendar
        );

        let rust_begin_astronomical_twilight = rust_calendar.get_begin_astronomical_twilight();
        let java_begin_astronomical_twilight = java_calendar.get_begin_astronomical_twilight();
        assert_eq!(
            rust_begin_astronomical_twilight, java_begin_astronomical_twilight,
            "getBeginAstronomicalTwilight using {:?} ",
            rust_calendar
        );

        let rust_sunset = rust_calendar.get_sunset();
        let java_sunset = java_calendar.get_sunset();
        assert_eq!(rust_sunset, java_sunset, "getSunset using {:?} ", rust_calendar);

        let rust_sea_level_sunset = rust_calendar.get_sea_level_sunset();
        let java_sea_level_sunset = java_calendar.get_sea_level_sunset();
        assert_eq!(
            rust_sea_level_sunset, java_sea_level_sunset,
            "getSeaLevelSunset using {:?} ",
            rust_calendar
        );

        let rust_end_civil_twilight = rust_calendar.get_end_civil_twilight();
        let java_end_civil_twilight = java_calendar.get_end_civil_twilight();
        assert_eq!(
            rust_end_civil_twilight, java_end_civil_twilight,
            "getEndCivilTwilight using {:?} ",
            rust_calendar
        );

        let rust_end_nautical_twilight = rust_calendar.get_end_nautical_twilight();
        let java_end_nautical_twilight = java_calendar.get_end_nautical_twilight();
        assert_eq!(
            rust_end_nautical_twilight, java_end_nautical_twilight,
            "getEndNauticalTwilight using {:?} ",
            rust_calendar
        );

        let rust_end_astronomical_twilight = rust_calendar.get_end_astronomical_twilight();
        let java_end_astronomical_twilight = java_calendar.get_end_astronomical_twilight();
        assert_eq!(
            rust_end_astronomical_twilight, java_end_astronomical_twilight,
            "getEndAstronomicalTwilight using {:?} ",
            rust_calendar
        );

        let rust_sunrise_offset_by_degrees = rust_calendar.get_sunrise_offset_by_degrees(offset_zenith);
        let java_sunrise_offset_by_degrees = java_calendar.get_sunrise_offset_by_degrees(offset_zenith);
        assert_eq!(
            rust_sunrise_offset_by_degrees, java_sunrise_offset_by_degrees,
            "getSunriseOffsetByDegrees using {:?} ",
            rust_calendar
        );

        let rust_sunset_offset_by_degrees = rust_calendar.get_sunset_offset_by_degrees(offset_zenith);
        let java_sunset_offset_by_degrees = java_calendar.get_sunset_offset_by_degrees(offset_zenith);
        assert_eq!(
            rust_sunset_offset_by_degrees, java_sunset_offset_by_degrees,
            "getSunsetOffsetByDegrees using {:?} ",
            rust_calendar
        );

        let rust_utc_sunrise = rust_calendar.get_utc_sunrise(zenith);
        let java_utc_sunrise = java_calendar.get_utc_sunrise(zenith);
        assert_eq!(
            rust_utc_sunrise, java_utc_sunrise,
            "getUTCSunrise using {:?} ",
            rust_calendar
        );

        let rust_utc_sea_level_sunrise = rust_calendar.get_utc_sea_level_sunrise(zenith);
        let java_utc_sea_level_sunrise = java_calendar.get_utc_sea_level_sunrise(zenith);
        assert_eq!(
            rust_utc_sea_level_sunrise, java_utc_sea_level_sunrise,
            "getUTCSeaLevelSunrise using {:?} ",
            rust_calendar
        );

        let rust_utc_sunset = rust_calendar.get_utc_sunset(zenith);
        let java_utc_sunset = java_calendar.get_utc_sunset(zenith);
        assert_eq!(
            rust_utc_sunset, java_utc_sunset,
            "getUTCSunset using {:?} ",
            rust_calendar
        );

        let rust_utc_sea_level_sunset = rust_calendar.get_utc_sea_level_sunset(zenith);
        let java_utc_sea_level_sunset = java_calendar.get_utc_sea_level_sunset(zenith);
        assert_eq!(
            rust_utc_sea_level_sunset, java_utc_sea_level_sunset,
            "getUTCSeaLevelSunset using {:?} ",
            rust_calendar
        );

        let rust_temporal_hour = rust_calendar.get_temporal_hour();
        let java_temporal_hour = java_calendar.get_temporal_hour();
        assert_eq!(
            rust_temporal_hour, java_temporal_hour,
            "getTemporalHour using {:?} ",
            rust_calendar
        );

        let rust_temporal_hour_from_times = rust_calendar.get_temporal_hour_from_times(start_of_day, end_of_day);
        let java_temporal_hour_from_times = java_calendar.get_temporal_hour_from_times(start_of_day, end_of_day);
        assert_eq!(
            rust_temporal_hour_from_times, java_temporal_hour_from_times,
            "getTemporalHourFromTimes using {:?} ",
            rust_calendar
        );

        let rust_sun_transit = rust_calendar.get_sun_transit();
        let java_sun_transit = java_calendar.get_sun_transit();
        assert_eq!(
            rust_sun_transit, java_sun_transit,
            "getSunTransit using {:?} ",
            rust_calendar
        );

        let rust_sun_transit_from_times = rust_calendar.get_sun_transit_from_times(start_of_day, end_of_day);
        let java_sun_transit_from_times = java_calendar.get_sun_transit_from_times(start_of_day, end_of_day);
        assert_eq!(
            rust_sun_transit_from_times, java_sun_transit_from_times,
            "getSunTransitFromTimes using {:?} ",
            rust_calendar
        );

        let rust_solar_midnight = rust_calendar.get_solar_midnight();
        let java_solar_midnight = java_calendar.get_solar_midnight();
        assert_eq!(
            rust_solar_midnight, java_solar_midnight,
            "getSolarMidnight using {:?} ",
            rust_calendar
        );

        let rust_local_mean_time = rust_calendar.get_local_mean_time(hours);
        let java_local_mean_time = java_calendar.get_local_mean_time(hours);
        assert_eq!(
            rust_local_mean_time, java_local_mean_time,
            "getLocalMeanTime using {:?} ",
            rust_calendar
        );
    }

    #[test]
    fn test_random_astronomical_calendar_against_java() {
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
            let offset_zenith = random_zenith(&mut rng);
            let zenith = random_zenith(&mut rng);

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

            let hours = rand::thread_rng().gen_range(-25.0..=25.0);

            compare_astronomical_calendars(
                &calendar,
                &java_calendar,
                offset_zenith,
                zenith,
                hours,
                &start_of_day,
                &end_of_day,
            );
        }
        assert!(ran, "No test cases were run");
    }
}
