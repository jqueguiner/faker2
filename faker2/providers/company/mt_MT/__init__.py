from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} u {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Realistic Maltese company/legal-form suffixes.
    company_suffixes = (
        "Ltd",
        "Ltd.",
        "Limited",
        "plc",
        "Co. Ltd",
        "u Wliedu",
        "Holdings Ltd",
        "Group",
        "& Sons Ltd",
    )
