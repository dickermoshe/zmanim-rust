use crate::constants::*;
use crate::jewish_calendar::{JewishCalendar, JewishCalendarTrait};
use crate::prelude::AstronomicalCalculatorTrait;

use chrono::Datelike;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct TefilaRules {
    pub tachanun_recited_end_of_tishrei: bool,

    pub tachanun_recited_week_after_shavuos: bool,

    pub tachanun_recited_13_sivan_out_of_israel: bool,

    pub tachanun_recited_pesach_sheni: bool,

    pub tachanun_recited_15_iyar_out_of_israel: bool,

    pub tachanun_recited_mincha_erev_lag_baomer: bool,

    pub tachanun_recited_shivas_yemei_hamiluim: bool,

    pub tachanun_recited_week_of_hod: bool,

    pub tachanun_recited_week_of_purim: bool,

    pub tachanun_recited_fridays: bool,

    pub tachanun_recited_sundays: bool,

    pub tachanun_recited_mincha_all_year: bool,

    pub mizmor_lesoda_recited_erev_yom_kippur_and_pesach: bool,
}

impl TefilaRules {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tachanun_recited_end_of_tishrei: bool,
        tachanun_recited_week_after_shavuos: bool,
        tachanun_recited_13_sivan_out_of_israel: bool,
        tachanun_recited_pesach_sheni: bool,
        tachanun_recited_15_iyar_out_of_israel: bool,
        tachanun_recited_mincha_erev_lag_baomer: bool,
        tachanun_recited_shivas_yemei_hamiluim: bool,
        tachanun_recited_week_of_hod: bool,
        tachanun_recited_week_of_purim: bool,
        tachanun_recited_fridays: bool,
        tachanun_recited_sundays: bool,
        tachanun_recited_mincha_all_year: bool,
        mizmor_lesoda_recited_erev_yom_kippur_and_pesach: bool,
    ) -> Self {
        Self {
            tachanun_recited_end_of_tishrei,
            tachanun_recited_week_after_shavuos,
            tachanun_recited_13_sivan_out_of_israel,
            tachanun_recited_pesach_sheni,
            tachanun_recited_15_iyar_out_of_israel,
            tachanun_recited_mincha_erev_lag_baomer,
            tachanun_recited_shivas_yemei_hamiluim,
            tachanun_recited_week_of_hod,
            tachanun_recited_week_of_purim,
            tachanun_recited_fridays,
            tachanun_recited_sundays,
            tachanun_recited_mincha_all_year,
            mizmor_lesoda_recited_erev_yom_kippur_and_pesach,
        }
    }
}

pub trait TefilaRulesTrait<C: JewishCalendarTrait> {
    fn is_tachanun_recited_shacharis(&self, jewish_calendar: &C) -> bool;

    fn is_tachanun_recited_mincha(&self, jewish_calendar: &C) -> Option<bool>;

    fn is_hallel_recited(&self, jewish_calendar: &C) -> bool;

    fn is_hallel_shalem_recited(&self, jewish_calendar: &C) -> bool;

    fn is_al_hanissim_recited(&self, jewish_calendar: &C) -> bool;

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &C) -> bool;

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &C) -> bool;

    fn is_vesein_tal_umatar_start_date(&self, jewish_calendar: &C) -> bool;

    fn is_vesein_tal_umatar_starting_tonight(&self, jewish_calendar: &C) -> bool;

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &C) -> bool;

    fn is_vesein_beracha_recited(&self, jewish_calendar: &C) -> bool;

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &C) -> bool;

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &C) -> bool;

    fn is_mashiv_haruach_recited(&self, jewish_calendar: &C) -> Option<bool>;

    fn is_morid_hatal_recited(&self, jewish_calendar: &C) -> Option<bool>;
}

impl<N: AstronomicalCalculatorTrait> TefilaRulesTrait<JewishCalendar<N>> for TefilaRules {
    fn is_tachanun_recited_shacharis(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        let holiday_index = jewish_calendar.get_yom_tov_index();
        let day = jewish_calendar.get_jewish_day_of_month();
        let month = jewish_calendar.get_jewish_month();
        let day_of_week = jewish_calendar.get_day_of_week();
        #[allow(clippy::nonminimal_bool)]
        if day_of_week == DayOfWeek::Shabbos
            || (!self.tachanun_recited_sundays && day_of_week == DayOfWeek::Sunday)
            || (!self.tachanun_recited_fridays && day_of_week == DayOfWeek::Friday)
            || month == JewishMonth::Nissan
            || (month == JewishMonth::Tishrei
                && ((!self.tachanun_recited_end_of_tishrei && day > 8)
                    || (self.tachanun_recited_end_of_tishrei && day > 8 && day < 22)))
            || (month == JewishMonth::Sivan
                && (self.tachanun_recited_week_after_shavuos && day < 7
                    || !self.tachanun_recited_week_after_shavuos
                        && day
                            < if !jewish_calendar.in_israel && !self.tachanun_recited_13_sivan_out_of_israel {
                                14
                            } else {
                                13
                            }))
            || jewish_calendar.is_erev_yom_tov()
            || (jewish_calendar.is_yom_tov()
                && (!jewish_calendar.is_taanis()
                    || (!self.tachanun_recited_pesach_sheni && holiday_index == Some(JewishHoliday::PesachSheni))))
            || (!jewish_calendar.in_israel
                && !self.tachanun_recited_pesach_sheni
                && !self.tachanun_recited_15_iyar_out_of_israel
                && month == JewishMonth::Iyar
                && day == 15)
            || holiday_index == Some(JewishHoliday::TishahBav)
            || jewish_calendar.is_isru_chag()
            || jewish_calendar.is_rosh_chodesh()
            || (!self.tachanun_recited_shivas_yemei_hamiluim
                && ((!jewish_calendar.is_jewish_leap_year() && month == JewishMonth::Adar)
                    || (jewish_calendar.is_jewish_leap_year() && month == JewishMonth::AdarII))
                && day > 22)
            || (!self.tachanun_recited_week_of_purim
                && ((!jewish_calendar.is_jewish_leap_year() && month == JewishMonth::Adar)
                    || (jewish_calendar.is_jewish_leap_year() && month == JewishMonth::AdarII))
                && day > 10
                && day < 18)
            || (jewish_calendar.use_modern_holidays
                && (holiday_index == Some(JewishHoliday::YomHaatzmaut)
                    || holiday_index == Some(JewishHoliday::YomYerushalayim)))
            || (!self.tachanun_recited_week_of_hod && month == JewishMonth::Iyar && day > 13 && day < 21)
        {
            return false;
        }
        true
    }

    fn is_tachanun_recited_mincha(&self, jewish_calendar: &JewishCalendar<N>) -> Option<bool> {
        // Create tomorrow's date by adding 1 day
        let greg_date = jewish_calendar.get_gregorian_date();

        let year = greg_date.year().extended_year();
        // Convert to chrono NaiveDate and add 1 day
        let month = greg_date.month().ordinal as u32;
        let day = greg_date.day_of_month().0 as u32;

        let naive_date = chrono::NaiveDate::from_ymd_opt(year, month, day)?;

        let tomorrow_date = naive_date.checked_add_days(chrono::Days::new(1))?;

        let tomorrow = JewishCalendar::from_gregorian_date(
            tomorrow_date.year(),
            tomorrow_date.month() as u8,
            tomorrow_date.day() as u8,
            jewish_calendar.in_israel,
            jewish_calendar.is_mukaf_choma,
            jewish_calendar.use_modern_holidays,
            jewish_calendar.calculator.clone(),
        )?;

        let tomorrow_yom_tov = tomorrow.get_yom_tov_index();

        if !self.tachanun_recited_mincha_all_year
            || jewish_calendar.get_day_of_week() == DayOfWeek::Friday
            || !self.is_tachanun_recited_shacharis(jewish_calendar)
            || (!self.is_tachanun_recited_shacharis(&tomorrow)
                && tomorrow_yom_tov != Some(JewishHoliday::ErevRoshHashana)
                && tomorrow_yom_tov != Some(JewishHoliday::ErevYomKippur)
                && tomorrow_yom_tov != Some(JewishHoliday::PesachSheni))
            || (!self.tachanun_recited_mincha_erev_lag_baomer && tomorrow_yom_tov == Some(JewishHoliday::LagBomer))
        {
            return Some(false);
        }
        Some(true)
    }

    fn is_hallel_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        let day = jewish_calendar.get_jewish_day_of_month();
        let month = jewish_calendar.get_jewish_month();
        let holiday_index = jewish_calendar.get_yom_tov_index();
        let in_israel = jewish_calendar.in_israel;

        if jewish_calendar.is_rosh_chodesh() {
            return true;
        }

        if jewish_calendar.is_chanukah() {
            return true;
        }

        match month {
            JewishMonth::Nissan => {
                if day >= 15 && ((in_israel && day <= 21) || (!in_israel && day <= 22)) {
                    return true;
                }
            }
            JewishMonth::Iyar => {
                // modern holidays
                if jewish_calendar.use_modern_holidays
                    && (holiday_index == Some(JewishHoliday::YomHaatzmaut)
                        || holiday_index == Some(JewishHoliday::YomYerushalayim))
                {
                    return true;
                }
            }
            JewishMonth::Sivan => {
                if day == 6 || (!in_israel && day == 7) {
                    return true;
                }
            }
            JewishMonth::Tishrei => {
                if day >= 15 && (day <= 22 || (!in_israel && day <= 23)) {
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn is_hallel_shalem_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        let day = jewish_calendar.get_jewish_day_of_month();
        let month = jewish_calendar.get_jewish_month();
        let in_israel = jewish_calendar.in_israel;

        if self.is_hallel_recited(jewish_calendar) {
            return !((jewish_calendar.is_rosh_chodesh() && !jewish_calendar.is_chanukah())
                || (month == JewishMonth::Nissan && ((in_israel && day > 15) || (!in_israel && day > 16))));
        }
        false
    }

    fn is_al_hanissim_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_purim() || jewish_calendar.is_chanukah()
    }

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_pesach()
            || jewish_calendar.is_shavuos()
            || jewish_calendar.is_rosh_hashana()
            || jewish_calendar.is_yom_kippur()
            || jewish_calendar.is_succos()
            || jewish_calendar.is_shemini_atzeres()
            || jewish_calendar.is_simchas_torah()
            || jewish_calendar.is_rosh_chodesh()
    }

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        if jewish_calendar.is_assur_bemelacha() {
            return false;
        }

        let holiday_index = jewish_calendar.get_yom_tov_index();
        if !self.mizmor_lesoda_recited_erev_yom_kippur_and_pesach
            && (holiday_index == Some(JewishHoliday::ErevYomKippur)
                || holiday_index == Some(JewishHoliday::ErevPesach)
                || jewish_calendar.is_chol_hamoed_pesach())
        {
            return false;
        }
        true
    }

    fn is_vesein_tal_umatar_start_date(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_vesein_tal_umatar_start_date()
    }

    fn is_vesein_tal_umatar_starting_tonight(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_vesein_tal_umatar_starting_tonight()
    }

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_vesein_tal_umatar_recited()
    }

    fn is_vesein_beracha_recited(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_vesein_beracha_recited()
    }

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_mashiv_haruach_start_date()
    }

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &JewishCalendar<N>) -> bool {
        jewish_calendar.is_mashiv_haruach_end_date()
    }

    fn is_mashiv_haruach_recited(&self, jewish_calendar: &JewishCalendar<N>) -> Option<bool> {
        jewish_calendar.is_mashiv_haruach_recited()
    }

    fn is_morid_hatal_recited(&self, jewish_calendar: &JewishCalendar<N>) -> Option<bool> {
        jewish_calendar.is_morid_hatal_recited()
    }
}
