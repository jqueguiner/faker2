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
        "Angelo", "Arnel", "Arturo", "Alberto", "Alejandro", "Aldrin",
        "Allan", "Arman", "Armando", "Aurelio", "Baltazar", "Benjamin",
        "Bernardo", "Cesar", "Christian", "Cornelio", "Crispin", "Dante",
        "Dominador", "Edgardo", "Edilberto", "Edmundo", "Edwin", "Elpidio",
        "Enrique", "Esteban", "Federico", "Felipe", "Felix", "Florante",
        "Francisco", "Gabriel", "Gaudencio", "Gerardo", "Gil", "Godofredo",
        "Guillermo", "Hermogenes", "Hilario", "Honorato", "Isagani",
        "Isidro", "Jaime", "Jayson", "Jerome", "Jesus", "Joaquin", "Joel",
        "Jomar", "Jonas", "Jonathan", "Jordan", "Julio", "Kidlat",
        "Leandro", "Leonardo", "Lorenzo", "Luis", "Mariano", "Melchor",
        "Miguel", "Nestor", "Nicanor", "Noel", "Onofre", "Orlando",
        "Oscar", "Pedro", "Perfecto", "Policarpio", "Ponciano", "Prospero",
        "Quirino", "Rafael", "Reynaldo", "Rizal", "Romeo", "Ronaldo",
        "Rosendo", "Santiago", "Segundo", "Serafin", "Severino", "Sixto",
        "Tirso", "Urbano", "Valentin", "Virgilio", "Wenceslao", "Zosimo",
    )

    first_names_female = (
        "Maria", "Corazon", "Ligaya", "Reyna", "Rosario", "Josefina",
        "Imelda", "Gloria", "Luzviminda", "Perlita", "Divina", "Milagros",
        "Marilou", "Cristina", "Aurora", "Bituin", "Dalisay", "Diwata",
        "Liwayway", "Malaya", "Mayumi", "Perla", "Rosalinda", "Sampaguita",
        "Tala", "Trinidad", "Angeles", "Consuelo", "Editha", "Filomena",
        "Herminia", "Leonora", "Natividad", "Remedios", "Soledad",
        "Amanda", "Amelia", "Ana", "Angelica", "Anicia", "Araceli",
        "Aurelia", "Beatriz", "Belen", "Benilda", "Bernadette", "Carmela",
        "Carmen", "Catalina", "Cecilia", "Celestina", "Chona", "Clarita",
        "Concepcion", "Delfina", "Dolores", "Elena", "Elisa", "Elvira",
        "Emilia", "Encarnacion", "Erlinda", "Esperanza", "Estrella",
        "Eufemia", "Evangelina", "Felicidad", "Felisa", "Fidela", "Flora",
        "Fortunata", "Genoveva", "Georgina", "Guadalupe", "Haydee",
        "Isabel", "Jacinta", "Jesusa", "Josefa", "Juana", "Leonila",
        "Lolita", "Loreta", "Lourdes", "Lucia", "Luisa", "Magdalena",
        "Marcela", "Margarita", "Maricel", "Mariposa", "Melinda",
        "Mercedes", "Modesta", "Nenita", "Nerissa", "Norma", "Ofelia",
        "Olivia", "Pacita", "Paz", "Pilar", "Priscila", "Purita",
        "Rebecca", "Regina", "Rizalina", "Rocio", "Rosa", "Rosalie",
        "Rowena", "Salome", "Sinforosa", "Susana", "Teodora", "Teresa",
        "Teresita", "Veronica", "Victoria", "Violeta", "Virginia",
        "Yolanda", "Zenaida",
    )

    first_names = first_names_male + first_names_female

    last_names = (
        "Santos", "Reyes", "Cruz", "Bautista", "Ocampo", "Garcia", "Mendoza",
        "Torres", "Tomas", "Andres", "Marquez", "Romualdez", "Mercado",
        "Aguilar", "Villanueva", "Ramos", "Castillo", "Flores", "Rosario",
        "Fernandez", "Gonzales", "De Guzman", "Dela Cruz", "Del Rosario",
        "Aquino", "Domingo", "Salvador", "Pascual", "Ramirez", "Rivera",
        "Navarro", "Padilla", "Magsaysay", "Lim", "Tan", "Sy",
        "Abad", "Abello", "Abrigo", "Acosta", "Agbayani", "Alcantara",
        "Aldana", "Alonzo", "Alvarez", "Amador", "Antonio", "Aragon",
        "Arceo", "Arellano", "Austria", "Avila", "Balagtas", "Banaag",
        "Bataclan", "Bello", "Beltran", "Benitez", "Bernabe",
        "Buenaventura", "Cabrera", "Cabral", "Cadiz", "Calderon",
        "Camacho", "Carpio", "Caballero", "Corpuz", "Cortez", "Custodio",
        "Dagohoy", "Dimaano", "Dimagiba", "Dimalanta", "Dionisio",
        "Enriquez", "Escano", "Espinosa", "Espiritu", "Estrada",
        "Evangelista", "Fajardo", "Galang", "Gatchalian", "Gatmaitan",
        "Guevarra", "Hernandez", "Ilagan", "Javier", "Katigbak", "Lacson",
        "Lagman", "Lansang", "Laurel", "Legaspi", "Liwanag", "Lopez",
        "Lozada", "Luna", "Mabini", "Macaraeg", "Magbanua", "Malvar",
        "Manalo", "Manansala", "Mangahas", "Maranan", "Matias", "Medina",
        "Nicolas", "Nolasco", "Pangilinan", "Panganiban", "Peralta",
        "Perez", "Quimson", "Quizon", "Rustia", "Sarmiento", "Silvestre",
        "Soriano", "Suarez", "Tolentino", "Trinidad", "Umali", "Valdez",
        "Vergara", "Zamora", "Zapanta", "Zulueta",
    )

    prefixes_male = ("Ginoo", "G.", "Dr.", "Atty.", " Engr.")
    prefixes_female = ("Ginang", "Binibini", "Gng.", "Bb.", "Dr.", "Atty.")
    prefixes = prefixes_male + prefixes_female
