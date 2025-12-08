use crate::constants::{_ZENITH_8_POINT_5, _ZENITH_16_POINT_1};
use crate::jewish_calendar::JewishCalendarTrait;
use crate::jewish_date::JewishDateTrait;
use crate::math::lossy_multiply_duration;
use crate::{
    astronomical_calculator::AstronomicalCalculatorTrait,
    astronomical_calendar::AstronomicalCalendarTrait,
    constants::{_GEOMETRIC_ZENITH, Zman},
    defmt::DefmtFormatTrait,
    geolocation::GeoLocationTrait,
    jewish_calendar::{InternalJewishCalendarTrait, JewishCalendar},
};
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};
use icu_calendar::{
    options::{DateAddOptions, Overflow},
    types::DateDuration,
};

pub(crate) trait InternalZmanimCalendarTrait<
    Tz: TimeZone,
    G: GeoLocationTrait,
    N: AstronomicalCalculatorTrait,
    J: AstronomicalCalendarTrait<Tz, G, N>,
>
{
    fn get_astronomical_calendar(&self) -> &J;
    fn get_use_astronomical_chatzos(&self) -> bool;
    #[allow(unused)]
    fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool;
    fn get_candle_lighting_offset(&self) -> Duration;
    #[allow(unused)]
    fn get_ateret_torah_sunset_offset(&self) -> Duration;
}
#[allow(private_bounds)]
pub trait ZmanimCalendarTrait<
    Tz: TimeZone,
    G: GeoLocationTrait,
    N: AstronomicalCalculatorTrait,
    J: AstronomicalCalendarTrait<Tz, G, N>,
>: Sized + DefmtFormatTrait + InternalZmanimCalendarTrait<Tz, G, N, J>
{
    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64>;
    fn get_shaah_zmanis_gra(&self) -> Option<Duration>;
    fn get_shaah_zmanis_mga(&self) -> Option<Duration>;
    fn get_zman(&self, zman: &Zman) -> Option<DateTime<Tz>>;
    fn get_half_day_based_zman_from_times(
        &self,
        start_of_half_day: DateTime<Tz>,
        end_of_half_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;
    fn get_half_day_based_shaah_zmanis_from_times(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration>;
    fn get_shaah_zmanis_based_zman_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_shma_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_tfila_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_gedola_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_plag_hamincha_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_samuch_le_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;
    fn get_sof_zman_kidush_levana_15_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_sof_zman_kidush_levana_between_moldos_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_tchilas_zman_kidush_levana_3_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZmanimCalendar<
    Tz: TimeZone,
    G: GeoLocationTrait,
    N: AstronomicalCalculatorTrait,
    J: AstronomicalCalendarTrait<Tz, G, N>,
> {
    pub astronomical_calendar: J,
    pub use_astronomical_chatzos: bool,
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    pub candle_lighting_offset: Duration,
    pub ateret_torah_sunset_offset: Duration,
    _phantom: core::marker::PhantomData<(Tz, G, N)>,
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait, J: AstronomicalCalendarTrait<Tz, G, N>>
    ZmanimCalendarTrait<Tz, G, N, J> for ZmanimCalendar<Tz, G, N, J>
{
    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_date().get_jewish_day_of_month() < 4
            || jewish_calendar.get_jewish_date().get_jewish_day_of_month() > 9
        {
            return None;
        }
        let molad_based_time = jewish_calendar.get_tchilaszman_kidush_levana_7_days()?;

        self._get_molad_based_time(molad_based_time, alos, tzais, true)
    }

    fn get_sof_zman_kidush_levana_15_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_date().get_jewish_day_of_month() < 11
            || jewish_calendar.get_jewish_date().get_jewish_day_of_month() > 17
        {
            return None;
        }
        let molad_based_time = jewish_calendar.get_sof_zman_kidush_levana_15_days()?;
        self._get_molad_based_time(molad_based_time, alos, tzais, false)
    }

    fn get_tchilas_zman_kidush_levana_3_days_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let mut jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_date().get_jewish_day_of_month() > 5
            && jewish_calendar.get_jewish_date().get_jewish_day_of_month() < 30
        {
            return None;
        }

        let zman = self._get_molad_based_time(
            jewish_calendar.get_tchilaszman_kidush_levana_3_days()?,
            alos,
            tzais,
            true,
        );
        if zman.is_none() && jewish_calendar.get_jewish_date().get_jewish_day_of_month() == 30 {
            let mut add_option = DateAddOptions::default();
            add_option.overflow = Some(Overflow::Constrain);

            jewish_calendar
                .jewish_date
                .hebrew_date
                .try_add_with_options(DateDuration::for_months(1), add_option)
                .ok()?;

            return self._get_molad_based_time(
                jewish_calendar.get_tchilaszman_kidush_levana_3_days()?,
                alos,
                tzais,
                true,
            );
        }
        zman
    }

    fn get_sof_zman_kidush_levana_between_moldos_from_times(
        &self,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_date().get_jewish_day_of_month() < 11
            || jewish_calendar.get_jewish_date().get_jewish_day_of_month() > 16
        {
            return None;
        }
        let molad_based_time = jewish_calendar.get_sof_zman_kidush_levana_between_moldos()?;

        self._get_molad_based_time(molad_based_time, alos, tzais, false)
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
                let shaah_zmanis = (sunset_time.timestamp_millis() - sunrise.timestamp_millis()) as f64 / 12.0;
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

    fn get_half_day_based_zman_from_times(
        &self,
        start_of_half_day: DateTime<Tz>,
        end_of_half_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis = self.get_half_day_based_shaah_zmanis_from_times(&start_of_half_day, &end_of_half_day)?;
        if hours >= 0.0 {
            Some(start_of_half_day + lossy_multiply_duration(shaah_zmanis, hours))
        } else {
            Some(end_of_half_day + lossy_multiply_duration(shaah_zmanis, hours))
        }
    }

    fn get_half_day_based_shaah_zmanis_from_times(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration> {
        Some((end_of_half_day.clone() - start_of_half_day) / 6)
    }

    fn get_shaah_zmanis_based_zman_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis = self
            .astronomical_calendar
            .get_temporal_hour_from_times(&start_of_day, &end_of_day)?;

        Some(start_of_day + lossy_multiply_duration(shaah_zmanis, hours))
    }

    fn get_sof_zman_shma_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(start_of_day, self.get_zman(&Zman::Chatzos)?, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day?, 3.0)
        }
    }

    fn get_mincha_gedola_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(self.get_zman(&Zman::Chatzos)?, end_of_day, 0.5)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 6.5)
        }
    }

    fn get_shaah_zmanis_gra(&self) -> Option<Duration> {
        self.astronomical_calendar.get_temporal_hour_from_times(
            &self.astronomical_calendar.get_sunrise()?,
            &self.astronomical_calendar.get_sunset()?,
        )
    }

    fn get_shaah_zmanis_mga(&self) -> Option<Duration> {
        self.astronomical_calendar
            .get_temporal_hour_from_times(&self.get_zman(&Zman::Alos72)?, &self.get_zman(&Zman::Tzais72)?)
    }
    fn get_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(self.get_zman(&Zman::Chatzos)?, end_of_day, 3.5)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 9.5)
        }
    }
    fn get_sof_zman_tfila_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(start_of_day, self.get_zman(&Zman::Chatzos)?, 4.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day?, 4.0)
        }
    }

    fn get_samuch_le_mincha_ketana_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(self.get_zman(&Zman::Chatzos)?, end_of_day, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 9.0)
        }
    }

    fn get_plag_hamincha_from_times(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(self.get_zman(&Zman::Chatzos)?, end_of_day, 4.75)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 10.75)
        }
    }

    fn get_zman(&self, zman: &Zman) -> Option<DateTime<Tz>> {
        let astro = self.get_astronomical_calendar();
        match zman {
            Zman::PlagHamincha => self.get_plag_hamincha_from_times(astro.get_sunrise(), astro.get_sunset()?, true),
            Zman::MinchaKetana => self.get_mincha_ketana_from_times(astro.get_sunrise(), astro.get_sunset()?, true),
            Zman::MinchaGedola => self.get_mincha_gedola_from_times(astro.get_sunrise(), astro.get_sunset()?, true),
            Zman::Tzais => astro.get_sunset_offset_by_degrees(_ZENITH_8_POINT_5),
            Zman::AlosHashachar => astro.get_sunrise_offset_by_degrees(_ZENITH_16_POINT_1),
            Zman::Alos72 => astro.get_sunrise().map(|sunrise| sunrise - Duration::minutes(72)),
            Zman::Chatzos => {
                if self.get_use_astronomical_chatzos() {
                    astro.get_sun_transit()
                } else {
                    self.get_zman(&Zman::ChatzosAsHalfDay).or(astro.get_sun_transit())
                }
            }
            Zman::ChatzosAsHalfDay => {
                let sunrise = astro.get_sea_level_sunrise()?;
                let sunset = astro.get_sea_level_sunset()?;
                astro.get_sun_transit_from_times(sunrise, sunset)
            }
            Zman::SofZmanShmaGRA => self.get_sof_zman_shma_from_times(astro.get_sunrise()?, astro.get_sunset(), true),
            Zman::SofZmanShmaMGA => {
                self.get_sof_zman_shma_from_times(self.get_zman(&Zman::Alos72)?, self.get_zman(&Zman::Tzais72), true)
            }
            Zman::Tzais72 => astro.get_sunset().map(|sunset| sunset + Duration::minutes(72)),
            Zman::CandleLighting => astro
                .get_sea_level_sunset()
                .map(|sunset| sunset - self.get_candle_lighting_offset()),
            Zman::SofZmanTfilaGRA => self.get_sof_zman_tfila_from_times(astro.get_sunrise()?, astro.get_sunset(), true),
            Zman::SofZmanTfilaMGA => {
                self.get_sof_zman_tfila_from_times(self.get_zman(&Zman::Alos72)?, self.get_zman(&Zman::Tzais72), true)
            }
        }
    }
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait, J: AstronomicalCalendarTrait<Tz, G, N>>
    InternalZmanimCalendarTrait<Tz, G, N, J> for ZmanimCalendar<Tz, G, N, J>
{
    fn get_astronomical_calendar(&self) -> &J {
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
    fn get_ateret_torah_sunset_offset(&self) -> Duration {
        self.ateret_torah_sunset_offset
    }
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait, J: AstronomicalCalendarTrait<Tz, G, N>>
    ZmanimCalendar<Tz, G, N, J>
{
    pub fn new(
        astronomical_calendar: J,
        candle_lighting_offset: Duration,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        ateret_torah_sunset_offset: Duration,
    ) -> Self {
        Self {
            astronomical_calendar,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
            ateret_torah_sunset_offset,
            _phantom: core::marker::PhantomData,
        }
    }

    fn _get_midnight_last_night(&self) -> Option<DateTime<Tz>> {
        let midnight = self
            .astronomical_calendar
            .get_date_time()
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;
        Some(midnight)
    }

    fn _get_midnight_tonight(&self) -> Option<DateTime<Tz>> {
        let midnight = self
            .astronomical_calendar
            .get_date_time()
            .clone()
            .checked_add_signed(chrono::Duration::days(1))?
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;
        Some(midnight)
    }

    fn _get_molad_based_time(
        &self,
        utc_molad_based_time: DateTime<Utc>,
        alos: &Option<DateTime<Tz>>,
        tzais: &Option<DateTime<Tz>>,
        techila: bool,
    ) -> Option<DateTime<Tz>> {
        let molad_based_time = self._localized_datetime(utc_molad_based_time);
        let last_midnight = self._get_midnight_last_night()?;
        let midnight_tonight = self._get_midnight_tonight()?;

        if molad_based_time < last_midnight || molad_based_time > midnight_tonight {
            None
        } else {
            match (alos, tzais) {
                (Some(alos), Some(tzais)) => {
                    if molad_based_time > *alos && molad_based_time < *tzais {
                        match techila {
                            true => Some(tzais.clone()),
                            false => Some(alos.clone()),
                        }
                    } else {
                        Some(molad_based_time)
                    }
                }
                (_, _) => Some(molad_based_time),
            }
        }
    }
    fn _get_jewish_calendar(&self) -> Option<JewishCalendar<N>> {
        JewishCalendar::from_gregorian_date(
            self.astronomical_calendar.get_date_time().year(),
            self.astronomical_calendar.get_date_time().month() as u8,
            self.astronomical_calendar.get_date_time().day() as u8,
            false,
            false,
            false,
            self.astronomical_calendar.get_calculator().clone(),
        )
    }
    fn _localized_datetime(&self, datetime: DateTime<Utc>) -> DateTime<Tz> {
        self.astronomical_calendar
            .get_date_time()
            .timezone()
            .from_utc_datetime(&datetime.naive_utc())
    }
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait, J: AstronomicalCalendarTrait<Tz, G, N>>
    DefmtFormatTrait for ZmanimCalendar<Tz, G, N, J>
{
}

#[cfg(feature = "defmt")]
impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait, J: AstronomicalCalendarTrait<Tz, G, N>>
    defmt::Format for ZmanimCalendar<Tz, G, N, J>
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ZmanimCalendar(astronomical_calendar={:?}, use_astronomical_chatzos={:?}, use_astronomical_chatzos_for_other_zmanim={:?}, candle_lighting_offset={:?}, ateret_torah_sunset_offset={:?})",
            self.astronomical_calendar,
            self.use_astronomical_chatzos,
            self.use_astronomical_chatzos_for_other_zmanim,
            self.candle_lighting_offset.as_seconds_f64(),
            self.ateret_torah_sunset_offset.as_seconds_f64()
        );
    }
}

// #[cfg(test)]
// mod jni_tests {

//     use std::f64;

//     use crate::test_utils::jni::{
//         DEFAULT_TEST_EPSILON, DEFAULT_TEST_ITERATIONS, assert_almost_equal_f64_option,
//         assert_almost_equal_i64_option, create_zmanim_calendars, init_jvm,
//     };

//     use super::*;

//     use j4rs::{Instance, InvocationArg, Jvm, Null};
//     use rand::Rng;

//     fn get_java_date_millis(jvm: &Jvm, date_instance: &Instance) -> Option<i64> {
//         let millis_result = jvm.invoke(date_instance, "getTime", InvocationArg::empty());
//         if millis_result.is_err() {
//             return None;
//         }
//         let millis = jvm.to_rust::<i64>(millis_result.unwrap()).ok()?;
//         Some(millis)
//     }

//     impl Zman {
//         fn java_method_name(&self) -> &str {
//             match self {
//                 Zman::AlosHashachar => "getAlosHashachar",
//                 Zman::Alos72 => "getAlos72",
//                 Zman::Chatzos => "getChatzos",
//                 Zman::ChatzosAsHalfDay => "getChatzosAsHalfDay",
//                 Zman::MinchaGedola => "getMinchaGedola",
//                 Zman::MinchaKetana => "getMinchaKetana",
//                 Zman::PlagHamincha => "getPlagHamincha",
//                 Zman::SofZmanShmaGRA => "getSofZmanShmaGRA",
//                 Zman::SofZmanShmaMGA => "getSofZmanShmaMGA",
//                 Zman::SofZmanTfilaGRA => "getSofZmanTfilaGRA",
//                 Zman::SofZmanTfilaMGA => "getSofZmanTfilaMGA",
//                 Zman::Tzais => "getTzais",
//                 Zman::Tzais72 => "getTzais72",
//                 Zman::CandleLighting => "getCandleLighting",
//             }
//         }
//     }

//     #[test]
//     fn test_java_method_name() {
//         let jvm = init_jvm();

//         for zman in Zman::values() {
//             let mut ran = false;
//             for _ in 0..DEFAULT_TEST_ITERATIONS {
//                 let test_case = create_zmanim_calendars(&jvm);
//                 if test_case.is_none() {
//                     continue;
//                 }
//                 ran = true;
//                 let (calendar, java_zmanim_calendar, message) = test_case.unwrap();
//                 let message = format!("{} against java {}", zman.java_method_name(), message);

//                 let result = calendar.get_zman(&zman).map(|d| d.timestamp_millis());

//                 let java_result = jvm
//                     .invoke(
//                         &java_zmanim_calendar,
//                         zman.java_method_name(),
//                         InvocationArg::empty(),
//                     )
//                     .unwrap();
//                 let java_result = get_java_date_millis(&jvm, &java_result);

//                 assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//             }
//             assert!(ran, "No test cases were run");
//         }
//     }

//     #[test]
//     fn test_get_sof_zman_shma_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();
//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let end_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(end_of_day)
//             };

//             let result = calendar
//                 ._get_sof_zman_shma(start_of_day.clone(), end_of_day, synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_end = if let Some(end_of_day) = end_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };
//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getSofZmanShma",
//                     &[InvocationArg::from(java_start), java_end, java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_sof_zman_tfila_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();
//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let end_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(end_of_day)
//             };

//             let result = calendar
//                 ._get_sof_zman_tfila(start_of_day.clone(), end_of_day, synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_end = if let Some(end_of_day) = end_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };
//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();
//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getSofZmanTfila",
//                     &[InvocationArg::from(java_start), java_end, java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_mincha_gedola_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();

//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let start_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(start_of_day)
//             };

//             let result = calendar
//                 ._get_mincha_gedola(start_of_day.clone(), end_of_day.clone(), synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = if let Some(start_of_day) = start_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };
//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getMinchaGedola",
//                     &[java_start, InvocationArg::from(java_end), java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_samuch_le_mincha_ketana_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();

//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let start_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(start_of_day)
//             };

//             let result = calendar
//                 ._get_samuch_le_mincha_ketana(start_of_day.clone(), end_of_day, synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = if let Some(start_of_day) = start_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getSamuchLeMinchaKetana",
//                     &[java_start, InvocationArg::from(java_end), java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_mincha_ketana_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();

//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let start_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(start_of_day)
//             };

//             let result = calendar
//                 ._get_mincha_ketana(start_of_day.clone(), end_of_day.clone(), synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = if let Some(start_of_day) = start_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getMinchaKetana",
//                     &[java_start, InvocationArg::from(java_end), java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_plag_hamincha_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();

//             let synchronous = rand::thread_rng().gen_bool(0.5);
//             let start_of_day = if rand::thread_rng().gen_bool(0.5) {
//                 None
//             } else {
//                 Some(start_of_day)
//             };

//             let result = calendar
//                 ._get_plag_hamincha(start_of_day.clone(), end_of_day.clone(), synchronous)
//                 .map(|d| d.timestamp_millis());

//             let java_start = if let Some(start_of_day) = start_of_day {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };
//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();

//             let java_synchronous = InvocationArg::try_from(synchronous)
//                 .unwrap()
//                 .into_primitive()
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getPlagHamincha",
//                     &[java_start, InvocationArg::from(java_end), java_synchronous],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_shaah_zmanis_gra_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let result = calendar.get_shaah_zmanis_gra();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getShaahZmanisGra",
//                     InvocationArg::empty(),
//                 )
//                 .unwrap();
//             let java_result = jvm.to_rust::<i64>(java_result).ok();
//             let java_result = if java_result == Some(-9223372036854775808i64) {
//                 None
//             } else {
//                 java_result
//             };

//             let result_millis = result.map(|d| d.num_milliseconds());

//             assert_almost_equal_i64_option(&result_millis, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_shaah_zmanis_mga_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let result = calendar.get_shaah_zmanis_mga();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getShaahZmanisMGA",
//                     InvocationArg::empty(),
//                 )
//                 .unwrap();
//             let java_result = jvm.to_rust::<i64>(java_result).ok();
//             let java_result = if java_result == Some(-9223372036854775808i64) {
//                 None
//             } else {
//                 java_result
//             };

//             let result_millis = result.map(|d| d.num_milliseconds());

//             assert_almost_equal_i64_option(&result_millis, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_shaah_zmanis_based_zman_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_day.is_none() || end_of_day.is_none() {
//                 continue;
//             }

//             let start_of_day = start_of_day.unwrap();
//             let end_of_day = end_of_day.unwrap();
//             let hours = rand::thread_rng().gen_range(0.0..=12.0);

//             let result = calendar
//                 ._get_shaah_zmanis_based_zman(start_of_day.clone(), end_of_day.clone(), hours)
//                 .map(|d| d.timestamp_millis());

//             let java_start = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(start_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();
//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getShaahZmanisBasedZman",
//                     &[
//                         InvocationArg::from(java_start),
//                         InvocationArg::from(java_end),
//                         InvocationArg::try_from(hours)
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                     ],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_percent_of_shaah_zmanis_from_degrees_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let degrees = rand::thread_rng().gen_range(-100.0..=100.0);
//             let sunset = rand::thread_rng().gen_bool(0.5);

//             let result = calendar.get_percent_of_shaah_zmanis_from_degrees(degrees, sunset);

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getPercentOfShaahZmanisFromDegrees",
//                     &[
//                         InvocationArg::try_from(degrees)
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                         InvocationArg::try_from(sunset)
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                     ],
//                 )
//                 .unwrap();
//             let java_result = jvm.to_rust::<f64>(java_result).ok();
//             let java_result = if java_result == Some(5e-324) {
//                 None
//             } else {
//                 java_result
//             };

//             assert_almost_equal_f64_option(&result, &java_result, DEFAULT_TEST_EPSILON, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_half_day_based_zman_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_half_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_half_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_half_day.is_none() || end_of_half_day.is_none() {
//                 continue;
//             }

//             let start_of_half_day = start_of_half_day.unwrap();
//             let end_of_half_day = end_of_half_day.unwrap();
//             let hours = rand::thread_rng().gen_range(-6.0..=6.0);

//             let result = calendar
//                 ._get_half_day_based_zman(start_of_half_day.clone(), end_of_half_day.clone(), hours)
//                 .map(|d| d.timestamp_millis());

//             let java_start = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[
//                         InvocationArg::try_from(start_of_half_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                     ],
//                 )
//                 .unwrap();
//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_half_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getHalfDayBasedZman",
//                     &[
//                         InvocationArg::from(java_start),
//                         InvocationArg::from(java_end),
//                         InvocationArg::try_from(hours)
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                     ],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_half_day_based_shaah_zmanis_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             ran = true;
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             let start_of_half_day = calendar.get_astronomical_calendar().get_sunrise();
//             let end_of_half_day = calendar.get_astronomical_calendar().get_sunset();

//             if start_of_half_day.is_none() || end_of_half_day.is_none() {
//                 continue;
//             }

//             let start_of_half_day = start_of_half_day.unwrap();
//             let end_of_half_day = end_of_half_day.unwrap();

//             let result =
//                 calendar._get_half_day_based_shaah_zmanis(&start_of_half_day, &end_of_half_day);

//             let java_start = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[
//                         InvocationArg::try_from(start_of_half_day.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap(),
//                     ],
//                 )
//                 .unwrap();
//             let java_end = jvm
//                 .create_instance(
//                     "java.util.Date",
//                     &[InvocationArg::try_from(end_of_half_day.timestamp_millis())
//                         .unwrap()
//                         .into_primitive()
//                         .unwrap()],
//                 )
//                 .unwrap();

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getHalfDayBasedShaahZmanis",
//                     &[
//                         InvocationArg::from(java_start),
//                         InvocationArg::from(java_end),
//                     ],
//                 )
//                 .unwrap();
//             let java_result = jvm.to_rust::<i64>(java_result).ok();
//             let java_result = if java_result == Some(-9223372036854775808i64) {
//                 None
//             } else {
//                 java_result
//             };

//             let result_millis = result.map(|d| d.num_milliseconds());

//             assert_almost_equal_i64_option(&result_millis, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_sof_zman_kidush_levana_15_days_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             // Check if the Jewish day is between 11-17 (inclusive)
//             let jewish_calendar = calendar._get_jewish_calendar();
//             let jewish_day = jewish_calendar.get_jewish_date().get_jewish_day_of_month();
//             if jewish_day < 11 || jewish_day > 17 {
//                 continue;
//             }

//             ran = true;

//             // Get alos and tzais (can be None or Some)
//             let alos = calendar.get_zman(&Zman::Alos72);
//             let tzais = calendar.get_zman(&Zman::Tzais72);

//             // Add some random bit of time to alos and tzais

//             let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let alos = alos.map(|d| d + alos_offset);
//             let tzais = tzais.map(|d| d + tzais_offset);

//             // Randomly decide whether to pass None or Some
//             let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
//             let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
//             let alos_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &alos
//                 } else {
//                     &none_alos
//                 };
//             let tzais_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &tzais
//                 } else {
//                     &none_tzais
//                 };

//             let result = calendar
//                 ._get_sof_zman_kidush_levana_15_days(alos_for_test, tzais_for_test)
//                 .map(|d| d.timestamp_millis());

//             let java_alos = if let Some(alos) = alos_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(alos.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_tzais = if let Some(tzais) = tzais_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(tzais.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getSofZmanKidushLevana15Days",
//                     &[java_alos, java_tzais],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_tchilas_zman_kidush_levana_7_days_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             // Check if the Jewish day is between 4-9 (inclusive)
//             let jewish_calendar = calendar._get_jewish_calendar();
//             let jewish_day = jewish_calendar.get_jewish_date().get_jewish_day_of_month();
//             if jewish_day < 4 || jewish_day > 9 {
//                 continue;
//             }

//             ran = true;

//             // Get alos and tzais (can be None or Some)
//             let alos = calendar.get_zman(&Zman::Alos72);
//             let tzais = calendar.get_zman(&Zman::Tzais72);

//             // Add some random bit of time to alos and tzais
//             let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let alos = alos.map(|d| d + alos_offset);
//             let tzais = tzais.map(|d| d + tzais_offset);

//             // Randomly decide whether to pass None or Some
//             let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
//             let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
//             let alos_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &alos
//                 } else {
//                     &none_alos
//                 };
//             let tzais_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &tzais
//                 } else {
//                     &none_tzais
//                 };

//             let result = calendar
//                 ._get_tchilas_zman_kidush_levana_7_days(alos_for_test, tzais_for_test)
//                 .map(|d| d.timestamp_millis());

//             let java_alos = if let Some(alos) = alos_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(alos.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_tzais = if let Some(tzais) = tzais_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(tzais.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getTchilasZmanKidushLevana7Days",
//                     &[java_alos, java_tzais],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_tchilas_zman_kidush_levana_3_days_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             // Check if the Jewish day is <= 5 OR >= 30
//             let jewish_calendar = calendar._get_jewish_calendar();
//             let jewish_day = jewish_calendar.get_jewish_date().get_jewish_day_of_month();
//             if jewish_day > 5 && jewish_day < 30 {
//                 continue;
//             }

//             ran = true;

//             // Get alos and tzais (can be None or Some)
//             let alos = calendar.get_zman(&Zman::Alos72);
//             let tzais = calendar.get_zman(&Zman::Tzais72);

//             // Add some random bit of time to alos and tzais
//             let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let alos = alos.map(|d| d + alos_offset);
//             let tzais = tzais.map(|d| d + tzais_offset);

//             // Randomly decide whether to pass None or Some
//             let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
//             let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
//             let alos_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &alos
//                 } else {
//                     &none_alos
//                 };
//             let tzais_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &tzais
//                 } else {
//                     &none_tzais
//                 };

//             let result = calendar
//                 ._get_tchilas_zman_kidush_levana_3_days(alos_for_test, tzais_for_test)
//                 .map(|d| d.timestamp_millis());

//             let java_alos = if let Some(alos) = alos_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(alos.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_tzais = if let Some(tzais) = tzais_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(tzais.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getTchilasZmanKidushLevana3Days",
//                     &[java_alos, java_tzais],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }

//     #[test]
//     fn test_get_sof_zman_kidush_levana_between_moldos_against_java() {
//         let jvm = init_jvm();
//         let mut ran = false;
//         for _ in 0..DEFAULT_TEST_ITERATIONS {
//             let test_case = create_zmanim_calendars(&jvm);
//             if test_case.is_none() {
//                 continue;
//             }
//             let (calendar, java_zmanim_calendar, message) = test_case.unwrap();

//             // Check if the Jewish day is between 11-16 (inclusive)
//             let jewish_calendar = calendar._get_jewish_calendar();
//             let jewish_day = jewish_calendar.get_jewish_date().get_jewish_day_of_month();
//             if jewish_day < 11 || jewish_day > 16 {
//                 continue;
//             }

//             ran = true;

//             // Get alos and tzais (can be None or Some)
//             let alos = calendar.get_zman(&Zman::Alos72);
//             let tzais = calendar.get_zman(&Zman::Tzais72);

//             // Add some random bit of time to alos and tzais
//             let alos_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let tzais_offset = Duration::minutes(rand::thread_rng().gen_range(-360..=360));
//             let alos = alos.map(|d| d + alos_offset);
//             let tzais = tzais.map(|d| d + tzais_offset);

//             // Randomly decide whether to pass None or Some
//             let none_alos: Option<DateTime<chrono_tz::Tz>> = None;
//             let none_tzais: Option<DateTime<chrono_tz::Tz>> = None;
//             let alos_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &alos
//                 } else {
//                     &none_alos
//                 };
//             let tzais_for_test: &Option<DateTime<chrono_tz::Tz>> =
//                 if rand::thread_rng().gen_bool(0.5) {
//                     &tzais
//                 } else {
//                     &none_tzais
//                 };

//             let result = calendar
//                 ._get_sof_zman_kidush_levana_between_moldos(alos_for_test, tzais_for_test)
//                 .map(|d| d.timestamp_millis());

//             let java_alos = if let Some(alos) = alos_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(alos.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_tzais = if let Some(tzais) = tzais_for_test {
//                 InvocationArg::from(
//                     jvm.create_instance(
//                         "java.util.Date",
//                         &[InvocationArg::try_from(tzais.timestamp_millis())
//                             .unwrap()
//                             .into_primitive()
//                             .unwrap()],
//                     )
//                     .unwrap(),
//                 )
//             } else {
//                 InvocationArg::try_from(Null::Of("java.util.Date")).unwrap()
//             };

//             let java_result = jvm
//                 .invoke(
//                     &java_zmanim_calendar,
//                     "getSofZmanKidushLevanaBetweenMoldos",
//                     &[java_alos, java_tzais],
//                 )
//                 .unwrap();
//             let java_result = get_java_date_millis(&jvm, &java_result);

//             assert_almost_equal_i64_option(&result, &java_result, 50, &message);
//         }
//         assert!(ran, "No test cases were run");
//     }
// }
