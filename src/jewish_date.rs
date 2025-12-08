#[cfg(feature = "defmt")]
use icu_calendar::types::{CyclicYear, YearInfo};
use icu_calendar::{
    Date, Gregorian,
    cal::Hebrew,
    options::DateAddOptions,
    types::{DateDuration, MonthCode, Weekday},
};

use crate::{constants::*, defmt::DefmtFormatTrait};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct JewishDate {
    pub(crate) hebrew_date: Date<Hebrew>,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct MoladData {
    pub hours: i64,
    pub minutes: i64,
    pub chalakim: i64,
}

pub(crate) trait InternalJewishDateTrait {
    fn get_gregorian_date(&self) -> Date<Gregorian>;
}

#[allow(private_bounds)]
pub trait JewishDateTrait: Sized + InternalJewishDateTrait + DefmtFormatTrait {
    fn get_jewish_year(&self) -> i32;

    fn get_jewish_month(&self) -> JewishMonth;

    fn get_jewish_day_of_month(&self) -> u8;

    fn get_gregorian_year(&self) -> i32;

    fn get_gregorian_month(&self) -> u8;

    fn get_gregorian_day_of_month(&self) -> u8;

    fn get_day_of_week(&self) -> DayOfWeek;

    fn is_jewish_leap_year(&self) -> bool;

    fn get_days_in_jewish_year(&self) -> i32;

    fn get_days_in_jewish_month(&self) -> u8;

    fn is_cheshvan_long(&self) -> bool;

    fn is_kislev_short(&self) -> bool;

    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;

    fn get_days_since_start_of_jewish_year(&self) -> i32;

    fn get_chalakim_since_molad_tohu(&self) -> i64;

    fn get_molad_as_date(&self) -> Option<impl JewishDateTrait>;

    fn get_molad(&self) -> Option<MoladData>;
}

impl InternalJewishDateTrait for JewishDate {
    fn get_gregorian_date(&self) -> Date<Gregorian> {
        self.get_hebrew_date().to_calendar(Gregorian)
    }
}

impl JewishDateTrait for JewishDate {
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
            _ => unreachable!(),
        }
    }

    fn get_jewish_day_of_month(&self) -> u8 {
        self.get_hebrew_date().day_of_month().0
    }

    fn get_gregorian_year(&self) -> i32 {
        self.get_gregorian_date().era_year().year
    }

    fn get_gregorian_month(&self) -> u8 {
        self.get_gregorian_date().month().ordinal - 1
    }

    fn get_gregorian_day_of_month(&self) -> u8 {
        self.get_gregorian_date().day_of_month().0
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

    fn get_days_in_jewish_year(&self) -> i32 {
        JewishDate::get_days_in_jewish_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_month(&self) -> u8 {
        JewishDate::get_days_in_jewish_month_static(self.get_jewish_month(), self.get_jewish_year())
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
        } else if !JewishDate::is_cheshvan_long_static(year) && JewishDate::is_kislev_short_static(year) {
            YearLengthType::Chaserim
        } else {
            YearLengthType::Kesidran
        }
    }

    fn get_days_since_start_of_jewish_year(&self) -> i32 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();

        let mut elapsed_days: i32 = day as i32;
        if month < JewishMonth::Tishrei {
            for m in JewishMonth::range_inclusive(JewishMonth::Tishrei, JewishDate::get_last_month_of_jewish_year(year))
            {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year) as i32;
            }
            for m in JewishMonth::range(JewishMonth::Nissan, month) {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year) as i32;
            }
        } else {
            for m in JewishMonth::range(JewishMonth::Tishrei, month) {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year) as i32;
            }
        }
        elapsed_days
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month();
        JewishDate::get_chalakim_since_molad_tohu_static(year, month.into())
    }

    fn get_molad_as_date(&self) -> Option<impl JewishDateTrait> {
        let (date, _) = self._get_molad()?;
        Some(date)
    }

    fn get_molad(&self) -> Option<MoladData> {
        let (_, molad) = self._get_molad()?;
        Some(molad)
    }

    fn get_jewish_year(&self) -> i32 {
        self.get_hebrew_date().era_year().year
    }
}

impl JewishDate {
    pub fn get_days_in_jewish_month_static(month: JewishMonth, year: i32) -> u8 {
        match month {
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
    pub fn get_days_in_jewish_year_static(year: i32) -> i32 {
        JewishDate::get_jewish_calendar_elapsed_days(year + 1) - JewishDate::get_jewish_calendar_elapsed_days(year)
    }
    pub fn get_jewish_calendar_elapsed_days(year: i32) -> i32 {
        let chalakim_since = JewishDate::get_chalakim_since_molad_tohu_static(year, JewishMonth::Tishrei.into());
        let molad_day = chalakim_since / _CHALAKIM_PER_DAY;
        let molad_parts = chalakim_since - molad_day * _CHALAKIM_PER_DAY;

        JewishDate::add_dechiyos(year, molad_day, molad_parts)
    }
    pub fn get_last_day_of_gregorian_month(month: u8, year: i32) -> u8 {
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

    pub fn is_jewish_leap_year_static(year: i32) -> bool {
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }
    pub fn from_jewish_date(year: i32, month: JewishMonth, day: u8) -> Option<Self> {
        let is_leap_year = Date::try_new_from_codes(Some("am"), year, MonthCode("M01".parse().ok()?), 1, Hebrew)
            .ok()?
            .is_in_leap_year();

        let month_code: MonthCode = match is_leap_year {
            true => {
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

                MonthCode(month_code_str.parse().ok()?)
            }
            false => {
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
                MonthCode(month_code_str.parse().ok()?)
            }
        };

        let hebrew_date = Date::try_new_from_codes(Some("am"), year, month_code, day, Hebrew);

        let hebrew_date = hebrew_date.ok()?;

        Some(JewishDate { hebrew_date })
    }
    pub fn from_gregorian_date(year: i32, month: u8, day: u8) -> Option<Self> {
        let gregorian_date = Date::try_new_iso(year, month, day).ok()?;

        Some(JewishDate {
            hebrew_date: gregorian_date.to_calendar(Hebrew),
        })
    }

    fn get_hebrew_date(&self) -> &Date<Hebrew> {
        &self.hebrew_date
    }
    fn _get_molad(&self) -> Option<(impl JewishDateTrait, MoladData)> {
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
            gregorian_date
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

    fn get_chalakim_since_molad_tohu_static(year: i32, month: u8) -> i64 {
        let month_of_year = JewishDate::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19))
            + (12 * ((year - 1) % 19))
            + ((7 * ((year - 1) % 19) + 1) / 19)
            + (month_of_year as i32 - 1);

        _CHALAKIM_MOLAD_TOHU + (_CHALAKIM_PER_MONTH * months_elapsed as i64)
    }

    fn get_jewish_month_of_year(year: i32, month: u8) -> u8 {
        let is_leap_year = JewishDate::is_jewish_leap_year_static(year);
        (month + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
    }

    fn add_dechiyos(year: i32, molad_day: i64, molad_parts: i64) -> i32 {
        let mut rosh_hashana_day = molad_day;

        if (molad_parts >= 19440)
            || (((molad_day % 7) == 2) && (molad_parts >= 9924) && !JewishDate::is_jewish_leap_year_static(year))
            || (((molad_day % 7) == 1) && (molad_parts >= 16789) && (JewishDate::is_jewish_leap_year_static(year - 1)))
        {
            rosh_hashana_day += 1;
        }

        if ((rosh_hashana_day % 7) == 0) || ((rosh_hashana_day % 7) == 3) || ((rosh_hashana_day % 7) == 5) {
            rosh_hashana_day += 1;
        }

        rosh_hashana_day as i32
    }

    fn is_cheshvan_long_static(year: i32) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 5
    }

    fn is_kislev_short_static(year: i32) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 3
    }

    fn get_last_month_of_jewish_year(year: i32) -> JewishMonth {
        if JewishDate::is_jewish_leap_year_static(year) {
            JewishMonth::AdarII
        } else {
            JewishMonth::Adar
        }
    }

    fn molad_to_abs_date(chalakim: i64) -> i64 {
        _JEWISH_EPOCH + (chalakim / _CHALAKIM_PER_DAY)
    }
    fn gregorian_date_to_abs_date(year: i32, month: u8, day_of_month: u8) -> i64 {
        let mut abs_date = day_of_month as i64;
        for m in (1..month).rev() {
            abs_date += JewishDate::get_last_day_of_gregorian_month(m, year) as i64;
        }
        let year: i64 = year as i64;
        abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400
    }

    fn abs_date_to_date(abs_date: i64) -> Option<Date<Gregorian>> {
        let mut year = (abs_date / 366) as i32;
        while abs_date >= JewishDate::gregorian_date_to_abs_date(year + 1, 1, 1) {
            year += 1;
        }
        let mut month: u8 = 1;
        while abs_date
            > JewishDate::gregorian_date_to_abs_date(
                year,
                month,
                JewishDate::get_last_day_of_gregorian_month(month, year),
            )
        {
            month += 1;
        }
        let day_of_month: u8 = (abs_date - JewishDate::gregorian_date_to_abs_date(year, month, 1) + 1) as u8;
        Date::try_new_gregorian(year, month, day_of_month).ok()
    }
}

impl DefmtFormatTrait for JewishDate {}

#[cfg(feature = "defmt")]
impl defmt::Format for JewishDate {
    fn format(&self, f: defmt::Formatter) {
        let month = self.hebrew_date.month().ordinal;
        let day = self.hebrew_date.day_of_month().0;
        match self.hebrew_date.year() {
            YearInfo::Era(era_year) => {
                defmt::write!(
                    f,
                    "JewishDate({}-{}-{}, {} era)",
                    era_year.year,
                    month,
                    day,
                    era_year.era.as_str()
                )
            }
            YearInfo::Cyclic(CyclicYear { year, related_iso, .. }) => {
                defmt::write!(f, "JewishDate({}-{}-{}, ISO year {})", year, month, day, related_iso)
            }
            _ => {
                defmt::write!(f, "JewishDate(unknown year-{}-{})", month, day)
            }
        }
    }
}
