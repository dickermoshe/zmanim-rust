use core::fmt::Debug;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Days;
use chrono::NaiveDate;
use chrono::Utc;
use icu_calendar::Date;
use icu_calendar::Gregorian;
use icu_calendar::cal::Hebrew;
use icu_calendar::options::DateAddOptions;
use icu_calendar::types::DateDuration;
use icu_calendar::types::MonthCode;
use icu_calendar::types::Weekday;

use crate::astronomical_calculator::AstronomicalCalculatorTrait;
use crate::astronomical_calculator::get_julian_day;
use crate::constants::*;
use crate::daf::*;
// use crate:::InternalJewishCalendarTrait;
// use crate:::JewishCalendar;
// use crate:::JewishCalendarTrait;
use crate::parshas::*;
#[allow(unused_imports)]
use core_maths::CoreFloat;

#[allow(private_bounds)]
pub trait JewishCalendarTrait {
    fn get_jewish_year(&self) -> i32;
    fn get_jewish_month(&self) -> JewishMonth;
    fn get_jewish_day_of_month(&self) -> u8;
    fn get_gregorian_year(&self) -> i32;
    fn get_gregorian_month(&self) -> u8;
    fn get_gregorian_day_of_month(&self) -> u8;
    fn get_molad_as_date(&self) -> Option<DateTime<Utc>>;
    fn get_molad_as_calendar(&self) -> Option<impl JewishCalendarTrait>;
    fn get_day_of_week(&self) -> DayOfWeek;
    fn is_jewish_leap_year(&self) -> bool;
    fn get_days_in_jewish_year(&self) -> i32;
    fn get_days_in_jewish_month(&self) -> u8;
    fn is_cheshvan_long(&self) -> bool;
    fn is_kislev_short(&self) -> bool;
    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;
    fn get_days_since_start_of_jewish_year(&self) -> i32;
    fn get_chalakim_since_molad_tohu(&self) -> i64;
    fn get_molad(&self) -> Option<MoladData>;
    fn get_yom_tov_index(&self) -> Option<JewishHoliday>;
    fn is_yom_tov(&self) -> bool;
    fn is_yom_tov_assur_bemelacha(&self) -> bool;
    fn is_assur_bemelacha(&self) -> bool;
    fn has_candle_lighting(&self) -> bool;
    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool;
    fn is_erev_yom_tov_sheni(&self) -> bool;
    fn is_aseres_yemei_teshuva(&self) -> bool;
    fn is_pesach(&self) -> bool;
    fn is_chol_hamoed_pesach(&self) -> bool;
    fn is_shavuos(&self) -> bool;
    fn is_rosh_hashana(&self) -> bool;
    fn is_yom_kippur(&self) -> bool;
    fn is_succos(&self) -> bool;
    fn is_hoshana_rabba(&self) -> bool;
    fn is_shemini_atzeres(&self) -> bool;
    fn is_simchas_torah(&self) -> bool;
    fn is_chol_hamoed_succos(&self) -> bool;
    fn is_chol_hamoed(&self) -> bool;
    fn is_erev_yom_tov(&self) -> bool;
    fn is_rosh_chodesh(&self) -> bool;
    fn is_isru_chag(&self) -> bool;
    fn is_taanis(&self) -> bool;
    fn is_taanis_bechoros(&self) -> bool;
    fn get_day_of_chanukah(&self) -> Option<u8>;
    fn is_chanukah(&self) -> bool;
    fn is_purim(&self) -> bool;
    fn get_day_of_omer(&self) -> Option<u8>;
    fn is_tisha_beav(&self) -> bool;
    fn get_parshah(&self) -> Option<Parsha>;
    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf>;
    fn get_daf_yomi_yerushalmi(&self) -> Option<YerushalmiDaf>;
    fn is_birkas_hachamah(&self) -> bool;
    fn is_erev_rosh_chodesh(&self) -> bool;
    fn is_yom_kippur_katan(&self) -> bool;
    fn is_be_hab(&self) -> bool;
    fn is_machar_chodesh(&self) -> bool;
    fn is_shabbos_mevorchim(&self) -> bool;
    fn get_upcoming_parshah(&self) -> Option<Parsha>;
    fn get_special_shabbos(&self) -> Option<Parsha>;
    fn get_tchilaszman_kidush_levana_3_days(&self) -> Option<DateTime<Utc>>;
    fn get_tchilaszman_kidush_levana_7_days(&self) -> Option<DateTime<Utc>>;
    fn get_sof_zman_kidush_levana_between_moldos(&self) -> Option<DateTime<Utc>>;
    fn get_sof_zman_kidush_levana_15_days(&self) -> Option<DateTime<Utc>>;
    fn get_tekufas_tishrei_elapsed_days(&self) -> i64;
    fn is_vesein_tal_umatar_start_date(&self) -> bool;
    fn is_vesein_tal_umatar_starting_tonight(&self) -> bool;
    fn is_vesein_tal_umatar_recited(&self) -> bool;
    fn is_vesein_beracha_recited(&self) -> bool;
    fn is_mashiv_haruach_start_date(&self) -> bool;
    fn is_mashiv_haruach_end_date(&self) -> bool;
    fn is_mashiv_haruach_recited(&self) -> Option<bool>;
    fn is_morid_hatal_recited(&self) -> Option<bool>;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct MoladData {
    pub hours: i64,
    pub minutes: i64,
    pub chalakim: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct JewishCalendar<N: AstronomicalCalculatorTrait> {
    pub(crate) hebrew_date: Date<Hebrew>,
    pub in_israel: bool,
    pub is_mukaf_choma: bool,
    pub use_modern_holidays: bool,
    pub(crate) calculator: N,
}

impl<N: AstronomicalCalculatorTrait> JewishCalendar<N> {
    pub(crate) fn get_gregorian_date(&self) -> Date<Gregorian> {
        self.get_hebrew_date().to_calendar(Gregorian)
    }
    pub fn get_days_in_jewish_month_static(month: JewishMonth, year: i32) -> u8 {
        match month {
            JewishMonth::Iyar | JewishMonth::Tammuz | JewishMonth::Elul | JewishMonth::Teves => 29,
            JewishMonth::Cheshvan => {
                if JewishCalendar::<N>::is_cheshvan_long_static(year) {
                    30
                } else {
                    29
                }
            }
            JewishMonth::Kislev => {
                if JewishCalendar::<N>::is_kislev_short_static(year) {
                    29
                } else {
                    30
                }
            }
            JewishMonth::Adar => {
                if JewishCalendar::<N>::is_jewish_leap_year_static(year) {
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
        JewishCalendar::<N>::get_jewish_calendar_elapsed_days(year + 1)
            - JewishCalendar::<N>::get_jewish_calendar_elapsed_days(year)
    }
    pub fn get_jewish_calendar_elapsed_days(year: i32) -> i32 {
        let chalakim_since =
            JewishCalendar::<N>::get_chalakim_since_molad_tohu_static(year, JewishMonth::Tishrei.into());
        let molad_day = chalakim_since / _CHALAKIM_PER_DAY;
        let molad_parts = chalakim_since - molad_day * _CHALAKIM_PER_DAY;

        JewishCalendar::<N>::add_dechiyos(year, molad_day, molad_parts)
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
    pub fn from_hebrew_date(
        year: i32,
        month: JewishMonth,
        day: u8,
        in_israel: bool,
        is_mukaf_choma: bool,
        use_modern_holidays: bool,
        calculator: N,
    ) -> Option<Self> {
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

        Some(JewishCalendar {
            hebrew_date,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            calculator,
        })
    }
    pub fn from_gregorian_date(
        year: i32,
        month: u8,
        day: u8,
        in_israel: bool,
        is_mukaf_choma: bool,
        use_modern_holidays: bool,
        calculator: N,
    ) -> Option<Self> {
        let gregorian_date = Date::try_new_iso(year, month, day).ok()?;

        Some(JewishCalendar {
            hebrew_date: gregorian_date.to_calendar(Hebrew),
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            calculator,
        })
    }
    pub(crate) fn copy_with_date(&self, date: Date<Hebrew>) -> Self {
        Self {
            hebrew_date: date,
            in_israel: self.in_israel,
            is_mukaf_choma: self.is_mukaf_choma,
            use_modern_holidays: self.use_modern_holidays,
            calculator: self.calculator.clone(),
        }
    }
    pub(crate) fn copy_with_hebrew_ymd(&self, year: i32, month: JewishMonth, day: u8) -> Option<Self> {
        Self::from_hebrew_date(
            year,
            month,
            day,
            self.in_israel,
            self.is_mukaf_choma,
            self.use_modern_holidays,
            self.calculator.clone(),
        )
    }
    pub(crate) fn copy_with_gregorian_ymd(&self, year: i32, month: u8, day: u8) -> Option<Self> {
        Self::from_gregorian_date(
            year,
            month,
            day,
            self.in_israel,
            self.is_mukaf_choma,
            self.use_modern_holidays,
            self.calculator.clone(),
        )
    }

    fn get_hebrew_date(&self) -> &Date<Hebrew> {
        &self.hebrew_date
    }
    fn _get_molad(&self) -> Option<(impl JewishCalendarTrait, MoladData)> {
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
        let molad_date = self.copy_with_date(gregorian_date.to_calendar(Hebrew));
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
        let month_of_year = JewishCalendar::<N>::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19))
            + (12 * ((year - 1) % 19))
            + ((7 * ((year - 1) % 19) + 1) / 19)
            + (month_of_year as i32 - 1);

        _CHALAKIM_MOLAD_TOHU + (_CHALAKIM_PER_MONTH * months_elapsed as i64)
    }

    fn get_jewish_month_of_year(year: i32, month: u8) -> u8 {
        let is_leap_year = JewishCalendar::<N>::is_jewish_leap_year_static(year);
        (month + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
    }

    fn add_dechiyos(year: i32, molad_day: i64, molad_parts: i64) -> i32 {
        let mut rosh_hashana_day = molad_day;

        if (molad_parts >= 19440)
            || (((molad_day % 7) == 2)
                && (molad_parts >= 9924)
                && !JewishCalendar::<N>::is_jewish_leap_year_static(year))
            || (((molad_day % 7) == 1)
                && (molad_parts >= 16789)
                && (JewishCalendar::<N>::is_jewish_leap_year_static(year - 1)))
        {
            rosh_hashana_day += 1;
        }

        if ((rosh_hashana_day % 7) == 0) || ((rosh_hashana_day % 7) == 3) || ((rosh_hashana_day % 7) == 5) {
            rosh_hashana_day += 1;
        }

        rosh_hashana_day as i32
    }

    fn is_cheshvan_long_static(year: i32) -> bool {
        JewishCalendar::<N>::get_days_in_jewish_year_static(year) % 10 == 5
    }

    fn is_kislev_short_static(year: i32) -> bool {
        JewishCalendar::<N>::get_days_in_jewish_year_static(year) % 10 == 3
    }

    fn get_last_month_of_jewish_year(year: i32) -> JewishMonth {
        if JewishCalendar::<N>::is_jewish_leap_year_static(year) {
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
            abs_date += JewishCalendar::<N>::get_last_day_of_gregorian_month(m, year) as i64;
        }
        let year: i64 = year as i64;
        abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400
    }

    fn abs_date_to_date(abs_date: i64) -> Option<Date<Gregorian>> {
        let mut year = (abs_date / 366) as i32;
        while abs_date >= JewishCalendar::<N>::gregorian_date_to_abs_date(year + 1, 1, 1) {
            year += 1;
        }
        let mut month: u8 = 1;
        while abs_date
            > JewishCalendar::<N>::gregorian_date_to_abs_date(
                year,
                month,
                JewishCalendar::<N>::get_last_day_of_gregorian_month(month, year),
            )
        {
            month += 1;
        }
        let day_of_month: u8 = (abs_date - JewishCalendar::<N>::gregorian_date_to_abs_date(year, month, 1) + 1) as u8;
        Date::try_new_gregorian(year, month, day_of_month).ok()
    }

    fn get_num_of_special_days(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Option<u64> {
        let start_year = self
            .copy_with_gregorian_ymd(start.year(), start.month() as u8, start.day() as u8)?
            .get_jewish_year();
        let end_year = self
            .copy_with_gregorian_ymd(end.year(), end.month() as u8, end.day() as u8)?
            .get_jewish_year();

        let mut special_days = 0u64;
        for i in start_year..=end_year {
            // Create new calendar instances for each year
            let yom_kippur_date = self.copy_with_hebrew_ymd(i, JewishMonth::Tishrei, 10)?;
            let tisha_beav_date = self.copy_with_hebrew_ymd(i, JewishMonth::Av, 9)?;

            // Get Gregorian dates and convert to DateTime<Utc>
            let yom_kippur_dt = icu_to_naive(&yom_kippur_date.get_gregorian_date())?;
            let tisha_beav_dt = icu_to_naive(&tisha_beav_date.get_gregorian_date())?;

            // Check if dates are strictly between start and end (matching Java's isBetween logic)
            if yom_kippur_dt > start && yom_kippur_dt < end {
                special_days += 1;
            }
            if tisha_beav_dt > start && tisha_beav_dt < end {
                special_days += 1;
            }
        }
        Some(special_days)
    }
    fn get_diff_between_days(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> u64 {
        end.signed_duration_since(start).num_days() as u64
    }

    fn get_parsha_list(&self) -> Option<ParshaList> {
        let rosh_hashana_day_of_week =
            (JewishCalendar::<N>::get_jewish_calendar_elapsed_days(self.get_jewish_year()) + 1) % 7;
        let rosh_hashana_day_of_week = if rosh_hashana_day_of_week == 0 {
            7
        } else {
            rosh_hashana_day_of_week
        };

        if self.is_jewish_leap_year() {
            match rosh_hashana_day_of_week {
                2 => {
                    if self.is_kislev_short() {
                        if self.in_israel {
                            Some(PARSHA_LIST_14)
                        } else {
                            Some(PARSHA_LIST_6)
                        }
                    } else if self.is_cheshvan_long() {
                        if self.in_israel {
                            Some(PARSHA_LIST_15)
                        } else {
                            Some(PARSHA_LIST_7)
                        }
                    } else {
                        None
                    }
                }
                3 => {
                    if self.in_israel {
                        Some(PARSHA_LIST_15)
                    } else {
                        Some(PARSHA_LIST_7)
                    }
                }
                5 => {
                    if self.is_kislev_short() {
                        Some(PARSHA_LIST_8)
                    } else if self.is_cheshvan_long() {
                        Some(PARSHA_LIST_9)
                    } else {
                        None
                    }
                }
                7 => {
                    if self.is_kislev_short() {
                        Some(PARSHA_LIST_10)
                    } else if self.is_cheshvan_long() {
                        if self.in_israel {
                            Some(PARSHA_LIST_16)
                        } else {
                            Some(PARSHA_LIST_11)
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            match rosh_hashana_day_of_week {
                2 => {
                    if self.is_kislev_short() {
                        Some(PARSHA_LIST_0)
                    } else if self.is_cheshvan_long() {
                        if self.in_israel {
                            Some(PARSHA_LIST_12)
                        } else {
                            Some(PARSHA_LIST_1)
                        }
                    } else {
                        None
                    }
                }
                3 => {
                    if self.in_israel {
                        Some(PARSHA_LIST_12)
                    } else {
                        Some(PARSHA_LIST_1)
                    }
                }
                5 => {
                    if self.is_cheshvan_long() {
                        Some(PARSHA_LIST_3)
                    } else if !self.is_kislev_short() {
                        if self.in_israel {
                            Some(PARSHA_LIST_13)
                        } else {
                            Some(PARSHA_LIST_2)
                        }
                    } else {
                        None
                    }
                }
                7 => {
                    if self.is_kislev_short() {
                        Some(PARSHA_LIST_4)
                    } else if self.is_cheshvan_long() {
                        Some(PARSHA_LIST_5)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

impl<N: AstronomicalCalculatorTrait> JewishCalendarTrait for JewishCalendar<N> {
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
        JewishCalendar::<N>::is_jewish_leap_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_year(&self) -> i32 {
        JewishCalendar::<N>::get_days_in_jewish_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_month(&self) -> u8 {
        JewishCalendar::<N>::get_days_in_jewish_month_static(self.get_jewish_month(), self.get_jewish_year())
    }

    fn is_cheshvan_long(&self) -> bool {
        JewishCalendar::<N>::is_cheshvan_long_static(self.get_jewish_year())
    }

    fn is_kislev_short(&self) -> bool {
        JewishCalendar::<N>::is_kislev_short_static(self.get_jewish_year())
    }

    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType {
        let year = self.get_jewish_year();
        if JewishCalendar::<N>::is_cheshvan_long_static(year) && !JewishCalendar::<N>::is_kislev_short_static(year) {
            YearLengthType::Shelaimim
        } else if !JewishCalendar::<N>::is_cheshvan_long_static(year)
            && JewishCalendar::<N>::is_kislev_short_static(year)
        {
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
            for m in JewishMonth::range_inclusive(
                JewishMonth::Tishrei,
                JewishCalendar::<N>::get_last_month_of_jewish_year(year),
            ) {
                elapsed_days += JewishCalendar::<N>::get_days_in_jewish_month_static(m, year) as i32;
            }
            for m in JewishMonth::range(JewishMonth::Nissan, month) {
                elapsed_days += JewishCalendar::<N>::get_days_in_jewish_month_static(m, year) as i32;
            }
        } else {
            for m in JewishMonth::range(JewishMonth::Tishrei, month) {
                elapsed_days += JewishCalendar::<N>::get_days_in_jewish_month_static(m, year) as i32;
            }
        }
        elapsed_days
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month();
        JewishCalendar::<N>::get_chalakim_since_molad_tohu_static(year, month.into())
    }

    fn get_molad(&self) -> Option<MoladData> {
        let (_, molad) = self._get_molad()?;
        Some(molad)
    }

    fn get_molad_as_calendar(&self) -> Option<impl JewishCalendarTrait> {
        let (date, _) = self._get_molad()?;
        Some(date)
    }

    fn get_jewish_year(&self) -> i32 {
        self.get_hebrew_date().era_year().year
    }
    fn get_yom_tov_index(&self) -> Option<JewishHoliday> {
        let day = self.get_jewish_day_of_month();
        let day_of_week = self.get_day_of_week();
        let month = self.get_jewish_month();

        match month {
            JewishMonth::Nissan => {
                if day == 14 {
                    return Some(JewishHoliday::ErevPesach);
                }
                if day == 15 || day == 21 || (!self.in_israel && (day == 16 || day == 22)) {
                    return Some(JewishHoliday::Pesach);
                }
                if (17..=20).contains(&day) || day == 16 {
                    return Some(JewishHoliday::CholHamoedPesach);
                }
                if day == 22 || day == 23 && !self.in_israel {
                    return Some(JewishHoliday::IsruChag);
                }
                if self.use_modern_holidays
                    && ((day == 26 && day_of_week == DayOfWeek::Thursday)
                        || (day == 28 && day_of_week == DayOfWeek::Monday)
                        || (day == 27 && day_of_week != DayOfWeek::Sunday && day_of_week != DayOfWeek::Friday))
                {
                    return Some(JewishHoliday::YomHaShoah);
                }
            }

            JewishMonth::Iyar => {
                if self.use_modern_holidays {
                    if (day == 4 && day_of_week == DayOfWeek::Tuesday)
                        || ((day == 3 || day == 2) && day_of_week == DayOfWeek::Wednesday)
                        || (day == 5 && day_of_week == DayOfWeek::Monday)
                    {
                        return Some(JewishHoliday::YomHazikaron);
                    }
                    if (day == 5 && day_of_week == DayOfWeek::Wednesday)
                        || ((day == 4 || day == 3) && day_of_week == DayOfWeek::Thursday)
                        || (day == 6 && day_of_week == DayOfWeek::Tuesday)
                    {
                        return Some(JewishHoliday::YomHaatzmaut);
                    }
                }
                if day == 14 {
                    return Some(JewishHoliday::PesachSheni);
                }
                if day == 18 {
                    return Some(JewishHoliday::LagBomer);
                }
                if self.use_modern_holidays && day == 28 {
                    return Some(JewishHoliday::YomYerushalayim);
                }
            }

            JewishMonth::Sivan => {
                if day == 5 {
                    return Some(JewishHoliday::ErevShavuos);
                }
                if day == 6 || (day == 7 && !self.in_israel) {
                    return Some(JewishHoliday::Shavuos);
                }
                if (day == 7 && self.in_israel) || (day == 8 && !self.in_israel) {
                    return Some(JewishHoliday::IsruChag);
                }
            }

            JewishMonth::Tammuz => {
                if (day == 17 && day_of_week != DayOfWeek::Shabbos) || (day == 18 && day_of_week == DayOfWeek::Sunday) {
                    return Some(JewishHoliday::SeventeenthOfTammuz);
                }
            }

            JewishMonth::Av => {
                if (day_of_week == DayOfWeek::Sunday && day == 10) || (day_of_week != DayOfWeek::Shabbos && day == 9) {
                    return Some(JewishHoliday::TishahBav);
                }
                if day == 15 {
                    return Some(JewishHoliday::TuBav);
                }
            }

            JewishMonth::Elul => {
                if day == 29 {
                    return Some(JewishHoliday::ErevRoshHashana);
                }
            }

            JewishMonth::Tishrei => {
                if day == 1 || day == 2 {
                    return Some(JewishHoliday::RoshHashana);
                }
                if (day == 3 && day_of_week != DayOfWeek::Shabbos) || (day == 4 && day_of_week == DayOfWeek::Sunday) {
                    return Some(JewishHoliday::FastOfGedalyah);
                }
                if day == 9 {
                    return Some(JewishHoliday::ErevYomKippur);
                }
                if day == 10 {
                    return Some(JewishHoliday::YomKippur);
                }
                if day == 14 {
                    return Some(JewishHoliday::ErevSuccos);
                }
                if day == 15 {
                    return Some(JewishHoliday::Succos);
                }
                if day == 16 && !self.in_israel {
                    return Some(JewishHoliday::Succos);
                }
                if (16..=20).contains(&day) {
                    return Some(JewishHoliday::CholHamoedSuccos);
                }
                if day == 21 {
                    return Some(JewishHoliday::HoshanaRabbah);
                }
                if day == 22 {
                    return Some(JewishHoliday::SheminiAtzeres);
                }
                if day == 23 && !self.in_israel {
                    return Some(JewishHoliday::SimchasTorah);
                }
                if day == 24 && !self.in_israel || (day == 23 && self.in_israel) {
                    return Some(JewishHoliday::IsruChag);
                }
            }

            JewishMonth::Kislev => {
                if day >= 25 {
                    return Some(JewishHoliday::Chanukah);
                }
            }

            JewishMonth::Teves => {
                if day == 1 || day == 2 || (day == 3 && self.is_kislev_short()) {
                    return Some(JewishHoliday::Chanukah);
                }
                if day == 10 {
                    return Some(JewishHoliday::TenthOfTeves);
                }
            }

            JewishMonth::Shevat => {
                if day == 15 {
                    return Some(JewishHoliday::TuBshvat);
                }
            }

            JewishMonth::Adar => {
                if !self.is_jewish_leap_year() {
                    if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                        || (day == 13 && !(day_of_week == DayOfWeek::Friday || day_of_week == DayOfWeek::Shabbos))
                    {
                        return Some(JewishHoliday::FastOfEsther);
                    }
                    if day == 14 {
                        return Some(JewishHoliday::Purim);
                    }
                    if day == 15 {
                        return Some(JewishHoliday::ShushanPurim);
                    }
                } else {
                    if day == 14 {
                        return Some(JewishHoliday::PurimKatan);
                    }
                    if day == 15 {
                        return Some(JewishHoliday::ShushanPurimKatan);
                    }
                }
            }

            JewishMonth::AdarII => {
                if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                    || (day == 13 && !(day_of_week == DayOfWeek::Friday || day_of_week == DayOfWeek::Shabbos))
                {
                    return Some(JewishHoliday::FastOfEsther);
                }
                if day == 14 {
                    return Some(JewishHoliday::Purim);
                }
                if day == 15 {
                    return Some(JewishHoliday::ShushanPurim);
                }
            }
            _ => {}
        }

        None
    }

    fn is_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        if self.is_erev_yom_tov()
            && !matches!(
                holiday_index,
                Some(JewishHoliday::HoshanaRabbah) | Some(JewishHoliday::CholHamoedPesach)
            )
        {
            return false;
        }
        if self.is_taanis() && holiday_index != Some(JewishHoliday::YomKippur) {
            return false;
        }
        if holiday_index == Some(JewishHoliday::IsruChag) {
            return false;
        }
        holiday_index.is_some()
    }

    fn is_yom_tov_assur_bemelacha(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(JewishHoliday::Pesach)
                | Some(JewishHoliday::Shavuos)
                | Some(JewishHoliday::Succos)
                | Some(JewishHoliday::SheminiAtzeres)
                | Some(JewishHoliday::SimchasTorah)
                | Some(JewishHoliday::RoshHashana)
                | Some(JewishHoliday::YomKippur)
        )
    }

    fn is_assur_bemelacha(&self) -> bool {
        self.get_day_of_week() == DayOfWeek::Shabbos || self.is_yom_tov_assur_bemelacha()
    }

    fn has_candle_lighting(&self) -> bool {
        self.is_tomorrow_shabbos_or_yom_tov()
    }

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.get_day_of_week() == DayOfWeek::Friday || self.is_erev_yom_tov() || self.is_erev_yom_tov_sheni()
    }

    fn is_erev_yom_tov_sheni(&self) -> bool {
        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();
        if month == JewishMonth::Tishrei && (day == 1) {
            return true;
        }
        if !self.in_israel {
            if month == JewishMonth::Nissan && (day == 15 || day == 21) {
                return true;
            }
            if month == JewishMonth::Tishrei && (day == 15 || day == 22) {
                return true;
            }
            if month == JewishMonth::Sivan && day == 6 {
                return true;
            }
        }
        false
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        let month = self.get_jewish_month() as i32;
        let day = self.get_jewish_day_of_month();
        month == JewishMonth::Tishrei as i32 && day <= 10
    }

    fn is_pesach(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(JewishHoliday::Pesach) | Some(JewishHoliday::CholHamoedPesach)
        )
    }

    fn is_chol_hamoed_pesach(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::CholHamoedPesach)
    }

    fn is_shavuos(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::Shavuos)
    }

    fn is_rosh_hashana(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::RoshHashana)
    }

    fn is_yom_kippur(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::YomKippur)
    }

    fn is_succos(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(JewishHoliday::Succos) | Some(JewishHoliday::CholHamoedSuccos) | Some(JewishHoliday::HoshanaRabbah)
        )
    }

    fn is_hoshana_rabba(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::HoshanaRabbah)
    }

    fn is_shemini_atzeres(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::SheminiAtzeres)
    }

    fn is_simchas_torah(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::SimchasTorah)
    }

    fn is_chol_hamoed_succos(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(JewishHoliday::CholHamoedSuccos) | Some(JewishHoliday::HoshanaRabbah)
        )
    }

    fn is_chol_hamoed(&self) -> bool {
        self.is_chol_hamoed_pesach() || self.is_chol_hamoed_succos()
    }

    fn is_erev_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        holiday_index == Some(JewishHoliday::ErevPesach)
            || holiday_index == Some(JewishHoliday::ErevShavuos)
            || holiday_index == Some(JewishHoliday::ErevRoshHashana)
            || holiday_index == Some(JewishHoliday::ErevYomKippur)
            || holiday_index == Some(JewishHoliday::ErevSuccos)
            || holiday_index == Some(JewishHoliday::HoshanaRabbah)
            || (holiday_index == Some(JewishHoliday::CholHamoedPesach) && self.get_jewish_day_of_month() == 20)
    }

    fn is_rosh_chodesh(&self) -> bool {
        let day = self.get_jewish_day_of_month();
        let month = self.get_jewish_month() as i32;
        (day == 1 && month != JewishMonth::Tishrei as i32) || day == 30
    }

    fn is_isru_chag(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::IsruChag)
    }

    fn is_taanis(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(JewishHoliday::SeventeenthOfTammuz)
                | Some(JewishHoliday::TishahBav)
                | Some(JewishHoliday::YomKippur)
                | Some(JewishHoliday::FastOfGedalyah)
                | Some(JewishHoliday::TenthOfTeves)
                | Some(JewishHoliday::FastOfEsther)
        )
    }

    fn is_taanis_bechoros(&self) -> bool {
        let day = self.get_jewish_day_of_month();
        let day_of_week = self.get_day_of_week() as i32;
        let month = self.get_jewish_month() as i32;

        month == JewishMonth::Nissan as i32 && ((day == 14 && day_of_week != 7) || (day == 12 && day_of_week == 5))
    }

    fn get_day_of_chanukah(&self) -> Option<u8> {
        if !self.is_chanukah() {
            return None;
        }

        let month = self.get_jewish_month() as i32;
        let day = self.get_jewish_day_of_month();

        if month == JewishMonth::Kislev as i32 {
            Some(day - 24)
        } else if self.is_kislev_short() {
            Some(day + 5)
        } else {
            Some(day + 6)
        }
    }

    fn is_chanukah(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::Chanukah)
    }

    fn is_purim(&self) -> bool {
        // TODO: It is silly that we return false here but get PURIM when askimg for the index
        // even when in a mukaf choma.
        let holiday_index = self.get_yom_tov_index();
        if self.is_mukaf_choma {
            holiday_index == Some(JewishHoliday::ShushanPurim)
        } else {
            holiday_index == Some(JewishHoliday::Purim)
        }
    }

    fn get_day_of_omer(&self) -> Option<u8> {
        let month = self.get_jewish_month() as i32;
        let day = self.get_jewish_day_of_month();

        if month == JewishMonth::Nissan as i32 && day >= 16 {
            Some(day - 15)
        } else if month == JewishMonth::Iyar as i32 {
            Some(day + 15)
        } else if month == JewishMonth::Sivan as i32 && day < 6 {
            Some(day + 44)
        } else {
            None
        }
    }

    fn is_tisha_beav(&self) -> bool {
        self.get_yom_tov_index() == Some(JewishHoliday::TishahBav)
    }

    fn get_parshah(&self) -> Option<Parsha> {
        if self.get_day_of_week() != DayOfWeek::Shabbos {
            return None;
        }

        let parsha_list = self.get_parsha_list()?;

        let rosh_hashana_day_of_week =
            JewishCalendar::<N>::get_jewish_calendar_elapsed_days(self.get_jewish_year()) % 7;
        let day = rosh_hashana_day_of_week + self.get_days_since_start_of_jewish_year();
        parsha_list[(day / 7) as usize]
    }

    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf> {
        let date = icu_to_naive(&self.get_gregorian_date())?;
        let milliseconds_since_epoch = date.timestamp_millis();

        let daf_yomi_julian_start = get_julian_day(&_BAVLI_DAF_YOMI_START_DAY) as i64;
        let shekalim_julian_change = get_julian_day(&_BAVLI_SHEKALIM_CHANGE_DAY) as i64;

        if milliseconds_since_epoch < _BAVLI_DAF_YOMI_START_DAY.timestamp_millis() {
            return None;
        }

        let julian_day = get_julian_day(&date) as i64;
        let (cycle_no, daf_no) = if milliseconds_since_epoch >= _BAVLI_SHEKALIM_CHANGE_DAY.timestamp_millis() {
            let cycle_no = 8 + ((julian_day - shekalim_julian_change) / 2711);
            let daf_no = (julian_day - shekalim_julian_change) % 2711;
            (cycle_no, daf_no)
        } else {
            let cycle_no = 1 + ((julian_day - daf_yomi_julian_start) / 2702);
            let daf_no = (julian_day - daf_yomi_julian_start) % 2702;
            (cycle_no, daf_no)
        };
        let mut blatt_per_bavli_tractate: [i64; 40] = [
            64, 157, 105, 121, 22, 88, 56, 40, 35, 31, 32, 29, 27, 122, 112, 91, 66, 49, 90, 82, 119, 119, 176, 113,
            24, 49, 76, 14, 120, 110, 142, 61, 34, 34, 28, 22, 4, 9, 5, 73,
        ];

        if cycle_no <= 7 {
            blatt_per_bavli_tractate[4] = 13;
        }

        let mut total = 0;
        let mut masechta = -1;
        let mut blatt = 0;

        for (i, &blatt_count) in blatt_per_bavli_tractate.iter().enumerate() {
            masechta = i as i8;
            total = total + blatt_count - 1;
            if daf_no < total {
                blatt = 1 + blatt_count - (total - daf_no);

                if masechta == 36 {
                    blatt += 21;
                } else if masechta == 37 {
                    blatt += 24;
                } else if masechta == 38 {
                    blatt += 32;
                }
                break;
            }
        }
        if masechta < 0 {
            None
        } else {
            let tractate: BavliTractate = (masechta as u8).try_into().ok()?;

            Some(BavliDaf {
                tractate,
                daf_index: blatt,
            })
        }
    }

    fn get_daf_yomi_yerushalmi(&self) -> Option<YerushalmiDaf> {
        let requested_date = icu_to_naive(&self.get_gregorian_date())?;

        let milliseconds_since_epoch = requested_date.timestamp_millis();
        let mut tractate: i64 = 0;
        if self.get_yom_tov_index() == Some(JewishHoliday::YomKippur)
            || self.get_yom_tov_index() == Some(JewishHoliday::TishahBav)
            || milliseconds_since_epoch < _YERUSHALMI_DAF_YOMI_START_DAY.timestamp_millis()
        {
            return None;
        }

        let mut prev_cycle = _YERUSHALMI_DAF_YOMI_START_DAY;
        let mut next_cycle = _YERUSHALMI_DAF_YOMI_START_DAY;

        next_cycle = next_cycle.checked_add_days(Days::new(_YERUSHALMI_LENGTH - 1))?;
        let special_days_in_cycle = self.get_num_of_special_days(prev_cycle, next_cycle)?;
        next_cycle = next_cycle.checked_add_days(Days::new(special_days_in_cycle))?;

        while requested_date > next_cycle {
            prev_cycle = next_cycle;
            prev_cycle = prev_cycle.checked_add_days(Days::new(1))?;

            next_cycle = next_cycle.checked_add_days(Days::new(_YERUSHALMI_LENGTH))?;
            let special_days_in_cycle = self.get_num_of_special_days(prev_cycle, next_cycle)?;
            next_cycle = next_cycle.checked_add_days(Days::new(special_days_in_cycle))?;
        }

        let daf_num = self.get_diff_between_days(prev_cycle, requested_date);

        let special_days = self.get_num_of_special_days(prev_cycle, requested_date)?;

        let total = if special_days > daf_num {
            return None;
        } else {
            daf_num - special_days
        };
        let mut total = total as i64;

        for blatt_count in BLATT_PER_YERUSHALMI_TRACTATE.iter() {
            if total < *blatt_count as i64 {
                let tractate: YerushalmiTractate = tractate.try_into().ok()?;

                return Some(YerushalmiDaf {
                    tractate,
                    daf_index: (total + 1) as i64,
                });
            }
            total -= *blatt_count as i64;
            tractate += 1;
        }

        None
    }

    fn is_birkas_hachamah(&self) -> bool {
        let elapsed_days = JewishCalendar::<N>::get_jewish_calendar_elapsed_days(self.get_jewish_year());
        let elapsed_days = elapsed_days + self.get_days_since_start_of_jewish_year();
        let cycle_length = 10227i32;
        (elapsed_days % cycle_length) == 172
    }

    // Extended holiday checks
    fn is_erev_rosh_chodesh(&self) -> bool {
        // Erev Rosh Hashana is not Erev Rosh Chodesh
        self.get_jewish_day_of_month() == 29 && self.get_jewish_month() != JewishMonth::Elul
    }

    fn is_yom_kippur_katan(&self) -> bool {
        let day_of_week = self.get_day_of_week();
        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();

        // Not observed in Elul, Tishrei, Kislev, or Nissan
        if matches!(
            month,
            JewishMonth::Elul | JewishMonth::Tishrei | JewishMonth::Kislev | JewishMonth::Nissan
        ) {
            return false;
        }

        // On 29th if not Friday or Shabbos
        if day == 29 && day_of_week != DayOfWeek::Friday && day_of_week != DayOfWeek::Shabbos {
            return true;
        }

        // On 27th or 28th if Thursday (moved back from Friday/Shabbos)
        (day == 27 || day == 28) && day_of_week == DayOfWeek::Thursday
    }

    fn is_be_hab(&self) -> bool {
        let day_of_week = self.get_day_of_week();
        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();

        // BeHaB is only in Cheshvan and Iyar
        if month == JewishMonth::Cheshvan || month == JewishMonth::Iyar {
            // Monday between 5-17 or Thursday between 8-13
            return (day_of_week == DayOfWeek::Monday && day > 4 && day < 18)
                || (day_of_week == DayOfWeek::Thursday && day > 7 && day < 14);
        }
        false
    }

    fn is_machar_chodesh(&self) -> bool {
        // Shabbos and tomorrow is Rosh Chodesh (30th or 29th of month)
        self.get_day_of_week() == DayOfWeek::Shabbos
            && (self.get_jewish_day_of_month() == 30 || self.get_jewish_day_of_month() == 29)
    }

    fn is_shabbos_mevorchim(&self) -> bool {
        // Shabbos between 23rd and 29th (but not in Elul)
        self.get_day_of_week() == DayOfWeek::Shabbos
            && self.get_jewish_day_of_month() >= 23
            && self.get_jewish_day_of_month() <= 29
            && self.get_jewish_month() != JewishMonth::Elul
    }

    // Parsha methods
    fn get_upcoming_parshah(&self) -> Option<Parsha> {
        // Calculate days to next Shabbos
        let day_of_week = self.get_day_of_week();
        let days_to_shabbos = if day_of_week == DayOfWeek::Shabbos {
            7 // If today is Shabbos, get next Shabbos
        } else {
            (DayOfWeek::Shabbos as u8 - day_of_week as u8 + 7) % 7
        };

        // Create a new calendar for the upcoming Shabbos
        let mut upcoming_year = self.get_jewish_year();
        let mut upcoming_month = self.get_jewish_month();
        let mut upcoming_day = self.get_jewish_day_of_month() + days_to_shabbos;

        // Handle month/year overflow
        let days_in_month = JewishCalendar::<N>::get_days_in_jewish_month_static(upcoming_month, upcoming_year);
        while upcoming_day > days_in_month {
            upcoming_day -= days_in_month;
            upcoming_month = match upcoming_month {
                JewishMonth::Elul => {
                    upcoming_year += 1;
                    JewishMonth::Tishrei
                }
                JewishMonth::Adar if !JewishCalendar::<N>::is_jewish_leap_year_static(upcoming_year) => {
                    JewishMonth::Nissan
                }
                JewishMonth::AdarII => JewishMonth::Nissan,
                _ => {
                    let month_num: u8 = upcoming_month.into();
                    (month_num + 1).try_into().ok()?
                }
            };
            let days_in_month = JewishCalendar::<N>::get_days_in_jewish_month_static(upcoming_month, upcoming_year);
            if upcoming_day > days_in_month {
                continue;
            }
        }

        // Get parshah for that date
        let upcoming_calendar = self.copy_with_hebrew_ymd(upcoming_year, upcoming_month, upcoming_day)?;

        let mut parshah = upcoming_calendar.get_parshah();

        // Keep advancing if None (Yom Tov)
        let mut temp_year = upcoming_year;
        let mut temp_month = upcoming_month;
        let mut temp_day = upcoming_day;

        while parshah.is_none() {
            temp_day += 7;
            let days_in_month = JewishCalendar::<N>::get_days_in_jewish_month_static(temp_month, temp_year);
            if temp_day > days_in_month {
                temp_day -= days_in_month;
                temp_month = match temp_month {
                    JewishMonth::Elul => {
                        temp_year += 1;
                        JewishMonth::Tishrei
                    }
                    JewishMonth::Adar if !JewishCalendar::<N>::is_jewish_leap_year_static(temp_year) => {
                        JewishMonth::Nissan
                    }
                    JewishMonth::AdarII => JewishMonth::Nissan,
                    _ => {
                        let month_num: u8 = temp_month.into();
                        (month_num + 1).try_into().ok()?
                    }
                };
            }
            let temp_calendar = self.copy_with_hebrew_ymd(temp_year, temp_month, temp_day)?;

            parshah = temp_calendar.get_parshah();
        }

        parshah
    }

    fn get_special_shabbos(&self) -> Option<Parsha> {
        if self.get_day_of_week() != DayOfWeek::Shabbos {
            return None;
        }

        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();
        let is_leap = self.is_jewish_leap_year();

        // Shkalim
        if ((month == JewishMonth::Shevat && !is_leap) || (month == JewishMonth::Adar && is_leap))
            && (day == 25 || day == 27 || day == 29)
        {
            return Some(Parsha::Shekalim);
        }

        if (month == JewishMonth::Adar && !is_leap) || month == JewishMonth::AdarII {
            if day == 1 {
                return Some(Parsha::Shekalim);
            }
            // Zachor
            if day == 8 || day == 9 || day == 11 || day == 13 {
                return Some(Parsha::Zachor);
            }
            // Para
            if day == 18 || day == 20 || day == 22 || day == 23 {
                return Some(Parsha::Parah);
            }
            // Hachodesh
            if day == 25 || day == 27 || day == 29 {
                return Some(Parsha::Hachodesh);
            }
        }

        if month == JewishMonth::Nissan {
            if day == 1 {
                return Some(Parsha::Hachodesh);
            }
            // Hagadol
            if (8..=14).contains(&day) {
                return Some(Parsha::Hagadol);
            }
        }

        if month == JewishMonth::Av {
            // Chazon
            if (4..=9).contains(&day) {
                return Some(Parsha::Chazon);
            }
            // Nachamu
            if (10..=16).contains(&day) {
                return Some(Parsha::Nachamu);
            }
        }

        if month == JewishMonth::Tishrei {
            // Shuva
            if (3..=8).contains(&day) {
                return Some(Parsha::Shuva);
            }
        }

        // Shira
        if self.get_parshah() == Some(Parsha::Beshalach) {
            return Some(Parsha::Shira);
        }

        None
    }

    // Molad and Kiddush Levana
    fn get_molad_as_date(&self) -> Option<DateTime<Utc>> {
        use chrono::TimeZone;

        let molad = self.get_molad_as_calendar()?;
        let molad_data = self.get_molad()?;

        // Get the Gregorian date components from molad JewishCalendar
        let year = molad.get_gregorian_year();
        let month = (molad.get_gregorian_month() + 1) as u32; // Convert from 0-based to 1-based
        let day = molad.get_gregorian_day_of_month() as u32;

        let molad_seconds = molad_data.chalakim as f64 * 10.0 / 3.0;
        let seconds = molad_seconds as u32;
        let millis = ((molad_seconds - seconds as f64) * 1000.0) as u32;

        let naive_datetime = chrono::NaiveDate::from_ymd_opt(year, month, day)?.and_hms_milli_opt(
            molad_data.hours as u32,
            molad_data.minutes as u32,
            seconds,
            millis,
        )?;

        // Molad is in Jerusalem standard time (GMT+2)
        let jerusalem_offset = chrono::FixedOffset::east_opt(2 * 3600)?;
        let datetime_jerusalem = jerusalem_offset.from_local_datetime(&naive_datetime).single()?;

        // Subtract local mean time offset (20.94 minutes = 1256.4 seconds)
        // Longitude of Har Habayis: 35.2354°
        // 35.2354° away from 35° (GMT+2 +  20 minutes) = 0.2354° = ~0.94 minutes
        // Total: 20 minutes 56.496 seconds ≈ 1256.496 seconds
        Some(datetime_jerusalem.to_utc() - chrono::Duration::milliseconds(1256496))
    }

    fn get_tchilaszman_kidush_levana_3_days(&self) -> Option<DateTime<Utc>> {
        let molad = self.get_molad_as_date()?;
        Some(molad + chrono::Duration::hours(72))
    }

    fn get_tchilaszman_kidush_levana_7_days(&self) -> Option<DateTime<Utc>> {
        let molad = self.get_molad_as_date()?;
        Some(molad + chrono::Duration::hours(168))
    }

    fn get_sof_zman_kidush_levana_between_moldos(&self) -> Option<DateTime<Utc>> {
        let molad = self.get_molad_as_date()?;
        // Half of 29 days, 12 hours, 793 chalakim (44 minutes, 3.3 seconds)
        // = 14 days, 18 hours, 22 minutes, 1.666 seconds
        Some(
            molad
                + chrono::Duration::hours(24 * 14 + 18)
                + chrono::Duration::minutes(22)
                + chrono::Duration::seconds(1)
                + chrono::Duration::milliseconds(666),
        )
    }

    fn get_sof_zman_kidush_levana_15_days(&self) -> Option<DateTime<Utc>> {
        let molad = self.get_molad_as_date()?;
        Some(molad + chrono::Duration::hours(24 * 15))
    }

    // Tekufos and Seasonal Prayers
    fn get_tekufas_tishrei_elapsed_days(&self) -> i64 {
        // Days since Rosh Hashana year 1, plus 1/2 day (0.5)
        let days = JewishCalendar::<N>::get_jewish_calendar_elapsed_days(self.get_jewish_year()) as f64
            + (self.get_days_since_start_of_jewish_year() - 1) as f64
            + 0.5;

        // Days of completed solar years
        let solar = (self.get_jewish_year() - 1) as f64 * 365.25;

        (days - solar).floor() as i64
    }

    fn is_vesein_tal_umatar_start_date(&self) -> bool {
        if self.in_israel {
            // 7th of Cheshvan (can't fall on Shabbos)
            self.get_jewish_month() == JewishMonth::Cheshvan && self.get_jewish_day_of_month() == 7
        } else {
            // Not recited on Friday night
            if self.get_day_of_week() == DayOfWeek::Shabbos {
                return false;
            }
            // On Sunday, could be start date or delayed from Shabbos
            if self.get_day_of_week() == DayOfWeek::Sunday {
                let elapsed = self.get_tekufas_tishrei_elapsed_days();
                elapsed == 48 || elapsed == 47
            } else {
                self.get_tekufas_tishrei_elapsed_days() == 47
            }
        }
    }

    fn is_vesein_tal_umatar_starting_tonight(&self) -> bool {
        if self.in_israel {
            // 6th of Cheshvan
            self.get_jewish_month() == JewishMonth::Cheshvan && self.get_jewish_day_of_month() == 6
        } else {
            // Not recited on Friday night
            if self.get_day_of_week() == DayOfWeek::Friday {
                return false;
            }
            // On Motzai Shabbos, could be start date or delayed from Friday night
            if self.get_day_of_week() == DayOfWeek::Shabbos {
                let elapsed = self.get_tekufas_tishrei_elapsed_days();
                elapsed == 47 || elapsed == 46
            } else {
                self.get_tekufas_tishrei_elapsed_days() == 46
            }
        }
    }

    fn is_vesein_tal_umatar_recited(&self) -> bool {
        let month = self.get_jewish_month();
        let day = self.get_jewish_day_of_month();

        // Until 15 Nissan
        if month == JewishMonth::Nissan && day < 15 {
            return true;
        }
        // Not before Cheshvan
        if (month as i64) < (JewishMonth::Cheshvan as i64) {
            return false;
        }

        if self.in_israel {
            // In Israel: from 7 Cheshvan
            month != JewishMonth::Cheshvan || day >= 7
        } else {
            // Outside Israel: from tekufas tishrei + 60 days
            self.get_tekufas_tishrei_elapsed_days() >= 47
        }
    }

    fn is_vesein_beracha_recited(&self) -> bool {
        !self.is_vesein_tal_umatar_recited()
    }

    fn is_mashiv_haruach_start_date(&self) -> bool {
        self.get_jewish_month() == JewishMonth::Tishrei && self.get_jewish_day_of_month() == 22
    }

    fn is_mashiv_haruach_end_date(&self) -> bool {
        self.get_jewish_month() == JewishMonth::Nissan && self.get_jewish_day_of_month() == 15
    }

    fn is_mashiv_haruach_recited(&self) -> Option<bool> {
        let now_hebrew_date = self.hebrew_date;
        let start_hebrew_date = self
            .copy_with_hebrew_ymd(now_hebrew_date.era_year().year, JewishMonth::Tishrei, 22)?
            .hebrew_date;
        let end_hebrew_date = self
            .copy_with_hebrew_ymd(now_hebrew_date.era_year().year, JewishMonth::Nissan, 15)?
            .hebrew_date;
        Some(now_hebrew_date > start_hebrew_date && now_hebrew_date < end_hebrew_date)
    }

    fn is_morid_hatal_recited(&self) -> Option<bool> {
        Some(
            !self.is_mashiv_haruach_recited()?
                || self.is_mashiv_haruach_start_date()
                || self.is_mashiv_haruach_end_date(),
        )
    }
}

const BLATT_PER_YERUSHALMI_TRACTATE: [u64; 39] = [
    68, 37, 34, 44, 31, 59, 26, 33, 28, 20, 13, 92, 65, 71, 22, 22, 42, 26, 26, 33, 34, 22, 19, 85, 72, 47, 40, 47, 54,
    48, 44, 37, 34, 44, 9, 57, 37, 19, 13,
];

fn icu_to_naive(date: &Date<Gregorian>) -> Option<DateTime<Utc>> {
    let year = date.year().extended_year();
    let month = date.month().ordinal as u32;
    let day = date.day_of_month().0 as u32;
    let naive_date = NaiveDate::from_ymd_opt(year, month, day)?;
    let datetime = naive_date.and_hms_opt(0, 0, 0)?.and_utc();
    Some(datetime)
}

#[cfg(feature = "defmt")]
impl<N: AstronomicalCalculatorTrait> defmt::Format for JewishCalendar<N> {
    fn format(&self, f: defmt::Formatter) {
        use icu_calendar::types::{CyclicYear, YearInfo};

        let month = self.hebrew_date.month().ordinal;
        let day = self.hebrew_date.day_of_month().0;
        match self.hebrew_date.year() {
            YearInfo::Era(era_year) => {
                defmt::write!(
                    f,
                    "JewishCalendar(year={}, month={}, day={}, era={}, in_israel={}, is_mukaf_choma={}, use_modern_holidays={}, calculator={:?})",
                    era_year.year,
                    month,
                    day,
                    era_year.era.as_str(),
                    self.in_israel,
                    self.is_mukaf_choma,
                    self.use_modern_holidays,
                    self.calculator
                )
            }
            YearInfo::Cyclic(CyclicYear { year, related_iso, .. }) => {
                defmt::write!(
                    f,
                    "JewishCalendar(year={}, month={}, day={}, ISO year={}, in_israel={}, is_mukaf_choma={}, use_modern_holidays={}, calculator={:?})",
                    year,
                    month,
                    day,
                    related_iso,
                    self.in_israel,
                    self.is_mukaf_choma,
                    self.use_modern_holidays,
                    self.calculator
                )
            }
            _ => {
                defmt::write!(
                    f,
                    "JewishCalendar(year=???, month={}, day={}, in_israel={}, is_mukaf_choma={}, use_modern_holidays={}, calculator={:?})",
                    month,
                    day,
                    self.in_israel,
                    self.is_mukaf_choma,
                    self.use_modern_holidays,
                    self.calculator
                )
            }
        }
    }
}
