use crate::constants::{_Formula, _MINUTE_MILLIS};
use chrono::{DateTime, Duration, Offset, TimeZone};
use core::f64::consts::PI;
#[allow(unused_imports)]
use core_maths::CoreFloat;
pub trait GeoLocationTrait {
    fn get_latitude(&self) -> f64;
    fn get_longitude(&self) -> f64;
    fn get_elevation(&self) -> f64;
    fn get_rhumb_line_distance(&self, location: &Self) -> f64;
    fn get_rhumb_line_bearing(&self, location: &Self) -> f64;
    fn get_geodesic_initial_bearing(&self, location: &Self) -> Option<f64>;
    fn get_geodesic_final_bearing(&self, location: &Self) -> Option<f64>;
    fn get_geodesic_distance(&self, location: &Self) -> Option<f64>;
    fn get_local_mean_time_offset<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> Duration;
    fn get_antimeridian_adjustment<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i8;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq, Default, PartialOrd)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
}
impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64, elevation: f64) -> Option<Self> {
        if latitude.is_nan() || longitude.is_nan() || elevation.is_nan() || elevation.is_infinite() {
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
    fn vincenty_inverse_formula(&self, location: &impl GeoLocationTrait, formula: _Formula) -> Option<f64> {
        let major_semi_axis = 6378137.0;
        let minor_semi_axis = 6356752.3142;
        let f = 1.0 / 298.257223563;
        let l = (location.get_longitude() - self.get_longitude()).to_radians();
        let u1 = ((1.0 - f) * self.get_latitude().to_radians().tan()).atan();
        let u2 = ((1.0 - f) * location.get_latitude().to_radians().tan()).atan();
        let sin_u1 = u1.sin();
        let cos_u1 = u1.cos();
        let sin_u2 = u2.sin();
        let cos_u2 = u2.cos();

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
            sin_lambda = lambda.sin();
            cos_lambda = lambda.cos();
            sin_sigma = ((cos_u2 * sin_lambda) * (cos_u2 * sin_lambda)
                + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda) * (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda))
                .sqrt();

            if sin_sigma == 0.0 {
                return Some(0.0);
            }

            cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            sigma = sin_sigma.atan2(cos_sigma);
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
                        + c * sin_sigma * (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

            iter_limit -= 1;
        }

        if iter_limit == 0 {
            return None;
        }

        let u_sq = cos_sq_alpha * (major_semi_axis * major_semi_axis - minor_semi_axis * minor_semi_axis)
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

        let fwd_az = (cos_u2 * sin_lambda)
            .atan2(cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda)
            .to_degrees();

        let rev_az = (cos_u1 * sin_lambda)
            .atan2(-sin_u1 * cos_u2 + cos_u1 * sin_u2 * cos_lambda)
            .to_degrees();

        match formula {
            _Formula::Distance => Some(distance),
            _Formula::InitialBearing => Some(fwd_az),
            _Formula::FinalBearing => Some(rev_az),
        }
    }
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
    fn get_rhumb_line_bearing(&self, location: &Self) -> f64 {
        let mut d_lon = (location.get_longitude() - self.get_longitude()).to_radians();
        let d_phi = ((location.get_latitude().to_radians() / 2.0 + PI / 4.0).tan()).ln()
            - ((self.get_latitude().to_radians() / 2.0 + PI / 4.0).tan()).ln();

        if d_lon.abs() > PI {
            d_lon = if d_lon > 0.0 {
                -(2.0 * PI - d_lon)
            } else {
                2.0 * PI + d_lon
            };
        }

        d_lon.atan2(d_phi).to_degrees()
    }

    fn get_rhumb_line_distance(&self, location: &Self) -> f64 {
        let earth_radius = 6378137.0;
        let d_lat = location.get_latitude().to_radians() - self.get_latitude().to_radians();
        let mut d_lon = (location.get_longitude().to_radians() - self.get_longitude().to_radians()).abs();
        let d_phi = ((location.get_latitude().to_radians() / 2.0 + PI / 4.0).tan()).ln()
            - ((self.get_latitude().to_radians() / 2.0 + PI / 4.0).tan()).ln();
        let mut q = d_lat / d_phi;

        if !q.is_finite() {
            q = self.get_latitude().to_radians().cos();
        }

        if d_lon > PI {
            d_lon = 2.0 * PI - d_lon;
        }

        let d = (d_lat * d_lat + q * q * d_lon * d_lon).sqrt();
        d * earth_radius
    }

    fn get_geodesic_initial_bearing(&self, location: &Self) -> Option<f64> {
        self.vincenty_inverse_formula(location, _Formula::InitialBearing)
    }

    fn get_geodesic_final_bearing(&self, location: &Self) -> Option<f64> {
        self.vincenty_inverse_formula(location, _Formula::FinalBearing)
    }

    fn get_geodesic_distance(&self, location: &Self) -> Option<f64> {
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
