from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "Ets {{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    # Legal company forms used in the Democratic Republic of the Congo
    # (OHADA forms plus historically common Belgian-Congo forms).
    company_suffixes = (
        "SARL",
        "SARLU",
        "SA",
        "SAS",
        "SNC",
        "Sprl",
        "SCS",
        "GIE",
    )
