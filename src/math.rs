use chrono::Duration;
#[cfg(feature = "no_std")]
use core_maths::CoreFloat;
/// A helper function to multiply a duration by a factor.
/// This is not accurate down the the millisecond, but is accurate enough for our purposes.
pub fn lossy_multiply_duration(dur: Duration, factor: f64) -> Duration {
    let milliseconds = dur.as_seconds_f64() * factor * 1000.0;
    Duration::milliseconds(milliseconds as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_positive_integer_multiple() {
        let original = Duration::seconds(10);
        let result = lossy_multiply_duration(original, 2.0);
        assert_eq!(result, Duration::seconds(20));
    }

    #[test]
    fn test_fractional_multiple() {
        let original = Duration::seconds(10);
        let result = lossy_multiply_duration(original, 1.5);
        assert_eq!(result, Duration::seconds(15));
    }

    #[test]
    fn test_subsecond_multiple() {
        let original = Duration::milliseconds(500);
        let result = lossy_multiply_duration(original, 0.5);
        assert_eq!(result, Duration::milliseconds(250));
    }

    #[test]
    fn test_negative_duration() {
        let original = Duration::seconds(-10);
        let result = lossy_multiply_duration(original, 1.5);
        assert_eq!(result, Duration::seconds(-15));
    }

    #[test]
    fn test_negative_factor() {
        let original = Duration::seconds(10);
        let result = lossy_multiply_duration(original, -0.5);
        assert_eq!(result, Duration::seconds(-5));
    }
}
