from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} און {{last_name}}",
        "{{last_name}}",
    )

    # Yiddish commercial suffixes plus transliterated German legal forms
    # realistic for businesses in Germany.
    company_suffixes = (
        "און זין",
        "און קאמפאני",
        "און שותפים",
        "האנדל",
        "פאבריק",
        "אינדוסטריע",
        "געזעלשאפט",
        "פאַרלאג",
        "קאאָפעראַטיוו",
        "געשעפט",
        "גמב״ה",
        "אַ״ג",
    )
