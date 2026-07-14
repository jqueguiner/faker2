from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for the mn_MN locale (Mongolian, Cyrillic script)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_sector}} {{company_suffix}}",
        "{{last_name}} {{company_sector}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_sector}}",
    )

    def company_sector(self):
        return self.random_element(self.company_sectors)

    company_sectors = (
        "Технологи",
        "Үйлдвэр",
        "Худалдаа",
        "Групп",
        "Барилга",
        "Үйлчилгээ",
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
