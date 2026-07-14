from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for ps_AF locale (Afghanistan, Pashto-language)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} او {{last_name}} {{company_suffix}}",
        "د {{last_name}} ګروپ",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}",
    )

    # Realistic Afghan company legal forms / suffixes in Pashto.
    company_suffixes = (
        "لمیټډ",
        "شرکت",
        "محدود شرکت",
        "سهامي شرکت",
        "ګروپ",
        "تصدي",
        "او شرکاء",
        "او زامن",
    )
