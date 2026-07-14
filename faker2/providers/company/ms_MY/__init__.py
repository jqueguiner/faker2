from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} Holdings {{company_suffix}}",
        "Syarikat {{last_name}} {{company_suffix}}",
    )

    company_suffixes = (
        "Sdn Bhd",
        "Bhd",
        "Enterprise",
        "Trading",
        "Holdings",
        "Corporation",
        "Group",
    )
