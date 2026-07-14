from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{company_suffix}} {{last_name}}",
    )

    # Indonesian legal entity / business forms used in Sundanese-speaking
    # West Java (Jawa Barat). See http://id.wikipedia.org/wiki/Jenis_badan_usaha
    company_suffixes = (
        "PT",
        "PT Tbk",
        "CV",
        "UD",
        "PD",
        "Perum",
        "Firma",
        "Koperasi",
        "(Persero) Tbk",
    )
