from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    company_suffixes = (
        "д.о.о.",
        "а.д.",
        "д.д.",
        "о.д.",
        "к.д.",
        "предузеће",
    )
