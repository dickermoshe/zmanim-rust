use icu_calendar::{Date, Gregorian};
use j4rs::{Instance, InvocationArg, Jvm, Null};

use crate::{
    constants::JewishMonth,
    defmt::DefmtFormatTrait,
    jewish_date::{InternalJewishDateTrait, JewishDateTrait, MoladData},
};

pub struct JavaJewishDate<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> DefmtFormatTrait for JavaJewishDate<'a> {}
impl<'a> JavaJewishDate<'a> {
    pub fn from_gregorian_date(jvm: &'a Jvm, year: i32, month: i32, day: i32) -> Option<Self> {
        let year_arg = InvocationArg::try_from(year).unwrap().into_primitive().unwrap();
        let month_arg = InvocationArg::try_from(month).unwrap().into_primitive().unwrap();
        let day_arg = InvocationArg::try_from(day).unwrap().into_primitive().unwrap();
        let local_date = jvm
            .invoke_static("java.time.LocalDate", "of", &[year_arg, month_arg, day_arg])
            .unwrap();
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
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
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                &[year_arg, month_arg, day_arg],
            )
            .ok();
        instance.as_ref()?;
        let instance = instance.unwrap();
        Some(Self { jvm, instance })
    }
}

impl<'a> InternalJewishDateTrait for JavaJewishDate<'a> {
    fn get_gregorian_date(&self) -> Date<Gregorian> {
        unimplemented!("get_gregorian_date is not implemented in this test and should not be called");
    }
}

impl<'a> JewishDateTrait for JavaJewishDate<'a> {
    fn get_jewish_year(&self) -> i32 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getJewishYear", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<i32>(java_result).unwrap()
    }

    fn get_jewish_month(&self) -> JewishMonth {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getJewishMonth", InvocationArg::empty())
            .unwrap();
        let java_result = self.jvm.to_rust::<u8>(java_result).unwrap();
        java_result.try_into().unwrap()
    }
    fn get_jewish_day_of_month(&self) -> u8 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getJewishDayOfMonth", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<u8>(java_result).unwrap()
    }
    fn get_gregorian_year(&self) -> i32 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getGregorianYear", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<i32>(java_result).unwrap()
    }
    fn get_gregorian_month(&self) -> u8 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getGregorianMonth", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<u8>(java_result).unwrap()
    }

    fn get_gregorian_day_of_month(&self) -> u8 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getGregorianDayOfMonth", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<u8>(java_result).unwrap()
    }

    fn get_day_of_week(&self) -> crate::constants::DayOfWeek {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDayOfWeek", InvocationArg::empty())
            .unwrap();
        let java_result = self.jvm.to_rust::<u8>(java_result).unwrap();
        java_result.try_into().unwrap()
    }

    fn is_jewish_leap_year(&self) -> bool {
        let java_result = self
            .jvm
            .invoke(&self.instance, "isJewishLeapYear", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn get_days_in_jewish_year(&self) -> i32 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDaysInJewishYear", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<i32>(java_result).unwrap()
    }

    fn get_days_in_jewish_month(&self) -> u8 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDaysInJewishMonth", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<u8>(java_result).unwrap()
    }

    fn is_cheshvan_long(&self) -> bool {
        let java_result = self
            .jvm
            .invoke(&self.instance, "isCheshvanLong", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn is_kislev_short(&self) -> bool {
        let java_result = self
            .jvm
            .invoke(&self.instance, "isKislevShort", InvocationArg::empty())
            .unwrap();

        self.jvm.to_rust::<bool>(java_result).unwrap()
    }

    fn get_cheshvan_kislev_kviah(&self) -> crate::constants::YearLengthType {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getCheshvanKislevKviah", InvocationArg::empty())
            .unwrap();
        let java_result = self.jvm.to_rust::<u8>(java_result).unwrap();
        java_result.try_into().unwrap()
    }

    fn get_days_since_start_of_jewish_year(&self) -> i32 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getDaysSinceStartOfJewishYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<i32>(java_result).unwrap()
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let java_result = self
            .jvm
            .invoke(&self.instance, "getChalakimSinceMoladTohu", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<i64>(java_result).unwrap()
    }
    #[allow(refining_impl_trait)]
    fn get_molad_as_date(&self) -> Option<JavaJewishDate<'a>> {
        let java_molad = self
            .jvm
            .invoke(&self.instance, "getMolad", InvocationArg::empty())
            .unwrap();
        let is_null = self
            .jvm
            .check_equals(
                &java_molad,
                InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap();
        if is_null {
            return None;
        }

        let java_year = self
            .jvm
            .invoke(&java_molad, "getJewishYear", InvocationArg::empty())
            .unwrap();
        let java_year = self.jvm.to_rust::<i32>(java_year).unwrap();

        let java_month = self
            .jvm
            .invoke(&java_molad, "getJewishMonth", InvocationArg::empty())
            .unwrap();
        let java_month = self.jvm.to_rust::<u8>(java_month).unwrap();

        let java_day = self
            .jvm
            .invoke(&java_molad, "getJewishDayOfMonth", InvocationArg::empty())
            .unwrap();
        let java_day = self.jvm.to_rust::<i32>(java_day).unwrap();
        Some(JavaJewishDate::from_jewish_date(self.jvm, java_year, java_month.try_into().unwrap(), java_day).unwrap())
    }

    fn get_molad(&self) -> Option<MoladData> {
        let java_molad = self
            .jvm
            .invoke(&self.instance, "getMolad", InvocationArg::empty())
            .unwrap();
        let is_null = self
            .jvm
            .check_equals(
                &java_molad,
                InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap();
        if is_null {
            return None;
        }

        let java_hours = self
            .jvm
            .invoke(&java_molad, "getMoladHours", InvocationArg::empty())
            .unwrap();
        let java_hours = self.jvm.to_rust::<i64>(java_hours).unwrap();

        let java_minutes = self
            .jvm
            .invoke(&java_molad, "getMoladMinutes", InvocationArg::empty())
            .unwrap();
        let java_minutes = self.jvm.to_rust::<i64>(java_minutes).unwrap();

        let java_chalakim = self
            .jvm
            .invoke(&java_molad, "getMoladChalakim", InvocationArg::empty())
            .unwrap();
        let java_chalakim = self.jvm.to_rust::<i64>(java_chalakim).unwrap();
        Some(MoladData {
            hours: java_hours,
            minutes: java_minutes,
            chalakim: java_chalakim,
        })
    }
}

#[cfg(test)]
mod tests {

    use chrono::Datelike;
    use rand::Rng;

    use crate::{
        jewish_date::JewishDate,
        tests::test_utils::{DEFAULT_TEST_ITERATIONS, init_jvm, random_date_time, random_hebrew_date},
    };

    use super::*;

    fn compare_jewish_date_against_java(
        jewish_date: &impl JewishDateTrait,
        java_date: &impl JewishDateTrait,
        is_recursive: bool,
    ) {
        assert_eq!(jewish_date.get_jewish_year(), java_date.get_jewish_year());
        assert_eq!(jewish_date.get_jewish_month(), java_date.get_jewish_month());
        assert_eq!(
            jewish_date.get_jewish_day_of_month(),
            java_date.get_jewish_day_of_month()
        );
        assert_eq!(jewish_date.get_gregorian_year(), java_date.get_gregorian_year());
        assert_eq!(jewish_date.get_gregorian_month(), java_date.get_gregorian_month());
        assert_eq!(
            jewish_date.get_gregorian_day_of_month(),
            java_date.get_gregorian_day_of_month()
        );
        assert_eq!(jewish_date.get_day_of_week(), java_date.get_day_of_week());
        assert_eq!(jewish_date.is_jewish_leap_year(), java_date.is_jewish_leap_year());
        assert_eq!(
            jewish_date.get_days_in_jewish_year(),
            java_date.get_days_in_jewish_year()
        );
        assert_eq!(
            jewish_date.get_days_in_jewish_month(),
            java_date.get_days_in_jewish_month()
        );
        assert_eq!(jewish_date.is_cheshvan_long(), java_date.is_cheshvan_long());
        assert_eq!(jewish_date.is_kislev_short(), java_date.is_kislev_short());
        assert_eq!(
            jewish_date.get_cheshvan_kislev_kviah(),
            java_date.get_cheshvan_kislev_kviah()
        );
        assert_eq!(
            jewish_date.get_days_since_start_of_jewish_year(),
            java_date.get_days_since_start_of_jewish_year()
        );
        assert_eq!(
            jewish_date.get_chalakim_since_molad_tohu(),
            java_date.get_chalakim_since_molad_tohu()
        );

        assert_eq!(jewish_date.get_molad(), java_date.get_molad());
        if !is_recursive
            && let (Some(jewish_molad), Some(java_molad)) =
                (jewish_date.get_molad_as_date(), java_date.get_molad_as_date())
        {
            compare_jewish_date_against_java(&jewish_molad, &java_molad, true);
        }
    }

    fn random_jewish_date<'a>(jvm: &'a Jvm, rng: &mut impl Rng) -> Option<(JewishDate, JavaJewishDate<'a>)> {
        let use_gregorian_date = rng.gen_bool(0.5);
        if use_gregorian_date {
            let date_time = random_date_time(rng, chrono_tz::Tz::UTC);
            let jewish_date = JewishDate::from_gregorian_date(
                date_time.year() as i32,
                date_time.month() as u8,
                date_time.day() as u8,
            );
            let java_date = JavaJewishDate::from_gregorian_date(
                jvm,
                date_time.year() as i32,
                date_time.month() as i32,
                date_time.day() as i32,
            );
            // Ensure both are null or not null
            assert_eq!(
                jewish_date.is_some(),
                java_date.is_some(),
                "gregorian date: year: {}, month: {}, day: {}",
                date_time.year(),
                date_time.month(),
                date_time.day()
            );
            return match (jewish_date, java_date) {
                (Some(jewish_date), Some(java_date)) => Some((jewish_date, java_date)),
                _ => None,
            };
        } else {
            let (year, month, day) = random_hebrew_date(rng);
            let jewish_date = JewishDate::from_jewish_date(year, month, day);
            let java_date = JavaJewishDate::from_jewish_date(jvm, year, month, day as i32);
            // Ensure both are null or not null
            assert_eq!(
                jewish_date.is_some(),
                java_date.is_some(),
                "hebrew date: year: {}, month: {:?}, day: {}",
                year,
                month,
                day
            );
            return match (jewish_date, java_date) {
                (Some(jewish_date), Some(java_date)) => Some((jewish_date, java_date)),
                _ => None,
            };
        }
    }

    #[test]
    fn test_compare_jewish_date_against_java() {
        let jvm = init_jvm();
        let mut rng = rand::thread_rng();
        let mut ran = false;
        for _ in 0..DEFAULT_TEST_ITERATIONS {
            if let Some((jewish_date, java_date)) = random_jewish_date(&jvm, &mut rng) {
                compare_jewish_date_against_java(&jewish_date, &java_date, false);
                ran = true;
            }
        }
        assert!(ran, "No test cases were run");
    }
}
