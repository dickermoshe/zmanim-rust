use chrono::TimeDelta;
use core::time::Duration as StdDuration;
#[cfg(feature = "no_std")]
use core_maths::CoreFloat;
use time::Duration as TimeDuration;
/// A helper function to multiply a duration by a factor.
/// This is not accurate down the the millisecond, but is accurate enough for our purposes.
pub fn lossy_multiply_duration(dur: TimeDelta, factor: f64) -> Option<TimeDelta> {
    let time_dur: TimeDuration = dur.to_std().ok()?.try_into().ok()?;
    let multiplied = time_dur * factor;
    let std_dur: StdDuration = multiplied.try_into().ok()?;
    let chrono_dur: TimeDelta = TimeDelta::from_std(std_dur).ok()?;
    Some(chrono_dur)
}
