use crate::{
    astronomical_calculator::AstronomicalCalculatorTrait,
    constants::*,
    geolocation::GeoLocationTrait,
    prelude::{GeoLocation, JewishCalendar, JewishCalendarTrait},
};
use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, Offset, TimeDelta, TimeZone, Timelike, Utc};
use core::time::Duration as StdDuration;
use icu_calendar::{
    options::{DateAddOptions, Overflow},
    types::DateDuration,
};
use time::Duration as TimeDuration;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZmanimCalendar<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> {
    pub date_time: DateTime<Tz>,
    pub geo_location: G,
    pub noaa_calculator: N,
    pub use_astronomical_chatzos: bool,
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    pub candle_lighting_offset: Duration,
    pub ateret_torah_sunset_offset: Duration,
}

impl<N: AstronomicalCalculatorTrait> ZmanimCalendar<Utc, GeoLocation, N> {
    pub fn naive(
        date: NaiveDate,
        geo_location: GeoLocation,
        calculator: N,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: Duration,
        ateret_torah_sunset_offset: Duration,
    ) -> Option<Self> {
        // We do not actually need the timezone to perform calculations.
        // It is only needed to convert gregorian dates to julian dates accurately 
        // for locations whose timezones are more than 12 hours in either direction
        // These locations are very close to the antimeridian. In order to support use cases
        // where a user has no knowlage of any timezone, but only naive dates, we can
        // assume that locations far from the antimeridian do not have a timezone offset >12:00 || < -12:00 
        if geo_location.longitude > 160.0 || geo_location.longitude < -160.0{
            return None;
        }
        Self::new(
            date,
            Utc,
            geo_location,
            calculator,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
            ateret_torah_sunset_offset,
        )
    }
}

impl<Tz: TimeZone, N: AstronomicalCalculatorTrait> ZmanimCalendar<Tz, GeoLocation, N> {
    pub fn new(
        date: NaiveDate,
        timezone: Tz,
        geo_location: GeoLocation,
        calculator: N,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: Duration,
        ateret_torah_sunset_offset: Duration,
    ) -> Option<Self> {
        let date_time = timezone.from_local_datetime(&date.and_hms_opt(0, 0, 0)?).single()?;
        Some(Self {
            date_time,
            geo_location,
            noaa_calculator: calculator,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
            ateret_torah_sunset_offset,
        })
    }

    fn get_adjusted_date_time(&self, date_time: &DateTime<Tz>) -> Option<DateTime<Tz>> {
        let offset = self.get_geo_location().get_antimeridian_adjustment(date_time);
        if offset == 0 {
            Some(date_time.clone())
        } else if offset > 0 {
            date_time
                .clone()
                .checked_add_days(Days::new(offset.unsigned_abs() as u64))
        } else {
            date_time
                .clone()
                .checked_sub_days(Days::new(offset.unsigned_abs() as u64))
        }
    }
    fn _get_midnight_last_night(&self) -> Option<DateTime<Tz>> {
        let midnight = self
            .get_date_time()
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;
        Some(midnight)
    }

    fn _get_midnight_tonight(&self) -> Option<DateTime<Tz>> {
        let midnight = self
            .get_date_time()
            .clone()
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?
            .checked_add_days(Days::new(1))?;
        Some(midnight)
    }

    fn _get_molad_based_time(
        &self,
        utc_molad_based_time: DateTime<Utc>,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
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
            self.get_date_time().year(),
            self.get_date_time().month() as u8,
            self.get_date_time().day() as u8,
            false,
            false,
            false,
            self.get_calculator().clone(),
        )
    }
    fn _localized_datetime(&self, datetime: DateTime<Utc>) -> DateTime<Tz> {
        self.get_date_time().timezone().from_utc_datetime(&datetime.naive_utc())
    }
}

pub trait ZmanimCalendarTrait<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> {
    fn get_date_time(&self) -> &DateTime<Tz>;
    fn get_geo_location(&self) -> &G;
    fn get_calculator(&self) -> &N;
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
    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64>;
    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64>;
    fn get_utc_sunset(&self, zenith: f64) -> Option<f64>;
    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64>;
    fn get_temporal_hour(&self) -> Option<Duration>;
    fn get_temporal_hour_from_times(&self, start_of_day: &DateTime<Tz>, end_of_day: &DateTime<Tz>) -> Option<Duration>;
    fn get_sun_transit(&self) -> Option<DateTime<Tz>>;
    fn get_solar_midnight(&self) -> Option<DateTime<Tz>>;
    fn get_sun_transit_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;
    fn get_date_from_time(&self, calculated_time: f64, solar_event: _SolarEvent) -> Option<DateTime<Tz>>;
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

impl<Tz: TimeZone, N: AstronomicalCalculatorTrait> ZmanimCalendarTrait<Tz, GeoLocation, N>
    for ZmanimCalendar<Tz, GeoLocation, N>
{
    fn get_tchilas_zman_kidush_levana_7_days_from_times(
        &self,
        alos: Option<&DateTime<Tz>>,
        tzais: Option<&DateTime<Tz>>,
    ) -> Option<DateTime<Tz>> {
        let jewish_calendar = self._get_jewish_calendar()?;
        println!(
            "jewish_calendar.get_jewish_day_of_month() = {}",
            jewish_calendar.get_jewish_day_of_month()
        );
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
                if self.use_astronomical_chatzos {
                    astro.get_sun_transit()
                } else {
                    self.get_zman(&Zman::ChatzosAsHalfDay).or(astro.get_sun_transit())
                }
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
        &self.date_time
    }

    fn get_geo_location(&self) -> &GeoLocation {
        &self.geo_location
    }

    fn get_calculator(&self) -> &N {
        &self.noaa_calculator
    }
    fn get_sunrise(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunrise(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
    }

    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sea_level_sunrise(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
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
        let result = self.get_utc_sunset(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
    }

    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sea_level_sunset(_GEOMETRIC_ZENITH)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
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
        let result = self.get_utc_sunrise(offset_zenith)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunrise)
    }

    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>> {
        let result = self.get_utc_sunset(offset_zenith)?;
        if result.is_nan() {
            return None;
        }
        self.get_date_from_time(result, _SolarEvent::Sunset)
    }

    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time())?;
        self.get_calculator()
            .get_utc_sunrise(&adjusted_date_time, self.get_geo_location(), zenith, true)
    }

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64> {
        self.get_calculator().get_utc_sunrise(
            &self.get_adjusted_date_time(self.get_date_time())?,
            self.get_geo_location(),
            zenith,
            false,
        )
    }

    fn get_utc_sunset(&self, zenith: f64) -> Option<f64> {
        self.get_calculator().get_utc_sunset(
            &self.get_adjusted_date_time(self.get_date_time())?,
            self.get_geo_location(),
            zenith,
            true,
        )
    }

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64> {
        self.get_calculator().get_utc_sunset(
            &self.get_adjusted_date_time(self.get_date_time())?,
            self.get_geo_location(),
            zenith,
            false,
        )
    }

    fn get_temporal_hour(&self) -> Option<Duration> {
        let sea_level_sunrise = self.get_sea_level_sunrise()?;
        let sea_level_sunset = self.get_sea_level_sunset()?;
        self.get_temporal_hour_from_times(&sea_level_sunrise, &sea_level_sunset)
    }

    fn get_temporal_hour_from_times(&self, start_of_day: &DateTime<Tz>, end_of_day: &DateTime<Tz>) -> Option<Duration> {
        Some((end_of_day.clone() - start_of_day) / 12)
    }

    fn get_sun_transit(&self) -> Option<DateTime<Tz>> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time())?;
        let noon = self
            .get_calculator()
            .get_utc_noon(&adjusted_date_time, self.get_geo_location());
        if noon.is_nan() {
            return None;
        }
        self.get_date_from_time(noon, _SolarEvent::Noon)
    }

    fn get_solar_midnight(&self) -> Option<DateTime<Tz>> {
        let adjusted_date_time = self.get_adjusted_date_time(self.get_date_time())?;
        let midnight = self
            .get_calculator()
            .get_utc_midnight(&adjusted_date_time, self.get_geo_location());
        if midnight.is_nan() {
            return None;
        }
        self.get_date_from_time(midnight, _SolarEvent::Midnight)
    }

    fn get_sun_transit_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<DateTime<Tz>> {
        let temporal_hour = self.get_temporal_hour_from_times(start_of_day, end_of_day)?;
        Some(start_of_day.clone() + (temporal_hour * 6))
    }

    fn get_date_from_time(&self, mut calculated_time: f64, solar_event: _SolarEvent) -> Option<DateTime<Tz>> {
        let adjusted_dt = self.get_adjusted_date_time(self.get_date_time())?;

        let cal_result = Utc.with_ymd_and_hms(adjusted_dt.year(), adjusted_dt.month(), adjusted_dt.day(), 0, 0, 0);

        let mut cal = match cal_result {
            chrono::LocalResult::Single(dt) => dt,
            _ => return None,
        };

        let hours = calculated_time as i64;
        calculated_time -= hours as f64;

        calculated_time *= 60.0;
        let minutes = calculated_time as i64;
        calculated_time -= minutes as f64;

        calculated_time *= 60.0;
        let seconds = calculated_time as i64;
        calculated_time -= seconds as f64;

        let local_time_hours = (self.get_geo_location().get_longitude() / 15.0) as i64;
        #[allow(clippy::if_same_then_else)]
        if solar_event == _SolarEvent::Sunrise && local_time_hours + hours > 18 {
            cal = cal.checked_sub_days(Days::new(1))?;
        } else if solar_event == _SolarEvent::Sunset && local_time_hours + hours < 6 {
            cal = cal.checked_add_days(Days::new(1))?;
        } else if solar_event == _SolarEvent::Midnight && local_time_hours + hours < 12 {
            cal = cal.checked_add_days(Days::new(1))?;
        } else if solar_event == _SolarEvent::Noon && local_time_hours + hours > 24 {
            cal = cal.checked_sub_days(Days::new(1))?;
        }

        cal = cal.checked_add_signed(
            TimeDelta::hours(hours)
                + TimeDelta::minutes(minutes)
                + TimeDelta::seconds(seconds)
                + TimeDelta::nanoseconds((calculated_time * 1_000_000_000.0) as i64),
        )?;

        Some(adjusted_dt.timezone().from_utc_datetime(&cal.naive_utc()))
    }

    fn get_local_mean_time(&self, hours: f64) -> Option<DateTime<Tz>> {
        if !(0.0..24.0).contains(&hours) {
            return None;
        }
        let timezone_offset_hours = self.date_time.offset().fix().local_minus_utc() as f64 / 60.0 / 60.0;
        let start = self.get_date_from_time(hours - timezone_offset_hours, _SolarEvent::Sunrise)?;
        let offset = self.get_geo_location().get_local_mean_time_offset(&self.date_time);
        Some(start - offset)
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
impl<Tz: TimeZone, N: AstronomicalCalculatorTrait> defmt::Format for ZmanimCalendar<Tz, GeoLocation, N> {
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
            "geo_location={:?}, noaa_calculator={:?}), use_astronomical_chatzos={:?}, use_astronomical_chatzos_for_other_zmanim={:?}, candle_lighting_offset={:?}, ateret_torah_sunset_offset={:?})",
            self.geo_location,
            self.noaa_calculator,
            self.use_astronomical_chatzos,
            self.use_astronomical_chatzos_for_other_zmanim,
            self.candle_lighting_offset.as_seconds_f64(),
            self.ateret_torah_sunset_offset.as_seconds_f64()
        );
    }
}
