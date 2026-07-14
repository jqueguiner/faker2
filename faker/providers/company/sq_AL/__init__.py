from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    company_suffixes = (
        "sh.p.k.",
        "sh.a.",
        "O.E.",
        "n.sh.",
    )
