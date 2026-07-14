from .. import Provider as PersonProvider


class Provider(PersonProvider):
    """
    A Faker provider for authentic Lingala (lingála) names as used in the
    Democratic Republic of the Congo (ln_CD). Names are written in the Latin
    script and drawn from genuine Lingala given names/vocabulary and common
    Congolese surnames (Lingala, Kikongo and Luba origins). No approximate
    transliteration is used.
    """

    formats_female = (
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}} {{last_name}}",
        "{{prefix_female}} {{first_name_female}} {{last_name}}",
        "{{prefix_female}} {{first_name_female}} {{last_name}}",
    )

    formats_male = (
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}} {{last_name}}",
        "{{prefix_male}} {{first_name_male}} {{last_name}}",
        "{{prefix_male}} {{first_name_male}} {{last_name}}",
    )

    formats = formats_female + formats_male

    # Authentic Lingala male given names / name-words (Latin script)
    first_names_male = (
        "Bosco",
        "Esengo",
        "Lisolo",
        "Mbongo",
        "Bolingo",
        "Bondeko",
        "Elombe",
        "Motema",
        "Molende",
        "Makasi",
        "Mokonzi",
        "Ngoma",
        "Lokole",
        "Mopepe",
        "Ebale",
        "Mabele",
        "Ntoto",
        "Mbula",
        "Sango",
        "Lelo",
        "Mokili",
        "Bomoi",
        "Bopeto",
        "Kembo",
        "Lisanga",
        "Bosolo",
        "Ndeko",
        "Kanda",
        "Ekonda",
        "Bokamba",
        "Lokua",
        "Mongala",
        "Litonga",
        "Bokungu",
        "Salongo",
        "Mputu",
        "Ngenge",
        "Bombole",
        "Lofombo",
        "Nzita",
        "Mbelo",
        "Botefe",
        "Lumbu",
        "Sengele",
        "Likita",
    )

    # Authentic Lingala / pan-Congolese female given names (Latin script)
    first_names_female = (
        "Esengo",
        "Bolingo",
        "Kembo",
        "Bosolo",
        "Lisapo",
        "Ndeko",
        "Bosana",
        "Ngonga",
        "Mbula",
        "Lokasa",
        "Botamba",
        "Bomoi",
        "Bopeto",
        "Bondeko",
        "Motema",
        "Lisanga",
        "Bosembo",
        "Ntima",
        "Molangi",
        "Bokeka",
        "Ngoya",
        "Kesa",
        "Bibiche",
        "Nsimba",
        "Kavira",
        "Lomboto",
        "Mwasi",
        "Kekeli",
        "Ngele",
        "Bolamba",
        "Furaha",
        "Malaika",
        "Mapendo",
        "Neema",
        "Zawadi",
        "Amani",
        "Baraka",
        "Rehema",
        "Tumaini",
        "Upendo",
        "Bokoko",
        "Litumba",
    )

    first_names = first_names_male + first_names_female

    # Common Congolese surnames (Lingala, Kikongo and Luba origins)
    last_names = (
        "Lukusa",
        "Kabongo",
        "Mukendi",
        "Ilunga",
        "Kalala",
        "Ngoyi",
        "Kasongo",
        "Mwamba",
        "Nzuzi",
        "Matadi",
        "Makengo",
        "Lokonda",
        "Bokungu",
        "Mputu",
        "Ngandu",
        "Mbuyi",
        "Kanku",
        "Luboya",
        "Mbombo",
        "Ntumba",
        "Mulumba",
        "Kalombo",
        "Bakala",
        "Lundula",
        "Nzita",
        "Diaka",
        "Kimbeni",
        "Lokwa",
        "Mavungu",
        "Nsimba",
        "Zola",
        "Luzolo",
        "Mbemba",
        "Nkanga",
        "Mabiala",
        "Loseke",
        "Bokele",
        "Mongita",
        "Litumba",
        "Boketsu",
        "Ekofo",
        "Lokombe",
        "Ngalula",
        "Kapinga",
    )

    # Authentic Lingala honorifics
    prefixes_female = ("Mama", "Ndeko", "Koko")
    prefixes_male = ("Tata", "Ndeko", "Mokonzi", "Koko")

    prefixes = ("Tata", "Mama", "Ndeko", "Koko", "Mokonzi")
