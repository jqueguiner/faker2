from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    Provider for company names for the haw_US locale (ʻōlelo Hawaiʻi,
    Hawaiʻi, United States).

    Company entity suffixes follow those used under United States (Hawaiʻi)
    company law, alongside the Hawaiian "Hui" (association / group).
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} a me {{last_name}} {{company_suffix}}",
    )

    company_suffixes = (
        "Inc",
        "LLC",
        "Corporation",
        "Company",
        "Co",
        "Ltd",
        "Hui",
    )
