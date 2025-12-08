use chrono::{DateTime, Datelike, Days, Duration, Offset, TimeDelta, TimeZone, Utc};

use crate::{
    astronomical_calculator::AstronomicalCalculatorTrait, constants::*, defmt::DefmtFormatTrait,
    geolocation::GeoLocationTrait,
};
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AstronomicalCalendar<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> {
    pub date_time: DateTime<Tz>,
    pub geo_location: G,
    pub noaa_calculator: N,
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> AstronomicalCalendar<Tz, G, N> {
    pub fn new(date_time: DateTime<Tz>, geo_location: G, calculator: N) -> Self {
        Self {
            date_time,
            geo_location,
            noaa_calculator: calculator,
        }
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
}

pub trait AstronomicalCalendarTrait<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait>:
    DefmtFormatTrait
{
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
}

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> AstronomicalCalendarTrait<Tz, G, N>
    for AstronomicalCalendar<Tz, G, N>
{
    fn get_date_time(&self) -> &DateTime<Tz> {
        &self.date_time
    }

    fn get_geo_location(&self) -> &G {
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

impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> DefmtFormatTrait
    for AstronomicalCalendar<Tz, G, N>
{
}

#[cfg(feature = "defmt")]
impl<Tz: TimeZone, G: GeoLocationTrait, N: AstronomicalCalculatorTrait> defmt::Format
    for AstronomicalCalendar<Tz, G, N>
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AstronomicalCalendar(date_time={:?},",
            self.date_time.timestamp_millis(),
        );

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
            "geo_location={:?}, noaa_calculator={:?})",
            self.geo_location,
            self.noaa_calculator
        );
    }
}
