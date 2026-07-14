from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_sector}} {{company_suffix}}",
        "{{last_name}} {{company_sector}}",
        "{{last_name}} {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_sector}}",
    )

    def company_sector(self):
        return self.random_element(self.company_sectors)

    company_sectors = (
        'उद्यमः',
        'व्यापारः',
        'निर्माणम्',
        'सेवाः',
        'समूहः',
        'प्रतिष्ठानम्',
    )

    # Realistic company/organisation suffixes rendered with authentic
    # Sanskrit (संस्कृत) business vocabulary in the Devanagari script.
    company_suffixes = (
        "निगमः",
        "उद्योगः",
        "संस्था",
        "समूहः",
        "प्रतिष्ठानम्",
        "वाणिज्यम्",
        "उद्यमः",
        "भाण्डागारम्",
        "व्यापारसंस्था",
        "सीमितम्",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
