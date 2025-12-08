use chrono::{DateTime, Duration, TimeZone};
use j4rs::{Instance, InvocationArg, Jvm, Null};

use crate::{
    astronomical_calculator::NOAACalculator,
    constants::Zman,
    defmt::DefmtFormatTrait,
    geolocation::GeoLocation,
    tests::{
        astronomical_calendar_test::JavaAstronomicalCalendar,
        test_utils::{dt_to_java_calendar, dt_to_java_date, geolocation_to_java_geolocation},
    },
    zmanim_calendar::{InternalZmanimCalendarTrait, ZmanimCalendarTrait},
};

pub struct JavaZmanimCalendar<'a, Tz: TimeZone> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
    pub date_time: DateTime<Tz>,
}

impl<'a, Tz: TimeZone> DefmtFormatTrait for JavaZmanimCalendar<'a, Tz> {}

impl<'a, Tz: TimeZone> JavaZmanimCalendar<'a, Tz> {
    pub fn new(
        jvm: &'a Jvm,
        date_time: DateTime<Tz>,
        geo_location: GeoLocation,
        candle_lighting_offset: Duration,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        ateret_torah_sunset_offset: Duration,
    ) -> Self {
        let java_geolocation = geolocation_to_java_geolocation(jvm, &geo_location, date_time.timezone()).unwrap();
        let java_date_time = dt_to_java_calendar(jvm, &date_time).unwrap();
        let java_zmanim_calendar = jvm
            .create_instance(
                "com.kosherjava.zmanim.ComplexZmanimCalendar",
                &[InvocationArg::from(java_geolocation)],
            )
            .unwrap();
        jvm.invoke(
            &java_zmanim_calendar,
            "setCalendar",
            &[InvocationArg::from(java_date_time)],
        )
        .unwrap();

        jvm.invoke(
            &java_zmanim_calendar,
            "setUseElevation",
            &[InvocationArg::try_from(true).unwrap().into_primitive().unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &java_zmanim_calendar,
            "setUseAstronomicalChatzos",
            &[InvocationArg::try_from(use_astronomical_chatzos)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &java_zmanim_calendar,
            "setUseAstronomicalChatzosForOtherZmanim",
            &[InvocationArg::try_from(use_astronomical_chatzos_for_other_zmanim)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &java_zmanim_calendar,
            "setAteretTorahSunsetOffset",
            &[
                InvocationArg::try_from(ateret_torah_sunset_offset.as_seconds_f64() / 60.0)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            ],
        )
        .unwrap();

        jvm.invoke(
            &java_zmanim_calendar,
            "setCandleLightingOffset",
            &[InvocationArg::try_from(candle_lighting_offset.as_seconds_f64() / 60.0)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();

        Self {
            jvm,
            instance: java_zmanim_calendar,
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

    fn get_java_duration_millis(&self, duration_instance: Instance) -> Option<i64> {
        let millis = self.jvm.to_rust::<i64>(duration_instance).ok()?;
        // DIFF: Java returns Long.MIN_VALUE (-9223372036854775808) to indicate null/None
        if millis == -9223372036854775808i64 {
            None
        } else {
            Some(millis)
        }
    }
}

impl<'a, Tz: TimeZone> ZmanimCalendarTrait<Tz, GeoLocation, NOAACalculator, JavaAstronomicalCalendar<'a, Tz>>
    for JavaZmanimCalendar<'a, Tz>
{
    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64> {
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getPercentOfShaahZmanisFromDegrees",
                &[
                    InvocationArg::try_from(degrees).unwrap().into_primitive().unwrap(),
                    InvocationArg::try_from(sunset).unwrap().into_primitive().unwrap(),
                ],
            )
            .ok()?;
        let result = self.jvm.to_rust::<f64>(java_result).ok()?;
        if result == 5e-324 { None } else { Some(result) }
    }

    fn get_shaah_zmanis_gra(&self) -> Option<Duration> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getShaahZmanisGra", InvocationArg::empty())
            .ok()?;
        self.get_java_duration_millis(java_result).map(Duration::milliseconds)
    }

    fn get_shaah_zmanis_mga(&self) -> Option<Duration> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getShaahZmanisMGA", InvocationArg::empty())
            .ok()?;
        self.get_java_duration_millis(java_result).map(Duration::milliseconds)
    }

    fn get_half_day_based_zman_from_times(
        &self,
        start_of_half_day: DateTime<Tz>,
        end_of_half_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, &start_of_half_day);
        let java_end = dt_to_java_date(self.jvm, &end_of_half_day);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getHalfDayBasedZman",
                &[
                    InvocationArg::from(java_start),
                    InvocationArg::from(java_end),
                    InvocationArg::try_from(hours).unwrap().into_primitive().unwrap(),
                ],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_half_day_based_shaah_zmanis_from_times(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration> {
        let java_start = dt_to_java_date(self.jvm, start_of_half_day);
        let java_end = dt_to_java_date(self.jvm, end_of_half_day);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getHalfDayBasedShaahZmanis",
                &[InvocationArg::from(java_start), InvocationArg::from(java_end)],
            )
            .ok()?;
        self.get_java_duration_millis(java_result).map(Duration::milliseconds)
    }

    fn get_shaah_zmanis_based_zman_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, &start_of_day);
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getShaahZmanisBasedZman",
                &[
                    InvocationArg::from(java_start),
                    InvocationArg::from(java_end),
                    InvocationArg::try_from(hours).unwrap().into_primitive().unwrap(),
                ],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sof_zman_shma_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, &start_of_day);
        let java_end = if let Some(end_of_day) = end_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &end_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSofZmanShma",
                &[InvocationArg::from(java_start), java_end, java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = if let Some(start_of_day) = start_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &start_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getMinchaKetana",
                &[java_start, InvocationArg::from(java_end), java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sof_zman_tfila_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = dt_to_java_date(self.jvm, &start_of_day);
        let java_end = if let Some(end_of_day) = end_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &end_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSofZmanTfila",
                &[InvocationArg::from(java_start), java_end, java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_mincha_gedola_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = if let Some(start_of_day) = start_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &start_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getMinchaGedola",
                &[java_start, InvocationArg::from(java_end), java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_plag_hamincha_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = if let Some(start_of_day) = start_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &start_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getPlagHamincha",
                &[java_start, InvocationArg::from(java_end), java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_samuch_le_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        let java_start = if let Some(start_of_day) = start_of_day {
            InvocationArg::from(dt_to_java_date(self.jvm, &start_of_day))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_end = dt_to_java_date(self.jvm, &end_of_day);
        let java_synchronous = InvocationArg::try_from(synchronous).unwrap().into_primitive().unwrap();
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSamuchLeMinchaKetana",
                &[java_start, InvocationArg::from(java_end), java_synchronous],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sof_zman_kidush_levana_15_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let java_alos = if let Some(alos) = alos {
            InvocationArg::from(dt_to_java_date(self.jvm, alos))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_tzais = if let Some(tzais) = tzais {
            InvocationArg::from(dt_to_java_date(self.jvm, tzais))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_result = self
            .jvm
            .invoke(&self.instance, "getSofZmanKidushLevana15Days", &[java_alos, java_tzais])
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_sof_zman_kidush_levana_between_moldos_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let java_alos = if let Some(alos) = alos {
            InvocationArg::from(dt_to_java_date(self.jvm, alos))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_tzais = if let Some(tzais) = tzais {
            InvocationArg::from(dt_to_java_date(self.jvm, tzais))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getSofZmanKidushLevanaBetweenMoldos",
                &[java_alos, java_tzais],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_tchilas_zman_kidush_levana_3_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let java_alos = if let Some(alos) = alos {
            InvocationArg::from(dt_to_java_date(self.jvm, alos))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_tzais = if let Some(tzais) = tzais {
            InvocationArg::from(dt_to_java_date(self.jvm, tzais))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getTchilasZmanKidushLevana3Days",
                &[java_alos, java_tzais],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let java_alos = if let Some(alos) = alos {
            InvocationArg::from(dt_to_java_date(self.jvm, alos))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_tzais = if let Some(tzais) = tzais {
            InvocationArg::from(dt_to_java_date(self.jvm, tzais))
        } else {
            InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
        };
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "getTchilasZmanKidushLevana7Days",
                &[java_alos, java_tzais],
            )
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn get_zman(&self, zman: &Zman) -> Option<DateTime<Tz>> {
        let method_name = match zman {
            Zman::AlosHashachar => "getAlosHashachar",
            Zman::Alos72 => "getAlos72",
            Zman::Chatzos => "getChatzos",
            Zman::ChatzosAsHalfDay => "getChatzosAsHalfDay",
            Zman::MinchaGedola => "getMinchaGedola",
            Zman::MinchaKetana => "getMinchaKetana",
            Zman::PlagHamincha => "getPlagHamincha",
            Zman::SofZmanShmaGRA => "getSofZmanShmaGRA",
            Zman::SofZmanShmaMGA => "getSofZmanShmaMGA",
            Zman::SofZmanTfilaGRA => "getSofZmanTfilaGRA",
            Zman::SofZmanTfilaMGA => "getSofZmanTfilaMGA",
            Zman::Tzais => "getTzais",
            Zman::Tzais72 => "getTzais72",
            Zman::CandleLighting => "getCandleLighting",
        };
        let java_result = self
            .jvm
            .invoke(&self.instance, method_name, InvocationArg::empty())
            .ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }
}

impl<'a, Tz: TimeZone> InternalZmanimCalendarTrait<Tz, GeoLocation, NOAACalculator, JavaAstronomicalCalendar<'a, Tz>>
    for JavaZmanimCalendar<'a, Tz>
{
    fn get_astronomical_calendar(&self) -> &JavaAstronomicalCalendar<'a, Tz> {
        unimplemented!("get_astronomical_calendar is not implemented in this test and should not be called");
    }

    fn get_use_astronomical_chatzos(&self) -> bool {
        unimplemented!("get_use_astronomical_chatzos is not implemented in this test and should not be called");
    }
    fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool {
        unimplemented!(
            "get_use_astronomical_chatzos_for_other_zmanim is not implemented in this test and should not be called"
        );
    }
    fn get_candle_lighting_offset(&self) -> Duration {
        unimplemented!("get_candle_lighting_offset is not implemented in this test and should not be called");
    }
    fn get_ateret_torah_sunset_offset(&self) -> Duration {
        unimplemented!("get_ateret_torah_sunset_offset is not implemented in this test and should not be called");
    }
}

#[cfg(test)]
mod jni_tests {
    use rand::Rng;

    use crate::{
        astronomical_calendar::{AstronomicalCalendar, AstronomicalCalendarTrait},
        tests::{
            geolocation_test::random_geolocations,
            test_utils::{
                DEFAULT_F64_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64_option,
                assert_almost_equal_i64_option, init_jvm, random_date_time,
            },
        },
        zmanim_calendar::ZmanimCalendar,
    };

    use super::*;
    #[allow(clippy::type_complexity)]
    fn create_zmanim_calendars<'a>(
        jvm: &'a Jvm,
    ) -> Option<(
        ZmanimCalendar<
            chrono_tz::Tz,
            GeoLocation,
            NOAACalculator,
            AstronomicalCalendar<chrono_tz::Tz, GeoLocation, NOAACalculator>,
        >,
        JavaZmanimCalendar<'a, chrono_tz::Tz>,
    )> {
        let mut rng = rand::thread_rng();
        let (geo_location, java_geo_location) = random_geolocations(jvm, &mut rng)?;

        let date_time = random_date_time(&mut rng, java_geo_location.timezone);
        let candle_lighting_offset = Duration::minutes(rng.gen_range(0..=60));
        let use_astronomical_chatzos = rng.gen_bool(0.5);
        let use_astronomical_chatzos_for_other_zmanim = rng.gen_bool(0.5);
        let ateret_torah_sunset_offset = Duration::minutes(rng.gen_range(0..=60));

        let rust_astronomical_calendar = AstronomicalCalendar::new(date_time, geo_location.clone(), NOAACalculator);
        let rust_calendar = ZmanimCalendar::new(
            rust_astronomical_calendar,
            candle_lighting_offset,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            ateret_torah_sunset_offset,
        );
        let java_calendar = JavaZmanimCalendar::new(
            jvm,
            date_time,
            geo_location,
            candle_lighting_offset,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            ateret_torah_sunset_offset,
        );

        Some((rust_calendar, java_calendar))
    }

    #[test]
    fn test_get_zman_against_java() {
        let jvm = init_jvm();
        let zman_variants = [
            Zman::AlosHashachar,
            Zman::Alos72,
            Zman::Chatzos,
            Zman::ChatzosAsHalfDay,
            Zman::MinchaGedola,
            Zman::MinchaKetana,
            Zman::PlagHamincha,
            Zman::SofZmanShmaGRA,
            Zman::SofZmanShmaMGA,
            Zman::SofZmanTfilaGRA,
            Zman::SofZmanTfilaMGA,
            Zman::Tzais,
            Zman::Tzais72,
            Zman::CandleLighting,
        ];

        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            for zman in &zman_variants {
                let test_case = create_zmanim_calendars(&jvm);
                if test_case.is_none() {
                    continue;
                }
                ran = true;
                let (rust_calendar, java_calendar) = test_case.unwrap();

                let result = rust_calendar.get_zman(zman).map(|d| d.timestamp_millis());
                let java_result = java_calendar.get_zman(zman).map(|d| d.timestamp_millis());

                assert_almost_equal_i64_option(
                    &result,
                    &java_result,
                    50,
                    &format!("get_zman({:?}) against java with calendar {:?}", zman, rust_calendar),
                );
            }
        }
        assert!(ran, "No test cases were ran");
    }

    #[test]
    fn test_get_percent_of_shaah_zmanis_from_degrees_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let degrees = rand::thread_rng().gen_range(-100.0..=100.0);
            let sunset = rand::thread_rng().gen_bool(0.5);

            let result = rust_calendar.get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);
            let java_result = java_calendar.get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);

            assert_almost_equal_f64_option(
                &result,
                &java_result,
                DEFAULT_F64_TEST_EPSILON,
                &format!("get_percent_of_shaah_zmanis_from_degrees({}, {})", degrees, sunset),
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_shaah_zmanis_gra_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let result = rust_calendar.get_shaah_zmanis_gra().map(|d| d.num_milliseconds());
            let java_result = java_calendar.get_shaah_zmanis_gra().map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_shaah_zmanis_gra");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_shaah_zmanis_mga_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let result = rust_calendar.get_shaah_zmanis_mga().map(|d| d.num_milliseconds());
            let java_result = java_calendar.get_shaah_zmanis_mga().map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_shaah_zmanis_mga");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_half_day_based_zman_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_half_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_half_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_half_day.is_none() || end_of_half_day.is_none() {
                continue;
            }

            let start_of_half_day = start_of_half_day.unwrap();
            let end_of_half_day = end_of_half_day.unwrap();
            let hours = rand::thread_rng().gen_range(-6.0..=6.0);

            let result = rust_calendar
                .get_half_day_based_zman_from_times(start_of_half_day, end_of_half_day, hours)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_half_day_based_zman_from_times(start_of_half_day, end_of_half_day, hours)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_half_day_based_zman_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_half_day_based_shaah_zmanis_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_half_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_half_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_half_day.is_none() || end_of_half_day.is_none() {
                continue;
            }

            let start_of_half_day = start_of_half_day.unwrap();
            let end_of_half_day = end_of_half_day.unwrap();

            let result = rust_calendar
                .get_half_day_based_shaah_zmanis_from_times(&start_of_half_day, &end_of_half_day)
                .map(|d| d.num_milliseconds());
            let java_result = java_calendar
                .get_half_day_based_shaah_zmanis_from_times(&start_of_half_day, &end_of_half_day)
                .map(|d| d.num_milliseconds());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_half_day_based_shaah_zmanis_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_shaah_zmanis_based_zman_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let hours = rand::thread_rng().gen_range(0.0..=12.0);

            let result = rust_calendar
                .get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day, hours)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day, hours)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_shaah_zmanis_based_zman_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_shma_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let end_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(end_of_day)
            };

            let result = rust_calendar
                .get_sof_zman_shma_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_sof_zman_shma_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_sof_zman_shma_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_tfila_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let end_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(end_of_day)
            };

            let result = rust_calendar
                .get_sof_zman_tfila_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_sof_zman_tfila_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_sof_zman_tfila_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_mincha_gedola_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let start_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(start_of_day)
            };

            let result = rust_calendar
                .get_mincha_gedola_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_mincha_gedola_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_mincha_gedola_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_mincha_ketana_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let start_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(start_of_day)
            };

            let result = rust_calendar
                .get_mincha_ketana_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_mincha_ketana_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_mincha_ketana_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_plag_hamincha_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let start_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(start_of_day)
            };

            let result = rust_calendar
                .get_plag_hamincha_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_plag_hamincha_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_plag_hamincha_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_samuch_le_mincha_ketana_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar) = test_case.unwrap();

            let start_of_day = rust_calendar.get_astronomical_calendar().get_sunrise();
            let end_of_day = rust_calendar.get_astronomical_calendar().get_sunset();

            if start_of_day.is_none() || end_of_day.is_none() {
                continue;
            }

            let start_of_day = start_of_day.unwrap();
            let end_of_day = end_of_day.unwrap();
            let synchronous = rand::thread_rng().gen_bool(0.5);
            let start_of_day = if rand::thread_rng().gen_bool(0.5) {
                None
            } else {
                Some(start_of_day)
            };

            let result = rust_calendar
                .get_samuch_le_mincha_ketana_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_samuch_le_mincha_ketana_from_times(start_of_day, end_of_day, synchronous)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(&result, &java_result, 50, "get_samuch_le_mincha_ketana_from_times");
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_kidush_levana_15_days_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar) = test_case.unwrap();

            // Get alos and tzais
            let alos = rust_calendar.get_zman(&Zman::Alos72);
            let tzais = rust_calendar.get_zman(&Zman::Tzais72);

            // Add some random offset
            let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let alos = alos.map(|d| d + alos_offset);
            let tzais = tzais.map(|d| d + tzais_offset);

            // Randomly decide whether to pass None or Some
            let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
            let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
            let alos_for_test = if rand::thread_rng().gen_bool(0.5) {
                &alos
            } else {
                &none_alos
            };
            let tzais_for_test = if rand::thread_rng().gen_bool(0.5) {
                &tzais
            } else {
                &none_tzais
            };

            ran = true;

            let result = rust_calendar
                .get_sof_zman_kidush_levana_15_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_sof_zman_kidush_levana_15_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                "get_sof_zman_kidush_levana_15_days_from_times",
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_sof_zman_kidush_levana_between_moldos_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar) = test_case.unwrap();

            // Get alos and tzais
            let alos = rust_calendar.get_zman(&Zman::Alos72);
            let tzais = rust_calendar.get_zman(&Zman::Tzais72);

            // Add some random offset
            let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let alos = alos.map(|d| d + alos_offset);
            let tzais = tzais.map(|d| d + tzais_offset);

            // Randomly decide whether to pass None or Some
            let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
            let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
            let alos_for_test = if rand::thread_rng().gen_bool(0.5) {
                &alos
            } else {
                &none_alos
            };
            let tzais_for_test = if rand::thread_rng().gen_bool(0.5) {
                &tzais
            } else {
                &none_tzais
            };

            ran = true;

            let result = rust_calendar
                .get_sof_zman_kidush_levana_between_moldos_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_sof_zman_kidush_levana_between_moldos_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                "get_sof_zman_kidush_levana_between_moldos_from_times",
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_tchilas_zman_kidush_levana_3_days_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar) = test_case.unwrap();

            // Get alos and tzais
            let alos = rust_calendar.get_zman(&Zman::Alos72);
            let tzais = rust_calendar.get_zman(&Zman::Tzais72);

            // Add some random offset
            let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let alos = alos.map(|d| d + alos_offset);
            let tzais = tzais.map(|d| d + tzais_offset);

            // Randomly decide whether to pass None or Some
            let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
            let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
            let alos_for_test = if rand::thread_rng().gen_bool(0.5) {
                &alos
            } else {
                &none_alos
            };
            let tzais_for_test = if rand::thread_rng().gen_bool(0.5) {
                &tzais
            } else {
                &none_tzais
            };

            ran = true;

            let result = rust_calendar
                .get_tchilas_zman_kidush_levana_3_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_tchilas_zman_kidush_levana_3_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                "get_tchilas_zman_kidush_levana_3_days_from_times",
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_tchilas_zman_kidush_levana_7_days_from_times_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_zmanim_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar) = test_case.unwrap();

            // Get alos and tzais
            let alos = rust_calendar.get_zman(&Zman::Alos72);
            let tzais = rust_calendar.get_zman(&Zman::Tzais72);

            // Add some random offset
            let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
            let alos = alos.map(|d| d + alos_offset);
            let tzais = tzais.map(|d| d + tzais_offset);

            // Randomly decide whether to pass None or Some
            let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
            let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
            let alos_for_test = if rand::thread_rng().gen_bool(0.5) {
                &alos
            } else {
                &none_alos
            };
            let tzais_for_test = if rand::thread_rng().gen_bool(0.5) {
                &tzais
            } else {
                &none_tzais
            };

            ran = true;

            let result = rust_calendar
                .get_tchilas_zman_kidush_levana_7_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());
            let java_result = java_calendar
                .get_tchilas_zman_kidush_levana_7_days_from_times(alos_for_test, tzais_for_test)
                .map(|d| d.timestamp_millis());

            assert_almost_equal_i64_option(
                &result,
                &java_result,
                50,
                "get_tchilas_zman_kidush_levana_7_days_from_times",
            );
        }
        assert!(ran, "No test cases were run");
    }
}
