use chrono::Weekday;
use icu_calendar::types::Weekday as IcuWeekday;
use icu_calendar::{Calendar, Date, Gregorian, cal::Hebrew};
use num_enum::{IntoPrimitive, TryFromPrimitive};
pub(crate) static _CHALAKIM_MOLAD_TOHU: i64 = 31524;
pub(crate) static _CHALAKIM_PER_MONTH: i64 = 765433;
pub(crate) static _CHALAKIM_PER_DAY: i64 = 25920;



 trait HebrewHolidayCalendar {
    /// Returns a copy of this date converted to the Gregorian calendar
    fn gregorian_date(&self) -> Date<Gregorian>;
    /// Returns the current month as a `HebrewMonth`
    fn hebrew_month(&self) -> HebrewMonth;
    /// Returns the day of week as a chrono::Weekday
    fn chrono_day_of_week(&self) -> chrono::Weekday;
    /// Returns the ammount of days in the hebrew year
    fn days_in_hebrew_year(year: i32) -> i32;
    /// Returns the ammount of days in the hebrew month
    fn days_in_hebrew_month(year: i32, month: HebrewMonth) -> u8;
    fn is_cheshvan_long(year: i32) -> bool;
    fn is_kislev_short(year: i32) -> bool;
    fn is_hebrew_leap_year(year: i32) -> bool;
    fn cheshvan_kislev_kviah(year: i32) -> YearLengthType;
    fn holiday(&self,in_israel:bool,use_modern_holidays:bool)->Option<Holiday>;
    fn assur_bemelacha(&self,in_israel:bool,);
}

/// Returns the `HebrewMonth` as a `u8` which is indexed starting from Tishrei
/// instead of Nissan.
fn hebrew_month_of_year(year: i32, month: HebrewMonth) -> u8 {
    let is_leap_year = Date::<Hebrew>::is_hebrew_leap_year(year);
    (month as u8 + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
}
// Returns the number of chalakim from the original hypothetical Molad Tohu
fn chalakim_since_molad_tohu(year: i32, month: HebrewMonth) -> i64 {
    let month_of_year = hebrew_month_of_year(year, month);
    let months_elapsed = (235 * ((year - 1) / 19))
        + (12 * ((year - 1) % 19))
        + ((7 * ((year - 1) % 19) + 1) / 19)
        + (month_of_year as i32 - 1);

    _CHALAKIM_MOLAD_TOHU + (_CHALAKIM_PER_MONTH * months_elapsed as i64)
}

/// Returns the number of days elapsed from the Sunday prior to the start of the Jewish calendar to the mean conjunction of Tishri of the Jewish year.
fn get_hebrew_elapsed_days(year: i32) -> i32 {
    let chalakim_since = chalakim_since_molad_tohu(year, HebrewMonth::Tishrei);
    let molad_day = chalakim_since / _CHALAKIM_PER_DAY;
    let molad_parts = chalakim_since - molad_day * _CHALAKIM_PER_DAY;
    let mut rosh_hashana_day = molad_day;

    if (molad_parts >= 19440)
        || (((molad_day % 7) == 2) && (molad_parts >= 9924) && !Date::<Hebrew>::is_hebrew_leap_year(year))
        || (((molad_day % 7) == 1) && (molad_parts >= 16789) && (Date::<Hebrew>::is_hebrew_leap_year(year - 1)))
    {
        rosh_hashana_day += 1;
    }

    if ((rosh_hashana_day % 7) == 0) || ((rosh_hashana_day % 7) == 3) || ((rosh_hashana_day % 7) == 5) {
        rosh_hashana_day += 1;
    }

    rosh_hashana_day as i32
}

impl HebrewHolidayCalendar for Date<Hebrew> {
    fn assur_bemelacha(&self,in_israel:bool,){
      if let is_assur =  self.holiday(in_israel, false).map(|i|i.is_assur_bemelacha())    }{
          if is_assur {
            true
          }
      }  

    #[inline]

    fn is_cheshvan_long(year: i32) -> bool {
        Self::days_in_hebrew_year(year) % 10 == 5
    }
    #[inline]

    fn is_kislev_short(year: i32) -> bool {
        Self::days_in_hebrew_year(year) % 10 == 3
    }
    #[inline]
    fn hebrew_month(&self) -> HebrewMonth {
        let month_code = self.month().formatting_code.0;
        match month_code.as_str() {
            "M01" => HebrewMonth::Tishrei,
            "M02" => HebrewMonth::Cheshvan,
            "M03" => HebrewMonth::Kislev,
            "M04" => HebrewMonth::Teves,
            "M05" => HebrewMonth::Shevat,
            "M05L" => HebrewMonth::Adar,
            "M06" => HebrewMonth::Adar,
            "M06L" => HebrewMonth::AdarII,
            "M07" => HebrewMonth::Nissan,
            "M08" => HebrewMonth::Iyar,
            "M09" => HebrewMonth::Sivan,
            "M10" => HebrewMonth::Tammuz,
            "M11" => HebrewMonth::Av,
            "M12" => HebrewMonth::Elul,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn gregorian_date(&self) -> Date<Gregorian> {
        self.to_calendar(Gregorian)
    }
    #[inline]

    fn chrono_day_of_week(&self) -> chrono::Weekday {
        let weekday = self.day_of_week();
        match weekday {
            IcuWeekday::Sunday => Weekday::Sun,
            IcuWeekday::Monday => Weekday::Mon,
            IcuWeekday::Tuesday => Weekday::Tue,
            IcuWeekday::Wednesday => Weekday::Wed,
            IcuWeekday::Thursday => Weekday::Thu,
            IcuWeekday::Friday => Weekday::Fri,
            IcuWeekday::Saturday => Weekday::Sat,
        }
    }
    #[inline]

    fn days_in_hebrew_year(year: i32) -> i32 {
        get_hebrew_elapsed_days(year + 1) - get_hebrew_elapsed_days(year)
    }
    #[inline]

    fn days_in_hebrew_month(year: i32, month: HebrewMonth) -> u8 {
        match month {
            HebrewMonth::Iyar | HebrewMonth::Tammuz | HebrewMonth::Elul | HebrewMonth::Teves => 29,
            HebrewMonth::Cheshvan => {
                if Self::is_cheshvan_long(year) {
                    30
                } else {
                    29
                }
            }
            HebrewMonth::Kislev => {
                if Self::is_kislev_short(year) {
                    29
                } else {
                    30
                }
            }
            HebrewMonth::Adar => {
                if Self::is_hebrew_leap_year(year) {
                    30
                } else {
                    29
                }
            }
            HebrewMonth::AdarII => 29,
            _ => 30,
        }
    }
    #[inline]

    fn is_hebrew_leap_year(year: i32) -> bool {
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }
    #[inline]

    fn cheshvan_kislev_kviah(year: i32) -> YearLengthType {
        if Self::is_cheshvan_long(year) && !Self::is_kislev_short(year) {
            YearLengthType::Shelaimim
        } else if !Self::is_cheshvan_long(year) && Self::is_kislev_short(year) {
            YearLengthType::Chaserim
        } else {
            YearLengthType::Kesidran
        }
    }
    
    fn holiday(&self, in_israel:bool,use_modern_holidays:bool)->Option<Holiday> {
        let day = self.day_of_month().0;
        let day_of_week = self.chrono_day_of_week();
        let month = self.hebrew_month();

        match month {
            HebrewMonth::Nissan => {
                if day == 14 {
                    return Some(Holiday::ErevPesach);
                }
                if day == 15 || day == 21 || (!in_israel && (day == 16 || day == 22)) {
                    return Some(Holiday::Pesach);
                }
                if (17..=20).contains(&day) || day == 16 {
                    return Some(Holiday::CholHamoedPesach);
                }
                if day == 22 || day == 23 && !in_israel {
                    return Some(Holiday::IsruChag);
                }
                if use_modern_holidays
                    && ((day == 26 && day_of_week == Weekday::Thu)
                        || (day == 28 && day_of_week == Weekday::Mon)
                        || (day == 27 && day_of_week != Weekday::Sun && day_of_week != Weekday::Fri))
                {
                    return Some(Holiday::YomHaShoah);
                }
            }

            HebrewMonth::Iyar => {
                if use_modern_holidays {
                    if (day == 4 && day_of_week == Weekday::Tue)
                        || ((day == 3 || day == 2) && day_of_week == Weekday::Wed)
                        || (day == 5 && day_of_week == Weekday::Mon)
                    {
                        return Some(Holiday::YomHazikaron);
                    }
                    if (day == 5 && day_of_week == Weekday::Wed)
                        || ((day == 4 || day == 3) && day_of_week == Weekday::Thu)
                        || (day == 6 && day_of_week == Weekday::Tue)
                    {
                        return Some(Holiday::YomHaatzmaut);
                    }
                }
                if day == 14 {
                    return Some(Holiday::PesachSheni);
                }
                if day == 18 {
                    return Some(Holiday::LagBomer);
                }
                if use_modern_holidays && day == 28 {
                    return Some(Holiday::YomYerushalayim);
                }
            }

            HebrewMonth::Sivan => {
                if day == 5 {
                    return Some(Holiday::ErevShavuos);
                }
                if day == 6 || (day == 7 && !in_israel) {
                    return Some(Holiday::Shavuos);
                }
                if (day == 7 && in_israel) || (day == 8 && !in_israel) {
                    return Some(Holiday::IsruChag);
                }
            }

            HebrewMonth::Tammuz => {
                if (day == 17 && day_of_week != Weekday::Sat) || (day == 18 && day_of_week == Weekday::Sun) {
                    return Some(Holiday::SeventeenthOfTammuz);
                }
            }

            HebrewMonth::Av => {
                if (day_of_week == Weekday::Sun && day == 10) || (day_of_week != Weekday::Sat && day == 9) {
                    return Some(Holiday::TishahBav);
                }
                if day == 15 {
                    return Some(Holiday::TuBav);
                }
            }

            HebrewMonth::Elul => {
                if day == 29 {
                    return Some(Holiday::ErevRoshHashana);
                }
            }

            HebrewMonth::Tishrei => {
                if day == 1 || day == 2 {
                    return Some(Holiday::RoshHashana);
                }
                if (day == 3 && day_of_week != Weekday::Sat) || (day == 4 && day_of_week == Weekday::Sun) {
                    return Some(Holiday::FastOfGedalyah);
                }
                if day == 9 {
                    return Some(Holiday::ErevYomKippur);
                }
                if day == 10 {
                    return Some(Holiday::YomKippur);
                }
                if day == 14 {
                    return Some(Holiday::ErevSuccos);
                }
                if day == 15 {
                    return Some(Holiday::Succos);
                }
                if day == 16 && !in_israel {
                    return Some(Holiday::Succos);
                }
                if (16..=20).contains(&day) {
                    return Some(Holiday::CholHamoedSuccos);
                }
                if day == 21 {
                    return Some(Holiday::HoshanaRabbah);
                }
                if day == 22 {
                    return Some(Holiday::SheminiAtzeres);
                }
                if day == 23 && !in_israel {
                    return Some(Holiday::SimchasTorah);
                }
                if day == 24 && !in_israel || (day == 23 && in_israel) {
                    return Some(Holiday::IsruChag);
                }
            }

            HebrewMonth::Kislev => {
                if day >= 25 {
                    return Some(Holiday::Chanukah);
                }
            }

            HebrewMonth::Teves => {
                if day == 1 || day == 2 || (day == 3 && Self::is_kislev_short(self.extended_year())) {
                    return Some(Holiday::Chanukah);
                }
                if day == 10 {
                    return Some(Holiday::TenthOfTeves);
                }
            }

            HebrewMonth::Shevat => {
                if day == 15 {
                    return Some(Holiday::TuBshvat);
                }
            }

            HebrewMonth::Adar => {
                if !self.is_in_leap_year() {
                    if ((day == 11 || day == 12) && day_of_week == Weekday::Thu)
                        || (day == 13 && !(day_of_week == Weekday::Fri || day_of_week == Weekday::Sat))
                    {
                        return Some(Holiday::FastOfEsther);
                    }
                    if day == 14 {
                        return Some(Holiday::Purim);
                    }
                    if day == 15 {
                        return Some(Holiday::ShushanPurim);
                    }
                } else {
                    if day == 14 {
                        return Some(Holiday::PurimKatan);
                    }
                    if day == 15 {
                        return Some(Holiday::ShushanPurimKatan);
                    }
                }
            }

            HebrewMonth::AdarII => {
                if ((day == 11 || day == 12) && day_of_week == Weekday::Thu)
                    || (day == 13 && !(day_of_week == Weekday::Fri || day_of_week == Weekday::Sat))
                {
                    return Some(Holiday::FastOfEsther);
                }
                if day == 14 {
                    return Some(Holiday::Purim);
                }
                if day == 15 {
                    return Some(Holiday::ShushanPurim);
                }
            }
            _ => {}
        }

        None
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Parsha {
    Bereshis = 0,
    Noach = 1,
    LechLecha = 2,
    Vayera = 3,
    ChayeiSara = 4,
    Toldos = 5,
    Vayetzei = 6,
    Vayishlach = 7,
    Vayeshev = 8,
    Miketz = 9,
    Vayigash = 10,
    Vayechi = 11,
    Shemos = 12,
    Vaera = 13,
    Bo = 14,
    Beshalach = 15,
    Yisro = 16,
    Mishpatim = 17,
    Terumah = 18,
    Tetzaveh = 19,
    KiSisa = 20,
    Vayakhel = 21,
    Pekudei = 22,
    Vayikra = 23,
    Tzav = 24,
    Shmini = 25,
    Tazria = 26,
    Metzora = 27,
    AchreiMos = 28,
    Kedoshim = 29,
    Emor = 30,
    Behar = 31,
    Bechukosai = 32,
    Bamidbar = 33,
    Nasso = 34,
    Behaaloscha = 35,
    Shlach = 36,
    Korach = 37,
    Chukas = 38,
    Balak = 39,
    Pinchas = 40,
    Matos = 41,
    Masei = 42,
    Devarim = 43,
    Vaeschanan = 44,
    Eikev = 45,
    Reeh = 46,
    Shoftim = 47,
    KiSeitzei = 48,
    KiSavo = 49,
    Nitzavim = 50,
    Vayeilech = 51,
    HaAzinu = 52,
    VezosHabracha = 53,
    VayakhelPekudei = 54,
    TazriaMetzora = 55,
    AchreiMosKedoshim = 56,
    BeharBechukosai = 57,
    ChukasBalak = 58,
    MatosMasei = 59,
    NitzavimVayeilech = 60,
    Shekalim = 61,
    Zachor = 62,
    Parah = 63,
    Hachodesh = 64,
    Shuva = 65,
    Shira = 66,
    Hagadol = 67,
    Chazon = 68,
    Nachamu = 69,
}
impl Parsha {
    pub fn en_string(&self) -> &str {
        match self {
            Parsha::Bereshis => "Bereshis",
            Parsha::Noach => "Noach",
            Parsha::LechLecha => "Lech Lecha",
            Parsha::Vayera => "Vayera",
            Parsha::ChayeiSara => "Chayei Sara",
            Parsha::Toldos => "Toldos",
            Parsha::Vayetzei => "Vayetzei",
            Parsha::Vayishlach => "Vayishlach",
            Parsha::Vayeshev => "Vayeshev",
            Parsha::Miketz => "Miketz",
            Parsha::Vayigash => "Vayigash",
            Parsha::Vayechi => "Vayechi",
            Parsha::Shemos => "Shemos",
            Parsha::Vaera => "Vaera",
            Parsha::Bo => "Bo",
            Parsha::Beshalach => "Beshalach",
            Parsha::Yisro => "Yisro",
            Parsha::Mishpatim => "Mishpatim",
            Parsha::Terumah => "Terumah",
            Parsha::Tetzaveh => "Tetzaveh",
            Parsha::KiSisa => "Ki Sisa",
            Parsha::Vayakhel => "Vayakhel",
            Parsha::Pekudei => "Pekudei",
            Parsha::Vayikra => "Vayikra",
            Parsha::Tzav => "Tzav",
            Parsha::Shmini => "Shmini",
            Parsha::Tazria => "Tazria",
            Parsha::Metzora => "Metzora",
            Parsha::AchreiMos => "Achrei Mos",
            Parsha::Kedoshim => "Kedoshim",
            Parsha::Emor => "Emor",
            Parsha::Behar => "Behar",
            Parsha::Bechukosai => "Bechukosai",
            Parsha::Bamidbar => "Bamidbar",
            Parsha::Nasso => "Nasso",
            Parsha::Behaaloscha => "Beha'aloscha",
            Parsha::Shlach => "Sh'lach",
            Parsha::Korach => "Korach",
            Parsha::Chukas => "Chukas",
            Parsha::Balak => "Balak",
            Parsha::Pinchas => "Pinchas",
            Parsha::Matos => "Matos",
            Parsha::Masei => "Masei",
            Parsha::Devarim => "Devarim",
            Parsha::Vaeschanan => "Vaeschanan",
            Parsha::Eikev => "Eikev",
            Parsha::Reeh => "Re'eh",
            Parsha::Shoftim => "Shoftim",
            Parsha::KiSeitzei => "Ki Seitzei",
            Parsha::KiSavo => "Ki Savo",
            Parsha::Nitzavim => "Nitzavim",
            Parsha::Vayeilech => "Vayeilech",
            Parsha::HaAzinu => "Ha'Azinu",
            Parsha::VezosHabracha => "Vezos Habracha",
            Parsha::VayakhelPekudei => "Vayakhel Pekudei",
            Parsha::TazriaMetzora => "Tazria Metzora",
            Parsha::AchreiMosKedoshim => "Achrei Mos Kedoshim",
            Parsha::BeharBechukosai => "Behar Bechukosai",
            Parsha::ChukasBalak => "Chukas Balak",
            Parsha::MatosMasei => "Matos Masei",
            Parsha::NitzavimVayeilech => "Nitzavim Vayeilech",
            Parsha::Shekalim => "Shekalim",
            Parsha::Zachor => "Zachor",
            Parsha::Parah => "Parah",
            Parsha::Hachodesh => "Hachodesh",
            Parsha::Shuva => "Shuva",
            Parsha::Shira => "Shira",
            Parsha::Hagadol => "Hagadol",
            Parsha::Chazon => "Chazon",
            Parsha::Nachamu => "Nachamu",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            Parsha::Bereshis => "בראשית",
            Parsha::Noach => "נח",
            Parsha::LechLecha => "לך לך",
            Parsha::Vayera => "וירא",
            Parsha::ChayeiSara => "חיי שרה",
            Parsha::Toldos => "תולדות",
            Parsha::Vayetzei => "ויצא",
            Parsha::Vayishlach => "וישלח",
            Parsha::Vayeshev => "וישב",
            Parsha::Miketz => "מקץ",
            Parsha::Vayigash => "ויגש",
            Parsha::Vayechi => "ויחי",
            Parsha::Shemos => "שמות",
            Parsha::Vaera => "וארא",
            Parsha::Bo => "בא",
            Parsha::Beshalach => "בשלח",
            Parsha::Yisro => "יתרו",
            Parsha::Mishpatim => "משפטים",
            Parsha::Terumah => "תרומה",
            Parsha::Tetzaveh => "תצוה",
            Parsha::KiSisa => "כי תשא",
            Parsha::Vayakhel => "ויקהל",
            Parsha::Pekudei => "פקודי",
            Parsha::Vayikra => "ויקרא",
            Parsha::Tzav => "צו",
            Parsha::Shmini => "שמיני",
            Parsha::Tazria => "תזריע",
            Parsha::Metzora => "מצרע",
            Parsha::AchreiMos => "אחרי מות",
            Parsha::Kedoshim => "קדושים",
            Parsha::Emor => "אמור",
            Parsha::Behar => "בהר",
            Parsha::Bechukosai => "בחקתי",
            Parsha::Bamidbar => "במדבר",
            Parsha::Nasso => "נשא",
            Parsha::Behaaloscha => "בהעלתך",
            Parsha::Shlach => "שלח לך",
            Parsha::Korach => "קרח",
            Parsha::Chukas => "חוקת",
            Parsha::Balak => "בלק",
            Parsha::Pinchas => "פינחס",
            Parsha::Matos => "מטות",
            Parsha::Masei => "מסעי",
            Parsha::Devarim => "דברים",
            Parsha::Vaeschanan => "ואתחנן",
            Parsha::Eikev => "עקב",
            Parsha::Reeh => "ראה",
            Parsha::Shoftim => "שופטים",
            Parsha::KiSeitzei => "כי תצא",
            Parsha::KiSavo => "כי תבוא",
            Parsha::Nitzavim => "נצבים",
            Parsha::Vayeilech => "וילך",
            Parsha::HaAzinu => "האזינו",
            Parsha::VezosHabracha => "וזאת הברכה ",
            Parsha::VayakhelPekudei => "ויקהל פקודי",
            Parsha::TazriaMetzora => "תזריע מצרע",
            Parsha::AchreiMosKedoshim => "אחרי מות קדושים",
            Parsha::BeharBechukosai => "בהר בחקתי",
            Parsha::ChukasBalak => "חוקת בלק",
            Parsha::MatosMasei => "מטות מסעי",
            Parsha::NitzavimVayeilech => "נצבים וילך",
            Parsha::Shekalim => "שקלים",
            Parsha::Zachor => "זכור",
            Parsha::Parah => "פרה",
            Parsha::Hachodesh => "החדש",
            Parsha::Shuva => "שובה",
            Parsha::Shira => "שירה",
            Parsha::Hagadol => "הגדול",
            Parsha::Chazon => "חזון",
            Parsha::Nachamu => "נחמו",
        }
    }
}
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Holiday {
    ErevPesach = 0,
    Pesach = 1,
    CholHamoedPesach = 2,
    PesachSheni = 3,
    ErevShavuos = 4,
    Shavuos = 5,
    SeventeenthOfTammuz = 6,
    TishahBav = 7,
    TuBav = 8,
    ErevRoshHashana = 9,
    RoshHashana = 10,
    FastOfGedalyah = 11,
    ErevYomKippur = 12,
    YomKippur = 13,
    ErevSuccos = 14,
    Succos = 15,
    CholHamoedSuccos = 16,
    HoshanaRabbah = 17,
    SheminiAtzeres = 18,
    SimchasTorah = 19,
    ErevChanukah = 20,
    Chanukah = 21,
    TenthOfTeves = 22,
    TuBshvat = 23,
    FastOfEsther = 24,
    Purim = 25,
    ShushanPurim = 26,
    PurimKatan = 27,
    RoshChodesh = 28,
    YomHaShoah = 29,
    YomHazikaron = 30,
    YomHaatzmaut = 31,
    YomYerushalayim = 32,
    LagBomer = 33,
    ShushanPurimKatan = 34,
    IsruChag = 35,
    YomKippurKatan = 36,
    Behab = 37,
}
impl Holiday {
    pub fn en_string(&self) -> &str {
        match self {
            Holiday::ErevPesach => "Erev Pesach",
            Holiday::Pesach => "Pesach",
            Holiday::CholHamoedPesach => "Chol Hamoed Pesach",
            Holiday::PesachSheni => "Pesach Sheni",
            Holiday::ErevShavuos => "Erev Shavuos",
            Holiday::Shavuos => "Shavuos",
            Holiday::SeventeenthOfTammuz => "Seventeenth of Tammuz",
            Holiday::TishahBav => "Tishah B'Av",
            Holiday::TuBav => "Tu B'Av",
            Holiday::ErevRoshHashana => "Erev Rosh Hashana",
            Holiday::RoshHashana => "Rosh Hashana",
            Holiday::FastOfGedalyah => "Fast of Gedalyah",
            Holiday::ErevYomKippur => "Erev Yom Kippur",
            Holiday::YomKippur => "Yom Kippur",
            Holiday::ErevSuccos => "Erev Succos",
            Holiday::Succos => "Succos",
            Holiday::CholHamoedSuccos => "Chol Hamoed Succos",
            Holiday::HoshanaRabbah => "Hoshana Rabbah",
            Holiday::SheminiAtzeres => "Shemini Atzeres",
            Holiday::SimchasTorah => "Simchas Torah",
            Holiday::ErevChanukah => "Erev Chanukah",
            Holiday::Chanukah => "Chanukah",
            Holiday::TenthOfTeves => "Tenth of Teves",
            Holiday::TuBshvat => "Tu B'Shvat",
            Holiday::FastOfEsther => "Fast of Esther",
            Holiday::Purim => "Purim",
            Holiday::ShushanPurim => "Shushan Purim",
            Holiday::PurimKatan => "Purim Katan",
            Holiday::RoshChodesh => "Rosh Chodesh",
            Holiday::YomHaShoah => "Yom HaShoah",
            Holiday::YomHazikaron => "Yom Hazikaron",
            Holiday::YomHaatzmaut => "Yom Ha'atzmaut",
            Holiday::YomYerushalayim => "Yom Yerushalayim",
            Holiday::LagBomer => "Lag B'Omer",
            Holiday::ShushanPurimKatan => "Shushan Purim Katan",
            Holiday::IsruChag => "Isru Chag",
            Holiday::YomKippurKatan => "Yom Kippur Katan",
            Holiday::Behab => "Behab",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            Holiday::ErevPesach => "ערב פסח",
            Holiday::Pesach => "פסח",
            Holiday::CholHamoedPesach => "חול המועד פסח",
            Holiday::PesachSheni => "פסח שני",
            Holiday::ErevShavuos => "ערב שבועות",
            Holiday::Shavuos => "שבועות",
            Holiday::SeventeenthOfTammuz => "שבעה עשר בתמוז",
            Holiday::TishahBav => "תשעה באב",
            Holiday::TuBav => "ט״ו באב",
            Holiday::ErevRoshHashana => "ערב ראש השנה",
            Holiday::RoshHashana => "ראש השנה",
            Holiday::FastOfGedalyah => "צום גדליה",
            Holiday::ErevYomKippur => "ערב יום כיפור",
            Holiday::YomKippur => "יום כיפור",
            Holiday::ErevSuccos => "ערב סוכות",
            Holiday::Succos => "סוכות",
            Holiday::CholHamoedSuccos => "חול המועד סוכות",
            Holiday::HoshanaRabbah => "הושענא רבה",
            Holiday::SheminiAtzeres => "שמיני עצרת",
            Holiday::SimchasTorah => "שמחת תורה",
            Holiday::ErevChanukah => "ערב חנוכה",
            Holiday::Chanukah => "חנוכה",
            Holiday::TenthOfTeves => "עשרה בטבת",
            Holiday::TuBshvat => "ט״ו בשבט",
            Holiday::FastOfEsther => "תענית אסתר",
            Holiday::Purim => "פורים",
            Holiday::ShushanPurim => "שושן פורים",
            Holiday::PurimKatan => "פורים קטן",
            Holiday::RoshChodesh => "ראש חודש",
            Holiday::YomHaShoah => "יום השואה",
            Holiday::YomHazikaron => "יום הזיכרון",
            Holiday::YomHaatzmaut => "יום העצמאות",
            Holiday::YomYerushalayim => "יום ירושלים",
            Holiday::LagBomer => "ל״ג בעומר",
            Holiday::ShushanPurimKatan => "שושן פורים קטן",
            Holiday::IsruChag => "אסרו חג",
            Holiday::YomKippurKatan => "יום העצמאות",
            Holiday::Behab => "יום כיפור קטן",
        }
    }
    fn is_assur_bemelacha(&self)->bool{
        matches!(
            self,
            Holiday::Pesach
                | Holiday::Shavuos
                | Holiday::Succos
                | Holiday::SheminiAtzeres
                | Holiday::SimchasTorah
                | Holiday::RoshHashana
                | Holiday::YomKippur
        )
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive, PartialOrd, Ord)]
#[repr(u8)]
pub enum HebrewMonth {
    Nissan = 1,
    Iyar = 2,
    Sivan = 3,
    Tammuz = 4,
    Av = 5,
    Elul = 6,
    Tishrei = 7,
    Cheshvan = 8,
    Kislev = 9,
    Teves = 10,
    Shevat = 11,
    Adar = 12,
    AdarII = 13,
}

impl HebrewMonth {
    pub(crate) fn next(&self, is_leap_year: bool) -> HebrewMonth {
        match self {
            HebrewMonth::Nissan => Self::Iyar,
            HebrewMonth::Iyar => Self::Sivan,
            HebrewMonth::Sivan => Self::Tammuz,
            HebrewMonth::Tammuz => Self::Av,
            HebrewMonth::Av => Self::Elul,
            HebrewMonth::Elul => Self::Tishrei,
            HebrewMonth::Tishrei => Self::Cheshvan,
            HebrewMonth::Cheshvan => Self::Kislev,
            HebrewMonth::Kislev => Self::Teves,
            HebrewMonth::Teves => Self::Shevat,
            HebrewMonth::Shevat => Self::Adar,
            HebrewMonth::Adar => {
                if is_leap_year {
                    Self::AdarII
                } else {
                    Self::Nissan
                }
            }
            HebrewMonth::AdarII => Self::Nissan,
        }
    }

    pub fn en_string(&self, is_leap_year: bool) -> &str {
        match self {
            HebrewMonth::Nissan => "Nissan",
            HebrewMonth::Iyar => "Iyar",
            HebrewMonth::Sivan => "Sivan",
            HebrewMonth::Tammuz => "Tammuz",
            HebrewMonth::Av => "Av",
            HebrewMonth::Elul => "Elul",
            HebrewMonth::Tishrei => "Tishrei",
            HebrewMonth::Cheshvan => "Cheshvan",
            HebrewMonth::Kislev => "Kislev",
            HebrewMonth::Teves => "Teves",
            HebrewMonth::Shevat => "Shevat",
            HebrewMonth::Adar => {
                if is_leap_year {
                    "Adar I"
                } else {
                    "Adar"
                }
            }
            HebrewMonth::AdarII => "Adar II",
        }
    }
    pub fn he_string(&self, is_leap_year: bool) -> &str {
        match self {
            HebrewMonth::Nissan => "ניסן",
            HebrewMonth::Iyar => "אייר",
            HebrewMonth::Sivan => "סיון",
            HebrewMonth::Tammuz => "תמוז",
            HebrewMonth::Av => "אב",
            HebrewMonth::Elul => "אלול",
            HebrewMonth::Tishrei => "תשרי",
            HebrewMonth::Cheshvan => "חשון",
            HebrewMonth::Kislev => "כסלו",
            HebrewMonth::Teves => "טבת",
            HebrewMonth::Shevat => "שבט",
            HebrewMonth::Adar => {
                if is_leap_year {
                    "אדר א"
                } else {
                    "אדר"
                }
            }
            HebrewMonth::AdarII => "אדר ב",
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum YearLengthType {
    Chaserim = 0,
    Kesidran = 1,
    Shelaimim = 2,
}
impl YearLengthType {
    pub fn en_string(&self) -> &str {
        match self {
            YearLengthType::Chaserim => "Chaserim",
            YearLengthType::Kesidran => "Kesidran",
            YearLengthType::Shelaimim => "Shelaimim",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            YearLengthType::Chaserim => "חסרים",
            YearLengthType::Kesidran => "כסדרן",
            YearLengthType::Shelaimim => "שלמים",
        }
    }
}
