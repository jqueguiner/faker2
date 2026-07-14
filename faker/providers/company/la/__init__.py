from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} et {{last_name}} {{company_suffix}}",
        "{{last_name}}, {{last_name}} et {{last_name}} {{company_suffix}}",
    )

    # Roman commercial / collegial designations (authentic classical Latin)
    company_suffixes = (
        "et Filii",
        "Societas",
        "Collegium",
        "Fabrica",
        "Officina",
        "et Frater",
    )
