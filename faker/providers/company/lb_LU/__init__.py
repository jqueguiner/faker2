from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Legal forms of companies registered in Luxembourg
    company_suffixes = (
        "S.A.",
        "S.A.",
        "S.A.",
        "S.à r.l.",
        "S.à r.l.",
        "S.à r.l.",
        "S.à r.l.",
        "S.à r.l.-S",
        "S.C.A.",
        "S.C.S.",
        "S.C.",
        "S.C.S.p.",
        "SE",
        "SICAV",
        "SICAF",
        "a.s.b.l.",
        "S.e.n.c.",
    )
