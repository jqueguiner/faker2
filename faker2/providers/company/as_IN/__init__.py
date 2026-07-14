from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Implement company provider for ``as_IN`` locale.

    Assamese language, written in the Bengali-Assamese (Eastern Nagari) script.
    Company suffixes reflect common Indian corporate designations.
    """

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
        "{{last_name}} আৰু {{last_name}} {{company_suffix}}",
    )

    company_suffixes = (
        "লিমিটেড",
        "প্ৰাইভেট লিমিটেড",
        "গোট",
        "ইণ্ডাষ্ট্ৰীজ",
        "এণ্টাৰপ্ৰাইজ",
        "কৰ্পোৰেচন",
        "আৰু পুত্ৰসকল",
        "ট্ৰেডাৰ্ছ",
        "এজেন্সী",
        "এলএলপি",
    )
