//! Currency + cryptocurrency formatters (ported).
#![allow(clippy::all)]
use crate::faker::Faker;

#[rustfmt::skip]
static CURRENCIES: &[(&str, &str)] = &[("AED", "United Arab Emirates dirham"), ("AFN", "Afghan afghani"), ("ALL", "Albanian lek"), ("AMD", "Armenian dram"), ("ANG", "Netherlands Antillean guilder"), ("AOA", "Angolan kwanza"), ("ARS", "Argentine peso"), ("AUD", "Australian dollar"), ("AWG", "Aruban florin"), ("AZN", "Azerbaijani manat"), ("BAM", "Bosnia and Herzegovina convertible mark"), ("BBD", "Barbadian dollar"), ("BDT", "Bangladeshi taka"), ("BGN", "Bulgarian lev"), ("BHD", "Bahraini dinar"), ("BIF", "Burundian franc"), ("BMD", "Bermudian dollar"), ("BND", "Brunei dollar"), ("BOB", "Bolivian boliviano"), ("BRL", "Brazilian real"), ("BSD", "Bahamian dollar"), ("BTN", "Bhutanese ngultrum"), ("BWP", "Botswana pula"), ("BYR", "Belarusian ruble"), ("BZD", "Belize dollar"), ("CAD", "Canadian dollar"), ("CDF", "Congolese franc"), ("CHF", "Swiss franc"), ("CLP", "Chilean peso"), ("CNY", "Renminbi"), ("COP", "Colombian peso"), ("CRC", "Costa Rican colón"), ("CUC", "Cuban convertible peso"), ("CUP", "Cuban peso"), ("CVE", "Cape Verdean escudo"), ("CZK", "Czech koruna"), ("DJF", "Djiboutian franc"), ("DKK", "Danish krone"), ("DOP", "Dominican peso"), ("DZD", "Algerian dinar"), ("EGP", "Egyptian pound"), ("ERN", "Eritrean nakfa"), ("ETB", "Ethiopian birr"), ("EUR", "Euro"), ("FJD", "Fijian dollar"), ("FKP", "Falkland Islands pound"), ("GBP", "Pound sterling"), ("GEL", "Georgian lari"), ("GGP", "Guernsey pound"), ("GHS", "Ghanaian cedi"), ("GIP", "Gibraltar pound"), ("GMD", "Gambian dalasi"), ("GNF", "Guinean franc"), ("GTQ", "Guatemalan quetzal"), ("GYD", "Guyanese dollar"), ("HKD", "Hong Kong dollar"), ("HNL", "Honduran lempira"), ("HRK", "Croatian kuna"), ("HTG", "Haitian gourde"), ("HUF", "Hungarian forint"), ("IDR", "Indonesian rupiah"), ("ILS", "Israeli new shekel"), ("NIS", "Israeli new shekel"), ("IMP", "Manx pound"), ("INR", "Indian rupee"), ("IQD", "Iraqi dinar"), ("IRR", "Iranian rial"), ("ISK", "Icelandic króna"), ("JEP", "Jersey pound"), ("JMD", "Jamaican dollar"), ("JOD", "Jordanian dinar"), ("JPY", "Japanese yen"), ("KES", "Kenyan shilling"), ("KGS", "Kyrgyzstani som"), ("KHR", "Cambodian riel"), ("KMF", "Comorian franc"), ("KPW", "North Korean won"), ("KRW", "South Korean won"), ("KWD", "Kuwaiti dinar"), ("KYD", "Cayman Islands dollar"), ("KZT", "Kazakhstani tenge"), ("LAK", "Lao kip"), ("LBP", "Lebanese pound"), ("LKR", "Sri Lankan rupee"), ("LRD", "Liberian dollar"), ("LSL", "Lesotho loti"), ("LTL", "Lithuanian litas"), ("LYD", "Libyan dinar"), ("MAD", "Moroccan dirham"), ("MDL", "Moldovan leu"), ("MGA", "Malagasy ariar"), ("MKD", "Macedonian denar"), ("MMK", "Burmese kyat"), ("MNT", "Mongolian tugrik"), ("MOP", "Macanese pataca"), ("MRO", "Mauritanian ouguiya"), ("MUR", "Mauritian rupee"), ("MVR", "Maldivian rufiyaa"), ("MWK", "Malawian kwacha"), ("MXN", "Mexican peso"), ("MYR", "Malaysian ringgit"), ("MZN", "Mozambican metical"), ("NAD", "Namibian dollar"), ("NGN", "Nigerian naira"), ("NIO", "Nicaraguan córdoba"), ("NOK", "Norwegian krone"), ("NPR", "Nepalese rupee"), ("NZD", "New Zealand dollar"), ("OMR", "Omani rial"), ("PAB", "Panamanian balboa"), ("PEN", "Peruvian sol"), ("PGK", "Papua New Guinean kina"), ("PHP", "Philippine peso"), ("PKR", "Pakistani rupee"), ("PLN", "Polish zloty"), ("PYG", "Paraguayan guarani"), ("QAR", "Qatari riyal"), ("RON", "Romanian leu"), ("RSD", "Serbian dinar"), ("RUB", "Russian ruble"), ("RWF", "Rwandan franc"), ("SAR", "Saudi riyal"), ("SBD", "Solomon Islands dollar"), ("SCR", "Seychellois rupee"), ("SDG", "Sudanese pound"), ("SEK", "Swedish krona"), ("SGD", "Singapore dollar"), ("SHP", "Saint Helena pound"), ("SLL", "Sierra Leonean leone"), ("SOS", "Somali shilling"), ("SPL", "Seborga luigino"), ("SRD", "Surinamese dollar"), ("STD", "São Tomé and Príncipe dobra"), ("SVC", "Salvadoran colón"), ("SYP", "Syrian pound"), ("SZL", "Swazi lilangeni"), ("THB", "Thai baht"), ("TJS", "Tajikistani somoni"), ("TMT", "Turkmenistan manat"), ("TND", "Tunisian dinar"), ("TOP", "Tongan paʻanga"), ("TRY", "Turkish lira"), ("TTD", "Trinidad and Tobago dollar"), ("TVD", "Tuvaluan dollar"), ("TWD", "New Taiwan dollar"), ("TZS", "Tanzanian shilling"), ("UAH", "Ukrainian hryvnia"), ("UGX", "Ugandan shilling"), ("USD", "United States dollar"), ("UYU", "Uruguayan peso"), ("UZS", "Uzbekistani soʻm"), ("VEF", "Venezuelan bolívar"), ("VND", "Vietnamese đồng"), ("VUV", "Vanuatu vatu"), ("WST", "Samoan tālā"), ("XAF", "Central African CFA franc"), ("XCD", "Eastern Caribbean dollar"), ("XDR", "Special drawing rights"), ("XOF", "West African CFA franc"), ("XPF", "CFP franc"), ("YER", "Yemeni rial"), ("ZAR", "South African rand"), ("ZMW", "Zambian kwacha"), ("ZWD", "Zimbabwean dollar")];

#[rustfmt::skip]
static CURRENCY_SYMBOLS: &[(&str, &str)] = &[
    ("AED", ".\u{062f}.\u{0625}"), ("AFN", "\u{060b}"), ("ALL", "Lek"), ("AMD", "\u{058f}"),
    ("ANG", "\u{0192}"), ("AOA", "Kz"), ("ARS", "$"), ("AUD", "$"), ("AWG", "\u{0192}"),
    ("AZN", "\u{20bc}"), ("BAM", "KM"), ("BBD", "$"), ("BDT", "\u{09f3}"), ("BGN", "Lev"),
    ("BHD", "\u{062f}\u{0628}"), ("BIF", "Fr"), ("BMD", "$"), ("BND", "$"), ("BOB", "$"),
    ("BRL", "$"), ("BSD", "$"), ("BTN", "Nu"), ("BWP", "P"), ("BYR", "R"), ("BZD", "$"),
    ("CAD", "$"), ("CDF", "Fr"), ("CHF", "Fr"), ("CLP", "$"), ("CNY", "\u{00a5}"),
    ("COP", "$"), ("CRC", "\u{20a1}"), ("CUC", "$"), ("CUP", "$"), ("CVE", "$"),
    ("CZK", "Kcs"), ("DJF", "Fr"), ("DKK", "kr"), ("DOP", "$"), ("DZD", "\u{062f}\u{062c}\u{200e}"),
    ("EGP", "\u{00a3}"), ("ERN", "Nfk"), ("ETB", "Br"), ("EUR", "\u{20ac}"), ("FJD", "$"),
    ("FKP", "\u{00a3}"), ("GBP", "\u{00a3}"), ("GEL", "\u{20be}"), ("GGP", "\u{00a3}"),
    ("GHS", "\u{20b5}"), ("GIP", "\u{00a3}"), ("GMD", "D"), ("GNF", "FG"), ("GTQ", "Q"),
    ("GYD", "$"), ("HKD", "$"), ("HNL", "L"), ("HRK", "kn"), ("HTG", "G"), ("HUF", "Ft"),
    ("IDR", "Rp"), ("ILS", "\u{20aa}"), ("IMP", "\u{00a3}"), ("INR", "\u{20b9}"),
    ("IQD", "\u{062f}\u{0639}"), ("IRR", "\u{fdfc}"), ("ISK", "kr"), ("JEP", "\u{00a3}"),
    ("JMD", "$"), ("JOD", "JD"), ("JPY", "\u{00a5}"), ("KES", "KSh"), ("KGS", "\u{20c0}"),
    ("KHR", "\u{17db}"), ("KMF", "FC"), ("KPW", "\u{20a9}"), ("KRW", "\u{20a9}"),
    ("KWD", "KD"), ("KYD", "$"), ("KZT", "\u{20b8}"), ("LAK", "\u{20ad}"), ("LBP", "\u{00a3}"),
    ("LKR", "\u{20a8}"), ("LRD", "$"), ("LSL", "M"), ("LTL", "L"), ("LYD", "LD"),
    ("MAD", "Dhs"), ("MDL", "leu"), ("MGA", "Ar"), ("MKD", "DEN"), ("MMK", "Ks"),
    ("MNT", "\u{20ae}"), ("MOP", "$"), ("MRO", "UM"), ("MUR", "\u{20a8}"), ("MVR", "x"),
    ("MWK", "K"), ("MXN", "$"), ("MYR", "RM"), ("MZN", "Mt"), ("NAD", "$"), ("NGN", "\u{20a6}"),
    ("NIO", "$"), ("NIS", "\u{20aa}"), ("NOK", "kr"), ("NPR", "\u{20a8}"), ("NZD", "$"),
    ("OMR", "\u{fdfc}"), ("PAB", "B/"), ("PEN", "S/"), ("PGK", "K"), ("PHP", "\u{20b1}"),
    ("PKR", "\u{20a8}"), ("PLN", "z\u{0142}"), ("PYG", "\u{20b2}"), ("QAR", "\u{fdfc}"),
    ("RON", "leu"), ("RSD", "\u{0434}\u{0438}\u{043d}"), ("RUB", "\u{20bd}"), ("RWF", "F"),
    ("SAR", "\u{fdfc}"), ("SBD", "$"), ("SCR", "\u{20a8}"), ("SDG", "\u{00a3}"), ("SEK", "kr"),
    ("SGD", "$"), ("SHP", "\u{00a3}"), ("SLL", "Le"), ("SOS", "Sh.So."), ("SPL", "L"),
    ("SRD", "$"), ("STD", "Db"), ("SVC", "\u{20a1}"), ("SYP", "\u{00a3}"), ("SZL", "E"),
    ("THB", "\u{0e3f}"), ("TJS", "SM"), ("TMT", "m"), ("TND", "DT"), ("TOP", "\u{00a2}"),
    ("TRY", "\u{20ba}"), ("TTD", "$"), ("TVD", "$"), ("TWD", "$"), ("TZS", "Tsh"),
    ("UAH", "\u{20b4}"), ("UGX", "USh"), ("USD", "$"), ("UYU", "$"), ("UZS", "\u{043b}\u{0432}"),
    ("VEF", "Bs"), ("VND", "\u{20ab}"), ("VUV", "VT"), ("WST", "$"), ("XAF", "Fr"),
    ("XCD", "$"), ("XDR", "SDR"), ("XOF", "Fr"), ("XPF", "Fr"), ("YER", "\u{fdfc}"),
    ("ZAR", "R"), ("ZMW", "K"), ("ZWD", "$"),
];

#[rustfmt::skip]
static CRYPTO: &[(&str, &str)] = &[("AMP", "AMP"), ("AUR", "Auroracoin"), ("BC", "BlackCoin"), ("BTC", "Bitcoin"), ("BURST", "Burstcoin"), ("DASH", "Dash"), ("DOGE", "Dogecoin"), ("EMC", "Emercoin"), ("ETH", "Ethereum"), ("ETC", "Ethereum Classic"), ("GRC", "Gridcoin"), ("KOI", "Coinye"), ("LTC", "Litecoin"), ("MSC", "Omni"), ("MZC", "MazaCoin"), ("NMC", "Namecoin"), ("NXT", "Nxt"), ("POT", "PotCoin"), ("PPC", "Peercoin"), ("TIT", "Titcoin"), ("VTC", "Vertcoin"), ("XDN", "DigitalNote"), ("XMR", "Monero"), ("XPM", "Primecoin"), ("XRP", "Ripple"), ("ZEC", "Zcash"), ("STC", "SwiftCoin"), ("BCN", "Bytecoin"), ("FTH", "Feathercoin"), ("NEO", "NEO"), ("NEM", "XEM"), ("USDT", "Tether"), ("IOTA", "IOTA"), ("DRC", "Decred"), ("WAVES", "Waves Platform"), ("LSK", "Lisk"), ("ZCL", "Zclassic"), ("BCH", "Bitcoin Cash"), ("UBQ", "Ubiq"), ("EOS", "EOS.IO"), ("SRN", "Sirin Labs"), ("TRX", "TRON"), ("ADA", "Cardano")];

fn pick<'a>(f: &Faker, table: &'a [(&str, &str)]) -> &'a (&'a str, &'a str) {
    &table[f.rng.below(table.len())]
}

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "currency" => {
            let c = pick(f, CURRENCIES);
            format!("('{}', '{}')", c.0, c.1)
        }
        "currency_code" => pick(f, CURRENCIES).0.to_string(),
        "currency_name" => pick(f, CURRENCIES).1.to_string(),
        "currency_symbol" => pick(f, CURRENCY_SYMBOLS).1.to_string(),
        "cryptocurrency" => {
            let c = pick(f, CRYPTO);
            format!("('{}', '{}')", c.0, c.1)
        }
        "cryptocurrency_code" => pick(f, CRYPTO).0.to_string(),
        "cryptocurrency_name" => pick(f, CRYPTO).1.to_string(),
        "pricetag" => {
            let code = pick(f, CURRENCIES).0;
            let fmt = f
                .lpick(locale, "currency", "price_formats")
                .unwrap_or_else(|| "#,##0.00".to_string());
            format!("{}\u{00a0}{}", code, f.rng.numerify(&fmt))
        }
        _ => return None,
    })
}
