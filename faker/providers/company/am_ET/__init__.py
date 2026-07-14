from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    """Company provider for the am_ET locale (Ethiopia, Amharic language)."""

    formats = (
        "{{last_name}} {{company_suffix}}",
        "{{last_name}} እና {{last_name}} {{company_suffix}}",
        "{{last_name}} ወንድማማች {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
    )

    # Realistic Ethiopian legal / trade forms in the Geʼez (Ethiopic) script
    company_suffixes = (
        "አክሲዮን ማህበር",
        "ኃላፊነቱ የተወሰነ የግል ማህበር",
        "ኃ/የተ/የግ/ማ",
        "የንግድ ማህበር",
        "ኩባንያ",
        "ድርጅት",
        "እና ቤተሰብ",
        "ኃላፊነቱ የተወሰነ",
    )
