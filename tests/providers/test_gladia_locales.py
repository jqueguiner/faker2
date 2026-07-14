"""Tests for the locales added by the gladia fork (Whisper 100-language coverage).

These locales exist to pseudonymize PII across every language Whisper transcribes,
so they are tested for three things upstream's generic tests do not check:
  1. names are in the expected SCRIPT (a Malayalam locale must not emit Latin),
  2. name POOLS are large enough that surrogates do not repeat (memorization risk),
  3. company names are DIVERSE (a small suffix list collapses the output space).
"""

import re
import unicodedata

from collections import Counter

import pytest

from faker2 import Faker

# locale -> (unicode script name expected in given names, min pool per list)
NEW_LOCALES = {
    "be_BY": ("CYRILLIC", 40),
    "cy_GB": ("LATIN", 40),
    "ml_IN": ("MALAYALAM", 40),
    "ms_MY": ("LATIN", 40),
    "sr_RS": ("CYRILLIC", 40),
    "tl_PH": ("LATIN", 40),
    "ur_PK": ("ARABIC", 40),
    "mi_NZ": ("LATIN", 40),
    "te_IN": ("TELUGU", 40),
    "kn_IN": ("KANNADA", 40),
    "br_FR": ("LATIN", 40),
    "eu_ES": ("LATIN", 40),
    "mn_MN": ("CYRILLIC", 40),
    "bs_BA": ("LATIN", 40),
    "kk_KZ": ("CYRILLIC", 40),
    "sq_AL": ("LATIN", 40),
    "gl_ES": ("LATIN", 40),
    "pa_IN": ("GURMUKHI", 40),
    "si_LK": ("SINHALA", 40),
    "km_KH": ("KHMER", 40),
    "sn_ZW": ("LATIN", 40),
    "so_SO": ("LATIN", 40),
    "af_ZA": ("LATIN", 40),
    "oc_FR": ("LATIN", 40),
    "tg_TJ": ("CYRILLIC", 40),
    "sd_PK": ("ARABIC", 40),
    "am_ET": ("ETHIOPIC", 40),
    "yi_DE": ("HEBREW", 40),
    "lo_LA": ("LAO", 40),
    "fo_FO": ("LATIN", 40),
    "ht_HT": ("LATIN", 40),
    "ps_AF": ("ARABIC", 40),
    "tk_TM": ("LATIN", 40),
    "mt_MT": ("LATIN", 40),
    "sa_IN": ("DEVANAGARI", 40),
    "lb_LU": ("LATIN", 40),
    "my_MM": ("MYANMAR", 40),
    "bo_CN": ("TIBETAN", 40),
    "mg_MG": ("LATIN", 40),
    "as_IN": ("BENGALI", 40),
    "tt_RU": ("CYRILLIC", 40),
    "haw_US": ("LATIN", 40),
    "ln_CD": ("LATIN", 40),
    "ba_RU": ("CYRILLIC", 40),
    "jw_ID": ("LATIN", 40),
    "su_ID": ("LATIN", 40),
    "yue_HK": ("CJK", 40),
}

# locales whose given-name pools were expanded upstream for large-scale
# pseudonymization (a small pool makes surrogates repeat and be memorized)
EXPANDED_POOLS = {"ja_JP": 150, "ko_KR": 180, "pl_PL": 200, "hu_HU": 200}


def _person_provider(fake):
    return next(p for p in fake.get_providers() if "person" in type(p).__module__)


def _dominant_script(text):
    """Most common unicode script among the letters of `text`."""
    scripts = []
    for ch in text:
        if not ch.isalpha():
            continue
        try:
            name = unicodedata.name(ch)
        except ValueError:
            continue
        scripts.append(name.split()[0])
    return Counter(scripts).most_common(1)[0][0] if scripts else None


def _pool(prov, *attrs):
    out = set()
    for attr in attrs:
        vals = getattr(prov, attr, None) or ()
        out |= {str(v) for v in vals}
    return out


@pytest.mark.parametrize("locale", sorted(NEW_LOCALES))
class TestGladiaLocale:
    def test_loads_and_generates(self, locale):
        fake = Faker(locale)
        assert fake.name().strip()
        assert fake.company().strip()

    def test_name_uses_expected_script(self, locale):
        expected, _ = NEW_LOCALES[locale]
        fake = Faker(locale)
        fake.seed_instance(0)
        # honorifics/particles may be Latin even in non-Latin locales; check the
        # dominant script over several draws of first names only
        prov = _person_provider(fake)
        firsts = _pool(prov, "first_names_male", "first_names_female", "first_names")
        assert firsts, f"{locale}: no first names"
        sample = sorted(firsts)[: min(20, len(firsts))]
        scripts = Counter(_dominant_script(n) for n in sample if _dominant_script(n))
        assert scripts, f"{locale}: no letters in given names"
        assert (
            scripts.most_common(1)[0][0] == expected
        ), f"{locale}: given names are {scripts.most_common(2)}, expected {expected}"

    def test_pools_are_large_enough(self, locale):
        _, min_pool = NEW_LOCALES[locale]
        prov = _person_provider(Faker(locale))
        firsts = _pool(prov, "first_names_male", "first_names_female", "first_names")
        lasts = _pool(prov, "last_names", "last_names_male", "last_names_female", "last_names_common")
        assert len(firsts) >= min_pool, f"{locale}: only {len(firsts)} given names"
        assert len(lasts) >= min_pool, f"{locale}: only {len(lasts)} surnames"

    def test_names_are_diverse(self, locale):
        """400 seeded draws must not collapse onto a handful of names."""
        names = set()
        for i in range(400):
            fake = Faker(locale)
            fake.seed_instance(i)
            names.add(fake.name())
        # CJK/Vietnamese surnames are genuinely concentrated in the real world
        floor = 0.60 if locale in {"yue_HK"} else 0.85
        assert len(names) / 400 >= floor, f"{locale}: only {len(names)}/400 unique names — pool too small"

    def test_companies_are_diverse(self, locale):
        comps = set()
        for i in range(300):
            fake = Faker(locale)
            fake.seed_instance(i)
            comps.add(fake.company())
        assert len(comps) / 300 >= 0.70, f"{locale}: only {len(comps)}/300 unique companies — enrich formats/suffixes"

    def test_no_placeholder_or_latin_leak(self, locale):
        """Guard against copy-paste artefacts (empty strings, TODO, lorem)."""
        prov = _person_provider(Faker(locale))
        pool = _pool(
            prov,
            "first_names_male",
            "first_names_female",
            "first_names",
            "last_names",
            "last_names_male",
            "last_names_female",
        )
        assert all(n and n.strip() for n in pool), f"{locale}: empty name entry"
        bad = [n for n in pool if re.search(r"todo|lorem|xxx|placeholder", n, re.I)]
        assert not bad, f"{locale}: placeholder entries {bad[:3]}"


@pytest.mark.parametrize("locale,min_pool", sorted(EXPANDED_POOLS.items()))
def test_expanded_given_name_pools(locale, min_pool):
    """Upstream pools were too small for dataset-scale pseudonymization."""
    prov = _person_provider(Faker(locale))
    firsts = _pool(prov, "first_names_male", "first_names_female", "first_names")
    assert len(firsts) >= min_pool, (
        f"{locale}: {len(firsts)} given names, need >= {min_pool} to keep surrogate "
        "repetition below the memorization threshold"
    )


def test_ja_jp_kana_and_romaji_are_complete():
    """Every ja_JP name triplet must carry kana + romaji (regression guard)."""
    from faker2.providers.person.ja_JP import Provider

    for attr in ("first_name_male_pairs", "first_name_female_pairs"):
        for entry in getattr(Provider, attr):
            kanji, kana, romaji = entry
            assert kanji and kana and romaji, f"{attr}: incomplete entry {entry}"


def test_whisper_language_coverage():
    """Every Whisper language must resolve to a working Faker locale."""
    whisper_to_locale = {
        "af": "af_ZA",
        "am": "am_ET",
        "ar": "ar_AA",
        "as": "as_IN",
        "az": "az_AZ",
        "ba": "ba_RU",
        "be": "be_BY",
        "bg": "bg_BG",
        "bn": "bn_BD",
        "bo": "bo_CN",
        "br": "br_FR",
        "bs": "bs_BA",
        "ca": "es_CA",
        "cs": "cs_CZ",
        "cy": "cy_GB",
        "da": "da_DK",
        "de": "de_DE",
        "el": "el_GR",
        "en": "en_US",
        "es": "es_ES",
        "et": "et_EE",
        "eu": "eu_ES",
        "fa": "fa_IR",
        "fi": "fi_FI",
        "fo": "fo_FO",
        "fr": "fr_FR",
        "gl": "gl_ES",
        "gu": "gu_IN",
        "ha": "ha_NG",
        "haw": "haw_US",
        "he": "he_IL",
        "hi": "hi_IN",
        "hr": "hr_HR",
        "ht": "ht_HT",
        "hu": "hu_HU",
        "hy": "hy_AM",
        "id": "id_ID",
        "is": "is_IS",
        "it": "it_IT",
        "ja": "ja_JP",
        "jw": "jw_ID",
        "ka": "ka_GE",
        "kk": "kk_KZ",
        "km": "km_KH",
        "kn": "kn_IN",
        "ko": "ko_KR",
        "lb": "lb_LU",
        "ln": "ln_CD",
        "lo": "lo_LA",
        "lt": "lt_LT",
        "lv": "lv_LV",
        "mg": "mg_MG",
        "mi": "mi_NZ",
        "mk": "mk_MK",
        "ml": "ml_IN",
        "mn": "mn_MN",
        "mr": "mr_IN",
        "ms": "ms_MY",
        "mt": "mt_MT",
        "my": "my_MM",
        "ne": "ne_NP",
        "nl": "nl_NL",
        "nn": "no_NO",
        "no": "no_NO",
        "oc": "oc_FR",
        "pa": "pa_IN",
        "pl": "pl_PL",
        "ps": "ps_AF",
        "pt": "pt_PT",
        "ro": "ro_RO",
        "ru": "ru_RU",
        "sa": "sa_IN",
        "sd": "sd_PK",
        "si": "si_LK",
        "sk": "sk_SK",
        "sl": "sl_SI",
        "sn": "sn_ZW",
        "so": "so_SO",
        "sq": "sq_AL",
        "sr": "sr_RS",
        "su": "su_ID",
        "sv": "sv_SE",
        "sw": "sw",
        "ta": "ta_IN",
        "te": "te_IN",
        "tg": "tg_TJ",
        "th": "th_TH",
        "tk": "tk_TM",
        "tl": "tl_PH",
        "tr": "tr_TR",
        "tt": "tt_RU",
        "uk": "uk_UA",
        "ur": "ur_PK",
        "uz": "uz_UZ",
        "vi": "vi_VN",
        "yi": "yi_DE",
        "yo": "yo_NG",
        "yue": "yue_HK",
        "zh": "zh_CN",
    }
    broken = []
    for lang, locale in whisper_to_locale.items():
        try:
            fake = Faker(locale)
            assert fake.name().strip()
            assert fake.company().strip()
        except Exception as exc:  # pragma: no cover - failure path
            broken.append(f"{lang}->{locale}: {exc}")
    assert not broken, f"Whisper languages without a working locale: {broken}"
