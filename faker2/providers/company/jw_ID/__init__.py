from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{company_suffix}} {{last_name}}",
        "{{company_suffix}} {{last_name}} {{last_name}}",
    )

    # Indonesian legal business entity forms (jenis badan usaha),
    # used across Java where Javanese (jw_ID) is spoken.
    company_suffixes = (
        "PT",
        "CV",
        "UD",
        "PD",
        "Perum",
        "Tbk",
        "(Persero) Tbk",
    )
