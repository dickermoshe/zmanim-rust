use crate::tefila_rules::TefilaRulesTrait;
use crate::{jewish_calendar::JewishCalendarTrait, tests::jewish_calendar_test::JavaJewishCalendar};
use j4rs::{Instance, InvocationArg, Jvm};

struct JavaTefilaRules<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
}
fn bool_to_invocation_arg(bool: bool) -> InvocationArg {
    InvocationArg::try_from(bool).unwrap().into_primitive().unwrap()
}
impl<'a> JavaTefilaRules<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        jvm: &'a Jvm,
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
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.TefilaRules",
                InvocationArg::empty(),
            )
            .unwrap();
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekOfPurim",
            &[bool_to_invocation_arg(tachanun_recited_week_of_purim)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekOfHod",
            &[bool_to_invocation_arg(tachanun_recited_week_of_hod)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedEndOfTishrei",
            &[bool_to_invocation_arg(tachanun_recited_end_of_tishrei)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedWeekAfterShavuos",
            &[bool_to_invocation_arg(tachanun_recited_week_after_shavuos)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecited13SivanOutOfIsrael",
            &[bool_to_invocation_arg(tachanun_recited_13_sivan_out_of_israel)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedPesachSheni",
            &[bool_to_invocation_arg(tachanun_recited_pesach_sheni)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecited15IyarOutOfIsrael",
            &[bool_to_invocation_arg(tachanun_recited_15_iyar_out_of_israel)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedMinchaErevLagBaomer",
            &[bool_to_invocation_arg(tachanun_recited_mincha_erev_lag_baomer)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedShivasYemeiHamiluim",
            &[bool_to_invocation_arg(tachanun_recited_shivas_yemei_hamiluim)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedFridays",
            &[bool_to_invocation_arg(tachanun_recited_fridays)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedSundays",
            &[bool_to_invocation_arg(tachanun_recited_sundays)],
        );
        let _ = jvm.invoke(
            &instance,
            "setTachanunRecitedMinchaAllYear",
            &[bool_to_invocation_arg(tachanun_recited_mincha_all_year)],
        );
        let _ = jvm.invoke(
            &instance,
            "setMizmorLesodaRecitedErevYomKippurAndPesach",
            &[bool_to_invocation_arg(mizmor_lesoda_recited_erev_yom_kippur_and_pesach)],
        );
        Self { jvm, instance }
    }
}

/// Very sketchy function to convert a JewishCalendarTrait to a JavaJewishCalendar instance
/// This is acceptable for testing purposes
fn java_jewish_calendar_trait_to_java_instance<T: JewishCalendarTrait>(jvm: &Jvm, jewish_calendar: &T) -> Instance {
    unsafe {
        let java_calendar = &*(jewish_calendar as *const T as *const JavaJewishCalendar);
        jvm.clone_instance(&java_calendar.instance).ok().unwrap()
    }
}

impl<'a> TefilaRulesTrait for JavaTefilaRules<'a> {
    fn is_tachanun_recited_shacharis(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isTachanunRecitedShacharis",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_tachanun_recited_mincha(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> Option<bool> {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isTachanunRecitedMincha",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        let result: bool = self.jvm.to_rust(java_result).unwrap();
        Some(result)
    }

    fn is_hallel_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(&self.instance, "isHallelRecited", &[InvocationArg::from(java_calendar)])
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_hallel_shalem_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isHallelShalemRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_al_hanissim_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isAlHanissimRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isYaalehVeyavoRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isMizmorLesodaRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_vesein_tal_umatar_start_date(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isVeseinTalUmatarStartDate",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_vesein_tal_umatar_starting_tonight(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isVeseinTalUmatarStartingTonight",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isVeseinTalUmatarRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_vesein_beracha_recited(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isVeseinBerachaRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isMashivHaruachStartDate",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait) -> bool {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isMashivHaruachEndDate",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_mashiv_haruach_recited(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> Option<bool> {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isMashivHaruachRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        let result: bool = self.jvm.to_rust(java_result).unwrap();
        Some(result)
    }

    fn is_morid_hatal_recited(
        &self,
        jewish_calendar: &impl crate::jewish_calendar::JewishCalendarTrait,
    ) -> Option<bool> {
        let java_calendar = java_jewish_calendar_trait_to_java_instance(self.jvm, jewish_calendar);
        let java_result = self
            .jvm
            .invoke(
                &self.instance,
                "isMoridHatalRecited",
                &[InvocationArg::from(java_calendar)],
            )
            .unwrap();
        let result: bool = self.jvm.to_rust(java_result).unwrap();
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        astronomical_calculator::NOAACalculator,
        jewish_calendar::JewishCalendar,
        tefila_rules::TefilaRules,
        tests::{DEFAULT_TEST_ITERATIONS, init_jvm, jewish_calendar_test::JavaJewishCalendar},
    };
    use j4rs::Jvm;
    #[allow(mismatched_lifetime_syntaxes)]
    fn create_teffila_rules(jvm: &Jvm) -> (TefilaRules, JavaTefilaRules) {
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
        let java_tefila_rules = JavaTefilaRules::new(
            jvm,
            tefila_rules.tachanun_recited_end_of_tishrei,
            tefila_rules.tachanun_recited_week_after_shavuos,
            tefila_rules.tachanun_recited_13_sivan_out_of_israel,
            tefila_rules.tachanun_recited_pesach_sheni,
            tefila_rules.tachanun_recited_15_iyar_out_of_israel,
            tefila_rules.tachanun_recited_mincha_erev_lag_baomer,
            tefila_rules.tachanun_recited_shivas_yemei_hamiluim,
            tefila_rules.tachanun_recited_week_of_hod,
            tefila_rules.tachanun_recited_week_of_purim,
            tefila_rules.tachanun_recited_fridays,
            tefila_rules.tachanun_recited_sundays,
            tefila_rules.tachanun_recited_mincha_all_year,
            tefila_rules.mizmor_lesoda_recited_erev_yom_kippur_and_pesach,
        );

        (tefila_rules, java_tefila_rules)
    }

    fn compare_tefila_rules(
        rust_tefila_rules: &TefilaRules,
        java_tefila_rules: &JavaTefilaRules,
        rust_jewish_calendar: &JewishCalendar<NOAACalculator>,
        java_jewish_calendar: &JavaJewishCalendar,
        message: &str,
    ) {
        assert_eq!(
            rust_tefila_rules.is_tachanun_recited_shacharis(rust_jewish_calendar),
            java_tefila_rules.is_tachanun_recited_shacharis(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_tachanun_recited_mincha(rust_jewish_calendar),
            java_tefila_rules.is_tachanun_recited_mincha(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_hallel_recited(rust_jewish_calendar),
            java_tefila_rules.is_hallel_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_hallel_shalem_recited(rust_jewish_calendar),
            java_tefila_rules.is_hallel_shalem_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_al_hanissim_recited(rust_jewish_calendar),
            java_tefila_rules.is_al_hanissim_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_yaaleh_veyavo_recited(rust_jewish_calendar),
            java_tefila_rules.is_yaaleh_veyavo_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_mizmor_lesoda_recited(rust_jewish_calendar),
            java_tefila_rules.is_mizmor_lesoda_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_vesein_tal_umatar_start_date(rust_jewish_calendar),
            java_tefila_rules.is_vesein_tal_umatar_start_date(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_vesein_tal_umatar_starting_tonight(rust_jewish_calendar),
            java_tefila_rules.is_vesein_tal_umatar_starting_tonight(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_vesein_tal_umatar_recited(rust_jewish_calendar),
            java_tefila_rules.is_vesein_tal_umatar_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_vesein_beracha_recited(rust_jewish_calendar),
            java_tefila_rules.is_vesein_beracha_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_mashiv_haruach_start_date(rust_jewish_calendar),
            java_tefila_rules.is_mashiv_haruach_start_date(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_mashiv_haruach_end_date(rust_jewish_calendar),
            java_tefila_rules.is_mashiv_haruach_end_date(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_mashiv_haruach_recited(rust_jewish_calendar),
            java_tefila_rules.is_mashiv_haruach_recited(java_jewish_calendar),
            "{}",
            message
        );
        assert_eq!(
            rust_tefila_rules.is_morid_hatal_recited(rust_jewish_calendar),
            java_tefila_rules.is_morid_hatal_recited(java_jewish_calendar),
            "{}",
            message
        );
    }

    #[test]
    fn test_compare_tefila_rules_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = crate::tests::jewish_calendar_test::tests::create_jewish_calendars(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            let (rust_tefila_rules, java_tefila_rules) = create_teffila_rules(&jvm);
            compare_tefila_rules(
                &rust_tefila_rules,
                &java_tefila_rules,
                &rust_calendar,
                &java_calendar,
                &message,
            );
            ran = true;
        }
        assert!(ran, "No test cases were run");
    }
}
