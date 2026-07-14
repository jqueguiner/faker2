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
        'ടെക്നോളജീസ്',
        'ഇൻഡസ്ട്രീസ്',
        'എന്റർപ്രൈസസ്',
        'സൊല്യൂഷൻസ്',
        'ട്രേഡേഴ്സ്',
        'ഗ്രൂപ്പ്',
    )

    # Realistic Kerala/Indian company suffixes in Malayalam script
    company_suffixes = (
        "ലിമിറ്റഡ്",
        "പ്രൈവറ്റ് ലിമിറ്റഡ്",
        "ഗ്രൂപ്പ്",
        "ഇൻഡസ്ട്രീസ്",
        "എന്റർപ്രൈസസ്",
        "ട്രേഡേഴ്സ്",
        "ആൻഡ് കമ്പനി",
        "ആൻഡ് സൺസ്",
        "അസോസിയേറ്റ്സ്",
        "ടെക്നോളജീസ്",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
