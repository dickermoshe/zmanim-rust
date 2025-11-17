use crate::constants::*;
use chrono::{DateTime, Datelike, TimeZone, Timelike};
use core::f64::consts::PI;
use libm::{acos, asin, cos, floor, fmod, sin, tan};
/// TODO ADD DOCS
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NOAACalculator;

impl NOAACalculator {
    fn _get_elevation_adjustment(&self, elevation_meters: f64) -> f64 {
        acos(_EARTH_RADIUS / (_EARTH_RADIUS + (elevation_meters / 1000.0))).to_degrees()
    }

    pub fn _get_julian_day<Tz: TimeZone>(&self, date_time: &DateTime<Tz>) -> f64 {
        let mut year = date_time.year();
        let mut month: u8 = date_time.month() as u8;
        let day: i64 = date_time.day() as i64;
        if month <= 2 {
            year -= 1;
            month += 12;
        }
        let a = year / 100;
        let b = 2 - a + a / 4;

        floor(365.25 * (year + 4716) as f64)
            + floor(30.6001 * (month + 1) as f64)
            + day as f64
            + b as f64
            - 1524.5
    }

    fn _adjust_zenith(&self, zenith: f64, elevation: f64) -> f64 {
        let mut adjusted_zenith = zenith;
        if zenith == _GEOMETRIC_ZENITH {
            adjusted_zenith =
                zenith + (_SOLAR_RADIUS + _REFRACTION + self._get_elevation_adjustment(elevation));
        }
        adjusted_zenith
    }

    fn _get_julian_centuries_from_julian_day(&self, julian_day: f64) -> f64 {
        (julian_day - _JULIAN_DAY_JAN_1_2000) / _JULIAN_DAYS_PER_CENTURY
    }

    fn _get_sun_geometric_mean_longitude(&self, julian_centuries: f64) -> f64 {
        let longitude = 280.46646 + julian_centuries * (36000.76983 + 0.0003032 * julian_centuries);

        let mut r = fmod(longitude, 360.0);
        if r < 0.0 {
            r += 360.0;
        }
        r
    }

    fn _get_sun_geometric_mean_anomaly(&self, julian_centuries: f64) -> f64 {
        357.52911 + julian_centuries * (35999.05029 - 0.0001537 * julian_centuries)
    }

    fn _get_earth_orbit_eccentricity(&self, julian_centuries: f64) -> f64 {
        0.016708634 - julian_centuries * (0.000042037 + 0.0000001267 * julian_centuries)
    }

    fn _get_sun_equation_of_center(&self, julian_centuries: f64) -> f64 {
        let m = self._get_sun_geometric_mean_anomaly(julian_centuries);
        let m_rad = m.to_radians();
        let sin_m = sin(m_rad);
        let sin_2m = sin(m_rad * 2.0);
        let sin_3m = sin(m_rad * 3.0);

        sin_m * (1.914602 - julian_centuries * (0.004817 + 0.000014 * julian_centuries))
            + sin_2m * (0.019993 - 0.000101 * julian_centuries)
            + sin_3m * 0.000289
    }

    fn _get_sun_true_longitude(&self, julian_centuries: f64) -> f64 {
        let sun_longitude = self._get_sun_geometric_mean_longitude(julian_centuries);
        let center = self._get_sun_equation_of_center(julian_centuries);
        sun_longitude + center
    }

    fn _get_sun_apparent_longitude(&self, julian_centuries: f64) -> f64 {
        let sun_true_longitude = self._get_sun_true_longitude(julian_centuries);
        let omega = 125.04 - 1934.136 * julian_centuries;
        sun_true_longitude - 0.00569 - 0.00478 * sin(omega.to_radians())
    }

    fn _get_mean_obliquity_of_ecliptic(&self, julian_centuries: f64) -> f64 {
        let seconds = 21.448
            - julian_centuries
                * (46.8150 + julian_centuries * (0.00059 - julian_centuries * (0.001813)));
        23.0 + (26.0 + (seconds / 60.0)) / 60.0
    }

    fn _get_obliquity_correction(&self, julian_centuries: f64) -> f64 {
        let obliquity_of_ecliptic = self._get_mean_obliquity_of_ecliptic(julian_centuries);
        let omega = 125.04 - 1934.136 * julian_centuries;
        obliquity_of_ecliptic + 0.00256 * cos(omega.to_radians())
    }

    fn _get_sun_declination(&self, julian_centuries: f64) -> f64 {
        let obliquity_correction = self._get_obliquity_correction(julian_centuries);
        let lambda = self._get_sun_apparent_longitude(julian_centuries);
        let sin_t = sin(obliquity_correction.to_radians()) * sin(lambda.to_radians());
        asin(sin_t).to_degrees()
    }

    fn _get_equation_of_time(&self, julian_centuries: f64) -> f64 {
        let epsilon = self._get_obliquity_correction(julian_centuries);
        let geom_mean_long_sun = self._get_sun_geometric_mean_longitude(julian_centuries);
        let eccentricity_earth_orbit = self._get_earth_orbit_eccentricity(julian_centuries);
        let geom_mean_anomaly_sun = self._get_sun_geometric_mean_anomaly(julian_centuries);

        let mut y = tan(epsilon.to_radians() / 2.0);
        y *= y;

        let sin_2l0 = sin(2.0 * geom_mean_long_sun.to_radians());
        let sin_m = sin(geom_mean_anomaly_sun.to_radians());
        let cos_2l0 = cos(2.0 * geom_mean_long_sun.to_radians());
        let sin_4l0 = sin(4.0 * geom_mean_long_sun.to_radians());
        let sin_2m = sin(2.0 * geom_mean_anomaly_sun.to_radians());

        let equation_of_time = y * sin_2l0 - 2.0 * eccentricity_earth_orbit * sin_m
            + 4.0 * eccentricity_earth_orbit * y * sin_m * cos_2l0
            - 0.5 * y * y * sin_4l0
            - 1.25 * eccentricity_earth_orbit * eccentricity_earth_orbit * sin_2m;

        equation_of_time.to_degrees() * 4.0
    }

    fn _get_sun_rise_set_utc<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        latitude: f64,
        longitude: f64,
        zenith: f64,
        solar_event: _SolarEvent,
    ) -> f64 {
        let julian_day = self._get_julian_day(date_time);
        // println!("Rjulian_day: {:?}", julian_day);

        let noonmin = self._get_solar_noon_midnight_utc(julian_day, longitude, _SolarEvent::Noon);
        let tnoon = self._get_julian_centuries_from_julian_day(julian_day + noonmin / 1440.0);

        let mut equation_of_time = self._get_equation_of_time(tnoon);
        let mut solar_declination = self._get_sun_declination(tnoon);
        let mut hour_angle =
            self._get_sun_hour_angle(latitude, solar_declination, zenith, solar_event);
        let mut delta = longitude - hour_angle.to_degrees();
        let mut time_diff = 4.0 * delta;
        let mut time_utc = 720.0 + time_diff - equation_of_time;

        let newt = self._get_julian_centuries_from_julian_day(julian_day + time_utc / 1440.0);
        equation_of_time = self._get_equation_of_time(newt);
        solar_declination = self._get_sun_declination(newt);
        hour_angle = self._get_sun_hour_angle(latitude, solar_declination, zenith, solar_event);
        delta = longitude - hour_angle.to_degrees();
        time_diff = 4.0 * delta;
        time_utc = 720.0 + time_diff - equation_of_time;

        time_utc
    }

    fn _get_sun_hour_angle(
        &self,
        latitude: f64,
        solar_declination: f64,
        zenith: f64,
        solar_event: _SolarEvent,
    ) -> f64 {
        let lat_rad = latitude.to_radians();
        let sd_rad = solar_declination.to_radians();

        let hour_angle = acos(
            cos(zenith.to_radians()) / (cos(lat_rad) * cos(sd_rad)) - tan(lat_rad) * tan(sd_rad),
        );

        if solar_event == _SolarEvent::Sunset {
            -hour_angle
        } else {
            hour_angle
        }
    }

    fn _get_solar_noon_midnight_utc(
        &self,
        julian_day: f64,
        longitude: f64,
        solar_event: _SolarEvent,
    ) -> f64 {
        let julian_day = if solar_event == _SolarEvent::Noon {
            julian_day
        } else {
            julian_day + 0.5
        };

        let t_noon = self._get_julian_centuries_from_julian_day(julian_day + longitude / 360.0);
        let mut equation_of_time = self._get_equation_of_time(t_noon);
        let sol_noon_utc = (longitude * 4.0) - equation_of_time;

        let new_t = self._get_julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
        equation_of_time = self._get_equation_of_time(new_t);

        let base_minutes = if solar_event == _SolarEvent::Noon {
            720.0
        } else {
            1440.0
        };
        base_minutes + (longitude * 4.0) - equation_of_time
    }

    fn _get_solar_elevation_azimuth<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
        is_azimuth: bool,
    ) -> f64 {
        let date_time = date_time.to_utc();
        let latitude = geo_location.get_latitude();
        let longitude = geo_location.get_longitude();

        let minute: f64 = date_time.minute() as f64;
        let second: f64 = date_time.second() as f64;
        let hour: f64 = date_time.hour() as f64;
        let milli: f64 = date_time.nanosecond() as f64 / 1000000.0;

        let time: f64 = (hour + (minute + (second + (milli / 1000.0)) / 60.0) / 60.0) / 24.0;

        let julian_day = self._get_julian_day(&date_time) + time;
        let julian_centuries = self._get_julian_centuries_from_julian_day(julian_day);

        let eot = self._get_equation_of_time(julian_centuries);
        let theta = self._get_sun_declination(julian_centuries);

        let adjustment = time + eot / 1440.0;
        let true_solar_time = ((adjustment + longitude / 360.0) + 2.0) % 1.0;
        let hour_angle_rad = true_solar_time * PI * 2.0 - PI;

        let cos_zenith = sin(latitude.to_radians()) * sin(theta.to_radians())
            + cos(latitude.to_radians()) * cos(theta.to_radians()) * cos(hour_angle_rad);

        let cos_zenith_clamped = cos_zenith.clamp(-1.0, 1.0);
        let zenith = acos(cos_zenith_clamped).to_degrees();

        let az_denom = cos(latitude.to_radians()) * sin(zenith.to_radians());
        let refraction_adjustment = 0.0;
        let elevation = 90.0 - (zenith - refraction_adjustment);
        if is_azimuth {
            let azimuth = if az_denom.abs() > 0.001 {
                let az_rad = (sin(latitude.to_radians()) * cos(zenith.to_radians())
                    - sin(theta.to_radians()))
                    / az_denom;

                let az_rad_clamped = az_rad.clamp(-1.0, 1.0);
                180.0
                    - acos(az_rad_clamped).to_degrees()
                        * if hour_angle_rad > 0.0 { -1.0 } else { 1.0 }
            } else if latitude > 0.0 {
                180.0
            } else {
                0.0
            };
            azimuth % 360.0
        } else {
            elevation
        }
    }
}

impl NOAACalculatorTrait for NOAACalculator {
    fn get_utc_noon<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64 {
        let julian_day = self._get_julian_day(date_time);
        let noon = self._get_solar_noon_midnight_utc(
            julian_day,
            -geo_location.get_longitude(),
            _SolarEvent::Noon,
        );
        let noon_hours = noon / 60.0;
        if noon_hours > 0.0 {
            noon_hours % 24.0
        } else {
            noon_hours % 24.0 + 24.0
        }
    }

    fn get_utc_midnight<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64 {
        let julian_day = self._get_julian_day(date_time);
        let midnight = self._get_solar_noon_midnight_utc(
            julian_day,
            -geo_location.get_longitude(),
            _SolarEvent::Midnight,
        );
        let midnight_hours = midnight / 60.0;
        if midnight_hours > 0.0 {
            midnight_hours % 24.0
        } else {
            midnight_hours % 24.0 + 24.0
        }
    }

    fn get_utc_sunrise<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let elevation = if adjust_for_elevation {
            geo_location.get_elevation()
        } else {
            0.0
        };
        let adjusted_zenith = self._adjust_zenith(zenith, elevation);
        let sunrise = self._get_sun_rise_set_utc(
            date_time,
            geo_location.get_latitude(),
            -geo_location.get_longitude(),
            adjusted_zenith,
            _SolarEvent::Sunrise,
        );
        let sunrise_hours = sunrise / 60.0;
        let result = if sunrise_hours > 0.0 {
            sunrise_hours % 24.0
        } else {
            sunrise_hours % 24.0 + 24.0
        };
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_utc_sunset<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let elevation = if adjust_for_elevation {
            geo_location.get_elevation()
        } else {
            0.0
        };
        let adjusted_zenith = self._adjust_zenith(zenith, elevation);
        let sunset = self._get_sun_rise_set_utc(
            date_time,
            geo_location.get_latitude(),
            -geo_location.get_longitude(),
            adjusted_zenith,
            _SolarEvent::Sunset,
        );
        let sunset_hours = sunset / 60.0;
        let result = if sunset_hours > 0.0 {
            sunset_hours % 24.0
        } else {
            sunset_hours % 24.0 + 24.0
        };
        if result.is_nan() { None } else { Some(result) }
    }

    fn get_solar_elevation<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64 {
        self._get_solar_elevation_azimuth(date_time, geo_location, false)
    }

    fn get_solar_azimuth<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64 {
        self._get_solar_elevation_azimuth(date_time, geo_location, true)
    }
}
#[cfg(test)]
mod jni_tests {

    use crate::{
        geolocation::GeoLocation,
        test_utils::jni::{
            DEFAULT_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64,
            assert_almost_equal_f64_option, create_date_times_with_geolocation,
            create_java_noaa_calculator, init_jvm,
        },
    };

    use super::*;

    use chrono_tz::Tz;
    use j4rs::InvocationArg;
    use rand::Rng;

    fn f64_tester(
        fn_to_test: impl Fn(&NOAACalculator, &DateTime<Tz>, &GeoLocation) -> f64,
        method: &str,
    ) {
        let calculator = NOAACalculator;
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_date_times_with_geolocation(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (date_time, java_calendar, geo_location, java_geo_location, message) =
                test_case.unwrap();
            let noaa_calculator = create_java_noaa_calculator(&jvm);

            let result = fn_to_test(&calculator, &date_time, &geo_location);
            let raw_java_result = jvm
                .invoke(
                    &noaa_calculator,
                    method,
                    &[
                        InvocationArg::from(java_calendar),
                        InvocationArg::from(java_geo_location),
                    ],
                )
                .unwrap();
            let java_result = jvm.to_rust::<f64>(raw_java_result).unwrap();
            assert_almost_equal_f64(result, java_result, DEFAULT_TEST_EPSILON, &message);
        }
    }

    fn f64_option_tester_with_zenith_and_adjust_for_elevation(
        fn_to_test: impl Fn(&NOAACalculator, &DateTime<Tz>, &GeoLocation, f64, bool) -> Option<f64>,
        method: &str,
    ) {
        let calculator = NOAACalculator;
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_date_times_with_geolocation(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (date_time, java_calendar, geo_location, java_geo_location, message) =
                test_case.unwrap();
            let noaa_calculator = create_java_noaa_calculator(&jvm);
            let zenith = rand::thread_rng().gen_range(-180.0..=180.0);
            let adjust_for_elevation = rand::thread_rng().gen_bool(0.5);
            let result = fn_to_test(
                &calculator,
                &date_time,
                &geo_location,
                zenith,
                adjust_for_elevation,
            );
            let raw_java_result = jvm
                .invoke(
                    &noaa_calculator,
                    method,
                    &[
                        InvocationArg::from(java_calendar),
                        InvocationArg::from(java_geo_location),
                        InvocationArg::try_from(zenith)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(adjust_for_elevation)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_result = jvm.to_rust::<f64>(raw_java_result).unwrap();
            // Convert java's Nan to None
            let java_result = if java_result.is_nan() {
                None
            } else {
                Some(java_result)
            };
            assert_almost_equal_f64_option(&result, &java_result, DEFAULT_TEST_EPSILON, &message);
        }
    }

    /// Test Julian day calculation against Java implementation
    #[test]
    fn test_get_utc_noon_against_java() {
        f64_tester(NOAACalculator::get_utc_noon, "getUTCNoon");
    }

    #[test]
    fn test_get_utc_midnight_against_java() {
        f64_tester(NOAACalculator::get_utc_midnight, "getUTCMidnight");
    }
    #[test]
    fn test_get_utc_sunrise_against_java() {
        f64_option_tester_with_zenith_and_adjust_for_elevation(
            NOAACalculator::get_utc_sunrise,
            "getUTCSunrise",
        );
    }
    #[test]
    fn test_get_utc_sunset_against_java() {
        f64_option_tester_with_zenith_and_adjust_for_elevation(
            NOAACalculator::get_utc_sunset,
            "getUTCSunset",
        );
    }
    #[test]
    fn test_get_solar_elevation_against_java() {
        f64_tester(NOAACalculator::get_solar_elevation, "getSolarElevation");
    }

    #[test]
    fn test_get_solar_azimuth_against_java() {
        f64_tester(NOAACalculator::get_solar_azimuth, "getSolarAzimuth");
    }
}
