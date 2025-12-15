use crate::{
    constants::*,
    prelude::{JewishCalendar, JewishCalendarTrait, TimeAndPlace},
};

use chrono::{DateTime, Datelike, Duration, TimeDelta, TimeZone, Utc};
use core::time::Duration as StdDuration;
use icu_calendar::{
    options::{DateAddOptions, Overflow},
    types::DateDuration,
};
use solar_positioning::{Horizon, spa, time::DeltaT};
use time::Duration as TimeDuration;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZmanimCalendar<Tz: TimeZone> {
    pub time_and_place: TimeAndPlace<Tz>,
    pub use_astronomical_chatzos: bool,
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    pub candle_lighting_offset: Duration,
    pub ateret_torah_sunset_offset: Duration,
}

impl<Tz: TimeZone> ZmanimCalendar<Tz> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        time_and_place: TimeAndPlace<Tz>,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: Duration,
        ateret_torah_sunset_offset: Duration,
    ) -> Option<Self> {
        Some(Self {
            time_and_place,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
            ateret_torah_sunset_offset,
        })
    }

    fn _get_molad_based_time(
        &self,
        utc_molad_based_time: DateTime<Utc>,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
        techila: bool,
    ) -> Option<DateTime<Tz>> {
        let molad_based_time = self._localized_datetime(utc_molad_based_time);

        if molad_based_time.date_naive() != self.get_date_time().date_naive() {
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
    fn _get_jewish_calendar(&self) -> Option<JewishCalendar> {
        JewishCalendar::from_gregorian_date(
            self.get_date_time().year(),
            self.get_date_time().month() as u8,
            self.get_date_time().day() as u8,
            false,
            false,
            false,
        )
    }
    fn _localized_datetime(&self, datetime: DateTime<Utc>) -> DateTime<Tz> {
        self.get_date_time().timezone().from_utc_datetime(&datetime.naive_utc())
    }
}

pub trait ZmanimCalendarTrait<Tz: TimeZone> {
    fn get_date_time(&self) -> &DateTime<Tz>;
    fn get_time_and_place(&self) -> &TimeAndPlace<Tz>;
    fn get_sunrise(&self) -> Option<DateTime<Tz>>;
    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>>;
    fn get_begin_civil_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_begin_nautical_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_begin_astronomical_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_sunset(&self) -> Option<DateTime<Tz>>;
    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>>;
    fn get_end_civil_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_end_nautical_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_end_astronomical_twilight(&self) -> Option<DateTime<Tz>>;
    fn get_sunrise_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>>;
    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>>;
    fn get_temporal_hour(&self) -> Option<Duration>;
    fn get_temporal_hour_from_times(&self, start_of_day: &DateTime<Tz>, end_of_day: &DateTime<Tz>) -> Option<Duration>;
    fn get_sun_transit_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;
    fn get_local_mean_time(&self, hours: f64) -> Option<DateTime<Tz>>;
    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64>;
    fn get_shaah_zmanis_gra(&self) -> Option<Duration>;
    fn get_shaah_zmanis_mga(&self) -> Option<Duration>;
    fn get_zman(&self, zman: &Zman) -> Option<DateTime<Tz>>;
    fn get_half_day_based_zman_from_times(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;
    fn get_half_day_based_shaah_zmanis_from_times(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration>;
    fn get_shaah_zmanis_based_zman_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_shma_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: Option<&DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_ketana_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_tfila_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: Option<&DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_gedola_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_plag_hamincha_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_samuch_le_mincha_ketana_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;
    fn get_sof_zman_kidush_levana_15_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_sof_zman_kidush_levana_between_moldos_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_tchilas_zman_kidush_levana_3_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>>;
}

impl<Tz: TimeZone> ZmanimCalendarTrait<Tz> for ZmanimCalendar<Tz> {
    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_day_of_month() < 4 || jewish_calendar.get_jewish_day_of_month() > 9 {
            return None;
        }
        let molad_based_time = jewish_calendar.get_tchilaszman_kidush_levana_7_days()?;

        self._get_molad_based_time(molad_based_time, alos, tzais, true)
    }

    fn get_sof_zman_kidush_levana_15_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_day_of_month() < 11 || jewish_calendar.get_jewish_day_of_month() > 17 {
            return None;
        }
        let molad_based_time = jewish_calendar.get_sof_zman_kidush_levana_15_days()?;
        self._get_molad_based_time(molad_based_time, alos, tzais, false)
    }

    fn get_tchilas_zman_kidush_levana_3_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let mut jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_day_of_month() > 5 && jewish_calendar.get_jewish_day_of_month() < 30 {
            return None;
        }

        let zman = self._get_molad_based_time(
            jewish_calendar.get_tchilaszman_kidush_levana_3_days()?,
            alos,
            tzais,
            true,
        );
        if zman.is_none() && jewish_calendar.get_jewish_day_of_month() == 30 {
            let mut add_option = DateAddOptions::default();
            add_option.overflow = Some(Overflow::Constrain);

            jewish_calendar
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
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        if jewish_calendar.get_jewish_day_of_month() < 11 || jewish_calendar.get_jewish_day_of_month() > 16 {
            return None;
        }
        let molad_based_time = jewish_calendar.get_sof_zman_kidush_levana_between_moldos()?;

        self._get_molad_based_time(molad_based_time, alos, tzais, false)
    }

    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64> {
        let sea_level_sunrise = self.get_sea_level_sunrise();
        let sea_level_sunset = self.get_sea_level_sunset();

        let twilight = if sunset {
            self.get_sunset_offset_by_degrees(_GEOMETRIC_ZENITH + degrees)
        } else {
            self.get_sunrise_offset_by_degrees(_GEOMETRIC_ZENITH + degrees)
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
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis = self.get_half_day_based_shaah_zmanis_from_times(start_of_half_day, end_of_half_day)?;
        if hours >= 0.0 {
            Some(start_of_half_day.clone() + multiply_duration(shaah_zmanis, hours)?)
        } else {
            Some(end_of_half_day.clone() + multiply_duration(shaah_zmanis, hours)?)
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
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>> {
        let shaah_zmanis = self.get_temporal_hour_from_times(start_of_day, end_of_day)?;

        Some(start_of_day.clone() + multiply_duration(shaah_zmanis, hours)?)
    }

    fn get_sof_zman_shma_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: Option<&DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(start_of_day, &self.get_zman(&Zman::Chatzos)?, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day?, 3.0)
        }
    }

    fn get_mincha_gedola_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(&self.get_zman(&Zman::Chatzos)?, end_of_day, 0.5)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 6.5)
        }
    }

    fn get_shaah_zmanis_gra(&self) -> Option<Duration> {
        self.get_temporal_hour_from_times(&self.get_sunrise()?, &self.get_sunset()?)
    }

    fn get_shaah_zmanis_mga(&self) -> Option<Duration> {
        self.get_temporal_hour_from_times(&self.get_zman(&Zman::Alos72)?, &self.get_zman(&Zman::Tzais72)?)
    }
    fn get_mincha_ketana_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(&self.get_zman(&Zman::Chatzos)?, end_of_day, 3.5)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 9.5)
        }
    }
    fn get_sof_zman_tfila_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: Option<&DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(start_of_day, &self.get_zman(&Zman::Chatzos)?, 4.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day, end_of_day?, 4.0)
        }
    }

    fn get_samuch_le_mincha_ketana_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(&self.get_zman(&Zman::Chatzos)?, end_of_day, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 9.0)
        }
    }

    fn get_plag_hamincha_from_times(
        &self,
        start_of_day: Option<&DateTime<Tz>>,
        end_of_day: &DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman_from_times(&self.get_zman(&Zman::Chatzos)?, end_of_day, 4.75)
        } else {
            self.get_shaah_zmanis_based_zman_from_times(start_of_day?, end_of_day, 10.75)
        }
    }

    fn get_zman(&self, zman: &Zman) -> Option<DateTime<Tz>> {
        let astro = self;
        match zman {
            Zman::PlagHamincha => {
                self.get_plag_hamincha_from_times(astro.get_sunrise().as_ref(), &astro.get_sunset()?, true)
            }
            Zman::MinchaKetana => {
                self.get_mincha_ketana_from_times(astro.get_sunrise().as_ref(), &astro.get_sunset()?, true)
            }
            Zman::MinchaGedola => {
                self.get_mincha_gedola_from_times(astro.get_sunrise().as_ref(), &astro.get_sunset()?, true)
            }
            Zman::Tzais => astro.get_sunset_offset_by_degrees(_ZENITH_8_POINT_5),
            Zman::AlosHashachar => astro.get_sunrise_offset_by_degrees(_ZENITH_16_POINT_1),
            Zman::Alos72 => astro.get_sunrise().map(|sunrise| sunrise - Duration::minutes(72)),
            Zman::Chatzos => {
                todo!()

                // if self.use_astronomical_chatzos {
                //     astro.get_sun_transit()
                // } else {
                //     self.get_zman(&Zman::ChatzosAsHalfDay).or(astro.get_sun_transit())
                // }
            }
            Zman::ChatzosAsHalfDay => {
                let sunrise = astro.get_sea_level_sunrise()?;
                let sunset = astro.get_sea_level_sunset()?;
                astro.get_sun_transit_from_times(&sunrise, &sunset)
            }
            Zman::SofZmanShmaGRA => {
                self.get_sof_zman_shma_from_times(&astro.get_sunrise()?, astro.get_sunset().as_ref(), true)
            }
            Zman::SofZmanShmaMGA => self.get_sof_zman_shma_from_times(
                &self.get_zman(&Zman::Alos72)?,
                self.get_zman(&Zman::Tzais72).as_ref(),
                true,
            ),
            Zman::Tzais72 => astro.get_sunset().map(|sunset| sunset + Duration::minutes(72)),
            Zman::CandleLighting => astro
                .get_sea_level_sunset()
                .map(|sunset| sunset - self.candle_lighting_offset),
            Zman::SofZmanTfilaGRA => {
                self.get_sof_zman_tfila_from_times(&astro.get_sunrise()?, astro.get_sunset().as_ref(), true)
            }
            Zman::SofZmanTfilaMGA => self.get_sof_zman_tfila_from_times(
                &self.get_zman(&Zman::Alos72)?,
                self.get_zman(&Zman::Tzais72).as_ref(),
                true,
            ),
        }
    }
    fn get_date_time(&self) -> &DateTime<Tz> {
        &self.time_and_place.date_time
    }

    fn get_time_and_place(&self) -> &TimeAndPlace<Tz> {
        &self.time_and_place
    }

    fn get_sunrise(&self) -> Option<DateTime<Tz>> {
        get_sunrise(&self.time_and_place)
    }

    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>> {
        let mut time_and_place = self.time_and_place.clone();
        time_and_place.elevation = 0.0;
        get_sunrise(&time_and_place)
    }

    fn get_begin_civil_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_CIVIL_ZENITH)
    }

    fn get_begin_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_NAUTICAL_ZENITH)
    }

    fn get_begin_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunrise_offset_by_degrees(_ASTRONOMICAL_ZENITH)
    }

    fn get_sunset(&self) -> Option<DateTime<Tz>> {
        get_sunset(&self.time_and_place)
    }

    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>> {
        let mut time_and_place = self.time_and_place.clone();
        time_and_place.elevation = 0.0;
        get_sunset(&time_and_place)
    }

    fn get_end_civil_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_CIVIL_ZENITH)
    }

    fn get_end_nautical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_NAUTICAL_ZENITH)
    }

    fn get_end_astronomical_twilight(&self) -> Option<DateTime<Tz>> {
        self.get_sunset_offset_by_degrees(_ASTRONOMICAL_ZENITH)
    }

    fn get_sunrise_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        get_sunrise_offset_by_degrees(&self.time_and_place, offset_zenith)
    }

    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        get_sunset_offset_by_degrees(&self.time_and_place, offset_zenith)
    }

    fn get_temporal_hour(&self) -> Option<Duration> {
        let sea_level_sunrise = self.get_sea_level_sunrise()?;
        let sea_level_sunset = self.get_sea_level_sunset()?;
        self.get_temporal_hour_from_times(&sea_level_sunrise, &sea_level_sunset)
    }

    fn get_temporal_hour_from_times(&self, start_of_day: &DateTime<Tz>, end_of_day: &DateTime<Tz>) -> Option<Duration> {
        Some((end_of_day.clone() - start_of_day) / 12)
    }

    fn get_sun_transit_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        let temporal_hour = self.get_temporal_hour_from_times(start_of_day, end_of_day)?;
        Some(start_of_day.clone() + (temporal_hour * 6))
    }

    fn get_local_mean_time(&self, hours: f64) -> Option<DateTime<Tz>> {
        todo!();
        // if !(0.0..24.0).contains(&hours) {
        //     return None;
        // }
        // let timezone_offset_hours = self.date_time.offset().fix().local_minus_utc() as f64 / 60.0 / 60.0;
        // println!("timezone_offset_hours: {:?}", timezone_offset_hours);
        // println!("millis: {:?}", self.date_time.timestamp_millis());
        // let start = self.get_date_from_time(hours - timezone_offset_hours, _SolarEvent::Sunrise)?;
        // let offset = self.get_time_and_place().get_local_mean_time_offset(&self.date_time);
        // Some(start - offset)
    }
}

/// A helper function to multiply a duration by a factor.
/// This uses a clever workaround to handle negative durations which std duration does not support.
fn multiply_duration(core_timedelta: TimeDelta, factor: f64) -> Option<TimeDelta> {
    let is_timedelta_negative = core_timedelta < TimeDelta::zero();
    let factor_is_negative = factor < 0.0;
    let std_duration = core_timedelta.abs().to_std().ok()?;
    let time_duration: TimeDuration = std_duration.try_into().ok()?;
    let std_duration: StdDuration = (time_duration * factor.abs()).try_into().ok()?;
    let core_timedelta = TimeDelta::from_std(std_duration).ok()?;

    if (is_timedelta_negative && !factor_is_negative) || (!is_timedelta_negative && factor_is_negative) {
        core_timedelta.checked_mul(-1)
    } else {
        Some(core_timedelta)
    }
}

#[cfg(feature = "defmt")]
impl<Tz: TimeZone> defmt::Format for ZmanimCalendar<Tz, TimeAndPlace> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ZmanimCalendar(date_time={:?},", self.date_time.timestamp_millis(),);

        let offset = self.date_time.offset().fix().local_minus_utc();
        let (sign, offset) = if offset < 0 { ('-', -offset) } else { ('+', offset) };
        let sec = offset.rem_euclid(60);
        let mins = offset.div_euclid(60);
        let min = mins.rem_euclid(60);
        let hour = mins.div_euclid(60);
        if sec == 0 {
            defmt::write!(f, "offset={}{:02}:{:02},", sign, hour, min)
        } else {
            defmt::write!(f, "offset={}{:02}:{:02}:{:02},", sign, hour, min, sec)
        }

        defmt::write!(
            f,
            "time_and_place={:?},  use_astronomical_chatzos={:?}, use_astronomical_chatzos_for_other_zmanim={:?}, candle_lighting_offset={:?}, ateret_torah_sunset_offset={:?})",
            self.time_and_place,
            self.use_astronomical_chatzos,
            self.use_astronomical_chatzos_for_other_zmanim,
            self.candle_lighting_offset.as_seconds_f64(),
            self.ateret_torah_sunset_offset.as_seconds_f64()
        );
    }
}
static EARTH_RADIUS: f64 = 6356.9;

fn get_elevation_adjustment(elevation: f64) -> f64 {
    (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1000.0)) as f64)
        .acos()
        .to_degrees()
}

fn get_sunrise<Tz: TimeZone>(time_and_place: &TimeAndPlace<Tz>) -> Option<DateTime<Tz>> {
    let elevation_adjustment = get_elevation_adjustment(time_and_place.elevation);
    let angle = -0.83337 - elevation_adjustment;
    spa::sunrise_sunset_for_horizon(
        time_and_place.date_time.clone(),
        time_and_place.latitude,
        time_and_place.longitude,
        DeltaT::estimate_from_date_like(time_and_place.date_time.date_naive()).ok()?,
        Horizon::Custom(angle),
    )
    .map(|result| result.sunrise().cloned())
    .ok()
    .flatten()
}

fn get_sunset<Tz: TimeZone>(time_and_place: &TimeAndPlace<Tz>) -> Option<DateTime<Tz>> {
    let elevation_adjustment = get_elevation_adjustment(time_and_place.elevation);
    let angle = -0.83337 + elevation_adjustment;
    spa::sunrise_sunset_for_horizon(
        time_and_place.date_time.clone(),
        time_and_place.latitude,
        time_and_place.longitude,
        DeltaT::estimate_from_date_like(time_and_place.date_time.date_naive()).ok()?,
        Horizon::Custom(angle),
    )
    .map(|result| result.sunset().cloned())
    .ok()
    .flatten()
}

fn get_sunrise_offset_by_degrees<Tz: TimeZone>(
    time_and_place: &TimeAndPlace<Tz>,
    degrees: f64,
) -> Option<DateTime<Tz>> {
    let angle = 90.0 - degrees;
    spa::sunrise_sunset_for_horizon(
        time_and_place.date_time.clone(),
        time_and_place.latitude,
        time_and_place.longitude,
        DeltaT::estimate_from_date_like(time_and_place.date_time.date_naive()).ok()?,
        Horizon::Custom(angle),
    )
    .map(|result| result.sunrise().cloned())
    .ok()
    .flatten()
}

fn get_sunset_offset_by_degrees<Tz: TimeZone>(time_and_place: &TimeAndPlace<Tz>, degrees: f64) -> Option<DateTime<Tz>> {
    let angle = 90.0 - degrees;
    spa::sunrise_sunset_for_horizon(
        time_and_place.date_time.clone(),
        time_and_place.latitude,
        time_and_place.longitude,
        DeltaT::estimate_from_date_like(time_and_place.date_time.date_naive()).ok()?,
        Horizon::Custom(angle),
    )
    .map(|result| result.sunset().cloned())
    .ok()
    .flatten()
}
