use crate::constants::{_Formula, _MINUTE_MILLIS, GeoLocationTrait};
use chrono::{DateTime, Duration, Offset, TimeZone};
use core::f64::consts::PI;
use libm::{atan, atan2, cos, log, sin, sqrt, tan};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
}

impl GeoLocationTrait for GeoLocation {
    fn get_latitude(&self) -> f64 {
        self.latitude
    }

    fn get_longitude(&self) -> f64 {
        self.longitude
    }

    fn get_elevation(&self) -> f64 {
        self.elevation
    }
    fn get_rhumb_line_bearing(&self, location: &impl GeoLocationTrait) -> f64 {
        let mut d_lon = (location.get_longitude() - self.get_longitude()).to_radians();
        let d_phi = log(tan(location.get_latitude().to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.get_latitude().to_radians() / 2.0 + PI / 4.0));

        if d_lon.abs() > PI {
            d_lon = if d_lon > 0.0 {
                -(2.0 * PI - d_lon)
            } else {
                2.0 * PI + d_lon
            };
        }

        atan2(d_lon, d_phi).to_degrees()
    }

    fn get_rhumb_line_distance(&self, location: &impl GeoLocationTrait) -> f64 {
        let earth_radius = 6378137.0;
        let d_lat = location.get_latitude().to_radians() - self.get_latitude().to_radians();
        let mut d_lon =
            (location.get_longitude().to_radians() - self.get_longitude().to_radians()).abs();
        let d_phi = log(tan(location.get_latitude().to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.get_latitude().to_radians() / 2.0 + PI / 4.0));
        let mut q = d_lat / d_phi;

        if !q.is_finite() {
            q = cos(self.get_latitude().to_radians());
        }

        if d_lon > PI {
            d_lon = 2.0 * PI - d_lon;
        }

        let d = sqrt(d_lat * d_lat + q * q * d_lon * d_lon);
        d * earth_radius
    }

    fn get_geodesic_initial_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64> {
        self.vincenty_inverse_formula(location, _Formula::InitialBearing)
    }

    fn get_geodesic_final_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64> {
        self.vincenty_inverse_formula(location, _Formula::FinalBearing)
    }

    fn get_geodesic_distance(&self, location: &impl GeoLocationTrait) -> Option<f64> {
        self.vincenty_inverse_formula(location, _Formula::Distance)
    }

    fn get_local_mean_time_offset<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> Duration {
        let longitude_offset_ms = self.get_longitude() * 4.0 * _MINUTE_MILLIS as f64;
        let timezone_offset_sec = date.offset().fix().local_minus_utc();
        let timezone_offset_ms = timezone_offset_sec as f64 * 1000.0;
        Duration::milliseconds((longitude_offset_ms - timezone_offset_ms) as i64)
    }

    fn get_antimeridian_adjustment<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i8 {
        let local_hours_offset = self.get_local_mean_time_offset(date);
        if local_hours_offset >= Duration::hours(20) {
            return 1;
        } else if local_hours_offset <= Duration::hours(-20) {
            return -1;
        }
        0
    }
}
impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64, elevation: f64) -> Option<Self> {
        if latitude.is_nan() || longitude.is_nan() || elevation.is_nan() || elevation.is_infinite()
        {
            return None;
        }
        if !(-90.0..=90.0).contains(&latitude) {
            return None;
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return None;
        }
        if elevation < 0.0 {
            return None;
        }
        Some(Self {
            latitude,
            longitude,
            elevation,
        })
    }
    fn vincenty_inverse_formula(
        &self,
        location: &impl GeoLocationTrait,
        formula: _Formula,
    ) -> Option<f64> {
        let major_semi_axis = 6378137.0;
        let minor_semi_axis = 6356752.3142;
        let f = 1.0 / 298.257223563;
        let l = (location.get_longitude() - self.get_longitude()).to_radians();
        let u1 = atan((1.0 - f) * tan(self.get_latitude().to_radians()));
        let u2 = atan((1.0 - f) * tan(location.get_latitude().to_radians()));
        let sin_u1 = sin(u1);
        let cos_u1 = cos(u1);
        let sin_u2 = sin(u2);
        let cos_u2 = cos(u2);

        let mut lambda = l;
        let mut lambda_p = 2.0 * PI;
        let mut iter_limit = 20;
        let mut sin_lambda = 0.0;
        let mut cos_lambda = 0.0;
        let mut sin_sigma = 0.0;
        let mut cos_sigma = 0.0;
        let mut sigma = 0.0;
        #[allow(unused_assignments)]
        let mut sin_alpha = 0.0;
        let mut cos_sq_alpha = 0.0;
        let mut cos2_sigma_m = 0.0;

        while (lambda - lambda_p).abs() > 1e-12 && iter_limit > 0 {
            sin_lambda = sin(lambda);
            cos_lambda = cos(lambda);
            sin_sigma = sqrt(
                (cos_u2 * sin_lambda) * (cos_u2 * sin_lambda)
                    + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda)
                        * (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda),
            );

            if sin_sigma == 0.0 {
                return Some(0.0);
            }

            cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            sigma = atan2(sin_sigma, cos_sigma);
            sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
            cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
            cos2_sigma_m = cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha;

            if cos2_sigma_m.is_nan() {
                cos2_sigma_m = 0.0;
            }

            let c = f / 16.0 * cos_sq_alpha * (4.0 + f * (4.0 - 3.0 * cos_sq_alpha));
            lambda_p = lambda;
            lambda = l
                + (1.0 - c)
                    * f
                    * sin_alpha
                    * (sigma
                        + c * sin_sigma
                            * (cos2_sigma_m
                                + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

            iter_limit -= 1;
        }

        if iter_limit == 0 {
            return None;
        }

        let u_sq = cos_sq_alpha
            * (major_semi_axis * major_semi_axis - minor_semi_axis * minor_semi_axis)
            / (minor_semi_axis * minor_semi_axis);
        let a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
        let b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
        let delta_sigma = b
            * sin_sigma
            * (cos2_sigma_m
                + b / 4.0
                    * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                        - b / 6.0
                            * cos2_sigma_m
                            * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                            * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));
        let distance = minor_semi_axis * a * (sigma - delta_sigma);

        let fwd_az = atan2(
            cos_u2 * sin_lambda,
            cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda,
        )
        .to_degrees();

        let rev_az = atan2(
            cos_u1 * sin_lambda,
            -sin_u1 * cos_u2 + cos_u1 * sin_u2 * cos_lambda,
        )
        .to_degrees();

        match formula {
            _Formula::Distance => Some(distance),
            _Formula::InitialBearing => Some(fwd_az),
            _Formula::FinalBearing => Some(rev_az),
        }
    }
}

#[cfg(test)]
mod jni_tests {
    use crate::test_utils::jni::{
        DEFAULT_TEST_EPSILON, assert_almost_equal_f64, assert_almost_equal_f64_option,
        create_date_times_with_geolocation, create_random_geolocations, init_jvm,
    };

    use super::*;
    use crate::test_utils::jni::DEFAULT_TEST_ITERATIONS;

    use j4rs::{Instance, InvocationArg, Jvm};

    fn invoke_method(
        jvm: &Jvm,
        instance: &Instance,
        method_name: &str,
        other_instance: Instance,
    ) -> f64 {
        let result = jvm
            .invoke(
                &instance,
                method_name,
                &[InvocationArg::from(other_instance)],
            )
            .unwrap();
        jvm.to_rust::<f64>(result).unwrap()
    }

    fn f64_tester(fn_to_test: impl Fn(&GeoLocation, &GeoLocation) -> f64, method: &str) {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_random_geolocations(&jvm, "UTC");
            let other_test_case = create_random_geolocations(&jvm, "UTC");
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let distance = fn_to_test(&geo_location, &other_geo_location);
                    let java_distance =
                        invoke_method(&jvm, &java_geo_location, method, other_java_geo_location);
                    let message =
                        format!("{} from {} to {} failed", method, message, other_message);
                    assert_almost_equal_f64(
                        distance,
                        java_distance,
                        DEFAULT_TEST_EPSILON,
                        &message,
                    );
                }
            }
        }
    }
    fn f64_option_tester(
        fn_to_test: impl Fn(&GeoLocation, &GeoLocation) -> Option<f64>,
        method: &str,
    ) {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_random_geolocations(&jvm, "UTC");
            let other_test_case = create_random_geolocations(&jvm, "UTC");
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let distance = fn_to_test(&geo_location, &other_geo_location);
                    let java_result =
                        invoke_method(&jvm, &java_geo_location, method, other_java_geo_location);
                    let java_result = if java_result.is_nan() {
                        None
                    } else {
                        Some(java_result)
                    };
                    let message =
                        format!("{} from {} to {} failed", method, message, other_message);
                    assert_almost_equal_f64_option(
                        &distance,
                        &java_result,
                        DEFAULT_TEST_EPSILON,
                        &message,
                    );
                }
            }
        }
    }

    #[test]
    fn test_rhumb_line_distance() {
        f64_tester(GeoLocation::get_rhumb_line_distance, "getRhumbLineDistance");
    }
    #[test]
    fn test_rhumb_line_bearing() {
        f64_tester(GeoLocation::get_rhumb_line_bearing, "getRhumbLineBearing");
    }

    #[test]
    fn test_get_geodesic_initial_bearing() {
        f64_option_tester(
            GeoLocation::get_geodesic_initial_bearing,
            "getGeodesicInitialBearing",
        );
    }

    #[test]
    fn test_get_geodesic_final_bearing() {
        f64_option_tester(
            GeoLocation::get_geodesic_final_bearing,
            "getGeodesicFinalBearing",
        );
    }

    #[test]
    fn test_get_geodesic_distance() {
        f64_option_tester(GeoLocation::get_geodesic_distance, "getGeodesicDistance");
    }

    #[test]
    fn test_get_local_mean_time_offset() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_date_times_with_geolocation(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;

            let (date_time, java_calendar, geo_location, java_geo_location, message) =
                test_case.unwrap();

            let rust_offset = geo_location.get_local_mean_time_offset(&date_time);
            let rust_offset_ms = rust_offset.num_milliseconds();

            let result = jvm
                .invoke(
                    &java_geo_location,
                    "getLocalMeanTimeOffset",
                    &[InvocationArg::from(java_calendar)],
                )
                .unwrap();
            let java_offset_ms = jvm.to_rust::<i64>(result).unwrap();

            let error_message = format!("getLocalMeanTimeOffset failed for {}", message);
            assert_almost_equal_f64(
                rust_offset_ms as f64,
                java_offset_ms as f64,
                DEFAULT_TEST_EPSILON,
                &error_message,
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_antimeridian_adjustment() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_date_times_with_geolocation(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;

            let (date_time, java_calendar, geo_location, java_geo_location, message) =
                test_case.unwrap();

            let rust_adjustment = geo_location.get_antimeridian_adjustment(&date_time);
            let result = jvm
                .invoke(
                    &java_geo_location,
                    "getAntimeridianAdjustment",
                    &[InvocationArg::from(java_calendar)],
                )
                .unwrap();
            let java_adjustment = jvm.to_rust::<i32>(result).unwrap() as i8;

            assert_eq!(
                rust_adjustment, java_adjustment,
                "getAntimeridianAdjustment failed for {}",
                message
            );
        }
        assert!(ran, "No test cases were run");
    }
}
