from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    # Legal forms used by companies in Brittany (France),
    # plus authentic Breton "and son(s)" suffixes.
    company_suffixes = (
        "SA",
        "SARL",
        "SAS",
        "EIRL",
        "EARL",
        "ha Mab",
        "ha Mibien",
    )
