from typing import Tuple

from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} & {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Fòm legal antrepriz ann Ayiti (Haitian business/legal entity suffixes)
    company_suffixes: Tuple[str, ...] = (
        "S.A.",
        "SA",
        "S.A.R.L.",
        "SARL",
        "& Fils",
        "& Frères",
        "Enterprise",
        "Group",
    )
