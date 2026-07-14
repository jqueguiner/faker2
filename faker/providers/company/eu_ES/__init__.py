from collections import OrderedDict

from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    Provider for company names for the eu_ES locale (Basque, Euskara).

    Legal entity abbreviations reflect forms commonly used in the Basque
    Country, including the cooperative forms for which the region is known
    (e.g. Mondragon).

    Sources:
    - https://en.wikipedia.org/wiki/List_of_legal_entity_types_by_country
    - https://eu.wikipedia.org/wiki/Kooperatiba
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} eta {{last_name}} {{company_suffix}}",
    )

    company_suffixes = OrderedDict(
        [
            ("S.A.", 0.20),
            ("S.L.", 0.30),
            ("S.L.U.", 0.08),
            ("S.A.U.", 0.05),
            ("Koop. E.", 0.17),
            ("S. Koop.", 0.12),
            ("Elkartea", 0.08),
        ]
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
