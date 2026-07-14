from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} en {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
    )

    # Suid-Afrikaanse maatskappy-agtervoegsels
    # Bpk = Beperk (Ltd), (Edms) Bpk = Eiendoms Beperk (Pty Ltd),
    # Ing = Ingelyf (Inc), BK = Beslote Korporasie (CC)
    company_suffixes = (
        "Bpk",
        "(Edms) Bpk",
        "Edms Bpk",
        "BK",
        "Ing",
        "Vennootskap",
    )
