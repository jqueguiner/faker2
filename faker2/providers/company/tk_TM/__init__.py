from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Turkmen company/legal-form suffixes.
    company_suffixes = (
        "HJ",  # Hususy Kärhana
        "PJ",  # Paýdarlar Jemgyýeti
        "APJ",  # Açyk Paýdarlar Jemgyýeti
        "ÝGPJ",  # Ýapyk Görnüşli Paýdarlar Jemgyýeti
        "HK",  # Hojalyk Kärhanasy
        "DPKB",  # Döwlet Paýy bilen Kärhana
        "Söwda Öýi",
    )
