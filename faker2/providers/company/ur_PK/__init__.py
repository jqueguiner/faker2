from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for ur_PK locale (Pakistan, Urdu-language)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} اینڈ {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} برادران {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Realistic Pakistani company / legal-form suffixes in Nastaliq script.
    company_suffixes = (
        "پرائیویٹ لمیٹڈ",
        "لمیٹڈ",
        "اینڈ کمپنی",
        "اینڈ سنز",
        "کارپوریشن",
        "انڈسٹریز",
        "ٹریڈرز",
        "انٹرپرائزز",
        "ملز",
        "گروپ",
    )
