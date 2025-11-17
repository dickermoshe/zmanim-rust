from datetime import datetime
from typing import Annotated, Type
from pydantic import BaseModel
from caseconverter import pascalcase
import typer
import pathlib

app = typer.Typer()


class EnumMember(BaseModel):
    name: str
    value: int


class Enum(BaseModel):
    name: str
    members: list[EnumMember]


class TransliteratedEnumMember(BaseModel):
    name: str
    transliterated_name: str
    value: int


class Constant(BaseModel):
    name: str
    type: Type
    value: str


PARSHAS = [
    TransliteratedEnumMember(name="בראשית", transliterated_name="Bereshis", value=0),
    TransliteratedEnumMember(name="נח", transliterated_name="Noach", value=1),
    TransliteratedEnumMember(name="לך לך", transliterated_name="Lech Lecha", value=2),
    TransliteratedEnumMember(name="וירא", transliterated_name="Vayera", value=3),
    TransliteratedEnumMember(
        name="חיי שרה", transliterated_name="Chayei Sara", value=4
    ),
    TransliteratedEnumMember(name="תולדות", transliterated_name="Toldos", value=5),
    TransliteratedEnumMember(name="ויצא", transliterated_name="Vayetzei", value=6),
    TransliteratedEnumMember(name="וישלח", transliterated_name="Vayishlach", value=7),
    TransliteratedEnumMember(name="וישב", transliterated_name="Vayeshev", value=8),
    TransliteratedEnumMember(name="מקץ", transliterated_name="Miketz", value=9),
    TransliteratedEnumMember(name="ויגש", transliterated_name="Vayigash", value=10),
    TransliteratedEnumMember(name="ויחי", transliterated_name="Vayechi", value=11),
    TransliteratedEnumMember(name="שמות", transliterated_name="Shemos", value=12),
    TransliteratedEnumMember(name="וארא", transliterated_name="Vaera", value=13),
    TransliteratedEnumMember(name="בא", transliterated_name="Bo", value=14),
    TransliteratedEnumMember(name="בשלח", transliterated_name="Beshalach", value=15),
    TransliteratedEnumMember(name="יתרו", transliterated_name="Yisro", value=16),
    TransliteratedEnumMember(name="משפטים", transliterated_name="Mishpatim", value=17),
    TransliteratedEnumMember(name="תרומה", transliterated_name="Terumah", value=18),
    TransliteratedEnumMember(name="תצוה", transliterated_name="Tetzaveh", value=19),
    TransliteratedEnumMember(name="כי תשא", transliterated_name="Ki Sisa", value=20),
    TransliteratedEnumMember(name="ויקהל", transliterated_name="Vayakhel", value=21),
    TransliteratedEnumMember(name="פקודי", transliterated_name="Pekudei", value=22),
    TransliteratedEnumMember(name="ויקרא", transliterated_name="Vayikra", value=23),
    TransliteratedEnumMember(name="צו", transliterated_name="Tzav", value=24),
    TransliteratedEnumMember(name="שמיני", transliterated_name="Shmini", value=25),
    TransliteratedEnumMember(name="תזריע", transliterated_name="Tazria", value=26),
    TransliteratedEnumMember(name="מצרע", transliterated_name="Metzora", value=27),
    TransliteratedEnumMember(
        name="אחרי מות", transliterated_name="Achrei Mos", value=28
    ),
    TransliteratedEnumMember(name="קדושים", transliterated_name="Kedoshim", value=29),
    TransliteratedEnumMember(name="אמור", transliterated_name="Emor", value=30),
    TransliteratedEnumMember(name="בהר", transliterated_name="Behar", value=31),
    TransliteratedEnumMember(name="בחקתי", transliterated_name="Bechukosai", value=32),
    TransliteratedEnumMember(name="במדבר", transliterated_name="Bamidbar", value=33),
    TransliteratedEnumMember(name="נשא", transliterated_name="Nasso", value=34),
    TransliteratedEnumMember(
        name="בהעלתך", transliterated_name="Beha'aloscha", value=35
    ),
    TransliteratedEnumMember(name="שלח לך", transliterated_name="Sh'lach", value=36),
    TransliteratedEnumMember(name="קרח", transliterated_name="Korach", value=37),
    TransliteratedEnumMember(name="חוקת", transliterated_name="Chukas", value=38),
    TransliteratedEnumMember(name="בלק", transliterated_name="Balak", value=39),
    TransliteratedEnumMember(name="פינחס", transliterated_name="Pinchas", value=40),
    TransliteratedEnumMember(name="מטות", transliterated_name="Matos", value=41),
    TransliteratedEnumMember(name="מסעי", transliterated_name="Masei", value=42),
    TransliteratedEnumMember(name="דברים", transliterated_name="Devarim", value=43),
    TransliteratedEnumMember(name="ואתחנן", transliterated_name="Vaeschanan", value=44),
    TransliteratedEnumMember(name="עקב", transliterated_name="Eikev", value=45),
    TransliteratedEnumMember(name="ראה", transliterated_name="Re'eh", value=46),
    TransliteratedEnumMember(name="שופטים", transliterated_name="Shoftim", value=47),
    TransliteratedEnumMember(name="כי תצא", transliterated_name="Ki Seitzei", value=48),
    TransliteratedEnumMember(name="כי תבוא", transliterated_name="Ki Savo", value=49),
    TransliteratedEnumMember(name="נצבים", transliterated_name="Nitzavim", value=50),
    TransliteratedEnumMember(name="וילך", transliterated_name="Vayeilech", value=51),
    TransliteratedEnumMember(name="האזינו", transliterated_name="Ha'Azinu", value=52),
    TransliteratedEnumMember(
        name="וזאת הברכה ", transliterated_name="Vezos Habracha", value=53
    ),
    TransliteratedEnumMember(
        name="ויקהל פקודי", transliterated_name="Vayakhel Pekudei", value=54
    ),
    TransliteratedEnumMember(
        name="תזריע מצרע", transliterated_name="Tazria Metzora", value=55
    ),
    TransliteratedEnumMember(
        name="אחרי מות קדושים", transliterated_name="Achrei Mos Kedoshim", value=56
    ),
    TransliteratedEnumMember(
        name="בהר בחקתי", transliterated_name="Behar Bechukosai", value=57
    ),
    TransliteratedEnumMember(
        name="חוקת בלק", transliterated_name="Chukas Balak", value=58
    ),
    TransliteratedEnumMember(
        name="מטות מסעי", transliterated_name="Matos Masei", value=59
    ),
    TransliteratedEnumMember(
        name="נצבים וילך", transliterated_name="Nitzavim Vayeilech", value=60
    ),
    TransliteratedEnumMember(name="שקלים", transliterated_name="Shekalim", value=61),
    TransliteratedEnumMember(name="זכור", transliterated_name="Zachor", value=62),
    TransliteratedEnumMember(name="פרה", transliterated_name="Parah", value=63),
    TransliteratedEnumMember(name="החדש", transliterated_name="Hachodesh", value=64),
    TransliteratedEnumMember(name="שובה", transliterated_name="Shuva", value=65),
    TransliteratedEnumMember(name="שירה", transliterated_name="Shira", value=66),
    TransliteratedEnumMember(name="הגדול", transliterated_name="Hagadol", value=67),
    TransliteratedEnumMember(name="חזון", transliterated_name="Chazon", value=68),
    TransliteratedEnumMember(name="נחמו", transliterated_name="Nachamu", value=69),
]


HEBREW_MONTHS = [
    TransliteratedEnumMember(name="ניסן", transliterated_name="Nissan", value=1),
    TransliteratedEnumMember(name="אייר", transliterated_name="Iyar", value=2),
    TransliteratedEnumMember(name="סיון", transliterated_name="Sivan", value=3),
    TransliteratedEnumMember(name="תמוז", transliterated_name="Tammuz", value=4),
    TransliteratedEnumMember(name="אב", transliterated_name="Av", value=5),
    TransliteratedEnumMember(name="אלול", transliterated_name="Elul", value=6),
    TransliteratedEnumMember(name="תשרי", transliterated_name="Tishrei", value=7),
    TransliteratedEnumMember(name="חשון", transliterated_name="Cheshvan", value=8),
    TransliteratedEnumMember(name="כסלו", transliterated_name="Kislev", value=9),
    TransliteratedEnumMember(name="טבת", transliterated_name="Teves", value=10),
    TransliteratedEnumMember(name="שבט", transliterated_name="Shevat", value=11),
    TransliteratedEnumMember(name="אדר", transliterated_name="Adar", value=12),
    TransliteratedEnumMember(name="אדר ב", transliterated_name="Adar II", value=13),
]


DAY_OF_WEEKS = [
    TransliteratedEnumMember(name="ראשון", transliterated_name="Sunday", value=1),
    TransliteratedEnumMember(name="שני", transliterated_name="Monday", value=2),
    TransliteratedEnumMember(name="שלישי", transliterated_name="Tuesday", value=3),
    TransliteratedEnumMember(name="רביעי", transliterated_name="Wednesday", value=4),
    TransliteratedEnumMember(name="חמישי", transliterated_name="Thursday", value=5),
    TransliteratedEnumMember(name="שישי", transliterated_name="Friday", value=6),
    TransliteratedEnumMember(name="שבת", transliterated_name="Shabbos", value=7),
]

JEWISH_HOLIDAYS = [
    TransliteratedEnumMember(
        name="ערב פסח", transliterated_name="Erev Pesach", value=0
    ),
    TransliteratedEnumMember(name="פסח", transliterated_name="Pesach", value=1),
    TransliteratedEnumMember(
        name="חול המועד פסח", transliterated_name="Chol Hamoed Pesach", value=2
    ),
    TransliteratedEnumMember(
        name="פסח שני", transliterated_name="Pesach Sheni", value=3
    ),
    TransliteratedEnumMember(
        name="ערב שבועות", transliterated_name="Erev Shavuos", value=4
    ),
    TransliteratedEnumMember(name="שבועות", transliterated_name="Shavuos", value=5),
    TransliteratedEnumMember(
        name="שבעה עשר בתמוז", transliterated_name="Seventeenth of Tammuz", value=6
    ),
    TransliteratedEnumMember(
        name="תשעה באב", transliterated_name="Tishah B'Av", value=7
    ),
    TransliteratedEnumMember(name="ט״ו באב", transliterated_name="Tu B'Av", value=8),
    TransliteratedEnumMember(
        name="ערב ראש השנה", transliterated_name="Erev Rosh Hashana", value=9
    ),
    TransliteratedEnumMember(
        name="ראש השנה", transliterated_name="Rosh Hashana", value=10
    ),
    TransliteratedEnumMember(
        name="צום גדליה", transliterated_name="Fast of Gedalyah", value=11
    ),
    TransliteratedEnumMember(
        name="ערב יום כיפור", transliterated_name="Erev Yom Kippur", value=12
    ),
    TransliteratedEnumMember(
        name="יום כיפור", transliterated_name="Yom Kippur", value=13
    ),
    TransliteratedEnumMember(
        name="ערב סוכות", transliterated_name="Erev Succos", value=14
    ),
    TransliteratedEnumMember(name="סוכות", transliterated_name="Succos", value=15),
    TransliteratedEnumMember(
        name="חול המועד סוכות", transliterated_name="Chol Hamoed Succos", value=16
    ),
    TransliteratedEnumMember(
        name="הושענא רבה", transliterated_name="Hoshana Rabbah", value=17
    ),
    TransliteratedEnumMember(
        name="שמיני עצרת", transliterated_name="Shemini Atzeres", value=18
    ),
    TransliteratedEnumMember(
        name="שמחת תורה", transliterated_name="Simchas Torah", value=19
    ),
    TransliteratedEnumMember(
        name="ערב חנוכה", transliterated_name="Erev Chanukah", value=20
    ),
    TransliteratedEnumMember(name="חנוכה", transliterated_name="Chanukah", value=21),
    TransliteratedEnumMember(
        name="עשרה בטבת", transliterated_name="Tenth of Teves", value=22
    ),
    TransliteratedEnumMember(
        name="ט״ו בשבט", transliterated_name="Tu B'Shvat", value=23
    ),
    TransliteratedEnumMember(
        name="תענית אסתר", transliterated_name="Fast of Esther", value=24
    ),
    TransliteratedEnumMember(name="פורים", transliterated_name="Purim", value=25),
    TransliteratedEnumMember(
        name="שושן פורים", transliterated_name="Shushan Purim", value=26
    ),
    TransliteratedEnumMember(
        name="פורים קטן", transliterated_name="Purim Katan", value=27
    ),
    TransliteratedEnumMember(
        name="ראש חודש", transliterated_name="Rosh Chodesh", value=28
    ),
    TransliteratedEnumMember(
        name="יום השואה", transliterated_name="Yom HaShoah", value=29
    ),
    TransliteratedEnumMember(
        name="יום הזיכרון", transliterated_name="Yom Hazikaron", value=30
    ),
    TransliteratedEnumMember(
        name="יום העצמאות", transliterated_name="Yom Ha'atzmaut", value=31
    ),
    TransliteratedEnumMember(
        name="יום ירושלים", transliterated_name="Yom Yerushalayim", value=32
    ),
    TransliteratedEnumMember(
        name="ל״ג בעומר", transliterated_name="Lag B'Omer", value=33
    ),
    TransliteratedEnumMember(
        name="שושן פורים קטן", transliterated_name="Shushan Purim Katan", value=34
    ),
    TransliteratedEnumMember(name="אסרו חג", transliterated_name="Isru Chag", value=35),
    TransliteratedEnumMember(
        name="יום העצמאות", transliterated_name="Yom Kippur Katan", value=36
    ),
    TransliteratedEnumMember(
        name="יום כיפור קטן", transliterated_name="Behab", value=37
    ),
]


YEAR_LENGTH_TYPES = [
    TransliteratedEnumMember(name="חסרים", transliterated_name="Chaserim", value=0),
    TransliteratedEnumMember(name="כסדרן", transliterated_name="Kesidran", value=1),
    TransliteratedEnumMember(name="שלמים", transliterated_name="Shelaimim", value=2),
]

BAVLI_TRACTATES = [
    TransliteratedEnumMember(name="ברכות", transliterated_name="Berachos", value=0),
    TransliteratedEnumMember(name="שבת", transliterated_name="Shabbos", value=1),
    TransliteratedEnumMember(name="עירובין", transliterated_name="Eruvin", value=2),
    TransliteratedEnumMember(name="פסחים", transliterated_name="Pesachim", value=3),
    TransliteratedEnumMember(name="שקלים", transliterated_name="Shekalim", value=4),
    TransliteratedEnumMember(name="יומא", transliterated_name="Yoma", value=5),
    TransliteratedEnumMember(name="סוכה", transliterated_name="Sukkah", value=6),
    TransliteratedEnumMember(name="ביצה", transliterated_name="Beitzah", value=7),
    TransliteratedEnumMember(
        name="ראש השנה", transliterated_name="Rosh Hashana", value=8
    ),
    TransliteratedEnumMember(name="תענית", transliterated_name="Taanis", value=9),
    TransliteratedEnumMember(name="מגילה", transliterated_name="Megillah", value=10),
    TransliteratedEnumMember(
        name="מועד קטן", transliterated_name="Moed Katan", value=11
    ),
    TransliteratedEnumMember(name="חגיגה", transliterated_name="Chagigah", value=12),
    TransliteratedEnumMember(name="יבמות", transliterated_name="Yevamos", value=13),
    TransliteratedEnumMember(name="כתובות", transliterated_name="Kesubos", value=14),
    TransliteratedEnumMember(name="נדרים", transliterated_name="Nedarim", value=15),
    TransliteratedEnumMember(name="נזיר", transliterated_name="Nazir", value=16),
    TransliteratedEnumMember(name="סוטה", transliterated_name="Sotah", value=17),
    TransliteratedEnumMember(name="גיטין", transliterated_name="Gitin", value=18),
    TransliteratedEnumMember(name="קידושין", transliterated_name="Kiddushin", value=19),
    TransliteratedEnumMember(
        name="בבא קמא", transliterated_name="Bava Kamma", value=20
    ),
    TransliteratedEnumMember(
        name="בבא מציעא", transliterated_name="Bava Metzia", value=21
    ),
    TransliteratedEnumMember(
        name="בבא בתרא", transliterated_name="Bava Basra", value=22
    ),
    TransliteratedEnumMember(name="סנהדרין", transliterated_name="Sanhedrin", value=23),
    TransliteratedEnumMember(name="מכות", transliterated_name="Makkos", value=24),
    TransliteratedEnumMember(name="שבועות", transliterated_name="Shevuos", value=25),
    TransliteratedEnumMember(
        name="עבודה זרה", transliterated_name="Avodah Zarah", value=26
    ),
    TransliteratedEnumMember(name="הוריות", transliterated_name="Horiyos", value=27),
    TransliteratedEnumMember(name="זבחים", transliterated_name="Zevachim", value=28),
    TransliteratedEnumMember(name="מנחות", transliterated_name="Menachos", value=29),
    TransliteratedEnumMember(name="חולין", transliterated_name="Chullin", value=30),
    TransliteratedEnumMember(name="בכורות", transliterated_name="Bechoros", value=31),
    TransliteratedEnumMember(name="ערכין", transliterated_name="Arachin", value=32),
    TransliteratedEnumMember(name="תמורה", transliterated_name="Temurah", value=33),
    TransliteratedEnumMember(name="כריתות", transliterated_name="Kerisos", value=34),
    TransliteratedEnumMember(name="מעילה", transliterated_name="Meilah", value=35),
    TransliteratedEnumMember(name="קינים", transliterated_name="Kinnim", value=36),
    TransliteratedEnumMember(name="תמיד", transliterated_name="Tamid", value=37),
    TransliteratedEnumMember(name="מידות", transliterated_name="Midos", value=38),
    TransliteratedEnumMember(name="נדה", transliterated_name="Niddah", value=39),
]


YERUSHALMI_TRACTATES = [
    TransliteratedEnumMember(name="ברכות", transliterated_name="Berachos", value=0),
    TransliteratedEnumMember(name="פיאה", transliterated_name="Pe'ah", value=1),
    TransliteratedEnumMember(name="דמאי", transliterated_name="Demai", value=2),
    TransliteratedEnumMember(name="כלאים", transliterated_name="Kilayim", value=3),
    TransliteratedEnumMember(name="שביעית", transliterated_name="Shevi'is", value=4),
    TransliteratedEnumMember(name="תרומות", transliterated_name="Terumos", value=5),
    TransliteratedEnumMember(name="מעשרות", transliterated_name="Ma'asros", value=6),
    TransliteratedEnumMember(
        name="מעשר שני", transliterated_name="Ma'aser Sheni", value=7
    ),
    TransliteratedEnumMember(name="חלה", transliterated_name="Chalah", value=8),
    TransliteratedEnumMember(name="עורלה", transliterated_name="Orlah", value=9),
    TransliteratedEnumMember(name="ביכורים", transliterated_name="Bikurim", value=10),
    TransliteratedEnumMember(name="שבת", transliterated_name="Shabbos", value=11),
    TransliteratedEnumMember(name="עירובין", transliterated_name="Eruvin", value=12),
    TransliteratedEnumMember(name="פסחים", transliterated_name="Pesachim", value=13),
    TransliteratedEnumMember(name="ביצה", transliterated_name="Beitzah", value=14),
    TransliteratedEnumMember(
        name="ראש השנה", transliterated_name="Rosh Hashanah", value=15
    ),
    TransliteratedEnumMember(name="יומא", transliterated_name="Yoma", value=16),
    TransliteratedEnumMember(name="סוכה", transliterated_name="Sukah", value=17),
    TransliteratedEnumMember(name="תענית", transliterated_name="Ta'anis", value=18),
    TransliteratedEnumMember(name="שקלים", transliterated_name="Shekalim", value=19),
    TransliteratedEnumMember(name="מגילה", transliterated_name="Megilah", value=20),
    TransliteratedEnumMember(name="חגיגה", transliterated_name="Chagigah", value=21),
    TransliteratedEnumMember(
        name="מועד קטן", transliterated_name="Moed Katan", value=22
    ),
    TransliteratedEnumMember(name="יבמות", transliterated_name="Yevamos", value=23),
    TransliteratedEnumMember(name="כתובות", transliterated_name="Kesuvos", value=24),
    TransliteratedEnumMember(name="סוטה", transliterated_name="Sotah", value=25),
    TransliteratedEnumMember(name="נדרים", transliterated_name="Nedarim", value=26),
    TransliteratedEnumMember(name="נזיר", transliterated_name="Nazir", value=27),
    TransliteratedEnumMember(name="גיטין", transliterated_name="Gitin", value=28),
    TransliteratedEnumMember(name="קידושין", transliterated_name="Kidushin", value=29),
    TransliteratedEnumMember(name="בבא קמא", transliterated_name="Bava Kama", value=30),
    TransliteratedEnumMember(
        name="בבא מציעא", transliterated_name="Bava Metzia", value=31
    ),
    TransliteratedEnumMember(
        name="בבא בתרא", transliterated_name="Bava Basra", value=32
    ),
    TransliteratedEnumMember(name="שבועות", transliterated_name="Shevuos", value=33),
    TransliteratedEnumMember(name="מכות", transliterated_name="Makos", value=34),
    TransliteratedEnumMember(name="סנהדרין", transliterated_name="Sanhedrin", value=35),
    TransliteratedEnumMember(
        name="עבודה זרה", transliterated_name="Avodah Zarah", value=36
    ),
    TransliteratedEnumMember(name="הוריות", transliterated_name="Horayos", value=37),
    TransliteratedEnumMember(name="נידה", transliterated_name="Nidah", value=38),
]


CONSTANTS = [
    Constant(name="JULIAN_DAY_JAN_1_2000", value="2451545.0", type=float),
    Constant(name="JULIAN_DAYS_PER_CENTURY", value="36525.0", type=float),
    Constant(name="EARTH_RADIUS", value="6356.9", type=float),
    Constant(name="GEOMETRIC_ZENITH", value="90.0", type=float),
    Constant(name="CIVIL_ZENITH", value="96.0", type=float),
    Constant(name="NAUTICAL_ZENITH", value="102.0", type=float),
    Constant(name="ASTRONOMICAL_ZENITH", value="108.0", type=float),
    Constant(name="SOLAR_RADIUS", value="16.0 / 60.0", type=float),
    Constant(name="REFRACTION", value="34.0 / 60.0", type=float),
    Constant(name="ZENITH_16_POINT_1", value="90.0 + 16.1", type=float),
    Constant(name="ZENITH_8_POINT_5", value="90.0 + 8.5", type=float),
    Constant(name="ZENITH_3_POINT_7", value="90.0 + 3.7", type=float),
    Constant(name="ZENITH_3_POINT_8", value="90.0 + 3.8", type=float),
    Constant(name="ZENITH_5_POINT_95", value="90.0 + 5.95", type=float),
    Constant(name="ZENITH_7_POINT_083", value="90.0 + 7.0 + (5.0 / 60.0)", type=float),
    Constant(name="ZENITH_10_POINT_2", value="90.0 + 10.2", type=float),
    Constant(name="ZENITH_11_DEGREES", value="90.0 + 11.0", type=float),
    Constant(name="ZENITH_11_POINT_5", value="90.0 + 11.5", type=float),
    Constant(name="ZENITH_13_POINT_24", value="90.0 + 13.24", type=float),
    Constant(name="ZENITH_19_DEGREES", value="90.0 + 19.0", type=float),
    Constant(name="ZENITH_19_POINT_8", value="90.0 + 19.8", type=float),
    Constant(name="ZENITH_26_DEGREES", value="90.0 + 26.0", type=float),
    Constant(name="ZENITH_4_POINT_37", value="90.0 + 4.37", type=float),
    Constant(name="ZENITH_4_POINT_61", value="90.0 + 4.61", type=float),
    Constant(name="ZENITH_4_POINT_8", value="90.0 + 4.8", type=float),
    Constant(name="ZENITH_3_POINT_65", value="90.0 + 3.65", type=float),
    Constant(name="ZENITH_3_POINT_676", value="90.0 + 3.676", type=float),
    Constant(name="ZENITH_5_POINT_88", value="90.0 + 5.88", type=float),
    Constant(name="ZENITH_1_POINT_583", value="90.0 + 1.583", type=float),
    Constant(name="ZENITH_16_POINT_9", value="90.0 + 16.9", type=float),
    Constant(name="ZENITH_6_DEGREES", value="90.0 + 6.0", type=float),
    Constant(name="ZENITH_6_POINT_45", value="90.0 + 6.45", type=float),
    Constant(name="ZENITH_7_POINT_65", value="90.0 + 7.65", type=float),
    Constant(name="ZENITH_7_POINT_67", value="90.0 + 7.67", type=float),
    Constant(name="ZENITH_9_POINT_3", value="90.0 + 9.3", type=float),
    Constant(name="ZENITH_9_POINT_5", value="90.0 + 9.5", type=float),
    Constant(name="ZENITH_9_POINT_75", value="90.0 + 9.75", type=float),
    Constant(name="ZENITH_MINUS_2_POINT_1", value="90.0 - 2.1", type=float),
    Constant(name="ZENITH_MINUS_2_POINT_8", value="90.0 - 2.8", type=float),
    Constant(name="ZENITH_MINUS_3_POINT_05", value="90.0 - 3.05", type=float),
    Constant(name="CHALAKIM_PER_MINUTE", value="18", type=int),
    Constant(name="CHALAKIM_PER_HOUR", value="1080", type=int),
    Constant(name="CHALAKIM_PER_DAY", value="25920", type=int),
    Constant(name="CHALAKIM_PER_MONTH", value="765433", type=int),
    Constant(name="CHALAKIM_MOLAD_TOHU", value="31524", type=int),
    Constant(name="JEWISH_EPOCH", value="-1373429", type=int),
    Constant(name="MINUTE_MILLIS", value="60 * 1000", type=int),
    Constant(name="HOUR_MILLIS", value="60 * 1000 * 60", type=int),
    Constant(name="BAVLI_DAF_YOMI_START_DAY", value="-1461369600000", type=datetime),
    Constant(name="BAVLI_SHEKALIM_CHANGE_DAY", value="172800000000", type=datetime),
    Constant(name="YERUSHALMI_DAF_YOMI_START_DAY", value="318297600000", type=datetime),
    Constant(name="YERUSHALMI_LENGTH", value="1554", type=int),
]

ENUMS = [
    Enum(
        name="SOLAR_EVENT",
        members=[
            EnumMember(name="Sunrise", value=0),
            EnumMember(name="Sunset", value=1),
            EnumMember(name="Noon", value=2),
            EnumMember(name="Midnight", value=3),
        ],
    ),
    Enum(
        name="FORMULA",
        members=[
            EnumMember(name="Distance", value=0),
            EnumMember(name="InitialBearing", value=1),
            EnumMember(name="FinalBearing", value=2),
        ],
    ),
]


def type_to_rust(type: Type) -> str:
    if type is float:
        return "f64"
    elif type is int:
        return "i64"
    elif type == datetime:
        return "DateTime<Utc>"
    else:
        raise ValueError(f"Unknown type: {type}")


def transliterated_enums_to_rust(
    name: str, enums: list[TransliteratedEnumMember]
) -> str:
    impl_buffer = f"impl {name} {{\n"
    impl_buffer += "pub fn en_string(&self) -> &str { match self {"
    for member in enums:
        impl_buffer += f'    {name}::{pascalcase(member.transliterated_name)} => "{member.transliterated_name}",\n'
    impl_buffer += "}}\n"
    impl_buffer += "pub fn he_string(&self) -> &str { match self {"
    for member in enums:
        impl_buffer += f'    {name}::{pascalcase(member.transliterated_name)} => "{member.name}",\n'
    impl_buffer += "}}\n"
    impl_buffer += "}\n"
    buffer = ""
    buffer += "#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]\n"
    buffer += "#[repr(i64)]\n"
    buffer += f"pub enum {name} {{\n"
    for enum in enums:
        buffer += f"    {pascalcase(enum.transliterated_name)} = {enum.value},\n"
    buffer += "}\n"
    return buffer + impl_buffer


@app.command()
def constants(
    output_file: Annotated[pathlib.Path, typer.Argument()] = pathlib.Path(
        "constants.rs"
    ),
):
    buffer = """
    use chrono::{DateTime, Utc};
    use num_enum::{IntoPrimitive, TryFromPrimitive};
    """
    for constant in CONSTANTS:
        if constant.type is datetime:
            buffer += f"pub static _{constant.name}: DateTime<Utc> = DateTime::from_timestamp_millis({constant.value}).unwrap();\n"
        else:
            buffer += f"pub static _{constant.name}: {type_to_rust(constant.type)} = {constant.value};\n"
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(buffer)


@app.command()
def enums(
    output_file: Annotated[pathlib.Path, typer.Argument()] = pathlib.Path("enums.rs"),
):
    buffer = """
    use chrono::{DateTime, Utc};
    use num_enum::{IntoPrimitive, TryFromPrimitive};
    """
    for enum in ENUMS:
        buffer += "#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]\n"
        buffer += "#[repr(i64)]\n"
        buffer += f"pub enum _{pascalcase(enum.name)} {{\n"
        for member in enum.members:
            buffer += f"    {pascalcase(member.name)} = {member.value},\n"
        buffer += "}\n"
    buffer += transliterated_enums_to_rust("Parsha", PARSHAS)
    buffer += transliterated_enums_to_rust("JewishHoliday", JEWISH_HOLIDAYS)
    buffer += transliterated_enums_to_rust("DayOfWeek", DAY_OF_WEEKS)
    buffer += transliterated_enums_to_rust("JewishMonth", HEBREW_MONTHS)
    buffer += transliterated_enums_to_rust("YearLengthType", YEAR_LENGTH_TYPES)
    buffer += transliterated_enums_to_rust("BavliTractate", BAVLI_TRACTATES)
    buffer += transliterated_enums_to_rust("YerushalmiTractate", YERUSHALMI_TRACTATES)
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(buffer)

    print(
        "You must manually make the he_string and en_string methods on JewishMonth factor in the leap year (Adar I during leap year or just plain Adar otherwise)"
    )


if __name__ == "__main__":
    app()
