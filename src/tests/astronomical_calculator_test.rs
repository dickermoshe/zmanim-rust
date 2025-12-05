use j4rs::{Instance, InvocationArg, Jvm};

use crate::{
    astronomical_calculator::AstronomicalCalculatorTrait,
    geolocation::GeoLocationTrait,
    tests::test_utils::{dt_to_java_calendar, geolocation_to_java_geolocation},
};

pub struct JavaAstronomicalCalculator<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
}
impl<'a> JavaAstronomicalCalculator<'a> {
    pub fn new(jvm: &'a Jvm) -> Self {
        let instance = jvm
            .create_instance("com.kosherjava.zmanim.util.NOAACalculator", InvocationArg::empty())
            .unwrap();
        Self { jvm, instance }
    }
}

impl<'a, G: GeoLocationTrait> AstronomicalCalculatorTrait<G> for JavaAstronomicalCalculator<'a> {
    fn get_utc_noon<Tz: chrono::TimeZone>(&self, date_time: &chrono::DateTime<Tz>, geo_location: &G) -> f64 {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCNoon",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_midnight<Tz: chrono::TimeZone>(&self, date_time: &chrono::DateTime<Tz>, geo_location: &G) -> f64 {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCMidnight",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_sunrise<Tz: chrono::TimeZone>(
        &self,
        date_time: &chrono::DateTime<Tz>,
        geo_location: &G,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunrise",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                    InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(adjust_for_elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        let java_result = self.jvm.to_rust::<f64>(result).unwrap();
        // Convert java's NaN to None
        if java_result.is_nan() { None } else { Some(java_result) }
    }

    fn get_utc_sunset<Tz: chrono::TimeZone>(
        &self,
        date_time: &chrono::DateTime<Tz>,
        geo_location: &G,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunset",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                    InvocationArg::try_from(zenith).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(adjust_for_elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        let java_result = self.jvm.to_rust::<f64>(result).unwrap();
        // Convert java's NaN to None
        if java_result.is_nan() { None } else { Some(java_result) }
    }

    fn get_solar_elevation<Tz: chrono::TimeZone>(&self, date_time: &chrono::DateTime<Tz>, geo_location: &G) -> f64 {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSolarElevation",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_solar_azimuth<Tz: chrono::TimeZone>(&self, date_time: &chrono::DateTime<Tz>, geo_location: &G) -> f64 {
        let java_date_time = dt_to_java_calendar(self.jvm, date_time).unwrap();
        let java_geo_location = geolocation_to_java_geolocation(self.jvm, geo_location, date_time.timezone()).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSolarAzimuth",
                &[
                    InvocationArg::from(java_date_time),
                    InvocationArg::from(java_geo_location),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }
}

#[cfg(test)]
mod jni_tests {
    use rand::Rng;

    use crate::{
        astronomical_calculator::NOAACalculator,
        tests::{
            geolocation_test::random_geolocations,
            test_utils::{
                DEFAULT_F64_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64,
                assert_almost_equal_f64_option, init_jvm, random_date_time, random_zenith,
            },
        },
    };

    use super::*;

    #[test]
    fn test_random_astronomical_calculator() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran_once = false;
        let calculator = NOAACalculator;
        let java_calculator = JavaAstronomicalCalculator::new(&jvm);

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            if let Some((geo_location, java_geo_location)) = random_geolocations(&jvm, &mut rng) {
                let date = random_date_time(&mut rng, java_geo_location.timezone);

                // Test get_utc_noon
                let rust_noon = calculator.get_utc_noon(&date, &geo_location);
                let java_noon = java_calculator.get_utc_noon(&date, &java_geo_location);
                assert_almost_equal_f64(
                    rust_noon,
                    java_noon,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!("getUtcNoon of {:?} at {:?}", geo_location, date),
                );

                // Test get_utc_midnight
                let rust_midnight = calculator.get_utc_midnight(&date, &geo_location);
                let java_midnight = java_calculator.get_utc_midnight(&date, &java_geo_location);
                assert_almost_equal_f64(
                    rust_midnight,
                    java_midnight,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!("getUtcMidnight of {:?} at {:?}", geo_location, date),
                );

                // Test get_utc_sunrise
                let zenith = random_zenith(&mut rng);
                let adjust_for_elevation = rng.gen_bool(0.5);
                let rust_sunrise = calculator.get_utc_sunrise(&date, &geo_location, zenith, adjust_for_elevation);
                let java_sunrise =
                    java_calculator.get_utc_sunrise(&date, &java_geo_location, zenith, adjust_for_elevation);
                assert_almost_equal_f64_option(
                    &rust_sunrise,
                    &java_sunrise,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!(
                        "getUtcSunrise of {:?} at {:?} with zenith {} and adjust_for_elevation {}",
                        geo_location, date, zenith, adjust_for_elevation
                    ),
                );

                // Test get_utc_sunset
                let rust_sunset = calculator.get_utc_sunset(&date, &geo_location, zenith, adjust_for_elevation);
                let java_sunset =
                    java_calculator.get_utc_sunset(&date, &java_geo_location, zenith, adjust_for_elevation);
                assert_almost_equal_f64_option(
                    &rust_sunset,
                    &java_sunset,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!(
                        "getUtcSunset of {:?} at {:?} with zenith {} and adjust_for_elevation {}",
                        geo_location, date, zenith, adjust_for_elevation
                    ),
                );

                // Test get_solar_elevation
                let rust_elevation = calculator.get_solar_elevation(&date, &geo_location);
                let java_elevation = java_calculator.get_solar_elevation(&date, &java_geo_location);
                assert_almost_equal_f64(
                    rust_elevation,
                    java_elevation,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!("getSolarElevation of {:?} at {:?}", geo_location, date),
                );

                // Test get_solar_azimuth
                let rust_azimuth = calculator.get_solar_azimuth(&date, &geo_location);
                let java_azimuth = java_calculator.get_solar_azimuth(&date, &java_geo_location);
                assert_almost_equal_f64(
                    rust_azimuth,
                    java_azimuth,
                    DEFAULT_F64_TEST_EPSILON,
                    &format!("getSolarAzimuth of {:?} at {:?}", geo_location, date),
                );

                ran_once = true;
            }
        }
        assert!(ran_once, "No test cases were run");
    }
}
