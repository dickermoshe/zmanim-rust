use chrono::{DateTime, Duration, TimeZone};

use crate::{
    astronomical_calendar::{AstronomicalCalendar, AstronomicalCalendarTrait},
    constants::*,
    geolocation::GeoLocation,
    math::multiply_duration,
    noaa_calculator::NOAACalculator,
};

/// TODO ADD DOCS
pub struct ZmanimCalendar<Tz: TimeZone> {
    /// TODO ADD DOCS
    pub astronomical_calendar: AstronomicalCalendar<Tz>,
    /// TODO ADD DOCS
    pub use_astronomical_chatzos: bool,
    /// TODO ADD DOCS
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    /// TODO ADD DOCS
    pub candle_lighting_offset: Duration,
}

pub trait ZmanimCalendarTrait<Tz: TimeZone>: AstronomicalCalendarTrait<Tz> {
    fn get_astronomical_calendar(&self) -> &AstronomicalCalendar<Tz>;
    fn get_use_astronomical_chatzos(&self) -> bool;
    fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool;
    fn get_candle_lighting_offset(&self) -> Duration;

    fn get_tzais(&self) -> Option<DateTime<Tz>> {
        self.get_astronomical_calendar()
            .get_sunset_offset_by_degrees(_ZENITH_8_POINT_5)
    }

    fn get_alos_hashachar(&self) -> Option<DateTime<Tz>> {
        self.get_astronomical_calendar()
            .get_sunrise_offset_by_degrees(_ZENITH_16_POINT_1)
    }
    fn get_alos_72(&self) -> Option<DateTime<Tz>> {
        self.get_astronomical_calendar()
            .get_sunrise()
            .map(|sunrise| sunrise - Duration::minutes(72))
    }
    fn get_chatzos(&self) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos() {
            self.get_astronomical_calendar().get_sun_transit()
        } else {
            self.get_chatzos_as_half_day()
                .or(self.get_astronomical_calendar().get_sun_transit())
        }
    }
    fn get_chatzos_as_half_day(&self) -> Option<DateTime<Tz>> {
        let sunrise = self.get_astronomical_calendar().get_sea_level_sunrise()?;
        let sunset = self.get_astronomical_calendar().get_sea_level_sunset()?;
        let chatzos = self
            .get_astronomical_calendar()
            .get_sun_transit_from_times(sunrise, sunset)?;
        Some(chatzos)
    }

    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64> {
        let sea_level_sunrise = self.get_astronomical_calendar().get_sea_level_sunrise();
        let sea_level_sunset = self.get_astronomical_calendar().get_sea_level_sunset();

        let twilight = if sunset {
            self.get_astronomical_calendar()
                .get_sunset_offset_by_degrees(_GEOMETRIC_ZENITH + degrees)
        } else {
            self.get_astronomical_calendar()
                .get_sunrise_offset_by_degrees(_GEOMETRIC_ZENITH + degrees)
        };

        match (sea_level_sunrise, sea_level_sunset, twilight) {
            (Some(sunrise), Some(sunset_time), Some(twilight_time)) => {
                let shaah_zmanis =
                    (sunset_time.timestamp_millis() - sunrise.timestamp_millis()) as f64 / 12.0;
                let rise_set_to_twilight = if sunset {
                    twilight_time - sunset_time
                } else {
                    sunrise - twilight_time
                };
                let rise_set_to_twilight_millis = rise_set_to_twilight.num_milliseconds() as f64;
                Some(rise_set_to_twilight_millis / shaah_zmanis)
            }
            _ => None,
        }
    }

    fn get_half_day_based_zman(
        &self,
        start_of_half_day: DateTime<Tz>,
        end_of_half_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis =
            self.get_half_day_based_shaah_zmanis(&start_of_half_day, &end_of_half_day)?;
        if hours >= 0.0 {
            Some(start_of_half_day + multiply_duration(shaah_zmanis, hours))
        } else {
            Some(end_of_half_day + multiply_duration(shaah_zmanis, hours))
        }
    }

    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration> {
        Some((end_of_half_day.clone() - start_of_half_day) / 6)
    }

    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis = self
            .get_astronomical_calendar()
            .get_temporal_hour_from_times(&start_of_day, &end_of_day)?;

        Some(start_of_day + multiply_duration(shaah_zmanis, hours))
    }

    fn _get_sof_zman_shma(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(start_of_day, self.get_chatzos()?, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day, end_of_day?, 3.0)
        }
    }

    fn get_sof_zman_shma_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_sof_zman_shma(start_of_day, Some(end_of_day), false)
    }

    fn get_sof_zman_shma_gra(&self) -> Option<DateTime<Tz>> {
        self._get_sof_zman_shma(
            self.get_astronomical_calendar().get_sunrise()?,
            self.get_astronomical_calendar().get_sunset(),
            true,
        )
    }

    fn get_sof_zman_shma_mga(&self) -> Option<DateTime<Tz>> {
        self._get_sof_zman_shma(self.get_alos_72()?, self.get_tzais_72(), true)
    }

    fn get_tzais_72(&self) -> Option<DateTime<Tz>> {
        self.get_astronomical_calendar()
            .get_sunset()
            .map(|sunset| sunset + Duration::minutes(72))
    }
    // Broken
    fn get_candle_lighting(&self) -> Option<DateTime<Tz>> {
        self.get_astronomical_calendar()
            .get_sea_level_sunset()
            .map(|sunset| sunset - self.get_candle_lighting_offset())
    }

    fn _get_sof_zman_tfila(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(start_of_day, self.get_chatzos()?, 4.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day, end_of_day?, 4.0)
        }
    }

    fn get_sof_zman_tfila_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_sof_zman_tfila(start_of_day, Some(end_of_day), false)
    }

    fn get_sof_zman_tfila_gra(&self) -> Option<DateTime<Tz>> {
        self._get_sof_zman_tfila(
            self.get_astronomical_calendar().get_sunrise()?,
            self.get_astronomical_calendar().get_sunset(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga(&self) -> Option<DateTime<Tz>> {
        self._get_sof_zman_tfila(self.get_alos_72()?, self.get_tzais_72(), true)
    }

    fn _get_mincha_gedola(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 0.5)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 6.5)
        }
    }

    fn get_mincha_gedola_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_mincha_gedola(Some(start_of_day), end_of_day, false)
    }

    fn get_mincha_gedola_default(&self) -> Option<DateTime<Tz>> {
        self._get_mincha_gedola(
            self.get_astronomical_calendar().get_sunrise(),
            self.get_astronomical_calendar().get_sunset()?,
            true,
        )
    }

    fn _get_samuch_le_mincha_ketana(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 9.0)
        }
    }

    fn get_samuch_le_mincha_ketana_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_samuch_le_mincha_ketana(Some(start_of_day), end_of_day, false)
    }

    fn _get_mincha_ketana(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 3.5)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 9.5)
        }
    }

    fn get_mincha_ketana_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_mincha_ketana(Some(start_of_day), end_of_day, false)
    }

    fn get_mincha_ketana_default(&self) -> Option<DateTime<Tz>> {
        self._get_mincha_ketana(
            self.get_astronomical_calendar().get_sunrise(),
            self.get_astronomical_calendar().get_sunset()?,
            true,
        )
    }

    fn _get_plag_hamincha(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.get_use_astronomical_chatzos_for_other_zmanim() && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 4.75)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 10.75)
        }
    }

    fn get_plag_hamincha_simple(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        self._get_plag_hamincha(Some(start_of_day), end_of_day, false)
    }

    fn get_plag_hamincha_default(&self) -> Option<DateTime<Tz>> {
        self._get_plag_hamincha(
            self.get_astronomical_calendar().get_sunrise(),
            self.get_astronomical_calendar().get_sunset()?,
            true,
        )
    }

    fn get_shaah_zmanis_gra(&self) -> Option<Duration> {
        self.get_astronomical_calendar()
            .get_temporal_hour_from_times(
                &self.get_astronomical_calendar().get_sunrise()?,
                &self.get_astronomical_calendar().get_sunset()?,
            )
    }

    fn get_shaah_zmanis_mga(&self) -> Option<Duration> {
        self.get_astronomical_calendar()
            .get_temporal_hour_from_times(&self.get_alos_72()?, &self.get_tzais_72()?)
    }
}

impl<Tz: TimeZone> ZmanimCalendar<Tz> {
    pub fn new(astronomical_calendar: AstronomicalCalendar<Tz>) -> Self {
        Self {
            astronomical_calendar,
            use_astronomical_chatzos: false,
            use_astronomical_chatzos_for_other_zmanim: false,
            candle_lighting_offset: Duration::minutes(10),
        }
    }
}
impl<Tz: TimeZone> ZmanimCalendarTrait<Tz> for ZmanimCalendar<Tz> {
    fn get_astronomical_calendar(&self) -> &AstronomicalCalendar<Tz> {
        &self.astronomical_calendar
    }
    fn get_use_astronomical_chatzos(&self) -> bool {
        self.use_astronomical_chatzos
    }
    fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool {
        self.use_astronomical_chatzos_for_other_zmanim
    }
    fn get_candle_lighting_offset(&self) -> Duration {
        self.candle_lighting_offset
    }
}

impl<Tz: TimeZone> AstronomicalCalendarTrait<Tz> for ZmanimCalendar<Tz> {
    fn get_date_time(&self) -> &DateTime<Tz> {
        self.astronomical_calendar.get_date_time()
    }
    fn get_geo_location(&self) -> &GeoLocation {
        self.astronomical_calendar.get_geo_location()
    }
    fn get_noaa_calculator(&self) -> &NOAACalculator {
        self.astronomical_calendar.get_noaa_calculator()
    }
}

#[cfg(test)]
mod jni_tests {

    use std::f64;

    use crate::{
        geolocation::GeoLocation,
        test_utils::jni::{
            DEFAULT_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, RandomGeoLocation,
            assert_almost_equal_f64_option, assert_almost_equal_i64_option, create_java_calendar,
            create_java_geo_location, create_java_zmanim_calendar, init_jvm, random_date_time,
        },
    };

    use super::*;

    use j4rs::{Instance, InvocationArg, Jvm};
    use rand::Rng;

    struct TestCase {
        java_zmanim_calendar: Instance,
        calendar: ZmanimCalendar<chrono_tz::Tz>,
        message: String,
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

        let astronomical_calendar = AstronomicalCalendar {
            date_time,
            geo_location: geo_location.unwrap(),
            noaa_calculator: NOAACalculator,
        };

        let use_astronomical_chatzos = rand::thread_rng().gen_bool(0.5);
        let use_astronomical_chatzos_for_other_zmanim = rand::thread_rng().gen_bool(0.5);
        let candle_lighting_offset =
            Duration::milliseconds(rand::thread_rng().gen_range(-60000..=60000));
        let java_zmanim_calendar = create_java_zmanim_calendar(
            &jvm,
            java_geo_location.unwrap(),
            java_calendar,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
        );

        Some(TestCase {
            calendar: ZmanimCalendar {
                astronomical_calendar,
                use_astronomical_chatzos,
                use_astronomical_chatzos_for_other_zmanim,
                candle_lighting_offset,
            },
            java_zmanim_calendar,
            message,
        })
    }

    #[test]
    fn test_get_tzais_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_tzais().map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getTzais",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_alos_hashachar_against_java() {
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
                .get_alos_hashachar()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getAlosHashachar",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_alos_72_against_java() {
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
                .get_alos_72()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getAlos72",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_chatzos_against_java() {
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
                .get_chatzos()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getChatzos",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_chatzos_as_half_day_against_java() {
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
                .get_chatzos_as_half_day()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getChatzosAsHalfDay",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_shma_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_sof_zman_shma_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getSofZmanShma",
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

    #[test]
    fn test_get_sof_zman_shma_gra_against_java() {
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
                .get_sof_zman_shma_gra()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getSofZmanShmaGRA",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_shma_mga_against_java() {
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
                .get_sof_zman_shma_mga()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getSofZmanShmaMGA",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_tzais_72_against_java() {
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
                .get_tzais_72()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getTzais72",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_candle_lighting_against_java() {
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
                .get_candle_lighting()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getCandleLighting",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_tfila_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_sof_zman_tfila_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getSofZmanTfila",
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

    #[test]
    fn test_get_sof_zman_tfila_gra_against_java() {
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
                .get_sof_zman_tfila_gra()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getSofZmanTfilaGRA",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_tfila_mga_against_java() {
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
                .get_sof_zman_tfila_mga()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getSofZmanTfilaMGA",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_mincha_gedola_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_mincha_gedola_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getMinchaGedola",
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

    #[test]
    fn test_get_mincha_gedola_default_against_java() {
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
                .get_mincha_gedola_default()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getMinchaGedola",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_samuch_le_mincha_ketana_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_samuch_le_mincha_ketana_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getSamuchLeMinchaKetana",
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

    #[test]
    fn test_get_mincha_ketana_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_mincha_ketana_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getMinchaKetana",
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

    #[test]
    fn test_get_mincha_ketana_default_against_java() {
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
                .get_mincha_ketana_default()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getMinchaKetana",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_plag_hamincha_simple_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();

            let result = test_case
                .calendar
                .get_plag_hamincha_simple(start_of_day.clone(), end_of_day.clone())
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getPlagHamincha",
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

    #[test]
    fn test_get_plag_hamincha_default_against_java() {
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
                .get_plag_hamincha_default()
                .map(|d| d.timestamp_millis());

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getPlagHamincha",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_shaah_zmanis_gra_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_shaah_zmanis_gra();

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getShaahZmanisGra",
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
    fn test_get_shaah_zmanis_mga_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let result = test_case.calendar.get_shaah_zmanis_mga();

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getShaahZmanisMGA",
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
    fn test_get_shaah_zmanis_based_zman_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let hours = rand::thread_rng().gen_range(0.0..=12.0);

            let result = test_case
                .calendar
                .get_shaah_zmanis_based_zman(start_of_day.clone(), end_of_day.clone(), hours)
                .map(|d| d.timestamp_millis());

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
                    &test_case.java_zmanim_calendar,
                    "getShaahZmanisBasedZman",
                    &[
                        InvocationArg::from(java_start),
                        InvocationArg::from(java_end),
                        InvocationArg::try_from(hours)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_percent_of_shaah_zmanis_from_degrees_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let degrees = rand::thread_rng().gen_range(-100.0..=100.0);
            let sunset = rand::thread_rng().gen_bool(0.5);

            let result = test_case
                .calendar
                .get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getPercentOfShaahZmanisFromDegrees",
                    &[
                        InvocationArg::try_from(degrees)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                        InvocationArg::try_from(sunset)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_result = jvm.to_rust::<f64>(java_result).ok();
            let java_result = if java_result == Some(5e-324) {
                None
            } else {
                java_result
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
    fn test_get_half_day_based_zman_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_half_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_half_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_half_day.is_none() || end_of_half_day.is_none() {
                continue;
            }

            let start_of_half_day = start_of_half_day.unwrap();
            let end_of_half_day = end_of_half_day.unwrap();
            let hours = rand::thread_rng().gen_range(-6.0..=6.0);

            let result = test_case
                .calendar
                .get_half_day_based_zman(start_of_half_day.clone(), end_of_half_day.clone(), hours)
                .map(|d| d.timestamp_millis());

            let java_start = jvm
                .create_instance(
                    "java.util.Date",
                    &[
                        InvocationArg::try_from(start_of_half_day.timestamp_millis())
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_end = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(end_of_half_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getHalfDayBasedZman",
                    &[
                        InvocationArg::from(java_start),
                        InvocationArg::from(java_end),
                        InvocationArg::try_from(hours)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_result = get_java_date_millis(&jvm, &java_result);

            assert_almost_equal_i64_option(&result, &java_result, 1000, &test_case.message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_half_day_based_shaah_zmanis_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_test_case(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let test_case = test_case.unwrap();

            let start_of_half_day = test_case.calendar.get_astronomical_calendar().get_sunrise();
            let end_of_half_day = test_case.calendar.get_astronomical_calendar().get_sunset();

            if start_of_half_day.is_none() || end_of_half_day.is_none() {
                continue;
            }

            let start_of_half_day = start_of_half_day.unwrap();
            let end_of_half_day = end_of_half_day.unwrap();

            let result = test_case
                .calendar
                .get_half_day_based_shaah_zmanis(&start_of_half_day, &end_of_half_day);

            let java_start = jvm
                .create_instance(
                    "java.util.Date",
                    &[
                        InvocationArg::try_from(start_of_half_day.timestamp_millis())
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            let java_end = jvm
                .create_instance(
                    "java.util.Date",
                    &[InvocationArg::try_from(end_of_half_day.timestamp_millis())
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap();

            let java_result = jvm
                .invoke(
                    &test_case.java_zmanim_calendar,
                    "getHalfDayBasedShaahZmanis",
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
}
