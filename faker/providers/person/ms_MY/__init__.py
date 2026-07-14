from .. import Provider as PersonProvider


class Provider(PersonProvider):
    formats_male = (
        "{{first_name_male}} bin {{first_name_male}}",
        "{{first_name_male}} bin {{last_name}}",
        "{{first_name_male}} {{last_name}}",
        "{{prefix_male}} {{first_name_male}} bin {{last_name}}",
    )

    formats_female = (
        "{{first_name_female}} binti {{first_name_male}}",
        "{{first_name_female}} binti {{last_name}}",
        "{{first_name_female}} {{last_name}}",
        "{{prefix_female}} {{first_name_female}} binti {{last_name}}",
    )

    formats = formats_male + formats_female

    first_names_male = (
        "Ahmad", "Muhammad", "Mohd", "Abdul", "Ali", "Amir", "Azman", "Azlan",
        "Faizal", "Farid", "Firdaus", "Hafiz", "Haikal", "Hakim", "Hamdan",
        "Haris", "Hasan", "Hakimi", "Idris", "Ikmal", "Iskandar", "Ismail",
        "Jamal", "Kamal", "Khairul", "Zulkifli", "Mohd Ariff", "Nazri",
        "Razak", "Rizal", "Rosli", "Ridzuan", "Saiful", "Shahrul", "Syafiq",
        "Syazwan", "Taufik", "Wan Ahmad", "Yusof", "Zaid", "Zainal", "Zamri",
    )

    first_names_female = (
        "Siti", "Nur", "Nurul", "Aisyah", "Amirah", "Aini", "Azizah", "Fatimah",
        "Farah", "Hana", "Hidayah", "Intan", "Izzati", "Liyana", "Maryam",
        "Nabila", "Nadia", "Najwa", "Noraini", "Norhayati", "Nurhaliza",
        "Puteri", "Rohana", "Rosmah", "Sabariah", "Salmah", "Sharifah",
        "Suraya", "Wan Nor", "Zaleha", "Zarina", "Zulaikha",
    )

    first_names = first_names_male + first_names_female

    last_names = (
        "Abdullah", "Ahmad", "Ali", "Aziz", "Bakar", "Hamid", "Hashim",
        "Hassan", "Hussein", "Ibrahim", "Ismail", "Jaafar", "Kassim", "Latif",
        "Mahmud", "Mansor", "Mohamed", "Musa", "Mustafa", "Osman", "Othman",
        "Rahman", "Rashid", "Salleh", "Samad", "Sulaiman", "Talib", "Wahab",
        "Yaakob", "Yusof", "Zainal", "Zakaria",
    )

    prefixes_male = ("Encik", "Tuan", "Dato'", "Datuk", "Tan Sri", "Dr.")
    prefixes_female = ("Puan", "Cik", "Datin", "Puan Sri", "Dr.")
    prefixes = prefixes_male + prefixes_female
