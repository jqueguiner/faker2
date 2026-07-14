from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} a'i Feibion {{company_suffix}}",
        "{{last_name}} a {{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}}, {{last_name}} a {{last_name}} {{company_suffix}}",
    )

    # Welsh (Cymraeg) company suffixes
    # Cyf = Cyfyngedig (= Limited / Ltd)
    company_suffixes = (
        "Cyf",
        "Cyf",
        "Cyf",
        "PLC",
        "Partneriaeth",
        "Cwmni",
        "Cwmni Cyfyngedig",
        "Grŵp",
    )
