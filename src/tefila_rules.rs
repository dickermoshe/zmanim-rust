use crate::constants::*;
use crate::jewish_calendar::{JewishCalendar, JewishCalendarTrait};
use crate::jewish_date::JewishDateTrait;
use chrono::Datelike;

///

#[derive(Debug, Clone)]
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

pub trait TefilaRulesTrait {
    fn is_tachanun_recited_shacharis(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_tachanun_recited_mincha(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_hallel_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_hallel_shalem_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_al_hanissim_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_vesein_tal_umatar_start_date(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_vesein_tal_umatar_starting_tonight(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_vesein_beracha_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_mashiv_haruach_recited(&self, jewish_calendar: &JewishCalendar) -> bool;

    fn is_morid_hatal_recited(&self, jewish_calendar: &JewishCalendar) -> bool;
}

impl TefilaRulesTrait for TefilaRules {
    fn is_tachanun_recited_shacharis(&self, jewish_calendar: &JewishCalendar) -> bool {
        let holiday_index = jewish_calendar.get_yom_tov_index();
        let jewish_date = jewish_calendar.get_jewish_date();
        let day = jewish_date.get_jewish_day_of_month();
        let month = jewish_date.get_jewish_month();
        let day_of_week = jewish_date.get_day_of_week();

        if day_of_week == DayOfWeek::Saturday
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
                            < if !jewish_calendar.get_in_israel()
                                && !self.tachanun_recited_13_sivan_out_of_israel
                            {
                                14
                            } else {
                                13
                            }))
            || jewish_calendar.is_erev_yom_tov()
            || (jewish_calendar.is_yom_tov()
                && (!jewish_calendar.is_taanis()
                    || (!self.tachanun_recited_pesach_sheni
                        && holiday_index == Some(JewishHoliday::PesachSheni))))
            || (!jewish_calendar.get_in_israel()
                && !self.tachanun_recited_pesach_sheni
                && !self.tachanun_recited_15_iyar_out_of_israel
                && month == JewishMonth::Iyar
                && day == 15)
            || holiday_index == Some(JewishHoliday::TishaBeav)
            || jewish_calendar.is_isru_chag()
            || jewish_calendar.is_rosh_chodesh()
            || (!self.tachanun_recited_shivas_yemei_hamiluim
                && ((!jewish_date.is_jewish_leap_year() && month == JewishMonth::Adar)
                    || (jewish_date.is_jewish_leap_year() && month == JewishMonth::Adarii))
                && day > 22)
            || (!self.tachanun_recited_week_of_purim
                && ((!jewish_date.is_jewish_leap_year() && month == JewishMonth::Adar)
                    || (jewish_date.is_jewish_leap_year() && month == JewishMonth::Adarii))
                && day > 10
                && day < 18)
            || (jewish_calendar.get_is_use_modern_holidays()
                && (holiday_index == Some(JewishHoliday::YomHaatzmaut)
                    || holiday_index == Some(JewishHoliday::YomYerushalayim)))
            || (!self.tachanun_recited_week_of_hod
                && month == JewishMonth::Iyar
                && day > 13
                && day < 21)
        {
            return false;
        }
        true
    }

    fn is_tachanun_recited_mincha(&self, jewish_calendar: &JewishCalendar) -> bool {
        // Create tomorrow's date by adding 1 day
        let jewish_date = jewish_calendar.get_jewish_date();
        let greg_date = jewish_date.get_gregorian_date();

        // Convert to chrono NaiveDate and add 1 day
        let year = greg_date.year().extended_year();
        let month = greg_date.month().ordinal as u32;
        let day = greg_date.day_of_month().0 as u32;

        let naive_date = chrono::NaiveDate::from_ymd_opt(year, month, day);

        if naive_date.is_none() {
            return false;
        }

        let tomorrow_date = naive_date.unwrap().checked_add_days(chrono::Days::new(1));

        if tomorrow_date.is_none() {
            return false;
        }

        let tomorrow_date = tomorrow_date.unwrap();
        let tomorrow = JewishCalendar::from_gregorian_date(
            tomorrow_date.year() as i64,
            tomorrow_date.month() as u8,
            tomorrow_date.day() as u8,
            jewish_calendar.get_in_israel(),
            jewish_calendar.get_is_mukaf_choma(),
            jewish_calendar.get_is_use_modern_holidays(),
        );

        if tomorrow.is_none() {
            return false;
        }

        let tomorrow = tomorrow.unwrap();
        let tomorrow_yom_tov = tomorrow.get_yom_tov_index();

        if !self.tachanun_recited_mincha_all_year
            || jewish_date.get_day_of_week() == DayOfWeek::Friday
            || !self.is_tachanun_recited_shacharis(jewish_calendar)
            || (!self.is_tachanun_recited_shacharis(&tomorrow)
                && tomorrow_yom_tov != Some(JewishHoliday::ErevRoshHashana)
                && tomorrow_yom_tov != Some(JewishHoliday::ErevYomKippur)
                && tomorrow_yom_tov != Some(JewishHoliday::PesachSheni))
            || (!self.tachanun_recited_mincha_erev_lag_baomer
                && tomorrow_yom_tov == Some(JewishHoliday::LagBaomer))
        {
            return false;
        }
        true
    }

    fn is_hallel_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        let jewish_date = jewish_calendar.get_jewish_date();
        let day = jewish_date.get_jewish_day_of_month();
        let month = jewish_date.get_jewish_month();
        let holiday_index = jewish_calendar.get_yom_tov_index();
        let in_israel = jewish_calendar.get_in_israel();

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
                if jewish_calendar.get_is_use_modern_holidays()
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

    fn is_hallel_shalem_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        let jewish_date = jewish_calendar.get_jewish_date();
        let day = jewish_date.get_jewish_day_of_month();
        let month = jewish_date.get_jewish_month();
        let in_israel = jewish_calendar.get_in_israel();

        if self.is_hallel_recited(jewish_calendar) {
            if (jewish_calendar.is_rosh_chodesh() && !jewish_calendar.is_chanukah())
                || (month == JewishMonth::Nissan
                    && ((in_israel && day > 15) || (!in_israel && day > 16)))
            {
                return false;
            } else {
                return true;
            }
        }
        false
    }

    fn is_al_hanissim_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_purim() || jewish_calendar.is_chanukah()
    }

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_pesach()
            || jewish_calendar.is_shavuos()
            || jewish_calendar.is_rosh_hashana()
            || jewish_calendar.is_yom_kippur()
            || jewish_calendar.is_succos()
            || jewish_calendar.is_shemini_atzeres()
            || jewish_calendar.is_simchas_torah()
            || jewish_calendar.is_rosh_chodesh()
    }

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
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

    fn is_vesein_tal_umatar_start_date(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_vesein_tal_umatar_start_date()
    }

    fn is_vesein_tal_umatar_starting_tonight(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_vesein_tal_umatar_starting_tonight()
    }

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_vesein_tal_umatar_recited()
    }

    fn is_vesein_beracha_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_vesein_beracha_recited()
    }

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_mashiv_haruach_start_date()
    }

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_mashiv_haruach_end_date()
    }

    fn is_mashiv_haruach_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_mashiv_haruach_recited()
    }

    fn is_morid_hatal_recited(&self, jewish_calendar: &JewishCalendar) -> bool {
        jewish_calendar.is_morid_hatal_recited()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::jni::{DEFAULT_TEST_ITERATIONS, create_jewish_calendars, init_jvm};
    use j4rs::{Instance, InvocationArg, Jvm};
    use rand::Rng;

    fn bool_to_invocation_arg(bool: bool) -> InvocationArg {
        InvocationArg::try_from(bool)
            .unwrap()
            .into_primitive()
            .unwrap()
    }

    fn create_teffila_rules(jvm: &Jvm) -> (TefilaRules, Instance) {
        let mut rng = rand::thread_rng();
        let tefila_rules = TefilaRules::new(
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
        );
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.TefilaRules",
                InvocationArg::empty(),
            )
            .unwrap();

        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekOfPurim",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_week_of_purim,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekOfHod",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_week_of_hod,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedEndOfTishrei",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_end_of_tishrei,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekAfterShavuos",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_week_after_shavuos,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecited13SivanOutOfIsrael",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_13_sivan_out_of_israel,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedPesachSheni",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_pesach_sheni,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecited15IyarOutOfIsrael",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_15_iyar_out_of_israel,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedMinchaErevLagBaomer",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_mincha_erev_lag_baomer,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedShivasYemeiHamiluim",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_shivas_yemei_hamiluim,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedFridays",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_fridays,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedSundays",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_sundays,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedMinchaAllYear",
            &[bool_to_invocation_arg(
                tefila_rules.tachanun_recited_mincha_all_year,
            )],
        );
        let _ = jvm.invoke(
            &instance,
            "setMizmorLesodaRecitedErevYomKippurAndPesach",
            &[bool_to_invocation_arg(
                tefila_rules.mizmor_lesoda_recited_erev_yom_kippur_and_pesach,
            )],
        );

        (tefila_rules, instance)
    }

    #[test]
    fn test_tachanun_recited_shacharis() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_tachanun_recited_shacharis(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isTachanunRecitedShacharis",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_tachanun_recited_mincha() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_tachanun_recited_mincha(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isTachanunRecitedMincha",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_hallel_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_hallel_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isHallelRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_hallel_shalem_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_hallel_shalem_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isHallelShalemRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_al_hanissim_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_al_hanissim_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isAlHanissimRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_yaaleh_veyavo_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_yaaleh_veyavo_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isYaalehVeyavoRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_mizmor_lesoda_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_mizmor_lesoda_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isMizmorLesodaRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_vesein_tal_umatar_start_date() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_vesein_tal_umatar_start_date(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isVeseinTalUmatarStartDate",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_vesein_tal_umatar_starting_tonight() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_vesein_tal_umatar_starting_tonight(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isVeseinTalUmatarStartingTonight",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_vesein_tal_umatar_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_vesein_tal_umatar_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isVeseinTalUmatarRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_vesein_beracha_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_vesein_beracha_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isVeseinBerachaRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_mashiv_haruach_start_date() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_mashiv_haruach_start_date(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isMashivHaruachStartDate",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_mashiv_haruach_end_date() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_mashiv_haruach_end_date(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isMashivHaruachEndDate",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_mashiv_haruach_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_mashiv_haruach_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isMashivHaruachRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_morid_hatal_recited() {
        let jvm = init_jvm();
        let mut ran = false;

        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm);
            if test_case.is_none() {
                continue;
            }
            ran = true;
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            let result = tefila_rules.is_morid_hatal_recited(&rust_calendar);

            let java_result = jvm
                .invoke(
                    &java_tefila_rules,
                    "isMoridHatalRecited",
                    &[InvocationArg::try_from(java_calendar).unwrap()],
                )
                .unwrap();
            let java_bool: bool = jvm.to_rust(java_result).unwrap();
            assert_eq!(result, java_bool, "{}", message);
        }
        assert!(ran, "No test cases were run");
    }
}
