from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    A Faker provider for generating fake Zimbabwean company names.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} & {{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Company suffixes commonly seen on Zimbabwean registered entities.
    company_suffixes = (
        "(Pvt) Ltd",
        "(Private) Limited",
        "Limited",
        "Holdings",
        "Investments",
        "Enterprises",
        "Trading (Pvt) Ltd",
        "Cooperative",
        "PLC",
    )
