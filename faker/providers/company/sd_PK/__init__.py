from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for sd_PK locale (Pakistan, Sindhi-language)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}} ۽ سنگتي {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Common company/legal-form suffixes used in Pakistan, in the Sindhi script.
    company_suffixes = (
        "لميٽيڊ",
        "پرائيويٽ لميٽيڊ",
        "ڪمپني",
        "ڪارپوريشن",
        "انڊسٽريز",
        "ٽريڊرز",
        "اينڊ سنز",
        "گروپ",
        "انٽرپرائزز",
    )
