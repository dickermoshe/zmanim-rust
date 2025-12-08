//! An implementation of the GeoLocationTrait which delegates to the GeoLocation class in Java
//! This is used to test our Rust implementation against the Java implementation.
use crate::{
    defmt::DefmtFormatTrait,
    geolocation::{GeoLocation, GeoLocationTrait},
    tests::{DoubleType, dt_to_java_calendar, rng_double_type, tz_to_java_timezone},
};
use chrono::Duration;
use chrono_tz::{TZ_VARIANTS, Tz};
use j4rs::{Instance, InvocationArg, Jvm};

pub struct JavaGeoLocation<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,

    pub timezone: Tz,
}

impl<'a> DefmtFormatTrait for JavaGeoLocation<'a> {}

impl<'a> JavaGeoLocation<'a> {
    pub fn new(jvm: &'a Jvm, latitude: f64, longitude: f64, elevation: f64, timezone: Tz) -> Option<Self> {
        let java_timezone = tz_to_java_timezone(jvm, timezone);

        // DIFF: Java will return a GMT timezone if it is unable to find a timezone
        // However we will return None if this is the case
        java_timezone.as_ref()?;
        let java_timezone = java_timezone.unwrap();
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.util.GeoLocation",
                &[
                    InvocationArg::try_from("Name").unwrap(),
                    InvocationArg::try_from(latitude).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(longitude).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(elevation).unwrap().into_primitive().unwrap(),
                    InvocationArg::from(java_timezone),
                ],
            )
            .ok();
        // DIFF: Java will throw an exception if it is unable to create a GeoLocation
        // However we will return None if this is the case
        instance.as_ref()?;
        let instance = instance.unwrap();
        Some(Self {
            jvm,
            instance,
            timezone,
        })
    }
}

impl<'a> GeoLocationTrait for JavaGeoLocation<'a> {
    fn get_latitude(&self) -> f64 {
        unimplemented!("get_latitude is not implemented in this test and should not be called");
    }

    fn get_longitude(&self) -> f64 {
        unimplemented!("get_longitude is not implemented in this test and should not be called");
    }

    fn get_elevation(&self) -> f64 {
        unimplemented!("get_elevation is not implemented in this test and should not be called");
    }

    fn get_rhumb_line_distance(&self, location: &JavaGeoLocation<'_>) -> f64 {
        let cloned_instance = self.jvm.clone_instance(&location.instance).unwrap();
        self.jvm
            .to_rust::<f64>(
                self.jvm
                    .invoke(
                        &self.instance,
                        "getRhumbLineDistance",
                        &[InvocationArg::from(cloned_instance)],
                    )
                    .unwrap(),
            )
            .unwrap()
    }

    fn get_rhumb_line_bearing(&self, location: &JavaGeoLocation<'_>) -> f64 {
        let cloned_instance = self.jvm.clone_instance(&location.instance).unwrap();
        self.jvm
            .to_rust::<f64>(
                self.jvm
                    .invoke(
                        &self.instance,
                        "getRhumbLineBearing",
                        &[InvocationArg::from(cloned_instance)],
                    )
                    .unwrap(),
            )
            .unwrap()
    }

    fn get_geodesic_initial_bearing(&self, location: &JavaGeoLocation<'_>) -> Option<f64> {
        let cloned_instance = self.jvm.clone_instance(&location.instance).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getGeodesicInitialBearing",
                &[InvocationArg::from(cloned_instance)],
            )
            .unwrap();

        self.jvm.to_rust::<f64>(result).ok()
    }

    fn get_geodesic_final_bearing(&self, location: &JavaGeoLocation<'_>) -> Option<f64> {
        let cloned_instance = self.jvm.clone_instance(&location.instance).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getGeodesicFinalBearing",
                &[InvocationArg::from(cloned_instance)],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).ok()
    }

    fn get_geodesic_distance(&self, location: &JavaGeoLocation<'_>) -> Option<f64> {
        let cloned_instance = self.jvm.clone_instance(&location.instance).unwrap();
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getGeodesicDistance",
                &[InvocationArg::from(cloned_instance)],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).ok()
    }

    fn get_local_mean_time_offset<Tz: chrono::TimeZone>(&self, date: &chrono::DateTime<Tz>) -> chrono::Duration {
        let java_date = dt_to_java_calendar(self.jvm, date).unwrap();
        let result = self
            .jvm
            .to_rust::<i64>(
                self.jvm
                    .invoke(
                        &self.instance,
                        "getLocalMeanTimeOffset",
                        &[InvocationArg::from(java_date)],
                    )
                    .unwrap(),
            )
            .unwrap();
        Duration::milliseconds(result)
    }

    fn get_antimeridian_adjustment<Tz: chrono::TimeZone>(&self, date: &chrono::DateTime<Tz>) -> i8 {
        let java_date = dt_to_java_calendar(self.jvm, date).unwrap();
        self.jvm
            .to_rust::<i32>(
                self.jvm
                    .invoke(
                        &self.instance,
                        "getAntimeridianAdjustment",
                        &[InvocationArg::from(java_date)],
                    )
                    .unwrap(),
            )
            .unwrap() as i8
    }
}

pub fn random_geolocations<'a, Rng: rand::Rng>(
    jvm: &'a Jvm,
    rng: &mut Rng,
) -> Option<(GeoLocation, JavaGeoLocation<'a>)> {
    let random_value = rng_double_type(rng);
    let latitude = match random_value {
        DoubleType::Finite => rng.gen_range(-90.0..=90.0),
        DoubleType::OutOfRange => {
            if rng.gen_bool(0.5) {
                -91.0
            } else {
                91.0
            }
        }
        DoubleType::Infinite => f64::INFINITY,
        DoubleType::Nan => f64::NAN,
    };
    let longitude = match random_value {
        DoubleType::Finite => rng.gen_range(-180.0..=180.0),
        DoubleType::OutOfRange => {
            if rng.gen_bool(0.5) {
                -181.0
            } else {
                181.0
            }
        }
        DoubleType::Infinite => f64::INFINITY,
        DoubleType::Nan => f64::NAN,
    };
    let elevation = match random_value {
        DoubleType::Finite => rng.gen_range(0.0..=1000.0),
        DoubleType::OutOfRange => -1.0,
        DoubleType::Infinite => f64::INFINITY,
        DoubleType::Nan => f64::NAN,
    };
    let tz = TZ_VARIANTS[rng.gen_range(0..TZ_VARIANTS.len())];
    // DIFF: Java cannot handle the some timezones
    if tz.name() == "ROC" || tz.name() == "America/Coyhaique" || tz.name() == "GMT" {
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

mod jni_tests {
    use crate::tests::{
        DEFAULT_F64_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_duration, assert_almost_equal_f64,
        assert_almost_equal_f64_option, init_jvm, random_date_time,
    };

    fn compare_geolocations<'a>(
        rust_geolocation: &GeoLocation,
        java_geolocation: &JavaGeoLocation<'a>,
        other_rust_geolocation: &GeoLocation,
        other_java_geolocation: &JavaGeoLocation<'a>,
        date: &chrono::DateTime<chrono_tz::Tz>,
    ) {
        assert_almost_equal_f64(
            rust_geolocation.get_rhumb_line_distance(other_rust_geolocation),
            java_geolocation.get_rhumb_line_distance(other_java_geolocation),
            DEFAULT_F64_TEST_EPSILON,
            &format!(
                "getRhumbLineDistance of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_almost_equal_f64(
            rust_geolocation.get_rhumb_line_bearing(other_rust_geolocation),
            java_geolocation.get_rhumb_line_bearing(other_java_geolocation),
            DEFAULT_F64_TEST_EPSILON,
            &format!(
                "getRhumbLineBearing of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_almost_equal_f64_option(
            &rust_geolocation.get_geodesic_initial_bearing(other_rust_geolocation),
            &java_geolocation.get_geodesic_initial_bearing(other_java_geolocation),
            DEFAULT_F64_TEST_EPSILON,
            &format!(
                "getGeodesicInitialBearing of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_almost_equal_f64_option(
            &rust_geolocation.get_geodesic_final_bearing(other_rust_geolocation),
            &java_geolocation.get_geodesic_final_bearing(other_java_geolocation),
            DEFAULT_F64_TEST_EPSILON,
            &format!(
                "getGeodesicFinalBearing of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_almost_equal_f64_option(
            &rust_geolocation.get_geodesic_distance(other_rust_geolocation),
            &java_geolocation.get_geodesic_distance(other_java_geolocation),
            DEFAULT_F64_TEST_EPSILON,
            &format!(
                "getGeodesicDistance of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_almost_equal_duration(
            &rust_geolocation.get_local_mean_time_offset(date),
            &java_geolocation.get_local_mean_time_offset(date),
            10, // 10 milliseconds
            &format!(
                "getLocalMeanTimeOffset of {:?} against {:?}",
                rust_geolocation, other_rust_geolocation
            ),
        );
        assert_eq!(
            rust_geolocation.get_antimeridian_adjustment(date),
            java_geolocation.get_antimeridian_adjustment(date),
            "getAntimeridianAdjustment of {:?} against {:?}",
            rust_geolocation,
            other_rust_geolocation
        );
    }

    use super::*;

    #[test]
    fn test_random_geolocations() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran_once = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            if let Some((geo_location, java_geo_location)) = random_geolocations(&jvm, &mut rng)
                && let Some((other_geo_location, other_java_geo_location)) = random_geolocations(&jvm, &mut rng)
            {
                let date = random_date_time(&mut rng, other_java_geo_location.timezone);
                compare_geolocations(
                    &geo_location,
                    &java_geo_location,
                    &other_geo_location,
                    &other_java_geo_location,
                    &date,
                );
                ran_once = true;
            }
        }
        assert!(ran_once, "No test cases were run");
    }
}
