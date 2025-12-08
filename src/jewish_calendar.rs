use core::fmt::Debug;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Days;
use chrono::NaiveDate;
use chrono::Utc;
use icu_calendar::Date;
use icu_calendar::Gregorian;

use crate::astronomical_calculator::AstronomicalCalculatorTrait;
use crate::astronomical_calculator::get_julian_day;
use crate::constants::*;
use crate::daf::*;
use crate::defmt::DefmtFormatTrait;
use crate::jewish_date::InternalJewishDateTrait;
use crate::jewish_date::JewishDate;
use crate::jewish_date::JewishDateTrait;
use crate::parshas::*;
#[cfg(feature = "no_std")]
use core_maths::CoreFloat;

pub(crate) trait InternalJewishCalendarTrait {
    fn get_is_use_modern_holidays(&self) -> bool;
    fn get_in_israel(&self) -> bool;
    fn get_jewish_date(&self) -> &impl JewishDateTrait;
    fn get_calculator(&self) -> &impl AstronomicalCalculatorTrait;
    fn get_is_mukaf_choma(&self) -> bool;
}
#[allow(private_bounds)]
pub trait JewishCalendarTrait: InternalJewishCalendarTrait + DefmtFormatTrait {
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
    fn get_molad_as_date(&self) -> Option<DateTime<Utc>>;
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
pub struct JewishCalendar<N: AstronomicalCalculatorTrait> {
    pub jewish_date: JewishDate,
    pub in_israel: bool,
    pub is_mukaf_choma: bool,
    pub use_modern_holidays: bool,
    calculator: N,
}

impl<N: AstronomicalCalculatorTrait> JewishCalendar<N> {
    pub fn from_gregorian_date(
        year: i32,
        month: u8,
        day: u8,
        in_israel: bool,
        is_mukaf_choma: bool,
        use_modern_holidays: bool,
        calculator: N,
    ) -> Option<Self> {
        let jewish_date = JewishDate::from_gregorian_date(year, month, day)?;
        Some(Self {
            jewish_date,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            calculator,
        })
    }

    pub fn from_jewish_date(
        year: i32,
        month: JewishMonth,
        day: u8,
        in_israel: bool,
        is_mukaf_choma: bool,
        use_modern_holidays: bool,
        calculator: N,
    ) -> Option<Self> {
        let jewish_date = JewishDate::from_jewish_date(year, month, day)?;
        Some(Self {
            jewish_date,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            calculator,
        })
    }

    fn get_num_of_special_days(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Option<u64> {
        let start_year =
            JewishDate::from_gregorian_date(start.year(), start.month() as u8, start.day() as u8)?.get_jewish_year();
        let end_year =
            JewishDate::from_gregorian_date(end.year(), end.month() as u8, end.day() as u8)?.get_jewish_year();
        println!("RUST: start_year: {}", start_year);
        println!("RUST: end_year: {}", end_year);
        let mut special_days = 0u64;
        for i in start_year..=end_year {
            // Create new calendar instances for each year
            let yom_kippur_date = JewishDate::from_jewish_date(i, JewishMonth::Tishrei, 10)?;
            let tisha_beav_date = JewishDate::from_jewish_date(i, JewishMonth::Av, 9)?;

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
            (JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year()) + 1) % 7;
        let rosh_hashana_day_of_week = if rosh_hashana_day_of_week == 0 {
            7
        } else {
            rosh_hashana_day_of_week
        };

        if self.jewish_date.is_jewish_leap_year() {
            match rosh_hashana_day_of_week {
                2 => {
                    if self.jewish_date.is_kislev_short() {
                        if self.in_israel {
                            Some(PARSHA_LIST_14)
                        } else {
                            Some(PARSHA_LIST_6)
                        }
                    } else if self.jewish_date.is_cheshvan_long() {
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
                    if self.jewish_date.is_kislev_short() {
                        Some(PARSHA_LIST_8)
                    } else if self.jewish_date.is_cheshvan_long() {
                        Some(PARSHA_LIST_9)
                    } else {
                        None
                    }
                }
                7 => {
                    if self.jewish_date.is_kislev_short() {
                        Some(PARSHA_LIST_10)
                    } else if self.jewish_date.is_cheshvan_long() {
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
                    if self.jewish_date.is_kislev_short() {
                        Some(PARSHA_LIST_0)
                    } else if self.jewish_date.is_cheshvan_long() {
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
                    if self.jewish_date.is_cheshvan_long() {
                        Some(PARSHA_LIST_3)
                    } else if !self.jewish_date.is_kislev_short() {
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
                    if self.jewish_date.is_kislev_short() {
                        Some(PARSHA_LIST_4)
                    } else if self.jewish_date.is_cheshvan_long() {
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

impl<N: AstronomicalCalculatorTrait> InternalJewishCalendarTrait for JewishCalendar<N> {
    fn get_is_use_modern_holidays(&self) -> bool {
        self.use_modern_holidays
    }
    fn get_in_israel(&self) -> bool {
        self.in_israel
    }
    fn get_jewish_date(&self) -> &impl JewishDateTrait {
        &self.jewish_date
    }

    fn get_calculator(&self) -> &impl AstronomicalCalculatorTrait {
        &self.calculator
    }

    fn get_is_mukaf_choma(&self) -> bool {
        self.is_mukaf_choma
    }
}
impl<N: AstronomicalCalculatorTrait> JewishCalendarTrait for JewishCalendar<N> {
    fn get_yom_tov_index(&self) -> Option<JewishHoliday> {
        let day = self.jewish_date.get_jewish_day_of_month();
        let day_of_week = self.jewish_date.get_day_of_week();
        let month = self.jewish_date.get_jewish_month();

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
                if day == 1 || day == 2 || (day == 3 && self.jewish_date.is_kislev_short()) {
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
                if !self.jewish_date.is_jewish_leap_year() {
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
        self.jewish_date.get_day_of_week() == DayOfWeek::Shabbos || self.is_yom_tov_assur_bemelacha()
    }

    fn has_candle_lighting(&self) -> bool {
        self.is_tomorrow_shabbos_or_yom_tov()
    }

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Friday
            || self.is_erev_yom_tov()
            || self.is_erev_yom_tov_sheni()
    }

    fn is_erev_yom_tov_sheni(&self) -> bool {
        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();
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
        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();
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
            || (holiday_index == Some(JewishHoliday::CholHamoedPesach)
                && self.jewish_date.get_jewish_day_of_month() == 20)
    }

    fn is_rosh_chodesh(&self) -> bool {
        let day = self.jewish_date.get_jewish_day_of_month();
        let month = self.jewish_date.get_jewish_month() as i32;
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
        let day = self.jewish_date.get_jewish_day_of_month();
        let day_of_week = self.jewish_date.get_day_of_week() as i32;
        let month = self.jewish_date.get_jewish_month() as i32;

        month == JewishMonth::Nissan as i32 && ((day == 14 && day_of_week != 7) || (day == 12 && day_of_week == 5))
    }

    fn get_day_of_chanukah(&self) -> Option<u8> {
        if !self.is_chanukah() {
            return None;
        }

        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

        if month == JewishMonth::Kislev as i32 {
            Some(day - 24)
        } else if self.jewish_date.is_kislev_short() {
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
        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

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
        if self.jewish_date.get_day_of_week() != DayOfWeek::Shabbos {
            return None;
        }

        let parsha_list = self.get_parsha_list()?;

        let rosh_hashana_day_of_week =
            JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year()) % 7;
        let day = rosh_hashana_day_of_week + self.jewish_date.get_days_since_start_of_jewish_year();
        parsha_list[(day / 7) as usize]
    }

    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf> {
        let date = icu_to_naive(&self.jewish_date.get_gregorian_date())?;
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
        let requested_date = icu_to_naive(&self.jewish_date.get_gregorian_date())?;

        println!("=== RUST get_daf_yomi_yerushalmi DEBUG ===");
        println!("RUST: Requested date: {:?}", requested_date);

        let milliseconds_since_epoch = requested_date.timestamp_millis();
        let mut tractate: i64 = 0;
        if self.get_yom_tov_index() == Some(JewishHoliday::YomKippur)
            || self.get_yom_tov_index() == Some(JewishHoliday::TishahBav)
            || milliseconds_since_epoch < _YERUSHALMI_DAF_YOMI_START_DAY.timestamp_millis()
        {
            return None;
        }

        fn calc_next_cycle<N: AstronomicalCalculatorTrait>(
            calendar: &JewishCalendar<N>,
            start: DateTime<Utc>,
        ) -> Option<DateTime<Utc>> {
            let mut next_cycle = start.checked_add_days(Days::new(_YERUSHALMI_LENGTH - 1))?;
            let special_days_in_cycle = calendar.get_num_of_special_days(start, next_cycle)?;
            next_cycle = next_cycle.checked_add_days(Days::new(special_days_in_cycle))?;
            Some(next_cycle)
        }

        let mut prev_cycle = _YERUSHALMI_DAF_YOMI_START_DAY;
        let mut next_cycle = calc_next_cycle(self, prev_cycle)?;

        fn get_next_cycle<N: AstronomicalCalculatorTrait>(
            calendar: &JewishCalendar<N>,
            next_cycle: DateTime<Utc>,
        ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
            let prev_cycle = next_cycle.checked_add_days(Days::new(1))?;
            let next_cycle = calc_next_cycle(calendar, prev_cycle)?;
            Some((prev_cycle, next_cycle))
        }

        println!("RUST: DAF_YOMI_START_DAY: {:?}", _YERUSHALMI_DAF_YOMI_START_DAY);
        println!("RUST: YERUSHALMI_LENGTH: {}", _YERUSHALMI_LENGTH);

        while requested_date > next_cycle {
            (prev_cycle, next_cycle) = get_next_cycle(self, next_cycle)?;
        }

        println!(
            "RUST: Found cycle: prev_cycle={:?}, next_cycle={:?}",
            prev_cycle, next_cycle
        );

        let daf_num = self.get_diff_between_days(prev_cycle, requested_date);
        println!("RUST: Days from cycle start (daf_num): {}", daf_num);

        let special_days = self.get_num_of_special_days(prev_cycle, requested_date)?;
        println!("RUST: Special days to subtract: {}", special_days);

        let total = if special_days > daf_num {
            println!(
                "RUST: Special days ({}) > daf_num ({}) - returning None",
                special_days, daf_num
            );
            return None;
        } else {
            daf_num - special_days
        };
        let mut total = total as i64;
        println!("RUST: Total daf number in cycle: {}", total);

        let original_total = total;
        for (idx, blatt_count) in BLATT_PER_YERUSHALMI_TRACTATE.iter().enumerate() {
            println!(
                "RUST: Checking tractate {} with {} blatt, remaining total={}",
                idx, blatt_count, total
            );
            if total < *blatt_count as i64 {
                let tractate: YerushalmiTractate = tractate.try_into().ok()?;
                println!(
                    "RUST: Found daf: tractate={:?} ({}), daf={}",
                    tractate,
                    tractate as i64,
                    total + 1
                );
                println!("=== RUST END DEBUG ===\n");
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
        let elapsed_days = JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year());
        let elapsed_days = elapsed_days + self.jewish_date.get_days_since_start_of_jewish_year();
        let cycle_length = 10227i32;
        (elapsed_days % cycle_length) == 172
    }

    // Extended holiday checks
    fn is_erev_rosh_chodesh(&self) -> bool {
        // Erev Rosh Hashana is not Erev Rosh Chodesh
        self.jewish_date.get_jewish_day_of_month() == 29 && self.jewish_date.get_jewish_month() != JewishMonth::Elul
    }

    fn is_yom_kippur_katan(&self) -> bool {
        let day_of_week = self.jewish_date.get_day_of_week();
        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();

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
        let day_of_week = self.jewish_date.get_day_of_week();
        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();

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
        self.jewish_date.get_day_of_week() == DayOfWeek::Shabbos
            && (self.jewish_date.get_jewish_day_of_month() == 30 || self.jewish_date.get_jewish_day_of_month() == 29)
    }

    fn is_shabbos_mevorchim(&self) -> bool {
        // Shabbos between 23rd and 29th (but not in Elul)
        self.jewish_date.get_day_of_week() == DayOfWeek::Shabbos
            && self.jewish_date.get_jewish_day_of_month() >= 23
            && self.jewish_date.get_jewish_day_of_month() <= 29
            && self.jewish_date.get_jewish_month() != JewishMonth::Elul
    }

    // Parsha methods
    fn get_upcoming_parshah(&self) -> Option<Parsha> {
        // Calculate days to next Shabbos
        let day_of_week = self.jewish_date.get_day_of_week();
        let days_to_shabbos = if day_of_week == DayOfWeek::Shabbos {
            7 // If today is Shabbos, get next Shabbos
        } else {
            (DayOfWeek::Shabbos as u8 - day_of_week as u8 + 7) % 7
        };

        // Create a new calendar for the upcoming Shabbos
        let mut upcoming_year = self.jewish_date.get_jewish_year();
        let mut upcoming_month = self.jewish_date.get_jewish_month();
        let mut upcoming_day = self.jewish_date.get_jewish_day_of_month() + days_to_shabbos;

        // Handle month/year overflow
        let days_in_month = JewishDate::get_days_in_jewish_month_static(upcoming_month, upcoming_year);
        while upcoming_day > days_in_month {
            upcoming_day -= days_in_month;
            upcoming_month = match upcoming_month {
                JewishMonth::Elul => {
                    upcoming_year += 1;
                    JewishMonth::Tishrei
                }
                JewishMonth::Adar if !JewishDate::is_jewish_leap_year_static(upcoming_year) => JewishMonth::Nissan,
                JewishMonth::AdarII => JewishMonth::Nissan,
                _ => {
                    let month_num: u8 = upcoming_month.into();
                    (month_num + 1).try_into().ok()?
                }
            };
            let days_in_month = JewishDate::get_days_in_jewish_month_static(upcoming_month, upcoming_year);
            if upcoming_day > days_in_month {
                continue;
            }
        }

        // Get parshah for that date
        let upcoming_calendar = JewishCalendar::from_jewish_date(
            upcoming_year,
            upcoming_month,
            upcoming_day,
            self.in_israel,
            self.is_mukaf_choma,
            self.use_modern_holidays,
            self.calculator.clone(),
        )?;

        let mut parshah = upcoming_calendar.get_parshah();

        // Keep advancing if None (Yom Tov)
        let mut temp_year = upcoming_year;
        let mut temp_month = upcoming_month;
        let mut temp_day = upcoming_day;

        while parshah.is_none() {
            temp_day += 7;
            let days_in_month = JewishDate::get_days_in_jewish_month_static(temp_month, temp_year);
            if temp_day > days_in_month {
                temp_day -= days_in_month;
                temp_month = match temp_month {
                    JewishMonth::Elul => {
                        temp_year += 1;
                        JewishMonth::Tishrei
                    }
                    JewishMonth::Adar if !JewishDate::is_jewish_leap_year_static(temp_year) => JewishMonth::Nissan,
                    JewishMonth::AdarII => JewishMonth::Nissan,
                    _ => {
                        let month_num: u8 = temp_month.into();
                        (month_num + 1).try_into().ok()?
                    }
                };
            }

            let temp_calendar = JewishCalendar::from_jewish_date(
                temp_year,
                temp_month,
                temp_day,
                self.in_israel,
                self.is_mukaf_choma,
                self.use_modern_holidays,
                self.calculator.clone(),
            )?;
            parshah = temp_calendar.get_parshah();
        }

        parshah
    }

    fn get_special_shabbos(&self) -> Option<Parsha> {
        if self.jewish_date.get_day_of_week() != DayOfWeek::Shabbos {
            return None;
        }

        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();
        let is_leap = self.jewish_date.is_jewish_leap_year();

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

        let molad = self.jewish_date.get_molad_as_date()?;
        let molad_data = self.jewish_date.get_molad()?;

        // Get the Gregorian date components from molad JewishDate
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
        let days = JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year()) as f64
            + (self.jewish_date.get_days_since_start_of_jewish_year() - 1) as f64
            + 0.5;

        // Days of completed solar years
        let solar = (self.jewish_date.get_jewish_year() - 1) as f64 * 365.25;

        (days - solar).floor() as i64
    }

    fn is_vesein_tal_umatar_start_date(&self) -> bool {
        if self.in_israel {
            // 7th of Cheshvan (can't fall on Shabbos)
            self.jewish_date.get_jewish_month() == JewishMonth::Cheshvan
                && self.jewish_date.get_jewish_day_of_month() == 7
        } else {
            // Not recited on Friday night
            if self.jewish_date.get_day_of_week() == DayOfWeek::Shabbos {
                return false;
            }
            // On Sunday, could be start date or delayed from Shabbos
            if self.jewish_date.get_day_of_week() == DayOfWeek::Sunday {
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
            self.jewish_date.get_jewish_month() == JewishMonth::Cheshvan
                && self.jewish_date.get_jewish_day_of_month() == 6
        } else {
            // Not recited on Friday night
            if self.jewish_date.get_day_of_week() == DayOfWeek::Friday {
                return false;
            }
            // On Motzai Shabbos, could be start date or delayed from Friday night
            if self.jewish_date.get_day_of_week() == DayOfWeek::Shabbos {
                let elapsed = self.get_tekufas_tishrei_elapsed_days();
                elapsed == 47 || elapsed == 46
            } else {
                self.get_tekufas_tishrei_elapsed_days() == 46
            }
        }
    }

    fn is_vesein_tal_umatar_recited(&self) -> bool {
        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();

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
        self.jewish_date.get_jewish_month() == JewishMonth::Tishrei && self.jewish_date.get_jewish_day_of_month() == 22
    }

    fn is_mashiv_haruach_end_date(&self) -> bool {
        self.jewish_date.get_jewish_month() == JewishMonth::Nissan && self.jewish_date.get_jewish_day_of_month() == 15
    }

    fn is_mashiv_haruach_recited(&self) -> Option<bool> {
        let now_hebrew_date = self.jewish_date.hebrew_date;
        let start_hebrew_date =
            JewishDate::from_jewish_date(now_hebrew_date.era_year().year, JewishMonth::Tishrei, 22)?.hebrew_date;
        let end_hebrew_date =
            JewishDate::from_jewish_date(now_hebrew_date.era_year().year, JewishMonth::Nissan, 15)?.hebrew_date;
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

impl<N: AstronomicalCalculatorTrait> DefmtFormatTrait for JewishCalendar<N> {}
