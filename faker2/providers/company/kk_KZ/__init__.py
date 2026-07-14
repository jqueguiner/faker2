from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{company_suffix}} «{{last_name}}»",
        "{{company_suffix}} «{{last_name}}-{{last_name}}»",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Kazakhstani legal entity forms
    company_suffixes = (
        "ЖШС",  # Жауапкершілігі шектеулі серіктестік (LLP)
        "АҚ",  # Акционерлік қоғам (JSC)
        "ЖК",  # Жеке кәсіпкер (Individual entrepreneur)
        "ЖАҚ",  # Жабық акционерлік қоғам (Closed JSC)
        "ААҚ",  # Ашық акционерлік қоғам (Open JSC)
        "ҰК",  # Ұлттық компания (National company)
        "КЕ",  # Коммуналдық кәсіпорын (Municipal enterprise)
        "МКК",  # Мемлекеттік коммуналдық кәсіпорын
    )
