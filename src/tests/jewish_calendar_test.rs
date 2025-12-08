use chrono::{DateTime, Utc};
use j4rs::{Instance, InvocationArg, Jvm, Null};

use crate::{
    constants::{JewishHoliday, JewishMonth, Parsha},
    daf::{BavliDaf, YerushalmiDaf},
    defmt::DefmtFormatTrait,
    jewish_calendar::{InternalJewishCalendarTrait, JewishCalendarTrait},
};
pub struct JavaJewishCalendar<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> DefmtFormatTrait for JavaJewishCalendar<'a> {}
impl<'a> JavaJewishCalendar<'a> {
    pub fn from_gregorian_date(jvm: &'a Jvm, year: i32, month: i32, day: i32) -> Option<Self> {
        let year_arg = InvocationArg::try_from(year).unwrap().into_primitive().unwrap();
        let month_arg = InvocationArg::try_from(month).unwrap().into_primitive().unwrap();
        let day_arg = InvocationArg::try_from(day).unwrap().into_primitive().unwrap();
        let local_date = jvm
            .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
            .unwrap();
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
                &[InvocationArg::from(local_date)],
            )
            .ok();
        instance.as_ref()?;
        let instance = instance.unwrap();
        Some(Self { jvm, instance })
    }
    pub fn from_jewish_date(jvm: &'a Jvm, year: i32, month: JewishMonth, day: i32) -> Option<Self> {
        let year_arg = InvocationArg::try_from(year).unwrap().into_primitive().unwrap();
        let month_arg = InvocationArg::try_from(month as i32).unwrap().into_primitive().unwrap();
        let day_arg = InvocationArg::try_from(day).unwrap().into_primitive().unwrap();
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
                &[year_arg, month_arg, day_arg],
            )
            .ok();
        instance.as_ref()?;
        let instance = instance.unwrap();
        Some(Self { jvm, instance })
    }

    fn invoke_bool(&self, method: &str) -> bool {
        let java_result = self.jvm.invoke(&self.instance, method, InvocationArg::empty()).unwrap();
        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn invoke_i64(&self, method: &str) -> i64 {
        let java_result = self.jvm.invoke(&self.instance, method, InvocationArg::empty()).unwrap();
        self.jvm.to_rust::<i64>(java_result).unwrap()
    }

    pub fn set_in_israel(&self, in_israel: bool) {
        self.jvm
            .invoke(
                &self.instance,
                "setInIsrael",
                &[InvocationArg::try_from(in_israel).unwrap().into_primitive().unwrap()],
            )
            .unwrap();
    }

    pub fn set_mukaf_choma(&self, mukaf_choma: bool) {
        self.jvm
            .invoke(
                &self.instance,
                "setIsMukafChoma",
                &[InvocationArg::try_from(mukaf_choma).unwrap().into_primitive().unwrap()],
            )
            .unwrap();
    }

    pub fn set_use_modern_holidays(&self, use_modern_holidays: bool) {
        self.jvm
            .invoke(
                &self.instance,
                "setUseModernHolidays",
                &[InvocationArg::try_from(use_modern_holidays)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }

    fn java_date_to_rust_datetime(&self, java_date: &Instance) -> Option<DateTime<Utc>> {
        let is_null = self
            .jvm
            .check_equals(java_date, InvocationArg::try_from(Null::Of("java.util.Date")).unwrap())
            .unwrap();
        if is_null {
            return None;
        }

        let millis = self
            .jvm
            .to_rust::<i64>(self.jvm.invoke(java_date, "getTime", InvocationArg::empty()).unwrap())
            .unwrap();

        DateTime::<Utc>::from_timestamp_millis(millis)
    }

    fn invoke_date(&self, method: &str) -> Option<DateTime<Utc>> {
        let java_result = self.jvm.invoke(&self.instance, method, InvocationArg::empty()).ok()?;
        self.java_date_to_rust_datetime(&java_result)
    }

    fn parsha_from_java(&self, method: &str) -> Option<Parsha> {
        let java_parsha = self.jvm.invoke(&self.instance, method, InvocationArg::empty()).ok()?;
        let ordinal_instance = self.jvm.invoke(&java_parsha, "ordinal", InvocationArg::empty()).ok()?;
        let ordinal = self.jvm.to_rust::<i32>(ordinal_instance).ok()?;
        if ordinal == 0 {
            None
        } else {
            Parsha::try_from((ordinal - 1) as u8).ok()
        }
    }
}

impl<'a> JewishCalendarTrait for JavaJewishCalendar<'a> {
    fn get_yom_tov_index(&self) -> Option<crate::constants::JewishHoliday> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getYomTovIndex", InvocationArg::empty())
            .unwrap();
        let index = self.jvm.to_rust::<i32>(java_result).unwrap();
        if index == -1 {
            None
        } else {
            JewishHoliday::try_from(index as u8).ok()
        }
    }

    fn is_yom_tov(&self) -> bool {
        self.invoke_bool("isYomTov")
    }

    fn is_yom_tov_assur_bemelacha(&self) -> bool {
        self.invoke_bool("isYomTovAssurBemelacha")
    }

    fn is_assur_bemelacha(&self) -> bool {
        self.invoke_bool("isAssurBemelacha")
    }

    fn has_candle_lighting(&self) -> bool {
        self.invoke_bool("hasCandleLighting")
    }

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.invoke_bool("isTomorrowShabbosOrYomTov")
    }

    fn is_erev_yom_tov_sheni(&self) -> bool {
        self.invoke_bool("isErevYomTovSheni")
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        self.invoke_bool("isAseresYemeiTeshuva")
    }

    fn is_pesach(&self) -> bool {
        self.invoke_bool("isPesach")
    }

    fn is_chol_hamoed_pesach(&self) -> bool {
        self.invoke_bool("isCholHamoedPesach")
    }

    fn is_shavuos(&self) -> bool {
        self.invoke_bool("isShavuos")
    }

    fn is_rosh_hashana(&self) -> bool {
        self.invoke_bool("isRoshHashana")
    }

    fn is_yom_kippur(&self) -> bool {
        self.invoke_bool("isYomKippur")
    }

    fn is_succos(&self) -> bool {
        self.invoke_bool("isSuccos")
    }

    fn is_hoshana_rabba(&self) -> bool {
        self.invoke_bool("isHoshanaRabba")
    }

    fn is_shemini_atzeres(&self) -> bool {
        self.invoke_bool("isShminiAtzeres")
    }

    fn is_simchas_torah(&self) -> bool {
        self.invoke_bool("isSimchasTorah")
    }

    fn is_chol_hamoed_succos(&self) -> bool {
        self.invoke_bool("isCholHamoedSuccos")
    }

    fn is_chol_hamoed(&self) -> bool {
        self.invoke_bool("isCholHamoed")
    }

    fn is_erev_yom_tov(&self) -> bool {
        self.invoke_bool("isErevYomTov")
    }

    fn is_rosh_chodesh(&self) -> bool {
        self.invoke_bool("isRoshChodesh")
    }

    fn is_isru_chag(&self) -> bool {
        self.invoke_bool("isIsruChag")
    }

    fn is_taanis(&self) -> bool {
        self.invoke_bool("isTaanis")
    }

    fn is_taanis_bechoros(&self) -> bool {
        self.invoke_bool("isTaanisBechoros")
    }

    fn get_day_of_chanukah(&self) -> Option<u8> {
        let result = self.invoke_i64("getDayOfChanukah");
        if result == -1 { None } else { Some(result as u8) }
    }

    fn is_chanukah(&self) -> bool {
        self.invoke_bool("isChanukah")
    }

    fn is_purim(&self) -> bool {
        self.invoke_bool("isPurim")
    }

    fn get_day_of_omer(&self) -> Option<u8> {
        let result = self.invoke_i64("getDayOfOmer");
        if result == -1 { None } else { Some(result as u8) }
    }

    fn is_tisha_beav(&self) -> bool {
        self.invoke_bool("isTishaBav")
    }

    fn get_parshah(&self) -> Option<crate::constants::Parsha> {
        self.parsha_from_java("getParshah")
    }

    fn get_daf_yomi_bavli(&self) -> Option<crate::daf::BavliDaf> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDafYomiBavli", InvocationArg::empty())
            .ok()?;
        let is_null = self
            .jvm
            .check_equals(
                &java_result,
                InvocationArg::try_from(Null::Of("com.kosherjava.zmanim.hebrewcalendar.Daf")).unwrap(),
            )
            .unwrap();
        if is_null {
            return None;
        }

        let masechta = self
            .jvm
            .invoke(&java_result, "getMasechtaNumber", InvocationArg::empty())
            .ok()
            .and_then(|m| self.jvm.to_rust::<i32>(m).ok())?;
        let daf_index = self
            .jvm
            .invoke(&java_result, "getDaf", InvocationArg::empty())
            .ok()
            .and_then(|d| self.jvm.to_rust::<i32>(d).ok())?;

        let tractate = crate::constants::BavliTractate::try_from(masechta as u8).ok()?;

        Some(BavliDaf {
            tractate,
            daf_index: daf_index as i64,
        })
    }

    fn get_daf_yomi_yerushalmi(&self) -> Option<crate::daf::YerushalmiDaf> {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDafYomiYerushalmi", InvocationArg::empty())
            .ok()?;
        let is_null = self
            .jvm
            .check_equals(
                &java_result,
                InvocationArg::try_from(Null::Of("com.kosherjava.zmanim.hebrewcalendar.Daf")).unwrap(),
            )
            .unwrap();
        if is_null {
            return None;
        }

        let masechta = self
            .jvm
            .invoke(&java_result, "getMasechtaNumber", InvocationArg::empty())
            .ok()
            .and_then(|m| self.jvm.to_rust::<i32>(m).ok())?;

        // The Java implementation uses 39 (No Daf Today) as a sentinel.
        if masechta >= 39 {
            return None;
        }

        let daf_index = self
            .jvm
            .invoke(&java_result, "getDaf", InvocationArg::empty())
            .ok()
            .and_then(|d| self.jvm.to_rust::<i32>(d).ok())?;

        let tractate = crate::constants::YerushalmiTractate::try_from(masechta as i64).ok()?;

        Some(YerushalmiDaf {
            tractate,
            daf_index: daf_index as i64,
        })
    }

    fn is_birkas_hachamah(&self) -> bool {
        self.invoke_bool("isBirkasHachamah")
    }

    fn is_erev_rosh_chodesh(&self) -> bool {
        self.invoke_bool("isErevRoshChodesh")
    }

    fn is_yom_kippur_katan(&self) -> bool {
        self.invoke_bool("isYomKippurKatan")
    }

    fn is_be_hab(&self) -> bool {
        self.invoke_bool("isBeHaB")
    }

    fn is_machar_chodesh(&self) -> bool {
        self.invoke_bool("isMacharChodesh")
    }

    fn is_shabbos_mevorchim(&self) -> bool {
        self.invoke_bool("isShabbosMevorchim")
    }

    fn get_upcoming_parshah(&self) -> Option<crate::constants::Parsha> {
        self.parsha_from_java("getUpcomingParshah")
    }

    fn get_special_shabbos(&self) -> Option<crate::constants::Parsha> {
        self.parsha_from_java("getSpecialShabbos")
    }

    fn get_molad_as_date(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.invoke_date("getMoladAsDate")
    }

    fn get_tchilaszman_kidush_levana_3_days(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.invoke_date("getTchilasZmanKidushLevana3Days")
    }

    fn get_tchilaszman_kidush_levana_7_days(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.invoke_date("getTchilasZmanKidushLevana7Days")
    }

    fn get_sof_zman_kidush_levana_between_moldos(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.invoke_date("getSofZmanKidushLevanaBetweenMoldos")
    }

    fn get_sof_zman_kidush_levana_15_days(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.invoke_date("getSofZmanKidushLevana15Days")
    }

    fn get_tekufas_tishrei_elapsed_days(&self) -> i64 {
        self.invoke_i64("getTekufasTishreiElapsedDays")
    }

    fn is_vesein_tal_umatar_start_date(&self) -> bool {
        self.invoke_bool("isVeseinTalUmatarStartDate")
    }

    fn is_vesein_tal_umatar_starting_tonight(&self) -> bool {
        self.invoke_bool("isVeseinTalUmatarStartingTonight")
    }

    fn is_vesein_tal_umatar_recited(&self) -> bool {
        self.invoke_bool("isVeseinTalUmatarRecited")
    }

    fn is_vesein_beracha_recited(&self) -> bool {
        self.invoke_bool("isVeseinBerachaRecited")
    }

    fn is_mashiv_haruach_start_date(&self) -> bool {
        self.invoke_bool("isMashivHaruachStartDate")
    }

    fn is_mashiv_haruach_end_date(&self) -> bool {
        self.invoke_bool("isMashivHaruachEndDate")
    }

    fn is_mashiv_haruach_recited(&self) -> Option<bool> {
        Some(self.invoke_bool("isMashivHaruachRecited"))
    }

    fn is_morid_hatal_recited(&self) -> Option<bool> {
        Some(self.invoke_bool("isMoridHatalRecited"))
    }
}

impl<'a> InternalJewishCalendarTrait for JavaJewishCalendar<'a> {
    // This function does not need to be tested, but it is here to satisfy the trait.
    #[allow(refining_impl_trait)]
    fn get_jewish_date(&self) -> &crate::jewish_date::JewishDate {
        unimplemented!("get_jewish_date is not implemented in this test and should not be called");
    }

    fn get_in_israel(&self) -> bool {
        unimplemented!("get_in_israel is not implemented in this test and should not be called");
    }
    fn get_is_use_modern_holidays(&self) -> bool {
        unimplemented!("get_use_modern_holidays is not implemented in this test and should not be called");
    }
    #[allow(refining_impl_trait)]
    fn get_calculator(&self) -> &crate::astronomical_calculator::NOAACalculator {
        unimplemented!("get_calculator is not implemented in this test and should not be called");
    }

    fn get_is_mukaf_choma(&self) -> bool {
        unimplemented!("get_is_mukaf_choma is not implemented in this test and should not be called");
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Datelike;
    use rand::Rng;

    use crate::{
        astronomical_calculator::NOAACalculator,
        jewish_calendar::JewishCalendar,
        tests::test_utils::{DEFAULT_TEST_ITERATIONS, init_jvm, random_date_time, random_hebrew_date},
    };

    use super::*;

    pub fn create_jewish_calendars<'a>(
        jvm: &'a Jvm,
        rng: &mut impl Rng,
    ) -> Option<(JewishCalendar<NOAACalculator>, JavaJewishCalendar<'a>, String)> {
        let use_gregorian_date = rng.gen_bool(0.5);
        let in_israel = rng.gen_bool(0.5);
        let is_mukaf_choma = rng.gen_bool(0.5);
        let use_modern_holidays = rng.gen_bool(0.5);

        if use_gregorian_date {
            let date_time = random_date_time(rng, chrono_tz::Tz::UTC);
            let message = format!(
                "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
                date_time.year(),
                date_time.month(),
                date_time.day(),
                in_israel,
                is_mukaf_choma,
                use_modern_holidays
            );

            let rust_calendar = JewishCalendar::from_gregorian_date(
                date_time.year(),
                date_time.month() as u8,
                date_time.day() as u8,
                in_israel,
                is_mukaf_choma,
                use_modern_holidays,
                NOAACalculator,
            );
            let java_calendar = JavaJewishCalendar::from_gregorian_date(
                jvm,
                date_time.year() as i32,
                date_time.month() as i32,
                date_time.day() as i32,
            );

            assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
            if rust_calendar.is_none() || java_calendar.is_none() {
                return None;
            }

            let java_calendar = java_calendar.unwrap();
            let in_israel_arg = InvocationArg::try_from(in_israel).unwrap().into_primitive().unwrap();
            let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
                .unwrap()
                .into_primitive()
                .unwrap();
            let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
                .unwrap()
                .into_primitive()
                .unwrap();

            jvm.invoke(&java_calendar.instance, "setInIsrael", &[in_israel_arg])
                .unwrap();
            jvm.invoke(&java_calendar.instance, "setIsMukafChoma", &[is_mukaf_choma_arg])
                .unwrap();
            jvm.invoke(
                &java_calendar.instance,
                "setUseModernHolidays",
                &[use_modern_holidays_arg],
            )
            .unwrap();

            Some((rust_calendar.unwrap(), java_calendar, message))
        } else {
            let (year, month, day) = random_hebrew_date(rng);
            let message = format!(
                "year: {}, month: {}, day: {}, in_israel: {}, is_mukaf_choma: {}, use_modern_holidays: {}",
                year, month as i32, day, in_israel, is_mukaf_choma, use_modern_holidays
            );

            let rust_calendar = JewishCalendar::from_jewish_date(
                year,
                month,
                day,
                in_israel,
                is_mukaf_choma,
                use_modern_holidays,
                NOAACalculator,
            );
            let java_calendar = JavaJewishCalendar::from_jewish_date(jvm, year, month, day as i32);

            assert_eq!(rust_calendar.is_some(), java_calendar.is_some(), "{}", message);
            if rust_calendar.is_none() || java_calendar.is_none() {
                return None;
            }

            let java_calendar = java_calendar.unwrap();
            let in_israel_arg = InvocationArg::try_from(in_israel).unwrap().into_primitive().unwrap();
            let is_mukaf_choma_arg = InvocationArg::try_from(is_mukaf_choma)
                .unwrap()
                .into_primitive()
                .unwrap();
            let use_modern_holidays_arg = InvocationArg::try_from(use_modern_holidays)
                .unwrap()
                .into_primitive()
                .unwrap();

            jvm.invoke(&java_calendar.instance, "setInIsrael", &[in_israel_arg])
                .unwrap();
            jvm.invoke(&java_calendar.instance, "setIsMukafChoma", &[is_mukaf_choma_arg])
                .unwrap();
            jvm.invoke(
                &java_calendar.instance,
                "setUseModernHolidays",
                &[use_modern_holidays_arg],
            )
            .unwrap();

            Some((rust_calendar.unwrap(), java_calendar, message))
        }
    }

    fn compare_jewish_calendars(
        rust_calendar: &JewishCalendar<NOAACalculator>,
        java_calendar: &JavaJewishCalendar,
        message: &str,
    ) {
        assert_eq!(
            rust_calendar.get_yom_tov_index(),
            java_calendar.get_yom_tov_index(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_yom_tov(), java_calendar.is_yom_tov(), "{}", message);
        assert_eq!(
            rust_calendar.is_yom_tov_assur_bemelacha(),
            java_calendar.is_yom_tov_assur_bemelacha(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_assur_bemelacha(), java_calendar.is_assur_bemelacha());
        assert_eq!(
            rust_calendar.has_candle_lighting(),
            java_calendar.has_candle_lighting(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_tomorrow_shabbos_or_yom_tov(),
            java_calendar.is_tomorrow_shabbos_or_yom_tov(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_erev_yom_tov_sheni(),
            java_calendar.is_erev_yom_tov_sheni(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_aseres_yemei_teshuva(),
            java_calendar.is_aseres_yemei_teshuva(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_pesach(), java_calendar.is_pesach());
        assert_eq!(
            rust_calendar.is_chol_hamoed_pesach(),
            java_calendar.is_chol_hamoed_pesach(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_shavuos(), java_calendar.is_shavuos(), "{}", message);
        assert_eq!(rust_calendar.is_rosh_hashana(), java_calendar.is_rosh_hashana());
        assert_eq!(
            rust_calendar.is_yom_kippur(),
            java_calendar.is_yom_kippur(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_succos(), java_calendar.is_succos());
        assert_eq!(
            rust_calendar.is_hoshana_rabba(),
            java_calendar.is_hoshana_rabba(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_shemini_atzeres(), java_calendar.is_shemini_atzeres());
        assert_eq!(
            rust_calendar.is_simchas_torah(),
            java_calendar.is_simchas_torah(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_chol_hamoed_succos(),
            java_calendar.is_chol_hamoed_succos(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_chol_hamoed(),
            java_calendar.is_chol_hamoed(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_erev_yom_tov(),
            java_calendar.is_erev_yom_tov(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_rosh_chodesh(),
            java_calendar.is_rosh_chodesh(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_isru_chag(),
            java_calendar.is_isru_chag(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_taanis(), java_calendar.is_taanis(), "{}", message);
        assert_eq!(
            rust_calendar.is_taanis_bechoros(),
            java_calendar.is_taanis_bechoros(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.get_day_of_chanukah(), java_calendar.get_day_of_chanukah());
        assert_eq!(rust_calendar.is_chanukah(), java_calendar.is_chanukah(), "{}", message);
        assert_eq!(rust_calendar.is_purim(), java_calendar.is_purim());
        assert_eq!(
            rust_calendar.get_day_of_omer(),
            java_calendar.get_day_of_omer(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_tisha_beav(), java_calendar.is_tisha_beav());
        assert_eq!(rust_calendar.get_parshah(), java_calendar.get_parshah(), "{}", message);
        assert_eq!(
            rust_calendar.get_daf_yomi_bavli(),
            java_calendar.get_daf_yomi_bavli(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_daf_yomi_yerushalmi(),
            java_calendar.get_daf_yomi_yerushalmi(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_birkas_hachamah(),
            java_calendar.is_birkas_hachamah(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_erev_rosh_chodesh(),
            java_calendar.is_erev_rosh_chodesh(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_yom_kippur_katan(),
            java_calendar.is_yom_kippur_katan(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.is_be_hab(), java_calendar.is_be_hab(), "{}", message);
        assert_eq!(rust_calendar.is_machar_chodesh(), java_calendar.is_machar_chodesh());
        assert_eq!(
            rust_calendar.is_shabbos_mevorchim(),
            java_calendar.is_shabbos_mevorchim(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_upcoming_parshah(),
            java_calendar.get_upcoming_parshah(),
            "{}",
            message
        );
        assert_eq!(rust_calendar.get_special_shabbos(), java_calendar.get_special_shabbos());
        assert_eq!(
            rust_calendar.get_molad_as_date(),
            java_calendar.get_molad_as_date(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_tchilaszman_kidush_levana_3_days(),
            java_calendar.get_tchilaszman_kidush_levana_3_days(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_tchilaszman_kidush_levana_7_days(),
            java_calendar.get_tchilaszman_kidush_levana_7_days(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_sof_zman_kidush_levana_between_moldos(),
            java_calendar.get_sof_zman_kidush_levana_between_moldos(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_sof_zman_kidush_levana_15_days(),
            java_calendar.get_sof_zman_kidush_levana_15_days(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.get_tekufas_tishrei_elapsed_days(),
            java_calendar.get_tekufas_tishrei_elapsed_days(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_vesein_tal_umatar_start_date(),
            java_calendar.is_vesein_tal_umatar_start_date(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_vesein_tal_umatar_starting_tonight(),
            java_calendar.is_vesein_tal_umatar_starting_tonight(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_vesein_tal_umatar_recited(),
            java_calendar.is_vesein_tal_umatar_recited(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_vesein_beracha_recited(),
            java_calendar.is_vesein_beracha_recited(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_mashiv_haruach_start_date(),
            java_calendar.is_mashiv_haruach_start_date(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_mashiv_haruach_end_date(),
            java_calendar.is_mashiv_haruach_end_date(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_mashiv_haruach_recited(),
            java_calendar.is_mashiv_haruach_recited(),
            "{}",
            message
        );
        assert_eq!(
            rust_calendar.is_morid_hatal_recited(),
            java_calendar.is_morid_hatal_recited(),
            "{}",
            message
        );
    }
    #[test]
    fn test_compare_jewish_calendars_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            let test_case = create_jewish_calendars(&jvm, &mut rng);
            if test_case.is_none() {
                continue;
            }
            let (rust_calendar, java_calendar, message) = test_case.unwrap();
            compare_jewish_calendars(&rust_calendar, &java_calendar, &message);
            ran = true;
        }
        assert!(ran, "No test cases were run");
    }

    #[test]
    fn test_yerushalmi_daf_yomi_transition_date() {
        let jvm = init_jvm();

        // This is the specific failing case from the random test (Jewish date)
        let jewish_year = 5778;
        let jewish_month = JewishMonth::Av;
        let jewish_day = 23;
        let in_israel = true;
        let is_mukaf_choma = true;
        let use_modern_holidays = true;

        let rust_calendar = JewishCalendar::from_jewish_date(
            jewish_year,
            jewish_month,
            jewish_day as u8,
            in_israel,
            is_mukaf_choma,
            use_modern_holidays,
            NOAACalculator,
        );

        let java_calendar = JavaJewishCalendar::from_jewish_date(&jvm, jewish_year, jewish_month, jewish_day);

        assert!(rust_calendar.is_some(), "Failed to create Rust calendar");
        assert!(java_calendar.is_some(), "Failed to create Java calendar");

        let rust_calendar = rust_calendar.unwrap();
        let java_calendar = java_calendar.unwrap();

        // Set the same flags in Java calendar
        java_calendar.set_in_israel(in_israel);
        java_calendar.set_mukaf_choma(is_mukaf_choma);
        java_calendar.set_use_modern_holidays(use_modern_holidays);

        let rust_result = rust_calendar.get_daf_yomi_yerushalmi();
        let java_result = java_calendar.get_daf_yomi_yerushalmi();

        assert_eq!(
            rust_result, java_result,
            "Yerushalmi Daf Yomi mismatch for Jewish date {}-{:?}-{}",
            jewish_year, jewish_month, jewish_day
        );
    }
}
