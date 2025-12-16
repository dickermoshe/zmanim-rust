use crate::constants::_MINUTE_MILLIS;
use chrono::{DateTime, Duration, NaiveDate, Offset, TimeZone};

// #[cfg_attr(feature = "defmt", derive(defmt::Format))] TODO: Add defmt formatting
#[derive(Debug, Clone, PartialEq)]
pub struct TimeAndPlace<Tz: TimeZone> {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub date_time: DateTime<Tz>,
    pub naive_date_time: NaiveDate,
}
impl<Tz: TimeZone> TimeAndPlace<Tz> {
    pub fn new(latitude: f64, longitude: f64, elevation: f64, date: NaiveDate, tz: Tz) -> Option<Self> {
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
        // Try to get a valid date time for this date and timezone
        // Try to get the date time for 00:00:00. This will fail on dates where a timezone transitioned at midnight.
        let date_time = date
            .and_hms_opt(0, 0, 0)
            .map(|naive_date_time| tz.from_local_datetime(&naive_date_time).single())
            .flatten()?;
        Some(Self {
            latitude,
            longitude,
            elevation,
            date_time,
            naive_date_time: date,
        })
    }
}

pub(crate) fn get_local_mean_time_offset<Tz: TimeZone>(longitude: f64, date: &DateTime<Tz>) -> Duration {
    let longitude_offset_ms = longitude * 4.0 * _MINUTE_MILLIS as f64;
    let timezone_offset_sec = date.offset().fix().local_minus_utc();
    let timezone_offset_ms = timezone_offset_sec as f64 * 1000.0;
    Duration::milliseconds((longitude_offset_ms - timezone_offset_ms) as i64)
}
