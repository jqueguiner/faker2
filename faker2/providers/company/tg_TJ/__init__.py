from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{company_suffix}} {{last_name}}",
    )

    company_suffixes = (
        "ҶДММ",
        "ҶСК",
        "ҶСП",
        "КВД",
        "ДХ",
        "СП",
    )
