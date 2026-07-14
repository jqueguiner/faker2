from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    # Cantonese (yue), Hong Kong — Traditional Chinese script (繁體中文)
    # Hong Kong company names: surname + business-type suffix ending in 有限公司 (Limited).

    formats = (
        "{{last_name}}{{company_suffix}}",
        "{{last_name}}{{last_name}}{{company_suffix}}",
    )

    company_suffixes = (
        "有限公司",
        "集團有限公司",
        "控股有限公司",
        "國際有限公司",
        "企業有限公司",
        "實業有限公司",
        "投資有限公司",
        "科技有限公司",
        "貿易有限公司",
        "發展有限公司",
        "置業有限公司",
        "顧問有限公司",
        "工程有限公司",
        "金融有限公司",
        "物流有限公司",
    )
