from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    A Faker provider for Somali (so_SO) company names.

    Somali business names are typically built from a founder's or family
    name followed by a company-type suffix, most commonly the borrowed
    English forms "Ltd", "Company", "Group", "Enterprises" or the Somali
    "Shirkadda" (company) / "Ganacsi" (trade) descriptors.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} & {{last_name}} {{company_suffix}}",
        "{{company_prefix}} {{last_name}}",
        "{{company_prefix}} {{last_name}} {{company_suffix}}",
    )

    company_prefixes = (
        "Shirkadda",
        "Ganacsiga",
        "Xarunta",
    )

    company_suffixes = (
        "Ltd",
        "Co.",
        "Company",
        "Group",
        "Enterprises",
        "Trading",
        "Holdings",
        "& Sons",
        "Shirkad",
        "Ganacsi",
    )

    def company_prefix(self) -> str:
        """
        :example: 'Shirkadda'
        """
        return self.random_element(self.company_prefixes)
