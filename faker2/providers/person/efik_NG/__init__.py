# Data sources:
# Efik names: https://efikeburutu.org/efik%20naming%20system.html
# Additional references:
# - https://en.wikipedia.org/wiki/Efik_name

from faker2.providers.person import Provider as PersonProvider


class Provider(PersonProvider):
    """
    A Faker provider for generating fake Efik names (Nigeria).
    """

    # Male first names
    first_names_male = [
        "Andem",
        "Antigha",
        "Archibong",
        "Asikpo",
        "Bassey",
        "Duke",
        "Edet",
        "Efefiom",
        "Efiwat",
        "Ekpe",
        "Etim",
        "Henshaw",
        "Mesembe",
        "Namondo",
        "Ndiyo",
        "Nkese",
        "Nyong",
        "Odionka",
        "Orok",
    ]

    # Female first names
    first_names_female = [
        "Ankwa",
        "Ansa",
        "Asari",
        "Atim",
        "Edemanwan",
        "Efioanwan",
        "Efiokanwan",
        "Eke",
        "Ekanem",
        "Ekerette",
        "Eyoanwan",
        "Inyang",
        "Itamanwan",
        "Minika",
        "Nsikak",
        "Okoho",
        "Orokanwan",
        "Udobong",
    ]

    # Combined list
    first_names = first_names_male + first_names_female

    # Prefixes
    prefixes_male = ["Mr.", "Dr.", "Prof."]
    prefixes_female = ["Miss", "Mrs.", "Dr.", "Prof."]

    prefixes = prefixes_male + prefixes_female

    # Last names
    last_names = [
        "Aye",
        "Cobham",
        "Efa",
        "Efiok",
        "Ekeng",
        "Ekpenyong",
        "Essien",
        "Etetim",
        "Effiong",
        "Esu",
        "Eyo",
        "Eyonsa",
        "Ewa",
        "Hogan",
        "Ibok",
        "Inyang",
        "Ita",
        "Itam",
        "Nsa",
        "Offiong",
        "Okokon",
        "Oku",
        "Orok",
        "Otu",
    ]
