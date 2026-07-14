from .. import Provider as PersonProvider


class Provider(PersonProvider):
    """
    A Faker provider for Malagasy (mg_MG) personal names in the Latin script,
    which is the official orthography of the Malagasy language.

    Malagasy given names are frequently short, meaning-bearing words (e.g.
    "Faly" = joy, "Soa" = good, "Tiana" = beloved). Family names are typically
    long compounds, very often prefixed with the Merina honorific particle
    "Ra-" / "Rado-" / "Andria-" (e.g. Rakotomalala, Randriamampionona,
    Andrianarisoa). Both given names and family names are drawn from the native
    Malagasy stock below.
    """

    formats_male = (
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{first_name_male}} {{last_name}}",
        "{{prefix}} {{first_name_male}} {{last_name}}",
        "{{prefix}} {{first_name_male}} {{last_name}}",
    )

    formats_female = (
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{first_name_female}} {{last_name}}",
        "{{prefix}} {{first_name_female}} {{last_name}}",
        "{{prefix}} {{first_name_female}} {{last_name}}",
    )

    formats = formats_male + formats_female

    # Authentic Malagasy male given names.
    first_names_male = (
        "Rakoto",
        "Rabe",
        "Rado",
        "Hery",
        "Faly",
        "Mamy",
        "Ando",
        "Toky",
        "Fidy",
        "Naina",
        "Lova",
        "Andry",
        "Herizo",
        "Njaka",
        "Tojo",
        "Rija",
        "Solofo",
        "Nirina",
        "Mahefa",
        "Fetra",
        "Tsiry",
        "Tiana",
        "Fanilo",
        "Sitraka",
        "Mendrika",
        "Herinjaka",
        "Tafita",
        "Aina",
        "Zo",
        "Miary",
        "Fitahiana",
        "Rojo",
        "Feno",
        "Hasina",
        "Tantely",
        "Onja",
        "Fenohasina",
        "Manda",
        "Nomena",
        "Harena",
        "Tsanta",
        "Iarivo",
        "Herilala",
        "Fanomezantsoa",
        "Tafitasoa",
        "Rivo",
        "Ny Aina",
        "Sedra",
    )

    # Authentic Malagasy female given names.
    first_names_female = (
        "Voahangy",
        "Fanja",
        "Hanta",
        "Nivo",
        "Vola",
        "Soa",
        "Miora",
        "Tahiry",
        "Domoina",
        "Volana",
        "Tsiky",
        "Fara",
        "Ravaka",
        "Malala",
        "Nofy",
        "Vololona",
        "Haingo",
        "Lalao",
        "Fitia",
        "Anja",
        "Hanitra",
        "Voary",
        "Rondro",
        "Bodo",
        "Bako",
        "Noro",
        "Vero",
        "Sahondra",
        "Njara",
        "Mialy",
        "Iharena",
        "Antsa",
        "Voninavoko",
        "Onja",
        "Tojo",
        "Fenosoa",
        "Tsara",
        "Vonjy",
        "Nofinala",
        "Voahirana",
        "Sarobidy",
        "Hasina",
        "Tantely",
        "Ony",
        "Lova",
        "Volatiana",
        "Miangaly",
        "Faniry",
    )

    first_names = first_names_male + first_names_female

    # Authentic Malagasy family names. Most are long compounds prefixed with
    # the honorific particles "Ra-", "Rado-", "Rand-" or "Andria-".
    last_names = (
        "Rakotomalala",
        "Rakotoarisoa",
        "Rabemananjara",
        "Randriamampionona",
        "Andrianarisoa",
        "Rasoanaivo",
        "Ratsimbazafy",
        "Rabemanantsoa",
        "Andrianjafy",
        "Razafindrakoto",
        "Rakotondrabe",
        "Ravelojaona",
        "Andriamihaja",
        "Rasolofoson",
        "Ranaivoson",
        "Andriantsitohaina",
        "Rakotobe",
        "Razanamparany",
        "Rakotonirina",
        "Randrianasolo",
        "Andriamanana",
        "Rasoamanarivo",
        "Rabearivelo",
        "Rakotoson",
        "Randriamihaja",
        "Andrianaivo",
        "Razafimahatratra",
        "Rakotovao",
        "Rasolofomanana",
        "Randrianarison",
        "Ravalomanana",
        "Rajaonarimampianina",
        "Rakotonanahary",
        "Ratsiraka",
        "Rabetsara",
        "Razafy",
        "Rajoelina",
        "Andrianampoinimerina",
        "Razafindramboa",
        "Rakotoarivelo",
        "Randrianja",
        "Rasoloarison",
        "Andriamasinoro",
        "Rabekoto",
        "Ramanantsoa",
        "Randriambeloson",
        "Rasamimanana",
        "Andriatsitohaina",
    )

    last_names_male = last_names
    last_names_female = last_names

    prefixes = (
        "Andriamatoa",
        "Ramatoa",
        "Ratompokolahy",
        "Ratompokovavy",
        "Dr.",
        "Prof.",
        "Atoa",
        "Rtoa",
    )
