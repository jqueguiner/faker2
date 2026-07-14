from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """
    A Faker provider for Malagasy (mg_MG) company names.

    Madagascar is a Francophone country, so registered businesses commonly use
    French legal-form suffixes (SARL, SA, SARLU, Ets, & Cie) alongside the
    native Malagasy descriptor "Orinasa" (company / enterprise). Company names
    are typically built from a founder's family name plus such a suffix.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} & {{last_name}} {{company_suffix}}",
        "{{company_prefix}} {{last_name}}",
        "{{company_prefix}} {{last_name}} {{company_suffix}}",
    )

    company_prefixes = (
        "Orinasa",
        "Etablissements",
        "Groupe",
    )

    company_suffixes = (
        "SARL",
        "SA",
        "SARLU",
        "SAS",
        "Ets",
        "& Cie",
        "GIE",
        "Orinasa",
    )

    def company_prefix(self) -> str:
        """
        :example: 'Orinasa'
        """
        return self.random_element(self.company_prefixes)
