use icu_calendar::{
    Date, Gregorian,
    cal::Hebrew,
    options::DateAddOptions,
    types::{DateDuration, MonthCode, Weekday},
};

use crate::constants::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct JewishDate {
    pub hebrew_date: Date<Hebrew>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct MoladData {
    pub hours: i64,
    pub minutes: i64,
    pub chalakim: i64,
}

impl MoladDataTrait for MoladData {
    fn get_hours(&self) -> i64 {
        self.hours
    }
    fn get_minutes(&self) -> i64 {
        self.minutes
    }
    fn get_chalakim(&self) -> i64 {
        self.chalakim
    }
}

impl JewishDateTrait for JewishDate {
    fn get_gregorian_date(&self) -> Date<Gregorian> {
        self.get_hebrew_date().to_calendar(Gregorian)
    }
    fn get_days_in_jewish_year_static(year: i64) -> i64 {
        JewishDate::get_jewish_calendar_elapsed_days(year + 1)
            - JewishDate::get_jewish_calendar_elapsed_days(year)
    }
    fn get_jewish_month(&self) -> JewishMonth {
        let month_code = self.get_hebrew_date().month().formatting_code.0;
        match month_code.as_str() {
            "M01" => JewishMonth::Tishrei,
            "M02" => JewishMonth::Cheshvan,
            "M03" => JewishMonth::Kislev,
            "M04" => JewishMonth::Teves,
            "M05" => JewishMonth::Shevat,
            "M05L" => JewishMonth::Adar,
            "M06" => JewishMonth::Adar,
            "M06L" => JewishMonth::AdarII,
            "M07" => JewishMonth::Nissan,
            "M08" => JewishMonth::Iyar,
            "M09" => JewishMonth::Sivan,
            "M10" => JewishMonth::Tammuz,
            "M11" => JewishMonth::Av,
            "M12" => JewishMonth::Elul,
            _ => {
                panic!("Unknown Hebrew month code: {}", month_code);
            }
        }
    }

    fn get_jewish_day_of_month(&self) -> i64 {
        self.get_hebrew_date().day_of_month().0.into()
    }

    fn get_gregorian_year(&self) -> i64 {
        self.get_gregorian_date().era_year().year.into()
    }

    fn get_gregorian_month(&self) -> i64 {
        self.get_gregorian_date().month().ordinal as i64 - 1
    }

    fn get_gregorian_day_of_month(&self) -> i64 {
        self.get_gregorian_date().day_of_month().0.into()
    }

    fn get_day_of_week(&self) -> DayOfWeek {
        let weekday = self.get_hebrew_date().day_of_week();
        match weekday {
            Weekday::Sunday => DayOfWeek::Sunday,
            Weekday::Monday => DayOfWeek::Monday,
            Weekday::Tuesday => DayOfWeek::Tuesday,
            Weekday::Wednesday => DayOfWeek::Wednesday,
            Weekday::Thursday => DayOfWeek::Thursday,
            Weekday::Friday => DayOfWeek::Friday,
            Weekday::Saturday => DayOfWeek::Shabbos,
        }
    }

    fn is_jewish_leap_year(&self) -> bool {
        JewishDate::is_jewish_leap_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_year(&self) -> i64 {
        JewishDate::get_days_in_jewish_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_month(&self) -> i64 {
        JewishDate::get_days_in_jewish_month_static(
            self.get_jewish_month().into(),
            self.get_jewish_year(),
        )
    }

    fn is_cheshvan_long(&self) -> bool {
        JewishDate::is_cheshvan_long_static(self.get_jewish_year())
    }

    fn is_kislev_short(&self) -> bool {
        JewishDate::is_kislev_short_static(self.get_jewish_year())
    }

    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType {
        let year = self.get_jewish_year();
        if JewishDate::is_cheshvan_long_static(year) && !JewishDate::is_kislev_short_static(year) {
            YearLengthType::Shelaimim
        } else if !JewishDate::is_cheshvan_long_static(year)
            && JewishDate::is_kislev_short_static(year)
        {
            YearLengthType::Chaserim
        } else {
            YearLengthType::Kesidran
        }
    }

    fn get_days_since_start_of_jewish_year(&self) -> i64 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month() as i64;
        let day = self.get_jewish_day_of_month();
        JewishDate::get_days_since_start_of_jewish_year_static(year, month, day)
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month() as i64;
        JewishDate::get_chalakim_since_molad_tohu_static(year, month)
    }

    fn get_molad_as_date(&self) -> Option<impl JewishDateTrait> {
        let (date, _) = self._get_molad()?;
        Some(date)
    }

    fn get_molad(&self) -> Option<impl MoladDataTrait> {
        let (_, molad) = self._get_molad()?;
        Some(molad)
    }
    fn from_hebrew_date(year: i64, month: JewishMonth, day: i64) -> Option<Self> {
        let is_leap_year = Date::try_new_from_codes(
            Some("am"),
            year as i32,
            MonthCode("M01".parse().ok()?),
            1,
            Hebrew,
        )
        .ok()?
        .is_in_leap_year();

        let month_code: MonthCode;
        if is_leap_year {
            let month_code_str = match month {
                JewishMonth::Tishrei => "M01",
                JewishMonth::Cheshvan => "M02",
                JewishMonth::Kislev => "M03",
                JewishMonth::Teves => "M04",
                JewishMonth::Shevat => "M05",
                JewishMonth::Adar => "M05L",
                JewishMonth::AdarII => "M06",
                JewishMonth::Nissan => "M07",
                JewishMonth::Iyar => "M08",
                JewishMonth::Sivan => "M09",
                JewishMonth::Tammuz => "M10",
                JewishMonth::Av => "M11",
                JewishMonth::Elul => "M12",
            };

            month_code = MonthCode(month_code_str.parse().ok()?);
        } else {
            let month_code_str = match month {
                JewishMonth::Tishrei => "M01",
                JewishMonth::Cheshvan => "M02",
                JewishMonth::Kislev => "M03",
                JewishMonth::Teves => "M04",
                JewishMonth::Shevat => "M05",
                JewishMonth::Adar => "M06",
                JewishMonth::Nissan => "M07",
                JewishMonth::Iyar => "M08",
                JewishMonth::Sivan => "M09",
                JewishMonth::Tammuz => "M10",
                JewishMonth::Av => "M11",
                JewishMonth::Elul => "M12",
                _ => return None,
            };
            month_code = MonthCode(month_code_str.parse().ok()?);
        }

        let hebrew_date =
            Date::try_new_from_codes(Some("am"), year as i32, month_code, day as u8, Hebrew);

        let hebrew_date = hebrew_date.ok()?;

        Some(JewishDate { hebrew_date })
    }
    fn from_gregorian_date(year: i64, month: u8, day: u8) -> Option<Self> {
        let gregorian_date = Date::try_new_iso(year as i32, month, day).ok()?;

        Some(JewishDate {
            hebrew_date: gregorian_date.to_calendar(Hebrew),
        })
    }

    fn _get_molad(&self) -> Option<(impl JewishDateTrait, impl MoladDataTrait)> {
        let chalakim_since_molad_tohu = self.get_chalakim_since_molad_tohu();
        let abs_date = Self::molad_to_abs_date(chalakim_since_molad_tohu);
        let mut gregorian_date = Self::abs_date_to_date(abs_date)?;
        let conjunction_day = chalakim_since_molad_tohu / _CHALAKIM_PER_DAY;
        let conjunction_parts = chalakim_since_molad_tohu - conjunction_day * _CHALAKIM_PER_DAY;
        let mut hours = conjunction_parts / _CHALAKIM_PER_HOUR;
        let adjusted_conjunction_parts = conjunction_parts - (hours * _CHALAKIM_PER_HOUR);
        let minutes = adjusted_conjunction_parts / _CHALAKIM_PER_MINUTE;
        let chalakim = adjusted_conjunction_parts - (minutes * _CHALAKIM_PER_MINUTE);
        if hours >= 6 {
            let _ = gregorian_date
                .try_add_with_options(DateDuration::for_days(1), DateAddOptions::default())
                .ok()?;
        }
        hours = (hours + 18) % 24;
        let molad_date = JewishDate {
            hebrew_date: (gregorian_date.to_calendar(Hebrew)),
        };
        Some((
            molad_date,
            MoladData {
                hours,
                minutes,
                chalakim,
            },
        ))
    }

    fn get_jewish_year(&self) -> i64 {
        self.get_hebrew_date().era_year().year.into()
    }
    fn get_jewish_calendar_elapsed_days(year: i64) -> i64 {
        let chalakim_since =
            JewishDate::get_chalakim_since_molad_tohu_static(year, JewishMonth::Tishrei.into());
        let molad_day = (chalakim_since / _CHALAKIM_PER_DAY) as i64;
        let molad_parts = (chalakim_since - molad_day as i64 * _CHALAKIM_PER_DAY) as i64;

        JewishDate::add_dechiyos(year, molad_day, molad_parts)
    }
    fn get_last_day_of_gregorian_month(month: i64, year: i64) -> i64 {
        match month {
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
}

impl JewishDate {
    fn get_hebrew_date(&self) -> &Date<Hebrew> {
        &self.hebrew_date
    }

    pub fn get_gregorian_date(&self) -> Date<Gregorian> {
        self.hebrew_date.to_calendar(Gregorian)
    }
    fn get_chalakim_since_molad_tohu_static(year: i64, month: i64) -> i64 {
        let month_of_year = JewishDate::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19))
            + (12 * ((year - 1) % 19))
            + ((7 * ((year - 1) % 19) + 1) / 19)
            + (month_of_year - 1);

        _CHALAKIM_MOLAD_TOHU + (_CHALAKIM_PER_MONTH * months_elapsed as i64)
    }

    fn get_jewish_month_of_year(year: i64, month: i64) -> i64 {
        let is_leap_year = JewishDate::is_jewish_leap_year_static(year);
        (month + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
    }

    fn add_dechiyos(year: i64, molad_day: i64, molad_parts: i64) -> i64 {
        let mut rosh_hashana_day = molad_day;

        if (molad_parts >= 19440)
            || (((molad_day % 7) == 2)
                && (molad_parts >= 9924)
                && !JewishDate::is_jewish_leap_year_static(year.into()))
            || (((molad_day % 7) == 1)
                && (molad_parts >= 16789)
                && (JewishDate::is_jewish_leap_year_static(year - 1)))
        {
            rosh_hashana_day += 1;
        }

        if ((rosh_hashana_day % 7) == 0)
            || ((rosh_hashana_day % 7) == 3)
            || ((rosh_hashana_day % 7) == 5)
        {
            rosh_hashana_day += 1;
        }

        rosh_hashana_day
    }

    fn is_cheshvan_long_static(year: i64) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 5
    }

    fn is_kislev_short_static(year: i64) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 3
    }

    fn get_days_since_start_of_jewish_year_static(year: i64, month: i64, day_of_month: i64) -> i64 {
        let mut elapsed_days = day_of_month;

        if month < JewishMonth::Tishrei.into() {
            for m in JewishMonth::Tishrei.into()..=JewishDate::get_last_month_of_jewish_year(year) {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
            for m in JewishMonth::Nissan.into()..month {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
        } else {
            for m in JewishMonth::Tishrei.into()..month {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
        }

        elapsed_days
    }

    pub fn is_jewish_leap_year_static(year: i64) -> bool {
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }
    fn get_last_month_of_jewish_year(year: i64) -> i64 {
        if JewishDate::is_jewish_leap_year_static(year) {
            13
        } else {
            12
        }
    }

    pub fn get_days_in_jewish_month_static(month: i64, year: i64) -> i64 {
        match month.try_into().unwrap() {
            JewishMonth::Iyar | JewishMonth::Tammuz | JewishMonth::Elul | JewishMonth::Teves => 29,
            JewishMonth::Cheshvan => {
                if JewishDate::is_cheshvan_long_static(year) {
                    30
                } else {
                    29
                }
            }
            JewishMonth::Kislev => {
                if JewishDate::is_kislev_short_static(year) {
                    29
                } else {
                    30
                }
            }
            JewishMonth::Adar => {
                if JewishDate::is_jewish_leap_year_static(year) {
                    30
                } else {
                    29
                }
            }
            JewishMonth::AdarII => 29,
            _ => 30,
        }
    }
    fn molad_to_abs_date(chalakim: i64) -> i64 {
        _JEWISH_EPOCH + (chalakim / _CHALAKIM_PER_DAY)
    }
    fn gregorian_date_to_abs_date(year: i64, month: i64, day_of_month: i64) -> i64 {
        let mut abs_date = day_of_month;
        for m in (1..month).rev() {
            abs_date += JewishDate::get_last_day_of_gregorian_month(m, year);
        }
        abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400
    }

    fn abs_date_to_date(abs_date: i64) -> Option<Date<Gregorian>> {
        let mut year: i64 = abs_date / 366;
        while abs_date >= JewishDate::gregorian_date_to_abs_date(year + 1, 1, 1) {
            year += 1;
        }
        let mut month: i64 = 1;
        while abs_date
            > JewishDate::gregorian_date_to_abs_date(
                year,
                month,
                JewishDate::get_last_day_of_gregorian_month(month, year),
            )
        {
            month += 1;
        }
        let day_of_month: i64 =
            abs_date - JewishDate::gregorian_date_to_abs_date(year, month, 1) + 1;
        Date::try_new_gregorian(year as i32, month as u8, day_of_month as u8).ok()
    }
}

#[cfg(test)]
mod jni_tests {

    use crate::test_utils::jni::{DEFAULT_TEST_ITERATIONS, create_jewish_dates, init_jvm};

    use super::*;

    use j4rs::InvocationArg;

    #[test]
    fn test_get_jewish_year_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_jewish_year();

            let java_result = jvm
                .invoke(&java_date, "getJewishYear", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_jewish_month_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_jewish_month() as i64;

            let java_result = jvm
                .invoke(&java_date, "getJewishMonth", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_jewish_day_of_month_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_jewish_day_of_month();

            let java_result = jvm
                .invoke(&java_date, "getJewishDayOfMonth", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_gregorian_year_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_gregorian_year();

            let java_result = jvm
                .invoke(&java_date, "getGregorianYear", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_gregorian_month_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_gregorian_month();

            let java_result = jvm
                .invoke(&java_date, "getGregorianMonth", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_gregorian_day_of_month_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_gregorian_day_of_month();

            let java_result = jvm
                .invoke(&java_date, "getGregorianDayOfMonth", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_day_of_week_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_day_of_week() as i64;

            let java_result = jvm
                .invoke(&java_date, "getDayOfWeek", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_is_jewish_leap_year_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.is_jewish_leap_year();

            let java_result = jvm
                .invoke(&java_date, "isJewishLeapYear", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<bool>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_days_in_jewish_year_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_days_in_jewish_year();

            let java_result = jvm
                .invoke(&java_date, "getDaysInJewishYear", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_days_in_jewish_month_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_days_in_jewish_month();

            let java_result = jvm
                .invoke(&java_date, "getDaysInJewishMonth", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_is_cheshvan_long_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.is_cheshvan_long();

            let java_result = jvm
                .invoke(&java_date, "isCheshvanLong", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<bool>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_is_kislev_short_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.is_kislev_short();

            let java_result = jvm
                .invoke(&java_date, "isKislevShort", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<bool>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_cheshvan_kislev_kviah_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_cheshvan_kislev_kviah() as i64;

            let java_result = jvm
                .invoke(&java_date, "getCheshvanKislevKviah", InvocationArg::empty())
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_days_since_start_of_jewish_year_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_days_since_start_of_jewish_year();

            let java_result = jvm
                .invoke(
                    &java_date,
                    "getDaysSinceStartOfJewishYear",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_chalakim_since_molad_tohu_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let result = rust_date.get_chalakim_since_molad_tohu();

            let java_result = jvm
                .invoke(
                    &java_date,
                    "getChalakimSinceMoladTohu",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_result = jvm.to_rust::<i64>(java_result).unwrap();

            assert_eq!(result, java_result, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_molad_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let rust_molad = rust_date.get_molad();
            if rust_molad.is_none() {
                continue;
            }
            let rust_molad = rust_molad.unwrap();

            let java_molad = jvm
                .invoke(&java_date, "getMolad", InvocationArg::empty())
                .unwrap();

            let java_hours = jvm
                .invoke(&java_molad, "getMoladHours", InvocationArg::empty())
                .unwrap();
            let java_hours = jvm.to_rust::<i64>(java_hours).unwrap();

            let java_minutes = jvm
                .invoke(&java_molad, "getMoladMinutes", InvocationArg::empty())
                .unwrap();
            let java_minutes = jvm.to_rust::<i64>(java_minutes).unwrap();

            let java_chalakim = jvm
                .invoke(&java_molad, "getMoladChalakim", InvocationArg::empty())
                .unwrap();
            let java_chalakim = jvm.to_rust::<i64>(java_chalakim).unwrap();

            assert_eq!(
                rust_molad.get_hours(),
                java_hours,
                "Hours mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad.get_minutes(),
                java_minutes,
                "Minutes mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad.get_chalakim(),
                java_chalakim,
                "Chalakim mismatch: {}",
                message
            );
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_get_molad_as_date_against_java() {
        let jvm = init_jvm();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_dates(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_date, java_date, message) = test_case.unwrap();

            let rust_molad_date = rust_date.get_molad_as_date();
            if rust_molad_date.is_none() {
                continue;
            }
            let rust_molad_date = rust_molad_date.unwrap();

            let java_molad = jvm
                .invoke(&java_date, "getMolad", InvocationArg::empty())
                .unwrap();

            let java_year = jvm
                .invoke(&java_molad, "getJewishYear", InvocationArg::empty())
                .unwrap();
            let java_year = jvm.to_rust::<i64>(java_year).unwrap();

            let java_month = jvm
                .invoke(&java_molad, "getJewishMonth", InvocationArg::empty())
                .unwrap();
            let java_month = jvm.to_rust::<i64>(java_month).unwrap();

            let java_day = jvm
                .invoke(&java_molad, "getJewishDayOfMonth", InvocationArg::empty())
                .unwrap();
            let java_day = jvm.to_rust::<i64>(java_day).unwrap();

            assert_eq!(
                rust_molad_date.get_jewish_year(),
                java_year,
                "Year mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad_date.get_jewish_month() as i64,
                java_month,
                "Month mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad_date.get_jewish_day_of_month(),
                java_day,
                "Day mismatch: {}",
                message
            );

            let java_gregorian_year = jvm
                .invoke(&java_molad, "getGregorianYear", InvocationArg::empty())
                .unwrap();
            let java_gregorian_year = jvm.to_rust::<i64>(java_gregorian_year).unwrap();

            let java_gregorian_month = jvm
                .invoke(&java_molad, "getGregorianMonth", InvocationArg::empty())
                .unwrap();
            let java_gregorian_month = jvm.to_rust::<i64>(java_gregorian_month).unwrap();

            let java_gregorian_day = jvm
                .invoke(
                    &java_molad,
                    "getGregorianDayOfMonth",
                    InvocationArg::empty(),
                )
                .unwrap();
            let java_gregorian_day = jvm.to_rust::<i64>(java_gregorian_day).unwrap();

            assert_eq!(
                rust_molad_date.get_gregorian_year(),
                java_gregorian_year,
                "Gregorian year mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad_date.get_gregorian_month(),
                java_gregorian_month,
                "Gregorian month mismatch: {}",
                message
            );
            assert_eq!(
                rust_molad_date.get_gregorian_day_of_month(),
                java_gregorian_day,
                "Gregorian day mismatch: {}",
                message
            );
        }
        assert!(ran, "No test cases were run");
    }
}
