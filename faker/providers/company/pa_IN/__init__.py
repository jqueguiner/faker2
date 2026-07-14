from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Realistic Punjabi/Indian company suffixes in Gurmukhi script
    company_suffixes = (
        "ਲਿਮਟਿਡ",
        "ਪ੍ਰਾਈਵੇਟ ਲਿਮਟਿਡ",
        "ਗਰੁੱਪ",
        "ਇੰਡਸਟਰੀਜ਼",
        "ਐਂਟਰਪ੍ਰਾਈਜ਼ਿਜ਼",
        "ਟ੍ਰੇਡਰਜ਼",
        "ਐਂਡ ਕੰਪਨੀ",
        "ਐਂਡ ਸੰਨਜ਼",
        "ਐਸੋਸੀਏਟਸ",
        "ਟੈਕਨਾਲੋਜੀਜ਼",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
