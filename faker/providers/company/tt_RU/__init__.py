from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{company_suffix}} «{{last_name}}»",
        "{{company_suffix}} «{{last_name}}-{{last_name}}»",
        "{{last_name}} {{last_name}} {{company_suffix}}",
    )

    # Tatarstan / Russian legal entity forms (Tatar-language abbreviations)
    company_suffixes = (
        "ҖЧҖ",  # Җаваплылыгы чикләнгән җәмгыять (ООО / LLC)
        "АҖ",  # Акционерлык җәмгыяте (АО / JSC)
        "ПАҖ",  # Публик акционерлык җәмгыяте (ПАО)
        "ААҖ",  # Ачык акционерлык җәмгыяте (ОАО)
        "ЯАҖ",  # Ябык акционерлык җәмгыяте (ЗАО)
        "ШК",  # Шәхси кәсәпче (ИП / Individual entrepreneur)
        "ДУП",  # Дәүләт унитар предприятиесе (ГУП)
        "ҖШ",  # Җитештерү кооперативы / production entity
    )
