use chrono::{DateTime, Days, TimeZone, offset::LocalResult};

pub fn add_days_to_date<Tz: TimeZone>(date: &DateTime<Tz>, days: i64) -> Option<DateTime<Tz>> {
    // convert to naive date time
    let naive_date_time = date.naive_utc();
    if days == 0 {
        return Some(date.clone());
    }
    let naive_date_time = if days > 0 {
        naive_date_time.checked_add_days(Days::new(days.abs() as u64))?
    } else {
        naive_date_time.checked_sub_days(Days::new(days.abs() as u64))?
    };
    let date_time = date.timezone().from_local_datetime(&naive_date_time);
    return match date_time {
        LocalResult::Single(dt) => Some(dt),
        LocalResult::Ambiguous(dt1, _) => {
            // return the earlier time to match java's behavior
            Some(dt1)
        }
        LocalResult::None => None,
    };
}
