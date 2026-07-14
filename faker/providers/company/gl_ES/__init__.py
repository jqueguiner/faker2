from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} e {{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    # Spanish legal company forms used in Galicia.
    company_suffixes = (
        "S.A.",
        "S.L.",
        "S.L.U.",
        "S.L.N.E.",
        "S.C.",
        "S. Coop. Galega",
        "e Cía.",
        "S.A.L.",
    )
