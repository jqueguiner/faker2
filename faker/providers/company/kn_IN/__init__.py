from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Realistic Karnataka/Indian company suffixes in Kannada script
    company_suffixes = (
        "ಲಿಮಿಟೆಡ್",
        "ಪ್ರೈವೇಟ್ ಲಿಮಿಟೆಡ್",
        "ಗ್ರೂಪ್",
        "ಇಂಡಸ್ಟ್ರೀಸ್",
        "ಎಂಟರ್‌ಪ್ರೈಸಸ್",
        "ಟ್ರೇಡರ್ಸ್",
        "ಅಂಡ್ ಕಂಪನಿ",
        "ಅಂಡ್ ಸನ್ಸ್",
        "ಅಸೋಸಿಯೇಟ್ಸ್",
        "ಟೆಕ್ನಾಲಜೀಸ್",
    )

    def company_suffix(self) -> str:
        return self.random_element(self.company_suffixes)
