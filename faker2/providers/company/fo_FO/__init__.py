# Faroese (Føroyskt) company names — Faroe Islands (fo_FO)

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
        "Tøkni",
        "Handil",
        "Bygging",
        "Íðnaður",
        "Bólkur",
        "Tænastur",
    )

    # Faroese legal company forms:
    #   P/F  = partafelag (public limited company)
    #   Sp/f = smápartafelag (private limited company)
    company_suffixes = (
        "P/F",
        "Sp/f",
    )
