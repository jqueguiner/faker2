from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} සහ {{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    company_suffixes = (
        "පෞද්ගලික සමාගම",
        "පීඑල්සී",
        "සමාගම",
        "හෝල්ඩිංග්ස්",
        "ග්‍රෝ‍ප්",
        "ලංකා",
        "ඉන්ටර්නැෂනල්",
        "සහ පුත්‍රයෝ",
        "එන්ටර්ප්‍රයිසස්",
        "ඉන්ඩස්ට්‍රීස්",
    )
