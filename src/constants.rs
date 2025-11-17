use core::fmt::Debug;

use chrono::{DateTime, Duration, TimeZone, Utc};
use icu_calendar::{Date, Gregorian};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub static _JULIAN_DAY_JAN_1_2000: f64 = 2451545.0;
pub static _JULIAN_DAYS_PER_CENTURY: f64 = 36525.0;
pub static _EARTH_RADIUS: f64 = 6356.9;
pub static _GEOMETRIC_ZENITH: f64 = 90.0;
pub static _CIVIL_ZENITH: f64 = 96.0;
pub static _NAUTICAL_ZENITH: f64 = 102.0;
pub static _ASTRONOMICAL_ZENITH: f64 = 108.0;
pub static _SOLAR_RADIUS: f64 = 16.0 / 60.0;
pub static _REFRACTION: f64 = 34.0 / 60.0;
pub static _ZENITH_16_POINT_1: f64 = 90.0 + 16.1;
pub static _ZENITH_8_POINT_5: f64 = 90.0 + 8.5;
pub static _ZENITH_3_POINT_7: f64 = 90.0 + 3.7;
pub static _ZENITH_3_POINT_8: f64 = 90.0 + 3.8;
pub static _ZENITH_5_POINT_95: f64 = 90.0 + 5.95;
pub static _ZENITH_7_POINT_083: f64 = 90.0 + 7.0 + (5.0 / 60.0);
pub static _ZENITH_10_POINT_2: f64 = 90.0 + 10.2;
pub static _ZENITH_11_DEGREES: f64 = 90.0 + 11.0;
pub static _ZENITH_11_POINT_5: f64 = 90.0 + 11.5;
pub static _ZENITH_13_POINT_24: f64 = 90.0 + 13.24;
pub static _ZENITH_19_DEGREES: f64 = 90.0 + 19.0;
pub static _ZENITH_19_POINT_8: f64 = 90.0 + 19.8;
pub static _ZENITH_26_DEGREES: f64 = 90.0 + 26.0;
pub static _ZENITH_4_POINT_37: f64 = 90.0 + 4.37;
pub static _ZENITH_4_POINT_61: f64 = 90.0 + 4.61;
pub static _ZENITH_4_POINT_8: f64 = 90.0 + 4.8;
pub static _ZENITH_3_POINT_65: f64 = 90.0 + 3.65;
pub static _ZENITH_3_POINT_676: f64 = 90.0 + 3.676;
pub static _ZENITH_5_POINT_88: f64 = 90.0 + 5.88;
pub static _ZENITH_1_POINT_583: f64 = 90.0 + 1.583;
pub static _ZENITH_16_POINT_9: f64 = 90.0 + 16.9;
pub static _ZENITH_6_DEGREES: f64 = 90.0 + 6.0;
pub static _ZENITH_6_POINT_45: f64 = 90.0 + 6.45;
pub static _ZENITH_7_POINT_65: f64 = 90.0 + 7.65;
pub static _ZENITH_7_POINT_67: f64 = 90.0 + 7.67;
pub static _ZENITH_9_POINT_3: f64 = 90.0 + 9.3;
pub static _ZENITH_9_POINT_5: f64 = 90.0 + 9.5;
pub static _ZENITH_9_POINT_75: f64 = 90.0 + 9.75;
pub static _ZENITH_MINUS_2_POINT_1: f64 = 90.0 - 2.1;
pub static _ZENITH_MINUS_2_POINT_8: f64 = 90.0 - 2.8;
pub static _ZENITH_MINUS_3_POINT_05: f64 = 90.0 - 3.05;
pub static _CHALAKIM_PER_MINUTE: i64 = 18;
pub static _CHALAKIM_PER_HOUR: i64 = 1080;
pub static _CHALAKIM_PER_DAY: i64 = 25920;
pub static _CHALAKIM_PER_MONTH: i64 = 765433;
pub static _CHALAKIM_MOLAD_TOHU: i64 = 31524;
pub static _JEWISH_EPOCH: i64 = -1373429;
pub static _MINUTE_MILLIS: i64 = 60 * 1000;
pub static _HOUR_MILLIS: i64 = 60 * 1000 * 60;
pub static _BAVLI_DAF_YOMI_START_DAY: DateTime<Utc> =
    DateTime::from_timestamp_millis(-1461369600000).unwrap();
pub static _BAVLI_SHEKALIM_CHANGE_DAY: DateTime<Utc> =
    DateTime::from_timestamp_millis(172800000000).unwrap();
pub static _YERUSHALMI_DAF_YOMI_START_DAY: DateTime<Utc> =
    DateTime::from_timestamp_millis(318297600000).unwrap();
pub static _YERUSHALMI_LENGTH: i64 = 1554;
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum _SolarEvent {
    Sunrise = 0,
    Sunset = 1,
    Noon = 2,
    Midnight = 3,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum _Formula {
    Distance = 0,
    InitialBearing = 1,
    FinalBearing = 2,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum JewishHoliday {
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
impl JewishHoliday {
    pub fn en_string(&self) -> &str {
        match self {
            JewishHoliday::ErevPesach => "Erev Pesach",
            JewishHoliday::Pesach => "Pesach",
            JewishHoliday::CholHamoedPesach => "Chol Hamoed Pesach",
            JewishHoliday::PesachSheni => "Pesach Sheni",
            JewishHoliday::ErevShavuos => "Erev Shavuos",
            JewishHoliday::Shavuos => "Shavuos",
            JewishHoliday::SeventeenthOfTammuz => "Seventeenth of Tammuz",
            JewishHoliday::TishahBav => "Tishah B'Av",
            JewishHoliday::TuBav => "Tu B'Av",
            JewishHoliday::ErevRoshHashana => "Erev Rosh Hashana",
            JewishHoliday::RoshHashana => "Rosh Hashana",
            JewishHoliday::FastOfGedalyah => "Fast of Gedalyah",
            JewishHoliday::ErevYomKippur => "Erev Yom Kippur",
            JewishHoliday::YomKippur => "Yom Kippur",
            JewishHoliday::ErevSuccos => "Erev Succos",
            JewishHoliday::Succos => "Succos",
            JewishHoliday::CholHamoedSuccos => "Chol Hamoed Succos",
            JewishHoliday::HoshanaRabbah => "Hoshana Rabbah",
            JewishHoliday::SheminiAtzeres => "Shemini Atzeres",
            JewishHoliday::SimchasTorah => "Simchas Torah",
            JewishHoliday::ErevChanukah => "Erev Chanukah",
            JewishHoliday::Chanukah => "Chanukah",
            JewishHoliday::TenthOfTeves => "Tenth of Teves",
            JewishHoliday::TuBshvat => "Tu B'Shvat",
            JewishHoliday::FastOfEsther => "Fast of Esther",
            JewishHoliday::Purim => "Purim",
            JewishHoliday::ShushanPurim => "Shushan Purim",
            JewishHoliday::PurimKatan => "Purim Katan",
            JewishHoliday::RoshChodesh => "Rosh Chodesh",
            JewishHoliday::YomHaShoah => "Yom HaShoah",
            JewishHoliday::YomHazikaron => "Yom Hazikaron",
            JewishHoliday::YomHaatzmaut => "Yom Ha'atzmaut",
            JewishHoliday::YomYerushalayim => "Yom Yerushalayim",
            JewishHoliday::LagBomer => "Lag B'Omer",
            JewishHoliday::ShushanPurimKatan => "Shushan Purim Katan",
            JewishHoliday::IsruChag => "Isru Chag",
            JewishHoliday::YomKippurKatan => "Yom Kippur Katan",
            JewishHoliday::Behab => "Behab",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            JewishHoliday::ErevPesach => "ערב פסח",
            JewishHoliday::Pesach => "פסח",
            JewishHoliday::CholHamoedPesach => "חול המועד פסח",
            JewishHoliday::PesachSheni => "פסח שני",
            JewishHoliday::ErevShavuos => "ערב שבועות",
            JewishHoliday::Shavuos => "שבועות",
            JewishHoliday::SeventeenthOfTammuz => "שבעה עשר בתמוז",
            JewishHoliday::TishahBav => "תשעה באב",
            JewishHoliday::TuBav => "ט״ו באב",
            JewishHoliday::ErevRoshHashana => "ערב ראש השנה",
            JewishHoliday::RoshHashana => "ראש השנה",
            JewishHoliday::FastOfGedalyah => "צום גדליה",
            JewishHoliday::ErevYomKippur => "ערב יום כיפור",
            JewishHoliday::YomKippur => "יום כיפור",
            JewishHoliday::ErevSuccos => "ערב סוכות",
            JewishHoliday::Succos => "סוכות",
            JewishHoliday::CholHamoedSuccos => "חול המועד סוכות",
            JewishHoliday::HoshanaRabbah => "הושענא רבה",
            JewishHoliday::SheminiAtzeres => "שמיני עצרת",
            JewishHoliday::SimchasTorah => "שמחת תורה",
            JewishHoliday::ErevChanukah => "ערב חנוכה",
            JewishHoliday::Chanukah => "חנוכה",
            JewishHoliday::TenthOfTeves => "עשרה בטבת",
            JewishHoliday::TuBshvat => "ט״ו בשבט",
            JewishHoliday::FastOfEsther => "תענית אסתר",
            JewishHoliday::Purim => "פורים",
            JewishHoliday::ShushanPurim => "שושן פורים",
            JewishHoliday::PurimKatan => "פורים קטן",
            JewishHoliday::RoshChodesh => "ראש חודש",
            JewishHoliday::YomHaShoah => "יום השואה",
            JewishHoliday::YomHazikaron => "יום הזיכרון",
            JewishHoliday::YomHaatzmaut => "יום העצמאות",
            JewishHoliday::YomYerushalayim => "יום ירושלים",
            JewishHoliday::LagBomer => "ל״ג בעומר",
            JewishHoliday::ShushanPurimKatan => "שושן פורים קטן",
            JewishHoliday::IsruChag => "אסרו חג",
            JewishHoliday::YomKippurKatan => "יום העצמאות",
            JewishHoliday::Behab => "יום כיפור קטן",
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum DayOfWeek {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Shabbos = 7,
}
impl DayOfWeek {
    pub fn en_string(&self) -> &str {
        match self {
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
            DayOfWeek::Shabbos => "Shabbos",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            DayOfWeek::Sunday => "ראשון",
            DayOfWeek::Monday => "שני",
            DayOfWeek::Tuesday => "שלישי",
            DayOfWeek::Wednesday => "רביעי",
            DayOfWeek::Thursday => "חמישי",
            DayOfWeek::Friday => "שישי",
            DayOfWeek::Shabbos => "שבת",
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum JewishMonth {
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
impl JewishMonth {
    pub fn en_string(&self, is_leap_year: bool) -> &str {
        match self {
            JewishMonth::Nissan => "Nissan",
            JewishMonth::Iyar => "Iyar",
            JewishMonth::Sivan => "Sivan",
            JewishMonth::Tammuz => "Tammuz",
            JewishMonth::Av => "Av",
            JewishMonth::Elul => "Elul",
            JewishMonth::Tishrei => "Tishrei",
            JewishMonth::Cheshvan => "Cheshvan",
            JewishMonth::Kislev => "Kislev",
            JewishMonth::Teves => "Teves",
            JewishMonth::Shevat => "Shevat",
            JewishMonth::Adar => {
                if is_leap_year {
                    "Adar I"
                } else {
                    "Adar"
                }
            }
            JewishMonth::AdarII => "Adar II",
        }
    }
    pub fn he_string(&self, is_leap_year: bool) -> &str {
        match self {
            JewishMonth::Nissan => "ניסן",
            JewishMonth::Iyar => "אייר",
            JewishMonth::Sivan => "סיון",
            JewishMonth::Tammuz => "תמוז",
            JewishMonth::Av => "אב",
            JewishMonth::Elul => "אלול",
            JewishMonth::Tishrei => "תשרי",
            JewishMonth::Cheshvan => "חשון",
            JewishMonth::Kislev => "כסלו",
            JewishMonth::Teves => "טבת",
            JewishMonth::Shevat => "שבט",
            JewishMonth::Adar => {
                if is_leap_year {
                    "אדר א"
                } else {
                    "אדר"
                }
            }
            JewishMonth::AdarII => "אדר ב",
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum BavliTractate {
    Berachos = 0,
    Shabbos = 1,
    Eruvin = 2,
    Pesachim = 3,
    Shekalim = 4,
    Yoma = 5,
    Sukkah = 6,
    Beitzah = 7,
    RoshHashana = 8,
    Taanis = 9,
    Megillah = 10,
    MoedKatan = 11,
    Chagigah = 12,
    Yevamos = 13,
    Kesubos = 14,
    Nedarim = 15,
    Nazir = 16,
    Sotah = 17,
    Gitin = 18,
    Kiddushin = 19,
    BavaKamma = 20,
    BavaMetzia = 21,
    BavaBasra = 22,
    Sanhedrin = 23,
    Makkos = 24,
    Shevuos = 25,
    AvodahZarah = 26,
    Horiyos = 27,
    Zevachim = 28,
    Menachos = 29,
    Chullin = 30,
    Bechoros = 31,
    Arachin = 32,
    Temurah = 33,
    Kerisos = 34,
    Meilah = 35,
    Kinnim = 36,
    Tamid = 37,
    Midos = 38,
    Niddah = 39,
}
impl BavliTractate {
    pub fn en_string(&self) -> &str {
        match self {
            BavliTractate::Berachos => "Berachos",
            BavliTractate::Shabbos => "Shabbos",
            BavliTractate::Eruvin => "Eruvin",
            BavliTractate::Pesachim => "Pesachim",
            BavliTractate::Shekalim => "Shekalim",
            BavliTractate::Yoma => "Yoma",
            BavliTractate::Sukkah => "Sukkah",
            BavliTractate::Beitzah => "Beitzah",
            BavliTractate::RoshHashana => "Rosh Hashana",
            BavliTractate::Taanis => "Taanis",
            BavliTractate::Megillah => "Megillah",
            BavliTractate::MoedKatan => "Moed Katan",
            BavliTractate::Chagigah => "Chagigah",
            BavliTractate::Yevamos => "Yevamos",
            BavliTractate::Kesubos => "Kesubos",
            BavliTractate::Nedarim => "Nedarim",
            BavliTractate::Nazir => "Nazir",
            BavliTractate::Sotah => "Sotah",
            BavliTractate::Gitin => "Gitin",
            BavliTractate::Kiddushin => "Kiddushin",
            BavliTractate::BavaKamma => "Bava Kamma",
            BavliTractate::BavaMetzia => "Bava Metzia",
            BavliTractate::BavaBasra => "Bava Basra",
            BavliTractate::Sanhedrin => "Sanhedrin",
            BavliTractate::Makkos => "Makkos",
            BavliTractate::Shevuos => "Shevuos",
            BavliTractate::AvodahZarah => "Avodah Zarah",
            BavliTractate::Horiyos => "Horiyos",
            BavliTractate::Zevachim => "Zevachim",
            BavliTractate::Menachos => "Menachos",
            BavliTractate::Chullin => "Chullin",
            BavliTractate::Bechoros => "Bechoros",
            BavliTractate::Arachin => "Arachin",
            BavliTractate::Temurah => "Temurah",
            BavliTractate::Kerisos => "Kerisos",
            BavliTractate::Meilah => "Meilah",
            BavliTractate::Kinnim => "Kinnim",
            BavliTractate::Tamid => "Tamid",
            BavliTractate::Midos => "Midos",
            BavliTractate::Niddah => "Niddah",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            BavliTractate::Berachos => "ברכות",
            BavliTractate::Shabbos => "שבת",
            BavliTractate::Eruvin => "עירובין",
            BavliTractate::Pesachim => "פסחים",
            BavliTractate::Shekalim => "שקלים",
            BavliTractate::Yoma => "יומא",
            BavliTractate::Sukkah => "סוכה",
            BavliTractate::Beitzah => "ביצה",
            BavliTractate::RoshHashana => "ראש השנה",
            BavliTractate::Taanis => "תענית",
            BavliTractate::Megillah => "מגילה",
            BavliTractate::MoedKatan => "מועד קטן",
            BavliTractate::Chagigah => "חגיגה",
            BavliTractate::Yevamos => "יבמות",
            BavliTractate::Kesubos => "כתובות",
            BavliTractate::Nedarim => "נדרים",
            BavliTractate::Nazir => "נזיר",
            BavliTractate::Sotah => "סוטה",
            BavliTractate::Gitin => "גיטין",
            BavliTractate::Kiddushin => "קידושין",
            BavliTractate::BavaKamma => "בבא קמא",
            BavliTractate::BavaMetzia => "בבא מציעא",
            BavliTractate::BavaBasra => "בבא בתרא",
            BavliTractate::Sanhedrin => "סנהדרין",
            BavliTractate::Makkos => "מכות",
            BavliTractate::Shevuos => "שבועות",
            BavliTractate::AvodahZarah => "עבודה זרה",
            BavliTractate::Horiyos => "הוריות",
            BavliTractate::Zevachim => "זבחים",
            BavliTractate::Menachos => "מנחות",
            BavliTractate::Chullin => "חולין",
            BavliTractate::Bechoros => "בכורות",
            BavliTractate::Arachin => "ערכין",
            BavliTractate::Temurah => "תמורה",
            BavliTractate::Kerisos => "כריתות",
            BavliTractate::Meilah => "מעילה",
            BavliTractate::Kinnim => "קינים",
            BavliTractate::Tamid => "תמיד",
            BavliTractate::Midos => "מידות",
            BavliTractate::Niddah => "נדה",
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum YerushalmiTractate {
    Berachos = 0,
    Peah = 1,
    Demai = 2,
    Kilayim = 3,
    Sheviis = 4,
    Terumos = 5,
    Maasros = 6,
    MaaserSheni = 7,
    Chalah = 8,
    Orlah = 9,
    Bikurim = 10,
    Shabbos = 11,
    Eruvin = 12,
    Pesachim = 13,
    Beitzah = 14,
    RoshHashanah = 15,
    Yoma = 16,
    Sukah = 17,
    Taanis = 18,
    Shekalim = 19,
    Megilah = 20,
    Chagigah = 21,
    MoedKatan = 22,
    Yevamos = 23,
    Kesuvos = 24,
    Sotah = 25,
    Nedarim = 26,
    Nazir = 27,
    Gitin = 28,
    Kidushin = 29,
    BavaKama = 30,
    BavaMetzia = 31,
    BavaBasra = 32,
    Shevuos = 33,
    Makos = 34,
    Sanhedrin = 35,
    AvodahZarah = 36,
    Horayos = 37,
    Nidah = 38,
}
impl YerushalmiTractate {
    pub fn en_string(&self) -> &str {
        match self {
            YerushalmiTractate::Berachos => "Berachos",
            YerushalmiTractate::Peah => "Pe'ah",
            YerushalmiTractate::Demai => "Demai",
            YerushalmiTractate::Kilayim => "Kilayim",
            YerushalmiTractate::Sheviis => "Shevi'is",
            YerushalmiTractate::Terumos => "Terumos",
            YerushalmiTractate::Maasros => "Ma'asros",
            YerushalmiTractate::MaaserSheni => "Ma'aser Sheni",
            YerushalmiTractate::Chalah => "Chalah",
            YerushalmiTractate::Orlah => "Orlah",
            YerushalmiTractate::Bikurim => "Bikurim",
            YerushalmiTractate::Shabbos => "Shabbos",
            YerushalmiTractate::Eruvin => "Eruvin",
            YerushalmiTractate::Pesachim => "Pesachim",
            YerushalmiTractate::Beitzah => "Beitzah",
            YerushalmiTractate::RoshHashanah => "Rosh Hashanah",
            YerushalmiTractate::Yoma => "Yoma",
            YerushalmiTractate::Sukah => "Sukah",
            YerushalmiTractate::Taanis => "Ta'anis",
            YerushalmiTractate::Shekalim => "Shekalim",
            YerushalmiTractate::Megilah => "Megilah",
            YerushalmiTractate::Chagigah => "Chagigah",
            YerushalmiTractate::MoedKatan => "Moed Katan",
            YerushalmiTractate::Yevamos => "Yevamos",
            YerushalmiTractate::Kesuvos => "Kesuvos",
            YerushalmiTractate::Sotah => "Sotah",
            YerushalmiTractate::Nedarim => "Nedarim",
            YerushalmiTractate::Nazir => "Nazir",
            YerushalmiTractate::Gitin => "Gitin",
            YerushalmiTractate::Kidushin => "Kidushin",
            YerushalmiTractate::BavaKama => "Bava Kama",
            YerushalmiTractate::BavaMetzia => "Bava Metzia",
            YerushalmiTractate::BavaBasra => "Bava Basra",
            YerushalmiTractate::Shevuos => "Shevuos",
            YerushalmiTractate::Makos => "Makos",
            YerushalmiTractate::Sanhedrin => "Sanhedrin",
            YerushalmiTractate::AvodahZarah => "Avodah Zarah",
            YerushalmiTractate::Horayos => "Horayos",
            YerushalmiTractate::Nidah => "Nidah",
        }
    }
    pub fn he_string(&self) -> &str {
        match self {
            YerushalmiTractate::Berachos => "ברכות",
            YerushalmiTractate::Peah => "פיאה",
            YerushalmiTractate::Demai => "דמאי",
            YerushalmiTractate::Kilayim => "כלאים",
            YerushalmiTractate::Sheviis => "שביעית",
            YerushalmiTractate::Terumos => "תרומות",
            YerushalmiTractate::Maasros => "מעשרות",
            YerushalmiTractate::MaaserSheni => "מעשר שני",
            YerushalmiTractate::Chalah => "חלה",
            YerushalmiTractate::Orlah => "עורלה",
            YerushalmiTractate::Bikurim => "ביכורים",
            YerushalmiTractate::Shabbos => "שבת",
            YerushalmiTractate::Eruvin => "עירובין",
            YerushalmiTractate::Pesachim => "פסחים",
            YerushalmiTractate::Beitzah => "ביצה",
            YerushalmiTractate::RoshHashanah => "ראש השנה",
            YerushalmiTractate::Yoma => "יומא",
            YerushalmiTractate::Sukah => "סוכה",
            YerushalmiTractate::Taanis => "תענית",
            YerushalmiTractate::Shekalim => "שקלים",
            YerushalmiTractate::Megilah => "מגילה",
            YerushalmiTractate::Chagigah => "חגיגה",
            YerushalmiTractate::MoedKatan => "מועד קטן",
            YerushalmiTractate::Yevamos => "יבמות",
            YerushalmiTractate::Kesuvos => "כתובות",
            YerushalmiTractate::Sotah => "סוטה",
            YerushalmiTractate::Nedarim => "נדרים",
            YerushalmiTractate::Nazir => "נזיר",
            YerushalmiTractate::Gitin => "גיטין",
            YerushalmiTractate::Kidushin => "קידושין",
            YerushalmiTractate::BavaKama => "בבא קמא",
            YerushalmiTractate::BavaMetzia => "בבא מציעא",
            YerushalmiTractate::BavaBasra => "בבא בתרא",
            YerushalmiTractate::Shevuos => "שבועות",
            YerushalmiTractate::Makos => "מכות",
            YerushalmiTractate::Sanhedrin => "סנהדרין",
            YerushalmiTractate::AvodahZarah => "עבודה זרה",
            YerushalmiTractate::Horayos => "הוריות",
            YerushalmiTractate::Nidah => "נידה",
        }
    }
}

pub trait GeoLocationTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_latitude(&self) -> f64;
    fn get_longitude(&self) -> f64;
    fn get_elevation(&self) -> f64;
    fn get_rhumb_line_distance(&self, location: &impl GeoLocationTrait) -> f64;
    fn get_rhumb_line_bearing(&self, location: &impl GeoLocationTrait) -> f64;
    fn get_geodesic_initial_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64>;
    fn get_geodesic_final_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64>;
    fn get_geodesic_distance(&self, location: &impl GeoLocationTrait) -> Option<f64>;
    fn get_local_mean_time_offset<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i64;
    fn get_antimeridian_adjustment<Tz: TimeZone>(&self, date: &DateTime<Tz>) -> i64;
}

pub trait BavliDafTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_tractate(&self) -> BavliTractate;

    fn get_daf_index(&self) -> i64;
}

pub trait YerushalmiDafTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_tractate(&self) -> YerushalmiTractate;

    fn get_daf_index(&self) -> i64;
}

pub trait NOAACalculatorTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_utc_noon<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64;

    fn get_utc_midnight<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64;

    fn get_utc_sunrise<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64>;

    fn get_utc_sunset<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64>;

    fn get_solar_elevation<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64;

    fn get_solar_azimuth<Tz: TimeZone>(
        &self,
        date_time: &DateTime<Tz>,
        geo_location: &impl GeoLocationTrait,
    ) -> f64;
}
pub trait MoladDataTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_hours(&self) -> i64;
    fn get_minutes(&self) -> i64;
    fn get_chalakim(&self) -> i64;
}

pub trait JewishDateTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn from_hebrew_date(year: i64, month: JewishMonth, day: i64) -> Option<Self>;
    fn from_gregorian_date(year: i64, month: u8, day: u8) -> Option<Self>;
    fn get_gregorian_date(&self) -> Date<Gregorian>;

    fn _get_molad(&self) -> Option<(impl JewishDateTrait, impl MoladDataTrait)>;
    fn get_jewish_year(&self) -> i64;

    fn get_jewish_month(&self) -> JewishMonth;

    fn get_jewish_day_of_month(&self) -> i64;

    fn get_gregorian_year(&self) -> i64;

    fn get_gregorian_month(&self) -> i64;

    fn get_gregorian_day_of_month(&self) -> i64;

    fn get_day_of_week(&self) -> DayOfWeek;

    fn is_jewish_leap_year(&self) -> bool;

    fn get_days_in_jewish_year(&self) -> i64;

    fn get_days_in_jewish_month(&self) -> i64;

    fn is_cheshvan_long(&self) -> bool;

    fn is_kislev_short(&self) -> bool;

    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;

    fn get_days_since_start_of_jewish_year(&self) -> i64;

    fn get_chalakim_since_molad_tohu(&self) -> i64;

    fn get_molad_as_date(&self) -> Option<impl JewishDateTrait>;

    fn get_molad(&self) -> Option<impl MoladDataTrait>;
    fn get_jewish_calendar_elapsed_days(year: i64) -> i64;
    fn get_days_in_jewish_year_static(year: i64) -> i64;
    fn get_last_day_of_gregorian_month(month: i64, year: i64) -> i64;
}

pub trait AstronomicalCalendarTrait<Tz: TimeZone>: Sized + Clone {
    fn get_date_time(&self) -> &DateTime<Tz>;
    fn get_geo_location(&self) -> &impl GeoLocationTrait;
    fn get_noaa_calculator(&self) -> &impl NOAACalculatorTrait;

    fn get_sunrise(&self) -> Option<DateTime<Tz>>;

    fn get_sea_level_sunrise(&self) -> Option<DateTime<Tz>>;

    fn get_begin_civil_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_begin_nautical_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_begin_astronomical_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_sunset(&self) -> Option<DateTime<Tz>>;

    fn get_sea_level_sunset(&self) -> Option<DateTime<Tz>>;

    fn get_end_civil_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_end_nautical_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_end_astronomical_twilight(&self) -> Option<DateTime<Tz>>;

    fn get_sunrise_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>>;
    fn get_sunset_offset_by_degrees(&self, offset_zenith: f64) -> Option<DateTime<Tz>>;

    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64>;

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64>;

    fn get_utc_sunset(&self, zenith: f64) -> Option<f64>;

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64>;

    fn get_temporal_hour(&self) -> Option<Duration>;
    fn get_temporal_hour_from_times(
        &self,
        start_of_day: &DateTime<Tz>,
        end_of_day: &DateTime<Tz>,
    ) -> Option<Duration>;

    fn get_sun_transit(&self) -> Option<DateTime<Tz>>;

    fn get_solar_midnight(&self) -> Option<DateTime<Tz>>;

    fn get_sun_transit_from_times(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_date_from_time(
        &self,
        calculated_time: f64,
        solar_event: _SolarEvent,
    ) -> Option<DateTime<Tz>>;
}

pub trait TefilaRulesTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn is_tachanun_recited_shacharis(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_tachanun_recited_mincha(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_hallel_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_hallel_shalem_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_al_hanissim_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_yaaleh_veyavo_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_mizmor_lesoda_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_vesein_tal_umatar_start_date(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_vesein_tal_umatar_starting_tonight(
        &self,
        jewish_calendar: &impl JewishCalendarTrait,
    ) -> bool;

    fn is_vesein_tal_umatar_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_vesein_beracha_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_mashiv_haruach_start_date(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_mashiv_haruach_end_date(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_mashiv_haruach_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;

    fn is_morid_hatal_recited(&self, jewish_calendar: &impl JewishCalendarTrait) -> bool;
}

pub trait JewishCalendarTrait: Debug + Clone + PartialEq + PartialOrd + Send + Sync {
    fn get_jewish_date(&self) -> &impl JewishDateTrait;
    fn get_in_israel(&self) -> bool;
    fn get_is_mukaf_choma(&self) -> bool;
    fn get_is_use_modern_holidays(&self) -> bool;
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
    fn get_day_of_chanukah(&self) -> i64;
    fn is_chanukah(&self) -> bool;

    fn is_purim(&self) -> bool;

    fn get_day_of_omer(&self) -> i64;

    fn is_tisha_beav(&self) -> bool;

    fn get_parshah(&self) -> Option<Parsha>;

    fn get_daf_yomi_bavli(&self) -> Option<impl BavliDafTrait>;

    fn get_daf_yomi_yerushalmi(&self) -> Option<impl YerushalmiDafTrait>;

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
    fn is_mashiv_haruach_recited(&self) -> bool;
    fn is_morid_hatal_recited(&self) -> bool;
}

pub trait ZmanimCalendarTrait<Tz: TimeZone>: Clone + Sized {
    fn get_astronomical_calendar(&self) -> &impl AstronomicalCalendarTrait<Tz>;
    fn get_use_astronomical_chatzos(&self) -> bool;
    fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool;
    fn get_candle_lighting_offset(&self) -> Duration;

    fn get_tzais(&self) -> Option<DateTime<Tz>>;

    fn get_alos_hashachar(&self) -> Option<DateTime<Tz>>;
    fn get_alos_72(&self) -> Option<DateTime<Tz>>;
    fn get_chatzos(&self) -> Option<DateTime<Tz>>;
    fn get_chatzos_as_half_day(&self) -> Option<DateTime<Tz>>;

    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64>;

    fn get_half_day_based_zman(
        &self,
        start_of_half_day: DateTime<Tz>,
        end_of_half_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;
    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: &DateTime<Tz>,
        end_of_half_day: &DateTime<Tz>,
    ) -> Option<Duration>;
    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
        hours: f64,
    ) -> Option<DateTime<Tz>>;

    fn _get_sof_zman_shma(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: Option<DateTime<Tz>>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_shma(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_shma_gra(&self) -> Option<DateTime<Tz>>;
    fn get_sof_zman_shma_mga(&self) -> Option<DateTime<Tz>>;

    fn get_tzais_72(&self) -> Option<DateTime<Tz>>;

    fn get_candle_lighting(&self) -> Option<DateTime<Tz>>;

    fn get_sof_zman_tfila(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_sof_zman_tfila_gra(&self) -> Option<DateTime<Tz>>;

    fn get_sof_zman_tfila_mga(&self) -> Option<DateTime<Tz>>;

    fn _get_mincha_gedola(
        &self,
        start_of_day: Option<DateTime<Tz>>,
        end_of_day: DateTime<Tz>,
        synchronous: bool,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_gedola(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;
    fn get_mincha_gedola_default(&self) -> Option<DateTime<Tz>>;

    fn get_samuch_le_mincha_ketana(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_ketana(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_mincha_ketana_default(&self) -> Option<DateTime<Tz>>;

    fn get_plag_hamincha(
        &self,
        start_of_day: DateTime<Tz>,
        end_of_day: DateTime<Tz>,
    ) -> Option<DateTime<Tz>>;

    fn get_plag_hamincha_default(&self) -> Option<DateTime<Tz>>;

    fn get_shaah_zmanis_gra(&self) -> Option<Duration>;

    fn get_shaah_zmanis_mga(&self) -> Option<Duration>;
}
