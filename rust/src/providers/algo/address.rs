//! Algorithmic formatters for the `address` provider.
//!
//! Handles the four ALGORITHMIC-tagged formatters in recipes.json:
//!   postcode, country_code, current_country_code, current_country
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ── country data hardcoded from faker2.providers.date_time.Provider.countries ─

static COUNTRIES: &[(&str, &str, &str)] = &[
    ("Andorra", "AD", "AND"),
    ("Afghanistan", "AF", "AFG"),
    ("Antigua and Barbuda", "AG", "ATG"),
    ("Albania", "AL", "ALB"),
    ("Armenia", "AM", "ARM"),
    ("Angola", "AO", "AGO"),
    ("Argentina", "AR", "ARG"),
    ("Austria", "AT", "AUT"),
    ("Australia", "AU", "AUS"),
    ("Azerbaijan", "AZ", "AZE"),
    ("Barbados", "BB", "BRB"),
    ("Bangladesh", "BD", "BGD"),
    ("Belgium", "BE", "BEL"),
    ("Burkina Faso", "BF", "BFA"),
    ("Bulgaria", "BG", "BGR"),
    ("Bahrain", "BH", "BHR"),
    ("Burundi", "BI", "BDI"),
    ("Benin", "BJ", "BEN"),
    ("Brunei Darussalam", "BN", "BRN"),
    ("Bolivia", "BO", "BOL"),
    ("Brazil", "BR", "BRA"),
    ("Bahamas", "BS", "BHS"),
    ("Bhutan", "BT", "BTN"),
    ("Botswana", "BW", "BWA"),
    ("Belarus", "BY", "BLR"),
    ("Belize", "BZ", "BLZ"),
    ("Canada", "CA", "CAN"),
    ("Democratic Republic of the Congo", "CD", "COD"),
    ("Republic of the Congo", "CG", "COG"),
    ("Cote d'Ivoire", "CI", "CIV"),
    ("Chile", "CL", "CHL"),
    ("Cameroon", "CM", "CMR"),
    ("People's Republic of China", "CN", "CHN"),
    ("Colombia", "CO", "COL"),
    ("Costa Rica", "CR", "CRI"),
    ("Cuba", "CU", "CUB"),
    ("Cape Verde", "CV", "CPV"),
    ("Cyprus", "CY", "CYP"),
    ("Czech Republic", "CZ", "CZE"),
    ("Germany", "DE", "DEU"),
    ("Djibouti", "DJ", "DJI"),
    ("Denmark", "DK", "DNK"),
    ("Dominica", "DM", "DMA"),
    ("Dominican Republic", "DO", "DOM"),
    ("Ecuador", "EC", "ECU"),
    ("Estonia", "EE", "EST"),
    ("Egypt", "EG", "EGY"),
    ("Eritrea", "ER", "ERI"),
    ("Ethiopia", "ET", "ETH"),
    ("Finland", "FI", "FIN"),
    ("Fiji", "FJ", "FJI"),
    ("France", "FR", "FRA"),
    ("Gabon", "GA", "GAB"),
    ("Georgia", "GE", "GEO"),
    ("Ghana", "GH", "GHA"),
    ("The Gambia", "GM", "GMB"),
    ("Guinea", "GN", "GIN"),
    ("Greece", "GR", "GRC"),
    ("Guatemala", "GT", "GTM"),
    ("Haiti", "HT", "HTI"),
    ("Guinea-Bissau", "GW", "GNB"),
    ("Guyana", "GY", "GUY"),
    ("Honduras", "HN", "HND"),
    ("Hungary", "HU", "HUN"),
    ("Indonesia", "ID", "IDN"),
    ("Republic of Ireland", "IE", "IRL"),
    ("Israel", "IL", "ISR"),
    ("India", "IN", "IND"),
    ("Iraq", "IQ", "IRQ"),
    ("Iran", "IR", "IRN"),
    ("Iceland", "IS", "ISL"),
    ("Italy", "IT", "ITA"),
    ("Jamaica", "JM", "JAM"),
    ("Jordan", "JO", "JOR"),
    ("Japan", "JP", "JPN"),
    ("Kenya", "KE", "KEN"),
    ("Kyrgyzstan", "KG", "KGZ"),
    ("Kiribati", "KI", "KIR"),
    ("North Korea", "KP", "PRK"),
    ("South Korea", "KR", "KOR"),
    ("Kuwait", "KW", "KWT"),
    ("Lebanon", "LB", "LBN"),
    ("Liechtenstein", "LI", "LIE"),
    ("Liberia", "LR", "LBR"),
    ("Lesotho", "LS", "LSO"),
    ("Lithuania", "LT", "LTU"),
    ("Luxembourg", "LU", "LUX"),
    ("Latvia", "LV", "LVA"),
    ("Libya", "LY", "LBY"),
    ("Madagascar", "MG", "MDG"),
    ("Marshall Islands", "MH", "MHL"),
    ("North Macedonia", "MK", "MKD"),
    ("Mali", "ML", "MLI"),
    ("Myanmar", "MM", "MMR"),
    ("Mongolia", "MN", "MNG"),
    ("Mauritania", "MR", "MRT"),
    ("Malta", "MT", "MLT"),
    ("Mauritius", "MU", "MUS"),
    ("Maldives", "MV", "MDV"),
    ("Malawi", "MW", "MWI"),
    ("Mexico", "MX", "MEX"),
    ("Malaysia", "MY", "MYS"),
    ("Mozambique", "MZ", "MOZ"),
    ("Namibia", "NA", "NAM"),
    ("Niger", "NE", "NER"),
    ("Nigeria", "NG", "NGA"),
    ("Nicaragua", "NI", "NIC"),
    ("Kingdom of the Netherlands", "NL", "NLD"),
    ("Norway", "NO", "NOR"),
    ("Nepal", "NP", "NPL"),
    ("Nauru", "NR", "NRU"),
    ("New Zealand", "NZ", "NZL"),
    ("Oman", "OM", "OMN"),
    ("Panama", "PA", "PAN"),
    ("Peru", "PE", "PER"),
    ("Papua New Guinea", "PG", "PNG"),
    ("Philippines", "PH", "PHL"),
    ("Pakistan", "PK", "PAK"),
    ("Poland", "PL", "POL"),
    ("Portugal", "PT", "PRT"),
    ("Palau", "PW", "PLW"),
    ("Paraguay", "PY", "PRY"),
    ("Qatar", "QA", "QAT"),
    ("Romania", "RO", "ROU"),
    ("Russia", "RU", "RUS"),
    ("Rwanda", "RW", "RWA"),
    ("Saudi Arabia", "SA", "SAU"),
    ("Solomon Islands", "SB", "SLB"),
    ("Seychelles", "SC", "SYC"),
    ("Sudan", "SD", "SDN"),
    ("Sweden", "SE", "SWE"),
    ("Singapore", "SG", "SGP"),
    ("Slovenia", "SI", "SVN"),
    ("Slovakia", "SK", "SVK"),
    ("Sierra Leone", "SL", "SLE"),
    ("San Marino", "SM", "SMR"),
    ("Senegal", "SN", "SEN"),
    ("Somalia", "SO", "SOM"),
    ("Suriname", "SR", "SUR"),
    ("Sao Tome and Principe", "ST", "STP"),
    ("Syria", "SY", "SYR"),
    ("Togo", "TG", "TGO"),
    ("Thailand", "TH", "THA"),
    ("Tajikistan", "TJ", "TJK"),
    ("Turkmenistan", "TM", "TKM"),
    ("Tunisia", "TN", "TUN"),
    ("Tonga", "TO", "TON"),
    ("Turkey", "TR", "TUR"),
    ("Trinidad and Tobago", "TT", "TTO"),
    ("Tuvalu", "TV", "TUV"),
    ("Tanzania", "TZ", "TZA"),
    ("Ukraine", "UA", "UKR"),
    ("Uganda", "UG", "UGA"),
    ("United States", "US", "USA"),
    ("Uruguay", "UY", "URY"),
    ("Uzbekistan", "UZ", "UZB"),
    ("Vatican City", "VA", "VAT"),
    ("Venezuela", "VE", "VEN"),
    ("Vietnam", "VN", "VNM"),
    ("Vanuatu", "VU", "VUT"),
    ("Yemen", "YE", "YEM"),
    ("Zambia", "ZM", "ZMB"),
    ("Zimbabwe", "ZW", "ZWE"),
    ("Algeria", "DZ", "DZA"),
    ("Bosnia and Herzegovina", "BA", "BIH"),
    ("Cambodia", "KH", "KHM"),
    ("Central African Republic", "CF", "CAF"),
    ("Chad", "TD", "TCD"),
    ("Comoros", "KM", "COM"),
    ("Croatia", "HR", "HRV"),
    ("East Timor", "TL", "TLS"),
    ("El Salvador", "SV", "SLV"),
    ("Equatorial Guinea", "GQ", "GNQ"),
    ("Grenada", "GD", "GRD"),
    ("Kazakhstan", "KZ", "KAZ"),
    ("Laos", "LA", "LAO"),
    ("Federated States of Micronesia", "FM", "FSM"),
    ("Moldova", "MD", "MDA"),
    ("Monaco", "MC", "MCO"),
    ("Montenegro", "ME", "MNE"),
    ("Morocco", "MA", "MAR"),
    ("Saint Kitts and Nevis", "KN", "KNA"),
    ("Saint Lucia", "LC", "LCA"),
    ("Saint Vincent and the Grenadines", "VC", "VCT"),
    ("Samoa", "WS", "WSM"),
    ("Serbia", "RS", "SRB"),
    ("South Africa", "ZA", "ZAF"),
    ("Spain", "ES", "ESP"),
    ("Sri Lanka", "LK", "LKA"),
    ("Swaziland", "SZ", "SWZ"),
    ("Switzerland", "CH", "CHE"),
    ("United Arab Emirates", "AE", "ARE"),
    ("United Kingdom", "GB", "GBR"),
    ("Taiwan", "TW", "TWN"),
    ("Palestine", "PS", "PSE"),
];

/// Return the alpha-2 code extracted from a locale string like "en_US" -> "US".
/// Returns `None` if the locale has no underscore or no matching country.
fn locale_to_alpha2(locale: &str) -> Option<&'static str> {
    let country_part = locale.split('_').nth(1)?;
    // Normalise to uppercase for comparison
    let upper = country_part.to_uppercase();
    COUNTRIES
        .iter()
        .find(|(_, a2, _)| a2.eq_ignore_ascii_case(&upper))
        .map(|(_, a2, _)| *a2)
}

/// Return the country name for an alpha-2 code.
fn alpha2_to_name(code: &str) -> Option<&'static str> {
    COUNTRIES
        .iter()
        .find(|(_, a2, _)| a2.eq_ignore_ascii_case(code))
        .map(|(name, _, _)| *name)
}

// ── postcode helper ──────────────────────────────────────────────────────────

fn postcode(f: &Faker, locale: &str) -> String {
    // Try locale-specific postcode_formats first, then en_US.
    let formats = f.lfield(locale, "address", "postcode_formats");
    if !formats.is_empty() {
        let tmpl = &formats[f.rng.below(formats.len())];
        return f.rng.bothify(tmpl).to_uppercase();
    }
    // Bare fallback: 5-digit zip.
    f.rng.numerify("#####")
}

// ── public dispatch ──────────────────────────────────────────────────────────

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "postcode" => postcode(f, locale),

        // country_code(representation="alpha-2") — default alpha-2
        "country_code" => {
            // Try locale data first (already stored in locales.json).
            if let Some(v) = f.lpick(locale, "address", "alpha_2_country_codes") {
                v
            } else {
                let idx = f.rng.below(COUNTRIES.len());
                COUNTRIES[idx].1.to_string()
            }
        }

        // current_country_code — the country portion of the locale, e.g. "US" for en_US
        "current_country_code" => {
            // Extract country code from locale (e.g. "en_US" -> "US").
            if let Some(part) = locale.split('_').nth(1) {
                part.to_uppercase()
            } else {
                // Locale has no country suffix — just return a random alpha-2.
                let idx = f.rng.below(COUNTRIES.len());
                COUNTRIES[idx].1.to_string()
            }
        }

        // current_country — the country whose alpha-2 matches the locale suffix
        "current_country" => {
            if let Some(alpha2) = locale_to_alpha2(locale) {
                alpha2_to_name(alpha2)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| alpha2.to_string())
            } else {
                // No country suffix in locale — pick a random country name.
                let idx = f.rng.below(COUNTRIES.len());
                COUNTRIES[idx].0.to_string()
            }
        }

        _ => return None,
    })
}
