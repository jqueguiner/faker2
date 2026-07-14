from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} e {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    # French legal company forms used across Occitania.
    company_suffixes = (
        "SA",
        "SARL",
        "SAS",
        "SASU",
        "EURL",
        "SNC",
        "SCOP",
        "e Companhiá",
        "e Filhs",
    )
