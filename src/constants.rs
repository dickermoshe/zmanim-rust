use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Julian Day for the J2000.0 epoch (2000-01-01 12:00 TT); common reference
pub static _JULIAN_DAY_JAN_1_2000: f64 = 2451545.0;
/// Number of days in a Julian century (100 × 365.25); used for time scales
pub static _JULIAN_DAYS_PER_CENTURY: f64 = 36525.0;
/// Earth radius in kilometers (approx. polar radius; WGS84 ≈ 6356.752 km)
pub static _EARTH_RADIUS: f64 = 6356.9;
/// 90° below the vertical. Used as a basis for most calculations since the location of the sun is 90° below the horizon at sunrise and sunset. Note: it is important to note that for sunrise and sunset the adjusted zenith is required to account for the radius of the sun and refraction. The adjusted zenith should not be used for calculations above or below 90° since they are usually calculated as an offset to 90°.
pub static _GEOMETRIC_ZENITH: f64 = 90.0;
/// Sun's zenith at civil twilight (96°)
pub static _CIVIL_ZENITH: f64 = 96.0;
/// Sun's zenith at nautical twilight (102°)
pub static _NAUTICAL_ZENITH: f64 = 102.0;
/// Sun's zenith at astronomical twilight (108°)
pub static _ASTRONOMICAL_ZENITH: f64 = 108.0;
/// Constant for milliseconds in a minute (60,000)
pub static _MINUTE_MILLIS: i64 = 60 * 1000;
/// Constant for milliseconds in an hour (3,600,000)
pub static _HOUR_MILLIS: i64 = 60 * 1000 * 60;
/// Sun’s apparent angular radius in degrees (~16 arcminutes ≈ 0.2667°)
pub static _SOLAR_RADIUS: f64 = 16.0 / 60.0;
/// Standard atmospheric refraction at the horizon in degrees (~34 arcminutes ≈ 0.5667° at sea level)
pub static _REFRACTION: f64 = 34.0 / 60.0;
///
pub static _ZENITH_16_POINT_1: f64 = 90.0 + 16.1;
///
pub static _ZENITH_8_POINT_5: f64 = 90.0 + 8.5;
///
pub static _ZENITH_3_POINT_7: f64 = 90.0 + 3.7;
///
pub static _ZENITH_3_POINT_8: f64 = 90.0 + 3.8;
///
pub static _ZENITH_5_POINT_95: f64 = 90.0 + 5.95;
///
pub static _ZENITH_7_POINT_083: f64 = 90.0 + 7.0 + (5.0 / 60.0);
///
pub static _ZENITH_10_POINT_2: f64 = 90.0 + 10.2;
///
pub static _ZENITH_11_DEGREES: f64 = 90.0 + 11.0;
///
pub static _ZENITH_11_POINT_5: f64 = 90.0 + 11.5;
///
pub static _ZENITH_13_POINT_24: f64 = 90.0 + 13.24;
///
pub static _ZENITH_19_DEGREES: f64 = 90.0 + 19.0;
///
pub static _ZENITH_19_POINT_8: f64 = 90.0 + 19.8;
///
pub static _ZENITH_26_DEGREES: f64 = 90.0 + 26.0;
///
pub static _ZENITH_4_POINT_37: f64 = 90.0 + 4.37;
///
pub static _ZENITH_4_POINT_61: f64 = 90.0 + 4.61;
///
pub static _ZENITH_4_POINT_8: f64 = 90.0 + 4.8;
///
pub static _ZENITH_3_POINT_65: f64 = 90.0 + 3.65;
///
pub static _ZENITH_3_POINT_676: f64 = 90.0 + 3.676;
///
pub static _ZENITH_5_POINT_88: f64 = 90.0 + 5.88;
///
pub static _ZENITH_1_POINT_583: f64 = 90.0 + 1.583;
///
pub static _ZENITH_16_POINT_9: f64 = 90.0 + 16.9;
///
pub static _ZENITH_6_DEGREES: f64 = 90.0 + 6.0;
///
pub static _ZENITH_6_POINT_45: f64 = 90.0 + 6.45;
///
pub static _ZENITH_7_POINT_65: f64 = 90.0 + 7.65;
///
pub static _ZENITH_7_POINT_67: f64 = 90.0 + 7.67;
///
pub static _ZENITH_9_POINT_3: f64 = 90.0 + 9.3;
///
pub static _ZENITH_9_POINT_5: f64 = 90.0 + 9.5;
///
pub static _ZENITH_9_POINT_75: f64 = 90.0 + 9.75;
///
pub static _ZENITH_MINUS_2_POINT_1: f64 = 90.0 - 2.1;
///
pub static _ZENITH_MINUS_2_POINT_8: f64 = 90.0 - 2.8;
///
pub static _ZENITH_MINUS_3_POINT_05: f64 = 90.0 - 3.05;
///
pub static _CHALAKIM_PER_MINUTE: i64 = 18;
///
pub static _CHALAKIM_PER_HOUR: i64 = 1080;
///
pub static _CHALAKIM_PER_DAY: i64 = 25920;
///
pub static _CHALAKIM_PER_MONTH: i64 = 765433;
///
pub static _CHALAKIM_MOLAD_TOHU: i64 = 31524;
///
pub static _JEWISH_EPOCH: i64 = -1373429;
///
pub static _BAVLI_DAF_YOMI_START_DAY: i64 = -1461369600000;
///
pub static _BAVLI_SHEKALIM_CHANGE_DAY: i64 = 172800000000;
///
pub static _YERUSHALMI_DAF_YOMI_START_DAY: i64 = 318297600000;
///
pub static _YERUSHALMI_LENGTH: i64 = 1554;

/// An enum to indicate what type of solar event is being calculated.
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum _SolarEvent {
    /// A solar event related to sunrise
    Sunrise,

    /// A solar event related to sunset
    Sunset,

    /// A solar event related to noon
    Noon,

    /// A solar event related to midnight
    Midnight,
}
/// Constant for geodesic calculation type.
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum _Formula {
    /// Constant for a distance type calculation.
    Distance,

    /// Constant for an initial bearing type calculation.
    InitialBearing,

    /// Constant for a final bearing type calculation.
    FinalBearing,
}
/// An enum representing the days of the week.
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum DayOfWeek {
    /// Sunday (value: 1)
    Sunday = 1,

    /// Monday (value: 2)
    Monday = 2,

    /// Tuesday (value: 3)
    Tuesday = 3,

    /// Wednesday (value: 4)
    Wednesday = 4,

    /// Thursday (value: 5)
    Thursday = 5,

    /// Friday (value: 6)
    Friday = 6,

    /// Saturday (value: 7)
    Saturday = 7,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum YearLengthType {
    ///
    Chaserim = 0,

    ///
    Kesidran = 1,

    ///
    Shelaimim = 2,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum JewishMonth {
    ///
    Nissan = 1,

    ///
    Iyar = 2,

    ///
    Sivan = 3,

    ///
    Tammuz = 4,

    ///
    Av = 5,

    ///
    Elul = 6,

    ///
    Tishrei = 7,

    ///
    Cheshvan = 8,

    ///
    Kislev = 9,

    ///
    Teves = 10,

    ///
    Shevat = 11,

    ///
    Adar = 12,

    ///
    Adarii = 13,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum JewishHoliday {
    ///
    ErevPesach = 0,

    ///
    Pesach = 1,

    ///
    CholHamoedPesach = 2,

    ///
    PesachSheni = 3,

    ///
    ErevShavuos = 4,

    ///
    Shavuos = 5,

    ///
    SeventeenOfTammuz = 6,

    ///
    TishaBeav = 7,

    ///
    TuBeav = 8,

    ///
    ErevRoshHashana = 9,

    ///
    RoshHashana = 10,

    ///
    FastOfGedalyah = 11,

    ///
    ErevYomKippur = 12,

    ///
    YomKippur = 13,

    ///
    ErevSuccos = 14,

    ///
    Succos = 15,

    ///
    CholHamoedSuccos = 16,

    ///
    HoshanaRabba = 17,

    ///
    SheminiAtzeres = 18,

    ///
    SimchasTorah = 19,

    ///
    Chanukah = 21,

    ///
    TenthOfTeves = 22,

    ///
    TuBeshvat = 23,

    ///
    FastOfEsther = 24,

    ///
    Purim = 25,

    ///
    ShushanPurim = 26,

    ///
    PurimKatan = 27,

    ///
    RoshChodesh = 28,

    ///
    YomHashoah = 29,

    ///
    YomHazikaron = 30,

    ///
    YomHaatzmaut = 31,

    ///
    YomYerushalayim = 32,

    ///
    LagBaomer = 33,

    ///
    ShushanPurimKatan = 34,

    ///
    IsruChag = 35,

    ///
    YomKippurKatan = 36,

    ///
    Behab = 37,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum BavliTractate {
    ///
    Berachos = 0,

    ///
    Shabbos = 1,

    ///
    Eruvin = 2,

    ///
    Pesachim = 3,

    ///
    Shekalim = 4,

    ///
    Yoma = 5,

    ///
    Sukkah = 6,

    ///
    Beitzah = 7,

    ///
    RoshHashana = 8,

    ///
    Taanis = 9,

    ///
    Megillah = 10,

    ///
    MoedKatan = 11,

    ///
    Chagigah = 12,

    ///
    Yevamos = 13,

    ///
    Kesubos = 14,

    ///
    Nedarim = 15,

    ///
    Nazir = 16,

    ///
    Sotah = 17,

    ///
    Gitin = 18,

    ///
    Kiddushin = 19,

    ///
    BavaKamma = 20,

    ///
    BavaMetzia = 21,

    ///
    BavaBasra = 22,

    ///
    Sanhedrin = 23,

    ///
    Makkos = 24,

    ///
    Shevuos = 25,

    ///
    AvodahZarah = 26,

    ///
    Horiyos = 27,

    ///
    Zevachim = 28,

    ///
    Menachos = 29,

    ///
    Chullin = 30,

    ///
    Bechoros = 31,

    ///
    Arachin = 32,

    ///
    Temurah = 33,

    ///
    Kerisos = 34,

    ///
    Meilah = 35,

    ///
    Kinnim = 36,

    ///
    Tamid = 37,

    ///
    Midos = 38,

    ///
    Niddah = 39,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum YerushalmiTractate {
    ///
    Berachos = 0,

    ///
    Peah = 1,

    ///
    Demai = 2,

    ///
    Kilayim = 3,

    ///
    Sheviis = 4,

    ///
    Terumos = 5,

    ///
    Maasros = 6,

    ///
    MaaserSheni = 7,

    ///
    Chalah = 8,

    ///
    Orlah = 9,

    ///
    Bikurim = 10,

    ///
    Shabbos = 11,

    ///
    Eruvin = 12,

    ///
    Pesachim = 13,

    ///
    Beitzah = 14,

    ///
    RoshHashanah = 15,

    ///
    Yoma = 16,

    ///
    Sukah = 17,

    ///
    Taanis = 18,

    ///
    Shekalim = 19,

    ///
    Megilah = 20,

    ///
    Chagigah = 21,

    ///
    MoedKatan = 22,

    ///
    Yevamos = 23,

    ///
    Kesuvos = 24,

    ///
    Sotah = 25,

    ///
    Nedarim = 26,

    ///
    Nazir = 27,

    ///
    Gitin = 28,

    ///
    Kidushin = 29,

    ///
    BavaKama = 30,

    ///
    BavaMetzia = 31,

    ///
    BavaBasra = 32,

    ///
    Shevuos = 33,

    ///
    Makos = 34,

    ///
    Sanhedrin = 35,

    ///
    AvodahZarah = 36,

    ///
    Horayos = 37,

    ///
    Nidah = 38,
}
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum Parsha {
    ///
    Bereshis = 0,

    ///
    Noach = 1,

    ///
    LechLecha = 2,

    ///
    Vayera = 3,

    ///
    ChayeiSara = 4,

    ///
    Toldos = 5,

    ///
    Vayetzei = 6,

    ///
    Vayishlach = 7,

    ///
    Vayeshev = 8,

    ///
    Miketz = 9,

    ///
    Vayigash = 10,

    ///
    Vayechi = 11,

    ///
    Shemos = 12,

    ///
    Vaera = 13,

    ///
    Bo = 14,

    ///
    Beshalach = 15,

    ///
    Yisro = 16,

    ///
    Mishpatim = 17,

    ///
    Terumah = 18,

    ///
    Tetzaveh = 19,

    ///
    KiSisa = 20,

    ///
    Vayakhel = 21,

    ///
    Pekudei = 22,

    ///
    Vayikra = 23,

    ///
    Tzav = 24,

    ///
    Shmini = 25,

    ///
    Tazria = 26,

    ///
    Metzora = 27,

    ///
    AchreiMos = 28,

    ///
    Kedoshim = 29,

    ///
    Emor = 30,

    ///
    Behar = 31,

    ///
    Bechukosai = 32,

    ///
    Bamidbar = 33,

    ///
    Nasso = 34,

    ///
    Behaaloscha = 35,

    ///
    Shlach = 36,

    ///
    Korach = 37,

    ///
    Chukas = 38,

    ///
    Balak = 39,

    ///
    Pinchas = 40,

    ///
    Matos = 41,

    ///
    Masei = 42,

    ///
    Devarim = 43,

    ///
    Vaeschanan = 44,

    ///
    Eikev = 45,

    ///
    Reeh = 46,

    ///
    Shoftim = 47,

    ///
    KiSeitzei = 48,

    ///
    KiSavo = 49,

    ///
    Nitzavim = 50,

    ///
    Vayeilech = 51,

    ///
    Haazinu = 52,

    ///
    VzosHaberacha = 53,

    ///
    VayakhelPekudei = 54,

    ///
    TazriaMetzora = 55,

    ///
    AchreiMosKedoshim = 56,

    ///
    BeharBechukosai = 57,

    ///
    ChukasBalak = 58,

    ///
    MatosMasei = 59,

    ///
    NitzavimVayeilech = 60,

    ///
    Shkalim = 61,

    ///
    Zachor = 62,

    ///
    Para = 63,

    ///
    Hachodesh = 64,

    ///
    Shuva = 65,

    ///
    Shira = 66,

    ///
    Hagadol = 67,

    ///
    Chazon = 68,

    ///
    Nachamu = 69,
}

// /// A class that contains location information such as latitude and longitude required for astronomical calculations. The elevation field may not be used by some calculation engines and would be ignored if set. Check the documentation for specific implementations of the `AstronomicalCalculator` to see if elevation is calculated as part of the algorithm.
// ///
// pub trait GeoLocationTrait {
//     /// Returns the [rhumb line](https://en.wikipedia.org/wiki/Rhumb_line) distance from the current location to the GeoLocation passed in.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): the destination location
//     ///
//     /// **Returns:**
//     /// - `distance` (float64): the distance in Meters
//     fn get_rhumb_line_distance(&self, location: &impl GeoLocationTrait) -> f64;

//     /// Returns the [rhumb line](https://en.wikipedia.org/wiki/Rhumb_line) bearing from the current location to the GeoLocation passed in.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): destination location
//     ///
//     /// **Returns:**
//     /// - `bearing` (float64): the bearing in degrees
//     fn get_rhumb_line_bearing(&self, location: &impl GeoLocationTrait) -> f64;

//     /// Computes distance or bearings between two points on the ellipsoid using the Vincenty inverse formula. Returns NaN if the algorithm fails to converge.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): destination location
//     /// - `formula` (_Formula): which result to return (distance in meters, initial bearing in degrees, or final bearing in degrees)
//     ///
//     /// **Returns:**
//     /// - `result` (float64): distance in meters for `distance`, initial bearing in degrees for `initialBearing`, or final bearing in degrees for `finalBearing`; NaN on non-convergence
//     fn _vincenty_inverse_formula(
//         &self,
//         location: &impl GeoLocationTrait,
//         formula: _Formula,
//     ) -> Option<f64>;

//     /// Calculate the initial [geodesic](https://en.wikipedia.org/wiki/Great_circle) bearing using Vincenty's inverse formula.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): the destination location
//     ///
//     /// **Returns:**
//     /// - `bearing` (float64): the initial bearing in degrees
//     fn get_geodesic_initial_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64>;

//     /// Calculate the final [geodesic](https://en.wikipedia.org/wiki/Great_circle) bearing using Vincenty's inverse formula.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): the destination location
//     ///
//     /// **Returns:**
//     /// - `bearing` (float64): the final bearing in degrees
//     fn get_geodesic_final_bearing(&self, location: &impl GeoLocationTrait) -> Option<f64>;

//     /// Calculate [geodesic distance](https://en.wikipedia.org/wiki/Great-circle_distance) in meters using Vincenty's inverse formula.
//     ///
//     /// **Parameters:**
//     /// - `location` (GeoLocation): the destination location
//     ///
//     /// **Returns:**
//     /// - `distance` (float64): the geodesic distance in meters
//     fn get_geodesic_distance(&self, location: &impl GeoLocationTrait) -> Option<f64>;

//     /// Returns the longitude of the GeoLocation in degrees.
//     fn get_longitude(&self) -> f64;

//     /// Returns the latitude of the GeoLocation in degrees.
//     fn get_latitude(&self) -> f64;

//     /// Returns the elevation of the GeoLocation in meters.
//     fn get_elevation(&self) -> f64;
// }
// /// A calendar that calculates astronomical times such as **sunrise**, **sunset** and twilight times. **Note:** There are times when the algorithms can't calculate proper values for sunrise, sunset and twilight. This is usually caused by trying to calculate times for areas either very far North or South, where sunrise / sunset never happen on that date. This is common when calculating twilight with a deep dip below the horizon for locations as far south of the North Pole as London, in the northern hemisphere. The sun never reaches this dip at certain times of the year. When the calculations encounter this condition, `null` will be returned when a date/time value is expected and a minimum value when a numeric value is expected. The reason that exceptions are not thrown in these cases is because the lack of a rise/set or twilight is not an exception, but an expected condition in many parts of the world.
// ///
// ///
// pub trait AstronomicalCalendarTrait {
//     /// Returns the milliseconds since the Unix epoch (January 1, 1970 UTC).
//     fn get_milliseconds_since_epoch(&self) -> i64;

//     /// A method that returns the currently set GeoLocation which contains location information used for the astronomical calculations.
//     ///
//     /// **Returns:**
//     /// - `geoLocation` (GeoLocation): the GeoLocation used for calculations
//     fn get_geo_location(&self) -> impl GeoLocationTrait;

//     /// A method that returns the sunset in [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) without correction for time zone offset from GMT and without using daylight savings time.
//     ///
//     /// **Parameters:**
//     /// - `zenith` (float64): the degrees below the horizon. For time before sunset use negative numbers.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The time in the format: 18.75 for 18:45:00 UTC/GMT. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_utc_sunset(&self, zenith: f64) -> Option<f64>;

//     /// A method that returns the sunrise in [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) without correction for time zone offset from GMT and without using daylight savings time.
//     ///
//     /// **Parameters:**
//     /// - `zenith` (float64): the degrees below the horizon. For time after sunrise use negative numbers.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The time in the format: 18.75 for 18:45:00 UTC/GMT. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_utc_sunrise(&self, zenith: f64) -> Option<f64>;

//     /// A method that returns the sunrise in [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) without correction for time zone offset from GMT and without using daylight savings time. Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light, something that is not affected by elevation. This method returns UTC sunrise calculated at sea level. This forms the base for dawn calculations that are calculated as a dip below the horizon before sunrise.
//     ///
//     /// **Parameters:**
//     /// - `zenith` (float64): the degrees below the horizon. For time after sunrise use negative numbers.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The time in the format: 18.75 for 18:45:00 UTC/GMT. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64>;

//     /// A method that returns the sunset in [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) without correction for elevation, time zone offset from GMT and without using daylight savings time. Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light, something that is not affected by elevation. This method returns UTC sunset calculated at sea level. This forms the base for dusk calculations that are calculated as a dip below the horizon after sunset.
//     ///
//     /// **Parameters:**
//     /// - `zenith` (float64): the degrees below the horizon. For time before sunset use negative numbers.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The time in the format: 18.75 for 18:45:00 UTC/GMT. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64>;

//     /// A method that returns the sunset without elevation adjustment. Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light, something that is not affected by elevation. This method returns sunset calculated at sea level. This forms the base for dusk calculations that are calculated as a dip below the horizon after sunset.
//     ///
//     /// **Returns:**
//     /// - `sunset` (int64?): the exact sea-level sunset time in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sea_level_sunset(&self) -> Option<i64>;

//     /// The getSunset method returns an elevation adjusted sunset time. The zenith used for the calculation uses geometric zenith of 90&deg; plus elevation adjustment. This is adjusted by the AstronomicalCalculator to add approximately 50/60 of a degree to account for 34 archminutes of refraction and 16 archminutes for the sun's radius for a total of 90.83333&deg;. See documentation for the specific implementation of the AstronomicalCalculator that you are using. Note: In certain cases the calculated sunset will occur before sunrise. This will typically happen when a timezone other than the local timezone is used (calculating Los Angeles sunset using a GMT timezone for example). In this case the sunset date will be incremented to the following date.
//     ///
//     /// **Returns:**
//     /// - `sunset` (int64?): the exact sunset time in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sunset(&self) -> Option<i64>;

//     /// The getSunrise method returns an elevation adjusted sunrise time. The zenith used for the calculation uses geometric zenith of 90&deg; plus elevation adjustment. This is adjusted by the AstronomicalCalculator to add approximately 50/60 of a degree to account for 34 archminutes of refraction and 16 archminutes for the sun's radius for a total of 90.83333&deg;. See documentation for the specific implementation of the AstronomicalCalculator that you are using.
//     ///
//     /// **Returns:**
//     /// - `sunrise` (int64?): the exact sunrise time in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sunrise(&self) -> Option<i64>;

//     /// A method that returns the sunrise without elevation adjustment. Non-sunrise and sunset calculations such as dawn and dusk, depend on the amount of visible light, something that is not affected by elevation. This method returns sunrise calculated at sea level. This forms the base for dawn calculations that are calculated as a dip below the horizon before sunrise.
//     ///
//     /// **Returns:**
//     /// - `sunrise` (int64?): the exact sea-level sunrise time in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sea_level_sunrise(&self) -> Option<i64>;

//     /// A utility method that returns the time of an offset by degrees below or above the horizon of sunrise. Note that the degree offset is from the vertical, so for a calculation of 14&deg; before sunrise, an offset of 14 + GEOMETRIC_ZENITH = 104 would have to be passed as a parameter.
//     ///
//     /// **Parameters:**
//     /// - `degrees` (float64): the degrees before sunrise to use in the calculation. For time after sunrise use negative numbers. Note that the degree offset is from the vertical, so for a calculation of 14&deg; before sunrise, an offset of 14 + GEOMETRIC_ZENITH = 104 would have to be passed as a parameter.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The offset after (or before) sunrise in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sunrise_offset_by_degrees(&self, degrees: f64) -> Option<i64>;

//     /// A utility method that returns the time of an offset by degrees below or above the horizon of sunset. Note that the degree offset is from the vertical, so for a calculation of 14&deg; after sunset, an offset of 14 + GEOMETRIC_ZENITH = 104 would have to be passed as a parameter.
//     ///
//     /// **Parameters:**
//     /// - `degrees` (float64): the degrees after sunset to use in the calculation. For time before sunset use negative numbers. Note that the degree offset is from the vertical, so for a calculation of 14&deg; after sunset, an offset of 14 + GEOMETRIC_ZENITH = 104 would have to be passed as a parameter.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The offset after (or before) sunset in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sunset_offset_by_degrees(&self, degrees: f64) -> Option<i64>;

//     /// A method that returns the beginning of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) (dawn) using a zenith of 96&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The beginning of civil twilight using a zenith of 96&deg; in milliseconds since Unix epoch. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_begin_civil_twilight(&self) -> Option<i64>;

//     /// A method that returns the beginning of [nautical twilight](https://en.wikipedia.org/wiki/Twilight#Nautical_twilight) using a zenith of 102&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The beginning of nautical twilight using a zenith of 102&deg; in milliseconds since Unix epoch. If the calculation can't be computed `null` will be returned. See detailed explanation on top of the page.
//     fn get_begin_nautical_twilight(&self) -> Option<i64>;

//     /// A method that returns the beginning of [astronomical twilight](https://en.wikipedia.org/wiki/Twilight#Astronomical_twilight) using a zenith of 108&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The beginning of astronomical twilight using a zenith of 108&deg; in milliseconds since Unix epoch. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_begin_astronomical_twilight(&self) -> Option<i64>;

//     /// A method that returns the end of [civil twilight](https://en.wikipedia.org/wiki/Twilight#Civil_twilight) using a zenith of 96&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The end of civil twilight using a zenith of 96&deg; in milliseconds since Unix epoch. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_end_civil_twilight(&self) -> Option<i64>;

//     /// A method that returns the end of [nautical twilight](https://en.wikipedia.org/wiki/Twilight#Nautical_twilight) using a zenith of 102&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): The end of nautical twilight using a zenith of 102&deg; in milliseconds since Unix epoch. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_end_nautical_twilight(&self) -> Option<i64>;

//     /// A method that returns the end of [astronomical twilight](https://en.wikipedia.org/wiki/Twilight#Astronomical_twilight) using a zenith of 108&deg;.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): the end of astronomical twilight using a zenith of 108&deg; in milliseconds since Unix epoch. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_end_astronomical_twilight(&self) -> Option<i64>;

//     /// A method that returns a sea-level based temporal (solar) hour. The day from sea-level sunrise to sea-level sunset is split into 12 equal parts with each one being a temporal hour.
//     ///
//     /// **Returns:**
//     /// - `temporalHour` (int64?): the millisecond length of a temporal hour. If the calculation can't be computed, `null` will be returned. See detailed explanation on top of the page.
//     fn get_temporal_hour(&self) -> Option<i64>;

//     /// A method that returns sundial or [solar noon](https://en.wikipedia.org/wiki/Noon#Solar_noon). It occurs when the Sun is [transiting](https://en.wikipedia.org/wiki/Transit_%28astronomy%29) the [celestial meridian](https://en.wikipedia.org/wiki/Meridian_%28astronomy%29). The calculations used by this class depend on the AstronomicalCalculator used. If this calendar instance is set to use the NOAACalculator (the default) it will calculate astronomical noon. If the calendar instance is to use the SunTimesCalculator, that does not have code to calculate astronomical noon, the sun transit is calculated as halfway between sea level sunrise and sea level sunset, which can be slightly off the real transit time due to changes in declination (the lengthening or shortening day).
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): Sun's transit in milliseconds since Unix epoch. If the calculation can't be computed such as when using the USNO calculator that does not support getting solar noon for the Arctic Circle (where there is at least one day a year where the sun does not rise, and one where it does not set), `null` will be returned. See detailed explanation on top of the page.
//     fn get_sun_transit(&self) -> Option<i64>;

//     /// A method that returns [solar midnight](https://en.wikipedia.org/wiki/Midnight) at the end of the current day (that may actually be after midnight of the day it is being calculated for). It occurs when the Sun is [transiting](https://en.wikipedia.org/wiki/Transit_%28astronomy%29) the lower [celestial meridian](https://en.wikipedia.org/wiki/Meridian_%28astronomy%29), or when the sun is at it's [nadir](https://en.wikipedia.org/wiki/Nadir). The calculations used by this class depend on the AstronomicalCalculator used. If this calendar instance is set to use the NOAACalculator (the default) it will calculate astronomical midnight. If the calendar instance is to use the SunTimesCalculator USNO Calculator, that does not have code to calculate astronomical noon, midnight is calculated as 12 hours after halfway between sea level sunrise and sea level sunset of that day. This can be slightly off the real transit time due to changes in declination (the lengthening or shortening day).
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): Sun's lower transit at the end of the current day in milliseconds since Unix epoch. If the calculation can't be computed such as when using the USNO calculator that does not support getting solar noon or midnight for the Arctic Circle (where there is at least one day a year where the sun does not rise, and one where it does not set), `null` will be returned. This is not relevant when using the NOAA Calculator that is never expected to return `null`. See the detailed explanation on top of the page.
//     fn get_solar_midnight(&self) -> Option<i64>;

//     /// A utility method that will allow the calculation of a temporal (solar) hour based on the sunrise and sunset passed as parameters to this method. An example of the use of this method would be the calculation of an elevation adjusted temporal hour by passing in sunrise and sunset as parameters.
//     ///
//     /// **Parameters:**
//     /// - `startTime` (int64): The start of the day in milliseconds since Unix epoch.
//     /// - `endTime` (int64): The end of the day in milliseconds since Unix epoch.
//     ///
//     /// **Returns:**
//     /// - `temporalHour` (int64?): the millisecond length of the temporal hour. If the calculation can't be computed `null` will be returned. See detailed explanation on top of the page.
//     fn get_temporal_hour_with_start_and_end_times(
//         &self,
//         start_time: i64,
//         end_time: i64,
//     ) -> Option<i64>;

//     /// A method that returns sundial or [solar noon](https://en.wikipedia.org/wiki/Noon#Solar_noon). It occurs when the Sun is [transiting](https://en.wikipedia.org/wiki/Transit_%28astronomy%29) the [celestial meridian](https://en.wikipedia.org/wiki/Meridian_%28astronomy%29). In this class it is calculated as halfway between the sunrise and sunset passed to this method. This time can be slightly off the real transit time due to changes in declination (the lengthening or shortening day).
//     ///
//     /// **Parameters:**
//     /// - `startTime` (int64): the start of day for calculating the sun's transit in milliseconds since Unix epoch. This can be sea level sunrise, visual sunrise (or any arbitrary start of day) passed to this method.
//     /// - `endTime` (int64): the end of day for calculating the sun's transit in milliseconds since Unix epoch. This can be sea level sunset, visual sunset (or any arbitrary end of day) passed to this method.
//     ///
//     /// **Returns:**
//     /// - `time` (int64?): Sun's transit in milliseconds since Unix epoch. If the calculation can't be computed such as in the Arctic Circle where there is at least one day a year where the sun does not rise, and one where it does not set, `null` will be returned. See detailed explanation on top of the page.
//     fn get_sun_transit_with_start_and_end_times(
//         &self,
//         start_time: i64,
//         end_time: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_local_mean_time(&self, hours: f64, tz_offset_in_milliseconds: i64) -> Option<i64>;
// }
// /// /**
// ///  * Implementation of sunrise and sunset methods to calculate astronomical times based on the <a
// ///  * href="https://noaa.gov">NOAA</a> algorithm. This calculator uses the Java algorithm based on the implementation by <a
// ///  * href="https://noaa.gov">NOAA - National Oceanic and Atmospheric Administration</a>'s <a href =
// ///  * "https://www.srrb.noaa.gov/highlights/sunrise/sunrise.html">Surface Radiation Research Branch</a>. NOAA's <a
// ///  * href="https://www.srrb.noaa.gov/highlights/sunrise/solareqns.PDF">implementation</a> is based on equations from <a
// ///  * href="https://www.amazon.com/Astronomical-Table-Sun-Moon-Planets/dp/1942675038/">Astronomical Algorithms</a> by <a
// ///  * href="https://en.wikipedia.org/wiki/Jean_Meeus">Jean Meeus</a>. Added to the algorithm is an adjustment of the zenith
// ///  * to account for elevation. The algorithm can be found in the <a
// ///  * href="https://en.wikipedia.org/wiki/Sunrise_equation">Wikipedia Sunrise Equation</a> article.
// ///  *
// ///  * @author &copy; Eliyahu Hershfeld 2011 - 2025
// ///  */
// pub trait NOAACalculatorTrait {
//     /// Method to return the adjustment to the zenith required to account for the elevation.
//     ///
//     /// Since a person at a higher elevation can see farther below the horizon, the calculation for sunrise / sunset is calculated below the horizon used at sea level.
//     ///
//     /// This is only used for sunrise and sunset and not times before or after it such as `getBeginNauticalTwilight()` nautical twilight since those calculations are based on the level of available light at the given dip below the horizon, something that is not affected by elevation, the adjustment should only be made if the zenith == 90° adjusted for refraction and solar radius.
//     ///
//     /// The algorithm used is:
//     ///
//     /// > elevationAdjustment = Math.toDegrees(Math.acos(earthRadiusInMeters / (earthRadiusInMeters + elevationMeters)));
//     ///
//     /// The source of this algorithm is [Calendrical Calculations](https://www.cs.tau.ac.il/~nachum/calendar-book/index.shtml) by Edward M. Reingold and Nachum Dershowitz.
//     ///
//     /// An alternate algorithm that produces similar (but not completely accurate) result found in Ma'aglay Tzedek by Moishe Kosower and other sources is:
//     ///
//     /// > elevationAdjustment = 0.0347 * Math.sqrt(elevationMeters);
//     ///
//     /// **Parameters:**
//     /// - `elevationMeters` (float64): elevation in Meters.
//     ///
//     /// **Returns:**
//     /// - `elevationAdjustment` (float64): the adjusted zenith
//     ///
//     fn _get_elevation_adjustment(&self, elevation_meters: f64) -> f64;

//     /// Calculates the Julian Day number from a UTC timestamp.
//     ///
//     /// The Julian Day is a continuous count of days since the beginning of the Julian Period on January 1, 4713 BCE (proleptic Julian calendar).
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): milliseconds since Unix epoch (January 1, 1970 UTC)
//     ///
//     /// **Returns:**
//     /// - `julianDay` (float64): the Julian Day number
//     ///
//     fn _get_julian_day(&self, milliseconds_since_epoch: i64) -> Option<f64>;

//     /// Adjusts the zenith of astronomical sunrise and sunset to account for solar refraction, solar radius and elevation. The value for Sun's zenith and true rise/set Zenith (used in this class and subclasses) is the angle that the center of the Sun makes to a line perpendicular to the Earth's surface. If the Sun were a point and the Earth were without an atmosphere, true sunset and sunrise would correspond to a 90&deg; zenith. Because the Sun is not a point, and because the atmosphere refracts light, this 90&deg; zenith does not, in fact, correspond to true sunset or sunrise, instead the center of the Sun's disk must lie just below the horizon for the upper edge to be obscured. This means that a zenith of just above 90&deg; must be used. The Sun subtends an angle of 16 minutes of arc (this can be changed via the `setSolarRadius` method , and atmospheric refraction accounts for 34 minutes or so (this can be changed via the `setRefraction` method), giving a total of 50 arcminutes. The total value for ZENITH is 90+(5/6) or 90.8333333&deg; for true sunrise/sunset. Since a person at an elevation can see below the horizon of a person at sea level, this will also adjust the zenith to account for elevation if available. Note that this will only adjust the value if the zenith is exactly 90 degrees. For values below and above this no correction is done. As an example, astronomical twilight is when the sun is 18&deg; below the horizon or `ASTRONOMICAL_ZENITH` (108&deg; below the zenith). This is traditionally calculated with none of the above mentioned adjustments. The same goes for various <em>tzais</em> and <em>alos</em> times such as the `ZENITH_16_POINT_1` (16.1&deg;) dip used in `getAlos16Point1Degrees()`.
//     ///
//     /// **Parameters:**
//     /// - `zenith` (float64): the azimuth below the vertical zenith of 90&deg;. For sunset typically the `adjustZenith` zenith used for the calculation uses geometric zenith of 90&deg; and `adjustZenith` adjusts this slightly to account for solar refraction and the sun's radius. Another example would be `getEndNauticalTwilight()` that passes `NAUTICAL_ZENITH` to this method.
//     /// - `elevation` (float64): elevation in Meters.
//     ///
//     /// **Returns:**
//     /// - `adjustedZenith` (float64): The zenith adjusted to include the `getSolarRadius` sun's radius, `getRefraction` refraction and `getElevationAdjustment` elevation adjustment. This will only be adjusted for sunrise and sunset (if the zenith == 90&deg;)
//     ///
//     /// **See also:**
//     /// - `getElevationAdjustment`
//     ///
//     fn _adjust_zenith(&self, zenith: f64, elevation: f64) -> f64;

//     /// Convert [Julian day](https://en.wikipedia.org/wiki/Julian_day) to centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000).
//     ///
//     /// **Parameters:**
//     /// - `julianDay` (float64): the Julian Day to convert
//     ///
//     /// **Returns:**
//     /// - `julianCenturies` (float64): the centuries since 2000 Julian corresponding to the Julian Day
//     ///
//     fn _get_julian_centuries_from_julian_day(&self, julian_day: f64) -> f64;

//     /// Returns the Geometric [Mean Longitude](https://en.wikipedia.org/wiki/Mean_longitude) of the Sun.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `longitude` (float64): the Geometric Mean Longitude of the Sun in degrees
//     ///
//     fn _get_sun_geometric_mean_longitude(&self, julian_centuries: f64) -> f64;

//     /// Returns the Geometric [Mean Anomaly](https://en.wikipedia.org/wiki/Mean_anomaly) of the Sun in degrees.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `anomaly` (float64): the Geometric Mean Anomaly of the Sun in degrees
//     ///
//     fn _get_sun_geometric_mean_anomaly(&self, julian_centuries: f64) -> f64;

//     /// Return the unitless [eccentricity of earth's orbit](https://en.wikipedia.org/wiki/Eccentricity_%28orbit%29).
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `eccentricity` (float64): the unitless eccentricity
//     ///
//     fn _get_earth_orbit_eccentricity(&self, julian_centuries: f64) -> f64;

//     /// Returns the [equation of center](https://en.wikipedia.org/wiki/Equation_of_the_center) for the sun in degrees.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `center` (float64): the equation of center for the sun in degrees
//     ///
//     fn _get_sun_equation_of_center(&self, julian_centuries: f64) -> f64;

//     /// Return the [true longitude](https://en.wikipedia.org/wiki/True_longitude) of the sun.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `longitude` (float64): the sun's true longitude in degrees
//     ///
//     fn _get_sun_true_longitude(&self, julian_centuries: f64) -> f64;

//     /// Return the [apparent longitude](https://en.wikipedia.org/wiki/Apparent_longitude) of the sun.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `longitude` (float64): sun's apparent longitude in degrees
//     ///
//     fn _get_sun_apparent_longitude(&self, julian_centuries: f64) -> f64;

//     /// Returns the mean [obliquity of the ecliptic](https://en.wikipedia.org/wiki/Axial_tilt) (Axial tilt).
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `obliquity` (float64): the mean obliquity in degrees
//     ///
//     fn _get_mean_obliquity_of_ecliptic(&self, julian_centuries: f64) -> f64;

//     /// Returns the corrected [obliquity of the ecliptic](https://en.wikipedia.org/wiki/Axial_tilt) (Axial tilt).
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `obliquity` (float64): the corrected obliquity in degrees
//     ///
//     fn _get_obliquity_correction(&self, julian_centuries: f64) -> f64;

//     /// Return the [declination](https://en.wikipedia.org/wiki/Declination) of the sun.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `declination` (float64): the sun's declination in degrees
//     ///
//     fn _get_sun_declination(&self, julian_centuries: f64) -> f64;

//     /// Return the [Equation of Time](https://en.wikipedia.org/wiki/Equation_of_time) - the difference between true solar time and mean solar time.
//     ///
//     /// **Parameters:**
//     /// - `julianCenturies` (float64): the number of Julian centuries since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     ///
//     /// **Returns:**
//     /// - `equationOfTime` (float64): equation of time in minutes of time
//     ///
//     fn _get_equation_of_time(&self, julian_centuries: f64) -> f64;

//     /// Return the [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) of sunrise or sunset in minutes for the given day at the given location on earth.
//     ///
//     /// **Note:** Possibly increase the number of passes for improved accuracy, especially in the Arctic areas.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `latitude` (float64): The latitude of observer in degrees
//     /// - `longitude` (float64): Longitude of observer in degrees
//     /// - `zenith` (float64): Zenith
//     /// - `solarEvent` (SolarEvent): If the calculation is for `SolarEvent.sunrise` or `SolarEvent.sunset`
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): the time in minutes from zero Universal Coordinated Time (UTC), or null if calculation cannot be performed
//     ///
//     fn _get_sun_rise_set_utc(
//         &self,
//         milliseconds_since_epoch: i64,
//         latitude: f64,
//         longitude: f64,
//         zenith: f64,
//         solar_event: _SolarEvent,
//     ) -> Option<f64>;

//     /// Return the [hour angle](https://en.wikipedia.org/wiki/Hour_angle) of the sun in [radians](https://en.wikipedia.org/wiki/Radian) for the latitude.
//     ///
//     /// **Parameters:**
//     /// - `latitude` (float64): the latitude of observer in degrees
//     /// - `solarDeclination` (float64): the declination angle of sun in degrees
//     /// - `zenith` (float64): the zenith
//     /// - `solarEvent` (SolarEvent): If the hour angle is for `SolarEvent.sunrise` or `SolarEvent.sunset`
//     ///
//     /// **Returns:**
//     /// - `hourAngle` (float64): hour angle of sunrise or sunset in [radians](https://en.wikipedia.org/wiki/Radian)
//     ///
//     fn _get_sun_hour_angle(
//         &self,
//         latitude: f64,
//         solar_declination: f64,
//         zenith: f64,
//         solar_event: _SolarEvent,
//     ) -> f64;

//     /// Return the [Universal Coordinated Time](https://en.wikipedia.org/wiki/Universal_Coordinated_Time) (UTC) of the current day [solar noon](http://en.wikipedia.org/wiki/Noon#Solar_noon) or the upcoming midnight (about 12 hours after solar noon) of the given day at the given location on earth.
//     ///
//     /// **Parameters:**
//     /// - `julianDay` (float64): The Julian day since [J2000.0](https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000)
//     /// - `longitude` (float64): The longitude of observer in degrees
//     /// - `solarEvent` (SolarEvent): If the calculation is for `SolarEvent.noon` or `SolarEvent.midnight`
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64): the time in minutes from zero UTC
//     ///
//     fn _get_solar_noon_midnight_utc(
//         &self,
//         julian_day: f64,
//         longitude: f64,
//         solar_event: _SolarEvent,
//     ) -> f64;

//     /// Return the [Solar Elevation](https://en.wikipedia.org/wiki/Celestial_coordinate_system) or [Solar Azimuth](https://en.wikipedia.org/wiki/Celestial_coordinate_system) at the given location and time. Can be negative if the sun is below the horizon. Elevation is based on sea-level and is not adjusted for altitude.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): time of calculation, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location for calculating the elevation or azimuth
//     /// - `isAzimuth` (bool): `true` for azimuth, `false` for elevation
//     ///
//     /// **Returns:**
//     /// - `value` (float64?): solar elevation or azimuth in degrees, or null if calculation cannot be performed
//     ///
//     fn _get_solar_elevation_azimuth(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//         is_azimuth: bool,
//     ) -> Option<f64>;

//     /// Return [solar noon](https://en.wikipedia.org/wiki/Noon#Solar_noon) (UTC) for the given day at the given location on earth. The `NOAACalculator` implementation calculates true solar noon, while the `SunTimesCalculator` approximates it, calculating the time as halfway between sunrise and sunset.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): Used to calculate day of year, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information used for astronomical calculating sun times.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The UTC time of solar noon in 24-hour format. 12:30:00 PM will return 12.5. Returns null if calculation cannot be performed.
//     ///
//     fn get_utc_noon(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//     ) -> Option<f64>;

//     /// Return [solar midnight](https://en.wikipedia.org/wiki/Midnight) (UTC) for the given day at the given location on earth. The `NOAACalculator` implementation calculates true solar midnight, while the `SunTimesCalculator` approximates it, calculating the time as 12 hours after halfway between sunrise and sunset.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): Used to calculate day of year, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information used for astronomical calculating sun times.
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The UTC time of solar midnight in 24-hour format. 12:30:00 AM will return 0.5. Returns null if calculation cannot be performed.
//     ///
//     ///
//     fn get_utc_midnight(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//     ) -> Option<f64>;

//     /// A method that calculates UTC sunrise as well as any time based on an angle above or below sunrise. This abstract method is implemented by the classes that extend this class.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): Used to calculate day of year, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information used for astronomical calculating sun times.
//     /// - `zenith` (float64): the azimuth below the vertical zenith of 90 degrees. For sunrise typically the `adjustZenith` zenith used for the calculation uses geometric zenith of 90&deg; and `adjustZenith` adjusts this slightly to account for solar refraction and the sun's radius. Another example would be `getBeginNauticalTwilight()` that passes `NAUTICAL_ZENITH` to this method.
//     /// - `adjustForElevation` (bool): Should the time be adjusted for elevation
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The UTC time of sunrise in 24-hour format. 5:45:00 AM will return 5.75. If an error was encountered in the calculation (expected behavior for some locations such as near the poles), null will be returned.
//     ///
//     /// **See also:**
//     /// - `getElevationAdjustment`
//     ///
//     fn get_utc_sunrise(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//         zenith: f64,
//         adjust_for_elevation: bool,
//     ) -> Option<f64>;

//     /// A method that calculates UTC sunset as well as any time based on an angle above or below sunset. This abstract method is implemented by the classes that extend this class.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): Used to calculate day of year, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information used for astronomical calculating sun times.
//     /// - `zenith` (float64): the azimuth below the vertical zenith of 90&deg;. For sunset typically the `adjustZenith` zenith used for the calculation uses geometric zenith of 90&deg; and `adjustZenith` adjusts this slightly to account for solar refraction and the sun's radius. Another example would be `getEndNauticalTwilight()` that passes `NAUTICAL_ZENITH` to this method.
//     /// - `adjustForElevation` (bool): Should the time be adjusted for elevation
//     ///
//     /// **Returns:**
//     /// - `timeUtc` (float64?): The UTC time of sunset in 24-hour format. 5:45:00 AM will return 5.75. If an error was encountered in the calculation (expected behavior for some locations such as near the poles), null will be returned.
//     ///
//     /// **See also:**
//     /// - `getElevationAdjustment`
//     ///
//     fn get_utc_sunset(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//         zenith: f64,
//         adjust_for_elevation: bool,
//     ) -> Option<f64>;

//     /// Return the [Solar Elevation](https://en.wikipedia.org/wiki/Celestial_coordinate_system) for the horizontal coordinate system at the given location at the given time. Can be negative if the sun is below the horizon. Not corrected for altitude.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): time of calculation, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information
//     ///
//     /// **Returns:**
//     /// - `elevation` (float64?): solar elevation in degrees. The horizon (calculated in a vacuum using the solar radius as the point) is 0&deg;, civil twilight is -6&deg; etc. This means that sunrise and sunset that do use refraction and are calculated from the upper limb of the sun will return about 0.833&deg;. Returns null if calculation cannot be performed.
//     ///
//     fn get_solar_elevation(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//     ) -> Option<f64>;

//     /// Return the [Solar Azimuth](https://en.wikipedia.org/wiki/Celestial_coordinate_system) for the horizontal coordinate system at the given location at the given time. Not corrected for altitude. True south is 180 degrees.
//     ///
//     /// **Parameters:**
//     /// - `millisecondsSinceEpoch` (int64): time of calculation, milliseconds since Unix epoch (January 1, 1970 UTC)
//     /// - `geoLocation` (GeoLocation): The location information
//     ///
//     /// **Returns:**
//     /// - `azimuth` (float64?): the solar azimuth in degrees. Astronomical midday would be 180 in the northern hemisphere and 0 in the southern hemisphere. Depending on the location and time of year, sunrise will have an azimuth of about 90&deg; and sunset about 270&deg;. Returns null if calculation cannot be performed.
//     ///
//     fn get_solar_azimuth(
//         &self,
//         milliseconds_since_epoch: i64,
//         geo_location: &impl GeoLocationTrait,
//     ) -> Option<f64>;
// }
// ///
// pub trait ZmanimCalendarTrait {
//     ///
//     fn get_tzais(&self) -> Option<i64>;

//     ///
//     fn get_alos_hashachar(&self) -> Option<i64>;

//     ///
//     fn get_alos_72(&self) -> Option<i64>;

//     ///
//     fn get_chatzos(&self) -> Option<i64>;

//     ///
//     fn get_chatzos_as_half_day(&self) -> Option<i64>;

//     ///
//     fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64>;

//     ///
//     fn get_half_day_based_zman(
//         &self,
//         start_of_half_day: i64,
//         end_of_half_day: i64,
//         hours: f64,
//     ) -> Option<i64>;

//     ///
//     fn get_half_day_based_shaah_zmanis(
//         &self,
//         start_of_half_day: i64,
//         end_of_half_day: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_based_zman(
//         &self,
//         start_of_day: i64,
//         end_of_day: i64,
//         hours: f64,
//     ) -> Option<i64>;

//     ///
//     fn _get_sof_zman_shma(
//         &self,
//         start_of_day: i64,
//         end_of_day: Option<i64>,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_gra(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga(&self) -> Option<i64>;

//     ///
//     fn get_tzais_72(&self) -> Option<i64>;

//     ///
//     fn get_candle_lighting(&self) -> Option<i64>;

//     ///
//     fn _get_sof_zman_tfila(
//         &self,
//         start_of_day: i64,
//         end_of_day: Option<i64>,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_gra(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga(&self) -> Option<i64>;

//     ///
//     fn _get_mincha_gedola(
//         &self,
//         start_of_day: Option<i64>,
//         end_of_day: i64,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_default(&self) -> Option<i64>;

//     ///
//     fn _get_samuch_le_mincha_ketana(
//         &self,
//         start_of_day: Option<i64>,
//         end_of_day: i64,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_samuch_le_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64)
//     -> Option<i64>;

//     ///
//     fn _get_mincha_ketana(
//         &self,
//         start_of_day: Option<i64>,
//         end_of_day: i64,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_default(&self) -> Option<i64>;

//     ///
//     fn _get_plag_hamincha(
//         &self,
//         start_of_day: Option<i64>,
//         end_of_day: i64,
//         synchronous: bool,
//     ) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_default(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_gra(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_mga(&self) -> Option<i64>;
// }
// ///
// pub trait ComplexZmanimCalendarTrait {
//     ///
//     fn get_shaah_zmanis_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_26_degrees(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_60_minutes(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_90_minutes(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_90_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_96_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_96_minutes(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_120_minutes(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_120_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_shaah_zmanis_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_alos_60(&self) -> Option<i64>;

//     ///
//     fn get_alos_72_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_alos_96(&self) -> Option<i64>;

//     ///
//     fn get_alos_90_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_alos_96_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_alos_90(&self) -> Option<i64>;

//     ///
//     fn get_alos_120(&self) -> Option<i64>;

//     ///
//     fn get_alos_120_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_alos_26_degrees(&self) -> Option<i64>;

//     ///
//     fn get_alos_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_alos_19_degrees(&self) -> Option<i64>;

//     ///
//     fn get_alos_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_alos_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_alos_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_misheyakir_11_point_5_degrees(&self) -> Option<i64>;

//     ///
//     fn get_misheyakir_11_degrees(&self) -> Option<i64>;

//     ///
//     fn get_misheyakir_10_point_2_degrees(&self) -> Option<i64>;

//     ///
//     fn get_misheyakir_7_point_65_degrees(&self) -> Option<i64>;

//     ///
//     fn get_misheyakir_9_point_5_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_90_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_90_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_96_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_96_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_3_hours_before_chatzos(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_120_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_alos_16_point_1_to_sunset(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_kol_eliyahu(&self, tz_offset_in_milliseconds: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_90_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_90_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_96_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_96_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_mga_120_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_2_hours_before_chatzos(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_30_minutes(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_ahavat_shalom(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_greater_than_30(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_baal_hatanya_greater_than_30(&self) -> Option<i64>;

//     ///
//     fn get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_ahavat_shalom(&self) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_60_minutes(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_90_minutes(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_96_minutes(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_96_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_90_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_26_degrees(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_plag_alos_to_sunset(&self) -> Option<i64>;

//     ///
//     fn get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;

//     ///
//     fn get_plag_ahavat_shalom(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_120_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_120_minutes(&self) -> Option<i64>;

//     ///
//     fn get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(
//         &self,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_rt_13_point_24_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_rt_58_point_5_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_rt_2_stars(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_18_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_3_point_05_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_16_point_875_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_2_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_13_point_5_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hashmashos_yereim_2_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_3_point_7_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_3_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_5_point_95_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_3_point_65_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_3_point_676_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_4_point_61_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_4_point_37_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_5_point_88_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_4_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_6_point_45_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_7_point_67_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_8_point_5_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_9_point_3_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_geonim_9_point_75_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_60(&self) -> Option<i64>;

//     ///
//     fn get_tzais_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_tzais_72_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_tzais_90_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_tzais_96_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_tzais_90(&self) -> Option<i64>;

//     ///
//     fn get_tzais_120(&self) -> Option<i64>;

//     ///
//     fn get_tzais_120_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_tzais_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_26_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_18_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_19_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_tzais_96(&self) -> Option<i64>;

//     ///
//     fn get_tzais_50(&self) -> Option<i64>;

//     ///
//     fn get_tzais_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_fixed_local_chatzos(&self, tz_offset_in_milliseconds: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_shma_fixed_local(&self, tz_offset_in_milliseconds: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfila_fixed_local(&self, tz_offset_in_milliseconds: i64) -> Option<i64>;

//     ///
//     fn get_sof_zman_kidush_levana_between_moldos(
//         &self,
//         alos: Option<i64>,
//         tzais: Option<i64>,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_kidush_levana_between_moldos_default(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_kidush_levana_15_days(
//         &self,
//         alos: Option<i64>,
//         tzais: Option<i64>,
//     ) -> Option<i64>;

//     ///
//     fn get_sof_zman_kidush_levana_15_days_default(&self) -> Option<i64>;

//     ///
//     fn get_tchilas_zman_kidush_levana_3_days(&self) -> Option<i64>;

//     ///
//     fn get_tchilas_zman_kidush_levana_3_days_with_times(
//         &self,
//         alos: Option<i64>,
//         tzais: Option<i64>,
//     ) -> Option<i64>;

//     ///
//     fn get_zman_molad(&self) -> Option<i64>;

//     ///
//     fn get_tchilas_zman_kidush_levana_7_days(
//         &self,
//         alos: Option<i64>,
//         tzais: Option<i64>,
//     ) -> Option<i64>;

//     ///
//     fn get_tchilas_zman_kidush_levana_7_days_default(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_achilas_chametz_gra(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_achilas_chametz_mga_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_achilas_chametz_mga_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_biur_chametz_gra(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_biur_chametz_mga_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_biur_chametz_mga_72_minutes_zmanis(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_biur_chametz_mga_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_achilas_chametz_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_biur_chametz_baal_hatanya(&self) -> Option<i64>;

//     ///
//     fn get_samuch_le_mincha_ketana_gra(&self) -> Option<i64>;

//     ///
//     fn get_samuch_le_mincha_ketana_16_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_samuch_le_mincha_ketana_72_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosrt_13_point_24_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosrt_58_point_5_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosrt_2_stars(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_18_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_3_point_05_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_16_point_875_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_2_point_8_degrees(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_13_point_5_minutes(&self) -> Option<i64>;

//     ///
//     fn get_bain_hasmashosyereim_2_point_1_degrees(&self) -> Option<i64>;

//     ///
//     fn get_sof_zman_tfilah_ateret_torah(&self) -> Option<i64>;

//     ///
//     fn get_fixed_local_chatzos_based_zmanim(
//         &self,
//         start_of_half_day: i64,
//         end_of_half_day: i64,
//         hours: f64,
//     ) -> Option<i64>;

//     ///
//     fn _get_molad_based_time(
//         &self,
//         molad_based_time: i64,
//         alos: Option<i64>,
//         tzais: Option<i64>,
//         techila: bool,
//         tz_offset_in_milliseconds: i64,
//     ) -> Option<i64>;
// }
// ///
// pub trait JewishDateTrait {
//     ///
//     fn get_jewish_year(&self) -> i64;

//     ///
//     fn get_jewish_month(&self) -> JewishMonth;

//     ///
//     fn get_jewish_day_of_month(&self) -> i64;

//     ///
//     fn get_gregorian_year(&self) -> i64;

//     ///
//     fn get_gregorian_month(&self) -> i64;

//     ///
//     fn get_gregorian_day_of_month(&self) -> i64;

//     ///
//     fn get_day_of_week(&self) -> DayOfWeek;

//     ///
//     fn is_jewish_leap_year(&self) -> bool;

//     ///
//     fn get_days_in_jewish_year(&self) -> i64;

//     ///
//     fn get_days_in_jewish_month(&self) -> i64;

//     ///
//     fn is_cheshvan_long(&self) -> bool;

//     ///
//     fn is_kislev_short(&self) -> bool;

//     ///
//     fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;

//     ///
//     fn get_days_since_start_of_jewish_year(&self) -> i64;

//     ///
//     fn get_chalakim_since_molad_tohu(&self) -> i64;

//     ///
//     fn get_molad_as_date(&self) -> Option<impl JewishDateTrait>;

//     ///
//     fn get_molad(&self) -> Option<impl MoladDataTrait>;
// }
// ///
// pub trait MoladDataTrait {
//     ///
//     fn get_hours(&self) -> i64;

//     ///
//     fn get_minutes(&self) -> i64;

//     ///
//     fn get_chalakim(&self) -> i64;
// }
// ///
// pub trait BavliDafTrait {
//     ///
//     fn get_tractate(&self) -> BavliTractate;

//     ///
//     fn get_daf_index(&self) -> i64;
// }
// ///
// pub trait YerushalmiDafTrait {
//     ///
//     fn get_tractate(&self) -> YerushalmiTractate;

//     ///
//     fn get_daf_index(&self) -> i64;
// }
// ///
// pub trait JewishCalendarTrait {
//     ///
//     fn get_yom_tov_index(&self) -> Option<JewishHoliday>;

//     ///
//     fn is_yom_tov(&self) -> bool;

//     ///
//     fn is_yom_tov_assur_bemelacha(&self) -> bool;

//     ///
//     fn is_assur_bemelacha(&self) -> bool;

//     ///
//     fn has_candle_lighting(&self) -> bool;

//     ///
//     fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool;

//     ///
//     fn is_erev_yom_tov_sheni(&self) -> bool;

//     ///
//     fn is_aseres_yemei_teshuva(&self) -> bool;

//     ///
//     fn is_pesach(&self) -> bool;

//     ///
//     fn is_chol_hamoed_pesach(&self) -> bool;

//     ///
//     fn is_shavuos(&self) -> bool;

//     ///
//     fn is_rosh_hashana(&self) -> bool;

//     ///
//     fn is_yom_kippur(&self) -> bool;

//     ///
//     fn is_succos(&self) -> bool;

//     ///
//     fn is_hoshana_rabba(&self) -> bool;

//     ///
//     fn is_shemini_atzeres(&self) -> bool;

//     ///
//     fn is_simchas_torah(&self) -> bool;

//     ///
//     fn is_chol_hamoed_succos(&self) -> bool;

//     ///
//     fn is_chol_hamoed(&self) -> bool;

//     ///
//     fn is_erev_yom_tov(&self) -> bool;

//     ///
//     fn is_rosh_chodesh(&self) -> bool;

//     ///
//     fn is_isru_chag(&self) -> bool;

//     ///
//     fn is_taanis(&self) -> bool;

//     ///
//     fn is_taanis_bechoros(&self) -> bool;

//     ///
//     fn get_day_of_chanukah(&self) -> i64;

//     ///
//     fn is_chanukah(&self) -> bool;

//     ///
//     fn is_purim(&self) -> bool;

//     ///
//     fn get_day_of_omer(&self) -> i64;

//     ///
//     fn is_tisha_beav(&self) -> bool;

//     ///
//     fn get_parshah(&self) -> Option<Parsha>;

//     ///
//     fn get_daf_yomi_bavli(&self) -> Option<impl BavliDafTrait>;

//     ///
//     fn get_daf_yomi_yerushalmi(&self) -> Option<impl YerushalmiDafTrait>;
// }
