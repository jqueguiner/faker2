# Faroese (Føroyskt) company names — Faroe Islands (fo_FO)

from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} & {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Faroese legal company forms:
    #   P/F  = partafelag (public limited company)
    #   Sp/f = smápartafelag (private limited company)
    company_suffixes = (
        "P/F",
        "Sp/f",
    )
