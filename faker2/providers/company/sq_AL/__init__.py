from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
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
        "Teknologji",
        "Industri",
        "Ndërtim",
        "Tregti",
        "Grup",
        "Shërbime",
    )

    company_suffixes = (
        "sh.p.k.",
        "sh.a.",
        "O.E.",
        "n.sh.",
    )
