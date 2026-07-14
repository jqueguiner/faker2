from collections import OrderedDict

from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Implement company provider for the ``my_MM`` locale (Burmese, Myanmar script)."""

    formats = OrderedDict(
        (
            ("{{last_name}} {{company_suffix}}", 0.55),
            ("{{last_name}} {{last_name}} {{company_suffix}}", 0.25),
            ("{{last_name}}-{{last_name}} {{company_suffix}}", 0.20),
        )
    )

    # Common Myanmar business/organisation suffixes (Myanmar script)
    company_suffixes = (
        "ကုမ္ပဏီ လီမိတက်",
        "အများနှင့်သက်ဆိုင်သော ကုမ္ပဏီ လီမိတက်",
        "ကုမ္ပဏီ",
        "ဂရု",
        "ကုမ္ပဏီများ အုပ်စု",
        "ဟိုးလ်ဒင်း",
        "ကော်ပိုရေးရှင်း",
        "အင်တာပရိုက်",
        "ကုန်သွယ်ရေး",
        "စက်မှုလုပ်ငန်း",
        "ဆောက်လုပ်ရေး",
        "ကုန်စည်",
    )
