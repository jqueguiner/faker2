from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for the mn_MN locale (Mongolian, Cyrillic script)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    company_suffixes = (
        "ХХК",
        "ХК",
        "ТӨХК",
        "ТӨААТ",
        "ХЗХ",
        "ББСБ",
        "ХОРШОО",
        "Групп",
    )
