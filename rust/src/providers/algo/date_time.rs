//! Algorithmic formatters for the `date_time` provider.
//!
//! All date arithmetic uses civil-from-days math (no external crate needed).
//! The epoch is 1970-01-01 00:00:00 UTC.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---------------------------------------------------------------------------
// Civil-from-days: convert a Unix timestamp (seconds since 1970-01-01) to
// a (year, month, day, hour, minute, second) tuple.
// Algorithm: https://howardhinnant.github.io/date_algorithms.html
// ---------------------------------------------------------------------------

fn civil_from_unix(ts: i64) -> (i32, u32, u32, u32, u32, u32) {
    let secs_per_day: i64 = 86_400;
    let z = ts.div_euclid(secs_per_day) + 719_468; // shift epoch to 0000-03-01
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // year of era [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month of year [0, 11] starting from March
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let year = if m <= 2 { y + 1 } else { y } as i32;
    let rem = ts.rem_euclid(secs_per_day) as u32;
    let hour = rem / 3600;
    let minute = (rem % 3600) / 60;
    let second = rem % 60;
    (year, m as u32, d as u32, hour, minute, second)
}

/// Returns a zero-padded 2-digit string.
#[inline]
fn z2(n: u32) -> String {
    format!("{:02}", n)
}

/// Day-of-week name (0 = Thursday for Unix epoch 1970-01-01).
fn day_name(ts: i64) -> &'static str {
    // 1970-01-01 was a Thursday → day index 3 (Mon=0..Sun=6)
    let dow = (ts.div_euclid(86_400) + 3).rem_euclid(7) as usize;
    [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ][dow]
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// ---------------------------------------------------------------------------
// Constants embedded from the Python Provider class
// ---------------------------------------------------------------------------

const CENTURIES: [&str; 21] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV", "XV",
    "XVI", "XVII", "XVIII", "XIX", "XX", "XXI",
];

/// All timezone strings drawn from the `countries` list in the Python Provider.
const TIMEZONES: [&str; 329] = [
    "Europe/Andorra",
    "Asia/Kabul",
    "America/Antigua",
    "Europe/Tirane",
    "Asia/Yerevan",
    "Africa/Luanda",
    "America/Argentina/Buenos_Aires",
    "America/Argentina/Cordoba",
    "America/Argentina/Jujuy",
    "America/Argentina/Tucuman",
    "America/Argentina/Catamarca",
    "America/Argentina/La_Rioja",
    "America/Argentina/San_Juan",
    "America/Argentina/Mendoza",
    "America/Argentina/Rio_Gallegos",
    "America/Argentina/Ushuaia",
    "Europe/Vienna",
    "Australia/Lord_Howe",
    "Australia/Hobart",
    "Australia/Currie",
    "Australia/Melbourne",
    "Australia/Sydney",
    "Australia/Broken_Hill",
    "Australia/Brisbane",
    "Australia/Lindeman",
    "Australia/Adelaide",
    "Australia/Darwin",
    "Australia/Perth",
    "Asia/Baku",
    "America/Barbados",
    "Asia/Dhaka",
    "Europe/Brussels",
    "Africa/Ouagadougou",
    "Europe/Sofia",
    "Asia/Bahrain",
    "Africa/Bujumbura",
    "Africa/Porto-Novo",
    "Asia/Brunei",
    "America/La_Paz",
    "America/Noronha",
    "America/Belem",
    "America/Fortaleza",
    "America/Recife",
    "America/Araguaina",
    "America/Maceio",
    "America/Bahia",
    "America/Sao_Paulo",
    "America/Campo_Grande",
    "America/Cuiaba",
    "America/Porto_Velho",
    "America/Boa_Vista",
    "America/Manaus",
    "America/Eirunepe",
    "America/Rio_Branco",
    "America/Nassau",
    "Asia/Thimphu",
    "Africa/Gaborone",
    "Europe/Minsk",
    "America/Belize",
    "America/St_Johns",
    "America/Halifax",
    "America/Glace_Bay",
    "America/Moncton",
    "America/Goose_Bay",
    "America/Blanc-Sablon",
    "America/Montreal",
    "America/Toronto",
    "America/Nipigon",
    "America/Thunder_Bay",
    "America/Pangnirtung",
    "America/Iqaluit",
    "America/Atikokan",
    "America/Rankin_Inlet",
    "America/Winnipeg",
    "America/Rainy_River",
    "America/Cambridge_Bay",
    "America/Regina",
    "America/Swift_Current",
    "America/Edmonton",
    "America/Yellowknife",
    "America/Inuvik",
    "America/Dawson_Creek",
    "America/Vancouver",
    "America/Whitehorse",
    "America/Dawson",
    "Africa/Kinshasa",
    "Africa/Lubumbashi",
    "Africa/Brazzaville",
    "Africa/Abidjan",
    "America/Santiago",
    "Pacific/Easter",
    "Africa/Douala",
    "Asia/Shanghai",
    "Asia/Harbin",
    "Asia/Chongqing",
    "Asia/Urumqi",
    "Asia/Kashgar",
    "America/Bogota",
    "America/Costa_Rica",
    "America/Havana",
    "Atlantic/Cape_Verde",
    "Asia/Nicosia",
    "Europe/Prague",
    "Europe/Berlin",
    "Africa/Djibouti",
    "Europe/Copenhagen",
    "America/Dominica",
    "America/Santo_Domingo",
    "America/Guayaquil",
    "Pacific/Galapagos",
    "Europe/Tallinn",
    "Africa/Cairo",
    "Africa/Asmera",
    "Africa/Addis_Ababa",
    "Europe/Helsinki",
    "Pacific/Fiji",
    "Europe/Paris",
    "Africa/Libreville",
    "Asia/Tbilisi",
    "Africa/Accra",
    "Africa/Banjul",
    "Africa/Conakry",
    "Europe/Athens",
    "America/Guatemala",
    "Africa/Bissau",
    "America/Guyana",
    "America/Tegucigalpa",
    "Europe/Budapest",
    "Asia/Jakarta",
    "Asia/Pontianak",
    "Asia/Makassar",
    "Asia/Jayapura",
    "Europe/Dublin",
    "Asia/Jerusalem",
    "Asia/Calcutta",
    "Asia/Baghdad",
    "Asia/Tehran",
    "Atlantic/Reykjavik",
    "Europe/Rome",
    "America/Jamaica",
    "Asia/Amman",
    "Asia/Tokyo",
    "Africa/Nairobi",
    "Asia/Bishkek",
    "Pacific/Tarawa",
    "Pacific/Enderbury",
    "Pacific/Kiritimati",
    "Asia/Pyongyang",
    "Asia/Seoul",
    "Asia/Kuwait",
    "Asia/Beirut",
    "Europe/Vaduz",
    "Africa/Monrovia",
    "Africa/Maseru",
    "Europe/Vilnius",
    "Europe/Luxembourg",
    "Europe/Riga",
    "Africa/Tripoli",
    "Indian/Antananarivo",
    "Pacific/Majuro",
    "Pacific/Kwajalein",
    "Europe/Skopje",
    "Africa/Bamako",
    "Asia/Rangoon",
    "Asia/Ulaanbaatar",
    "Asia/Hovd",
    "Asia/Choibalsan",
    "Africa/Nouakchott",
    "Europe/Malta",
    "Indian/Mauritius",
    "Indian/Maldives",
    "Africa/Blantyre",
    "America/Mexico_City",
    "America/Cancun",
    "America/Merida",
    "America/Monterrey",
    "America/Mazatlan",
    "America/Chihuahua",
    "America/Hermosillo",
    "America/Tijuana",
    "Asia/Kuala_Lumpur",
    "Asia/Kuching",
    "Africa/Maputo",
    "Africa/Windhoek",
    "Africa/Niamey",
    "Africa/Lagos",
    "America/Managua",
    "Europe/Amsterdam",
    "Europe/Oslo",
    "Asia/Katmandu",
    "Pacific/Nauru",
    "Pacific/Auckland",
    "Pacific/Chatham",
    "Asia/Muscat",
    "America/Panama",
    "America/Lima",
    "Pacific/Port_Moresby",
    "Asia/Manila",
    "Asia/Karachi",
    "Europe/Warsaw",
    "Europe/Lisbon",
    "Atlantic/Madeira",
    "Atlantic/Azores",
    "Pacific/Palau",
    "America/Asuncion",
    "Asia/Qatar",
    "Europe/Bucharest",
    "Europe/Kaliningrad",
    "Europe/Moscow",
    "Europe/Volgograd",
    "Europe/Samara",
    "Asia/Yekaterinburg",
    "Asia/Omsk",
    "Asia/Novosibirsk",
    "Asia/Krasnoyarsk",
    "Asia/Irkutsk",
    "Asia/Yakutsk",
    "Asia/Vladivostok",
    "Asia/Sakhalin",
    "Asia/Magadan",
    "Asia/Kamchatka",
    "Asia/Anadyr",
    "Africa/Kigali",
    "Asia/Riyadh",
    "Pacific/Guadalcanal",
    "Indian/Mahe",
    "Africa/Khartoum",
    "Europe/Stockholm",
    "Asia/Singapore",
    "Europe/Ljubljana",
    "Europe/Bratislava",
    "Africa/Freetown",
    "Europe/San_Marino",
    "Africa/Dakar",
    "Africa/Mogadishu",
    "America/Paramaribo",
    "Africa/Sao_Tome",
    "Asia/Damascus",
    "Africa/Lome",
    "Asia/Bangkok",
    "Asia/Dushanbe",
    "Asia/Ashgabat",
    "Africa/Tunis",
    "Pacific/Tongatapu",
    "Europe/Istanbul",
    "America/Port_of_Spain",
    "Pacific/Funafuti",
    "Africa/Dar_es_Salaam",
    "Europe/Kyiv",
    "Europe/Uzhgorod",
    "Europe/Zaporozhye",
    "Europe/Simferopol",
    "Africa/Kampala",
    "America/New_York",
    "America/Detroit",
    "America/Kentucky/Louisville",
    "America/Kentucky/Monticello",
    "America/Indiana/Indianapolis",
    "America/Indiana/Marengo",
    "America/Indiana/Knox",
    "America/Indiana/Vevay",
    "America/Chicago",
    "America/Indiana/Vincennes",
    "America/Indiana/Petersburg",
    "America/Menominee",
    "America/North_Dakota/Center",
    "America/North_Dakota/New_Salem",
    "America/Denver",
    "America/Boise",
    "America/Shiprock",
    "America/Phoenix",
    "America/Los_Angeles",
    "America/Anchorage",
    "America/Juneau",
    "America/Yakutat",
    "America/Nome",
    "America/Adak",
    "Pacific/Honolulu",
    "America/Montevideo",
    "Asia/Samarkand",
    "Asia/Tashkent",
    "Europe/Vatican",
    "America/Caracas",
    "Asia/Saigon",
    "Pacific/Efate",
    "Asia/Aden",
    "Africa/Lusaka",
    "Africa/Harare",
    "Africa/Algiers",
    "Europe/Sarajevo",
    "Asia/Phnom_Penh",
    "Africa/Bangui",
    "Africa/Ndjamena",
    "Indian/Comoro",
    "Europe/Zagreb",
    "Asia/Dili",
    "America/El_Salvador",
    "Africa/Malabo",
    "America/Grenada",
    "Asia/Almaty",
    "Asia/Qyzylorda",
    "Asia/Aqtobe",
    "Asia/Aqtau",
    "Asia/Oral",
    "Asia/Vientiane",
    "Pacific/Truk",
    "Pacific/Ponape",
    "Pacific/Kosrae",
    "Europe/Chisinau",
    "Europe/Monaco",
    "Europe/Podgorica",
    "Africa/Casablanca",
    "America/St_Kitts",
    "America/St_Lucia",
    "America/St_Vincent",
    "Pacific/Apia",
    "Europe/Belgrade",
    "Africa/Johannesburg",
    "Europe/Madrid",
    "Africa/Ceuta",
    "Atlantic/Canary",
    "Asia/Colombo",
    "Africa/Mbabane",
    "Europe/Zurich",
    "Asia/Dubai",
    "Europe/London",
    "Asia/Taipei",
    "Asia/Gaza",
    "Asia/Hebron",
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Pick a random Unix timestamp in [0, now_approx).
/// We use a fixed "now" approximation: 2024-01-01 = 1704067200.
const NOW_TS: i64 = 1_704_067_200;

fn rand_ts_impl(f: &Faker, min: i64, max: i64) -> i64 {
    if max <= min {
        return min;
    }
    f.rng.random_int(min, max, 1)
}

// ---------------------------------------------------------------------------
// Formatter implementations
// ---------------------------------------------------------------------------

/// Random Unix timestamp 0..NOW_TS, returned as a float string.
fn unix_time(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    format!("{}.0", ts)
}

/// ISO 8601 datetime string (YYYY-MM-DDTHH:MM:SS).
fn iso8601(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{}T{}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// Date string YYYY-MM-DD.
fn date(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (y, mo, d, _, _, _) = civil_from_unix(ts);
    format!("{:04}-{}-{}", y, z2(mo), z2(d))
}

/// Time string HH:MM:SS.
fn time(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (_, _, _, h, mi, s) = civil_from_unix(ts);
    format!("{}:{}:{}", z2(h), z2(mi), z2(s))
}

/// datetime string — same as iso8601 but with a space separator.
fn date_time(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{} {}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// datetime from AD 1 (ts range: -62135596800..NOW_TS).
fn date_time_ad(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, -62_135_596_800, NOW_TS);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{}T{}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// AM or PM.
fn am_pm(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (_, _, _, h, _, _) = civil_from_unix(ts);
    if h < 12 { "AM" } else { "PM" }.to_string()
}

/// Day of month as zero-padded string.
fn day_of_month(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (_, _, d, _, _, _) = civil_from_unix(ts);
    z2(d)
}

/// Day of week as full name.
fn day_of_week(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    day_name(ts).to_string()
}

/// Month number as zero-padded string.
fn month(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (_, mo, _, _, _, _) = civil_from_unix(ts);
    z2(mo)
}

/// Full month name.
fn month_name(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (_, mo, _, _, _, _) = civil_from_unix(ts);
    MONTH_NAMES[(mo - 1) as usize].to_string()
}

/// Four-digit year string.
fn year(f: &Faker, _locale: &str) -> String {
    let ts = rand_ts_impl(f, 0, NOW_TS);
    let (y, _, _, _, _, _) = civil_from_unix(ts);
    format!("{:04}", y)
}

/// Roman-numeral century.
fn century(f: &Faker, locale: &str) -> String {
    // Try locale data first; fall back to hardcoded list.
    let v = f.lfield(locale, "date_time", "centuries");
    if !v.is_empty() {
        return v[f.rng.below(v.len())].clone();
    }
    CENTURIES[f.rng.below(CENTURIES.len())].to_string()
}

/// A random timezone string.
fn timezone(f: &Faker, _locale: &str) -> String {
    TIMEZONES[f.rng.below(TIMEZONES.len())].to_string()
}

/// A plausible date-of-birth (age 0-115) as YYYY-MM-DD.
fn date_of_birth(f: &Faker, _locale: &str) -> String {
    // maximum_age=115 → start = NOW_TS - 116*365.25*86400 ≈ -62135596800 won't go that far.
    // minimum_age=0  → end = NOW_TS.
    let min_ts = NOW_TS - 116_i64 * 365 * 86_400;
    let ts = rand_ts_impl(f, min_ts, NOW_TS);
    let (y, mo, d, _, _, _) = civil_from_unix(ts);
    format!("{:04}-{}-{}", y, z2(mo), z2(d))
}

/// past_date: between 30 days ago and yesterday.
fn past_date(f: &Faker, _locale: &str) -> String {
    let end = NOW_TS - 86_400;
    let start = NOW_TS - 30 * 86_400;
    let ts = rand_ts_impl(f, start, end);
    let (y, mo, d, _, _, _) = civil_from_unix(ts);
    format!("{:04}-{}-{}", y, z2(mo), z2(d))
}

/// future_date: between tomorrow and 30 days from now.
fn future_date(f: &Faker, _locale: &str) -> String {
    let start = NOW_TS + 86_400;
    let end = NOW_TS + 30 * 86_400;
    let ts = rand_ts_impl(f, start, end);
    let (y, mo, d, _, _, _) = civil_from_unix(ts);
    format!("{:04}-{}-{}", y, z2(mo), z2(d))
}

/// past_datetime: between 30 days ago and 1 second ago.
fn past_datetime(f: &Faker, _locale: &str) -> String {
    let end = NOW_TS - 1;
    let start = NOW_TS - 30 * 86_400;
    let ts = rand_ts_impl(f, start, end);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{}T{}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// future_datetime: between 1 second from now and 30 days from now.
fn future_datetime(f: &Faker, _locale: &str) -> String {
    let start = NOW_TS + 1;
    let end = NOW_TS + 30 * 86_400;
    let ts = rand_ts_impl(f, start, end);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{}T{}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// date_time_between: random datetime between -30y and now.
fn date_time_between(f: &Faker, _locale: &str) -> String {
    let start = NOW_TS - 30 * 365 * 86_400;
    let ts = rand_ts_impl(f, start, NOW_TS);
    let (y, mo, d, h, mi, s) = civil_from_unix(ts);
    format!(
        "{:04}-{}-{}T{}:{}:{}",
        y,
        z2(mo),
        z2(d),
        z2(h),
        z2(mi),
        z2(s)
    )
}

/// date_between: random date between -30y and today.
fn date_between(f: &Faker, _locale: &str) -> String {
    let start = NOW_TS - 30 * 365 * 86_400;
    let ts = rand_ts_impl(f, start, NOW_TS);
    let (y, mo, d, _, _, _) = civil_from_unix(ts);
    format!("{:04}-{}-{}", y, z2(mo), z2(d))
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "unix_time" => unix_time(f, locale),
        "iso8601" => iso8601(f, locale),
        "date_time" => date_time(f, locale),
        "date_time_ad" => date_time_ad(f, locale),
        "date" => date(f, locale),
        "time" => time(f, locale),
        "am_pm" => am_pm(f, locale),
        "day_of_month" => day_of_month(f, locale),
        "day_of_week" => day_of_week(f, locale),
        "month" => month(f, locale),
        "month_name" => month_name(f, locale),
        "year" => year(f, locale),
        "century" => century(f, locale),
        "timezone" => timezone(f, locale),
        "date_of_birth" => date_of_birth(f, locale),
        "past_date" => past_date(f, locale),
        "future_date" => future_date(f, locale),
        "past_datetime" => past_datetime(f, locale),
        "future_datetime" => future_datetime(f, locale),
        "date_time_between" => date_time_between(f, locale),
        "date_between" => date_between(f, locale),
        _ => return None,
    })
}
