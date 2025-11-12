use chrono::{DateTime, Offset, TimeZone};
use core::f64::consts::PI;
use libm::{atan, atan2, cos, log, sin, sqrt, tan};

use crate::constants::{_Formula, _HOUR_MILLIS, _MINUTE_MILLIS};
/// TODO ADD DOCS
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GeoLocation {
    /// TODO ADD DOCS
    pub latitude: f64,
    /// TODO ADD DOCS
    pub longitude: f64,
    /// TODO ADD DOCS
    pub elevation: f64,
}

impl GeoLocation {
    /// TODO ADD DOCS
    pub fn new(latitude: f64, longitude: f64, elevation: f64) -> Option<Self> {
        if latitude.is_nan() || longitude.is_nan() || elevation.is_nan() || elevation.is_infinite()
        {
            return None;
        }
        if latitude < -90.0 || latitude > 90.0 {
            return None;
        }
        if longitude < -180.0 || longitude > 180.0 {
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

    pub fn get_rhumb_line_distance(&self, location: &GeoLocation) -> f64 {
        let earth_radius = 6378137.0;
        let d_lat = location.latitude.to_radians() - self.latitude.to_radians();
        let mut d_lon = (location.longitude.to_radians() - self.longitude.to_radians()).abs();
        let d_phi = log(tan(location.latitude.to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.latitude.to_radians() / 2.0 + PI / 4.0));
        let mut q = d_lat / d_phi;

        if !q.is_finite() {
            q = cos(self.latitude.to_radians());
        }

        if d_lon > PI {
            d_lon = 2.0 * PI - d_lon;
        }

        let d = sqrt(d_lat * d_lat + q * q * d_lon * d_lon);
        d * earth_radius
    }

    pub fn get_rhumb_line_bearing(&self, location: &GeoLocation) -> f64 {
        let mut d_lon = (location.longitude - self.longitude).to_radians();
        let d_phi = log(tan(location.latitude.to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.latitude.to_radians() / 2.0 + PI / 4.0));

        if d_lon.abs() > PI {
            d_lon = if d_lon > 0.0 {
                -(2.0 * PI - d_lon)
            } else {
                2.0 * PI + d_lon
            };
        }

        atan2(d_lon, d_phi).to_degrees()
    }

    fn _vincenty_inverse_formula(&self, location: &GeoLocation, formula: _Formula) -> Option<f64> {
        let major_semi_axis = 6378137.0;
        let minor_semi_axis = 6356752.3142;
        let f = 1.0 / 298.257223563;
        let l = (location.longitude - self.longitude).to_radians();
        let u1 = atan((1.0 - f) * tan(self.latitude.to_radians()));
        let u2 = atan((1.0 - f) * tan(location.latitude.to_radians()));
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

    pub fn get_geodesic_initial_bearing(&self, location: &GeoLocation) -> Option<f64> {
        self._vincenty_inverse_formula(location, _Formula::InitialBearing)
    }

    pub fn get_geodesic_final_bearing(&self, location: &GeoLocation) -> Option<f64> {
        self._vincenty_inverse_formula(location, _Formula::FinalBearing)
    }

    pub fn get_geodesic_distance(&self, location: &GeoLocation) -> Option<f64> {
        self._vincenty_inverse_formula(location, _Formula::Distance)
    }

    pub fn get_local_mean_time_offset<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i64 {
        let longitude_offset_ms = self.longitude * 4.0 * _MINUTE_MILLIS as f64;
        let timezone_offset_sec = date.offset().fix().local_minus_utc();
        let timezone_offset_ms = timezone_offset_sec as f64 * 1000.0;
        // println!("Rtimezone_offset_ms: {:?}", timezone_offset_ms);
        (longitude_offset_ms - timezone_offset_ms) as i64
    }

    pub fn get_antimeridian_adjustment<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i64 {
        let local_hours_offset = self.get_local_mean_time_offset(date) as f64 / _HOUR_MILLIS as f64;
        // println!("Rlocal_hours_offset: {:?}", local_hours_offset);
        if local_hours_offset >= 20.0 {
            return 1;
        } else if local_hours_offset <= -20.0 {
            return -1;
        }
        return 0;
    }
}

#[cfg(test)]
mod jni_tests {
    use crate::test_utils::jni::{
        DEFAULT_TEST_EPSILON, assert_almost_equal_f64, assert_almost_equal_f64_option,
        create_java_geo_location, init_jvm,
    };

    use super::*;
    use crate::test_utils::jni::{DEFAULT_TEST_ITERATIONS, RandomGeoLocation};

    use j4rs::{Instance, InvocationArg, Jvm};

    pub fn create_geolocation_test_case(jvm: &Jvm) -> Option<(GeoLocation, Instance, String)> {
        let random_geo_location = RandomGeoLocation::new();
        let message = format!(
            "Latitude: {}, Longitude: {}, Elevation: {}",
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation
        );

        let java_geo_location = create_java_geo_location(
            &jvm,
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation,
            "UTC",
        );
        let geo_location = GeoLocation::new(
            random_geo_location.latitude,
            random_geo_location.longitude,
            random_geo_location.elevation,
        );
        assert_eq!(
            geo_location.is_some(),
            java_geo_location.is_some(),
            "Failed to create test case for {}",
            message
        );
        if let (Some(geo), Some(java)) = (geo_location, java_geo_location) {
            Some((geo, java, message))
        } else {
            None
        }
    }

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

    #[test]
    fn test_rhumb_line_distance() {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_geolocation_test_case(&jvm);
            let other_test_case = create_geolocation_test_case(&jvm);
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let distance = geo_location.get_rhumb_line_distance(&other_geo_location);
                    let java_distance = invoke_method(
                        &jvm,
                        &java_geo_location,
                        "getRhumbLineDistance",
                        other_java_geo_location,
                    );
                    assert!(
                        (distance - java_distance).abs() < DEFAULT_TEST_EPSILON,
                        "getRhumbLineDistance from {} to {} failed",
                        message,
                        other_message
                    );
                }
            }
        }
    }
    #[test]
    fn test_rhumb_line_bearing() {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_geolocation_test_case(&jvm);
            let other_test_case = create_geolocation_test_case(&jvm);
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let bearing = geo_location.get_rhumb_line_bearing(&other_geo_location);
                    let java_bearing = invoke_method(
                        &jvm,
                        &java_geo_location,
                        "getRhumbLineBearing",
                        other_java_geo_location,
                    );
                    let message = format!(
                        "getRhumbLineBearing from {} to {} failed",
                        message, other_message
                    );
                    assert_almost_equal_f64(bearing, java_bearing, DEFAULT_TEST_EPSILON, &message);
                }
            }
        }
    }

    #[test]
    fn test_get_geodesic_initial_bearing() {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_geolocation_test_case(&jvm);
            let other_test_case = create_geolocation_test_case(&jvm);
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let message = format!(
                        "getGeodesicInitialBearing from {} to {} failed",
                        message, other_message
                    );
                    let bearing = geo_location.get_geodesic_initial_bearing(&other_geo_location);
                    let java_bearing = invoke_method(
                        &jvm,
                        &java_geo_location,
                        "getGeodesicInitialBearing",
                        other_java_geo_location,
                    );
                    let java_bearing = if java_bearing.is_nan() {
                        None
                    } else {
                        Some(java_bearing)
                    };
                    assert_almost_equal_f64_option(
                        &bearing,
                        &java_bearing,
                        DEFAULT_TEST_EPSILON,
                        &message,
                    );
                }
            }
        }
    }

    #[test]
    fn test_get_geodesic_final_bearing() {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_geolocation_test_case(&jvm);
            let other_test_case = create_geolocation_test_case(&jvm);
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let message = format!(
                        "getGeodesicFinalBearing from {} to {} failed",
                        message, other_message
                    );
                    let bearing = geo_location.get_geodesic_final_bearing(&other_geo_location);
                    let java_bearing = invoke_method(
                        &jvm,
                        &java_geo_location,
                        "getGeodesicFinalBearing",
                        other_java_geo_location,
                    );
                    let java_bearing = if java_bearing.is_nan() {
                        None
                    } else {
                        Some(java_bearing)
                    };
                    assert_almost_equal_f64_option(
                        &bearing,
                        &java_bearing,
                        DEFAULT_TEST_EPSILON,
                        &message,
                    );
                }
            }
        }
    }

    #[test]
    fn test_get_geodesic_distance() {
        let jvm = init_jvm();
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_geolocation_test_case(&jvm);
            let other_test_case = create_geolocation_test_case(&jvm);
            if let Some((geo_location, java_geo_location, message)) = test_case {
                if let Some((other_geo_location, other_java_geo_location, other_message)) =
                    other_test_case
                {
                    let message = format!(
                        "getGeodesicDistance from {} to {} failed",
                        message, other_message
                    );
                    let distance = geo_location.get_geodesic_distance(&other_geo_location);
                    let java_distance = invoke_method(
                        &jvm,
                        &java_geo_location,
                        "getGeodesicDistance",
                        other_java_geo_location,
                    );
                    let java_distance = if java_distance.is_nan() {
                        None
                    } else {
                        Some(java_distance)
                    };
                    assert_almost_equal_f64_option(
                        &distance,
                        &java_distance,
                        DEFAULT_TEST_EPSILON,
                        &message,
                    );
                }
            }
        }
    }
}
