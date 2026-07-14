from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Implement company provider for ``bo_CN`` locale (Tibetan, Tibetan script).

    Company names combine a Tibetan family/name element (``last_name``) with a
    realistic company suffix written in the native Tibetan script.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}·{{last_name}} {{company_suffix}}",
    )

    # Realistic company suffixes used in the Tibetan-speaking regions of China,
    # written in the Tibetan script.
    company_suffixes = (
        "ཚད་ཡོད་ཀུང་སི",
        "ཁེར་དབང་ཚད་ཡོད་ཀུང་སི",
        "ཚོང་ལས་ཁང",
        "ཐོན་ལས་ཁང",
        "བཟོ་ལས་ཁང",
        "ཚོགས་པ",
        "ཚོང་སྒྱུར་ཁང",
        "ལས་གྲྭ",
        "འདུ་འཛོམས་ཁང",
        "ཆ་འཕྲིན་འཕྲུལ་རིག་ཀུང་སི",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
