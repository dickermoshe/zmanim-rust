#[cfg(test)]
mod spa_tests {
    use chrono::{Datelike, NaiveDate};
    use solar_positioning::{Horizon, spa, time::DeltaT};

    #[test]
    fn test_spa_time_dependent_from_julian() {
        let lat = -31.234047280158855;
        let lon = 89.24299583283118;
        for day in 1..31 {
            let date: NaiveDate = NaiveDate::from_ymd_opt(1962, 09, day).unwrap();
            let result = spa::sunrise_sunset_utc_for_horizon(
                date.year(),
                date.month(),
                date.day(),
                lat,
                lon,
                DeltaT::estimate_from_date_like(date).unwrap(),
                Horizon::SunriseSunset,
            )
            .unwrap();
            println!(
                "day: {} sunrise: {:?} sunset: {:?}",
                day,
                result.sunrise(),
                result.sunset()
            );
        }
        panic!();
    }
}
