from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{company_prefix}} «{{last_name}}»",
        "{{company_prefix}} «{{last_name}}-{{last_name}}»",
        "{{company_prefix}} «{{last_name}} і {{last_name}}»",
        "{{last_name}} {{company_suffix}}",
        "{{company_prefix}} «{{last_name}}»",
    )

    # Belarusian legal-form abbreviations used as prefixes.
    company_prefixes = (
        "ААТ",   # адкрытае акцыянернае таварыства
        "ЗАТ",   # закрытае акцыянернае таварыства
        "ТАА",   # таварыства з абмежаванай адказнасцю
        "УП",    # унітарнае прадпрыемства
        "ПУП",   # прыватнае унітарнае прадпрыемства
        "ДУП",   # дзяржаўнае унітарнае прадпрыемства
        "СТАА",  # сумеснае таварыства з абмежаванай адказнасцю
        "ІП",    # індывідуальны прадпрымальнік
    )

    company_suffixes = (
        "і партнёры",
        "Груп",
        "Холдынг",
        "Плюс",
        "Інвест",
        "Трэйд",
        "Сервіс",
    )

    def company_prefix(self) -> str:
        """
        Generate a random Belarusian company legal-form prefix.
        :sample:
        """
        return self.random_element(self.company_prefixes)
