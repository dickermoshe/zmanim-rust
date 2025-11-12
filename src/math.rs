use chrono::Duration;

pub fn multiply_duration(dur: Duration, factor: f64) -> Duration {
    let total_nanos: i64 = dur
        .num_nanoseconds()
        .expect("Duration too large: cannot represent total nanoseconds in i64 for multiplication");

    let total_nanos_f64 = total_nanos as f64;
    let new_total_nanos_f64 = total_nanos_f64 * factor;
    let new_total_nanos_i64 = new_total_nanos_f64.round() as i64;

    Duration::nanoseconds(new_total_nanos_i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_positive_integer_multiple() {
        let original = Duration::seconds(10);
        let result = multiply_duration(original, 2.0);
        assert_eq!(result, Duration::seconds(20));
    }

    #[test]
    fn test_fractional_multiple() {
        let original = Duration::seconds(10);
        let result = multiply_duration(original, 1.5);
        assert_eq!(result, Duration::seconds(15));
    }

    #[test]
    fn test_subsecond_multiple() {
        let original = Duration::milliseconds(500);
        let result = multiply_duration(original, 0.5);
        assert_eq!(result, Duration::milliseconds(250));
    }

    #[test]
    fn test_negative_duration() {
        let original = Duration::seconds(-10);
        let result = multiply_duration(original, 1.5);
        assert_eq!(result, Duration::seconds(-15));
    }

    #[test]
    fn test_negative_factor() {
        let original = Duration::seconds(10);
        let result = multiply_duration(original, -0.5);
        assert_eq!(result, Duration::seconds(-5));
    }

    #[test]
    #[should_panic(expected = "Duration too large")]
    fn test_overflow_original() {
        // This would require a duration larger than i64::MAX nanoseconds, which is impractical to construct
        // but tests the panic condition if num_nanoseconds() returns None.
        let huge = Duration::seconds(i64::MAX / 1_000);
        multiply_duration(huge, 1.0); // May or may not panic depending on exact calculation, but illustrates
    }
}
