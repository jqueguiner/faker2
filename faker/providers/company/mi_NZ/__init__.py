from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    Provider for company names for the mi_NZ locale (te reo Māori,
    Aotearoa New Zealand).

    Company entity suffixes follow those used under New Zealand company law.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} me {{last_name}} {{company_suffix}}",
    )

    company_suffixes = (
        "Limited",
        "Ltd",
        "Holdings Limited",
        "Group Limited",
        "Rōpū",
        "Trust",
    )
