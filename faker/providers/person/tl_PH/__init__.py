from .. import Provider as PersonProvider


class Provider(PersonProvider):
    formats_male = (
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{first_name_male}} {{last_name}} {{last_name}}",
        "{{prefix_male}} {{first_name_male}} {{last_name}}",
    )

    formats_female = (
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{first_name_female}} {{last_name}} {{last_name}}",
        "{{prefix_female}} {{first_name_female}} {{last_name}}",
    )

    formats = formats_male + formats_female

    first_names_male = (
        "Jose", "Juan", "Andres", "Emilio", "Antonio", "Manuel", "Ricardo",
        "Eduardo", "Roberto", "Carlos", "Ramon", "Alfredo", "Ernesto",
        "Fernando", "Rodrigo", "Danilo", "Rogelio", "Benigno", "Bayani",
        "Dakila", "Makisig", "Amado", "Bonifacio", "Crisanto", "Diosdado",
        "Efren", "Gregorio", "Ignacio", "Lakan", "Marcelo", "Narciso",
        "Pablo", "Teodoro", "Vicente", "Wilfredo",
    )

    first_names_female = (
        "Maria", "Corazon", "Ligaya", "Reyna", "Rosario", "Josefina",
        "Imelda", "Gloria", "Luzviminda", "Perlita", "Divina", "Milagros",
        "Marilou", "Cristina", "Aurora", "Bituin", "Dalisay", "Diwata",
        "Liwayway", "Malaya", "Mayumi", "Perla", "Rosalinda", "Sampaguita",
        "Tala", "Trinidad", "Angeles", "Consuelo", "Editha", "Filomena",
        "Herminia", "Leonora", "Natividad", "Remedios", "Soledad",
    )

    first_names = first_names_male + first_names_female

    last_names = (
        "Santos", "Reyes", "Cruz", "Bautista", "Ocampo", "Garcia", "Mendoza",
        "Torres", "Tomas", "Andres", "Marquez", "Romualdez", "Mercado",
        "Aguilar", "Villanueva", "Ramos", "Castillo", "Flores", "Rosario",
        "Fernandez", "Gonzales", "De Guzman", "Dela Cruz", "Del Rosario",
        "Aquino", "Domingo", "Salvador", "Pascual", "Ramirez", "Rivera",
        "Navarro", "Bautista", "Padilla", "Magsaysay", "Lim", "Tan", "Sy",
    )

    prefixes_male = ("Ginoo", "G.", "Dr.", "Atty.", " Engr.")
    prefixes_female = ("Ginang", "Binibini", "Gng.", "Bb.", "Dr.", "Atty.")
    prefixes = prefixes_male + prefixes_female
