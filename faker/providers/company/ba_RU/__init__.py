from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{company_suffix}} «{{last_name}}»",
        "{{company_suffix}} «{{last_name}}-{{last_name}}»",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Russian Federation legal entity forms (Bashkortostan is a republic of Russia)
    company_suffixes = (
        "ООО",  # Общество с ограниченной ответственностью (LLC)
        "АО",  # Акционерное общество (JSC)
        "ПАО",  # Публичное акционерное общество (Public JSC)
        "ЗАО",  # Закрытое акционерное общество (Closed JSC)
        "ОАО",  # Открытое акционерное общество (Open JSC)
        "НАО",  # Непубличное акционерное общество (Non-public JSC)
        "ИП",  # Индивидуальный предприниматель (Individual entrepreneur)
    )
