from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Realistic Telugu (Andhra Pradesh / Telangana) company suffixes in Telugu script
    company_suffixes = (
        "లిమిటెడ్",
        "ప్రైవేట్ లిమిటెడ్",
        "గ్రూప్",
        "ఇండస్ట్రీస్",
        "ఎంటర్‌ప్రైజెస్",
        "ట్రేడర్స్",
        "అండ్ కంపెనీ",
        "అండ్ సన్స్",
        "అసోసియేట్స్",
        "టెక్నాలజీస్",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
