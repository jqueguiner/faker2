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
        "Adam", "Adib", "Aidil", "Aiman", "Akmal", "Aliff", "Amirul", "Ammar",
        "Amsyar", "Anwar", "Aqil", "Arif", "Ariff", "Ashraf", "Asyraf",
        "Ayman", "Azim", "Aziz", "Azri", "Badrul", "Bakri", "Bashir",
        "Danial", "Danish", "Darwish", "Ehsan", "Emir", "Fadhil", "Fahmi",
        "Faiz", "Fauzi", "Fikri", "Firas", "Ghazali", "Hadi", "Hairi",
        "Haziq", "Hazwan", "Helmi", "Hisham", "Ihsan", "Imran", "Iqbal",
        "Irfan", "Izzat", "Johari", "Junaidi", "Kamarul", "Khalid", "Luqman",
        "Mahathir", "Marwan", "Nabil", "Naqib", "Nasir", "Naufal", "Nizam",
        "Nuh", "Omar", "Qayyum", "Rafi", "Raihan", "Rashidi", "Redzuan",
        "Ridwan", "Rizwan", "Sabri", "Sadiq", "Salman", "Shafiq", "Shamsul",
        "Sofian", "Suhaimi", "Syahmi", "Tarmizi", "Umar", "Uzair", "Wafiy",
        "Wazir", "Yaakub", "Yahya", "Zafran", "Zahid", "Zaki", "Zakwan",
        "Zharif", "Zubair", "Zuhair",
    )

    first_names_female = (
        "Siti", "Nur", "Nurul", "Aisyah", "Amirah", "Aini", "Azizah", "Fatimah",
        "Farah", "Hana", "Hidayah", "Intan", "Izzati", "Liyana", "Maryam",
        "Nabila", "Nadia", "Najwa", "Noraini", "Norhayati", "Nurhaliza",
        "Puteri", "Rohana", "Rosmah", "Sabariah", "Salmah", "Sharifah",
        "Suraya", "Wan Nor", "Zaleha", "Zarina", "Zulaikha",
        "Adawiyah", "Adilah", "Afiqah", "Aina", "Alia", "Alya", "Amani",
        "Anis", "Anisah", "Aqilah", "Ariana", "Aryana", "Athirah", "Balqis",
        "Batrisyia", "Damia", "Dayang", "Delisha", "Diana", "Dina", "Elina",
        "Elysha", "Fadhilah", "Farhana", "Fauziah", "Hafizah", "Halimah",
        "Haliza", "Hanis", "Haslinda", "Hasnah", "Hawa", "Husna", "Iman",
        "Insyirah", "Jamilah", "Juliana", "Kamariah", "Khadijah", "Laila",
        "Latifah", "Mahirah", "Maisarah", "Marlina", "Mastura", "Munirah",
        "Nabihah", "Nadhirah", "Nadwa", "Nasuha", "Nazirah", "Nilam", "Nisa",
        "Norashikin", "Norazlina", "Norliza", "Nuraini", "Nurfarah",
        "Nursyahirah", "Qaseh", "Qistina", "Rabiatul", "Rafidah", "Rahimah",
        "Ramlah", "Rania", "Rashidah", "Roslina", "Ruqayyah", "Safiya",
        "Sakinah", "Salwa", "Sanaa", "Shahida", "Shazwani", "Sofea", "Sofia",
        "Suhaila", "Sumayyah", "Syafiqah", "Syahirah", "Syazana", "Umairah",
        "Wafa", "Wardah", "Widad", "Yasmin", "Zahara", "Zahirah", "Zubaidah",
        "Zuriana",
    )

    first_names = first_names_male + first_names_female

    last_names = (
        "Abdullah", "Ahmad", "Ali", "Aziz", "Bakar", "Hamid", "Hashim",
        "Hassan", "Hussein", "Ibrahim", "Ismail", "Jaafar", "Kassim", "Latif",
        "Mahmud", "Mansor", "Mohamed", "Musa", "Mustafa", "Osman", "Othman",
        "Rahman", "Rashid", "Salleh", "Samad", "Sulaiman", "Talib", "Wahab",
        "Yaakob", "Yusof", "Zainal", "Zakaria",
        "Abas", "Abdul Rahman", "Abu Bakar", "Adnan", "Amran", "Anuar",
        "Ariffin", "Awang", "Bahari", "Baharuddin", "Bakri", "Basir",
        "Buang", "Che Wan", "Daud", "Deraman", "Din", "Embong", "Endut",
        "Fauzi", "Ghaffar", "Ghani", "Halim", "Hamdan", "Hamdi", "Hamzah",
        "Harun", "Hussin", "Ishak", "Jalil", "Jamaluddin", "Jantan",
        "Johari", "Jusoh", "Kadir", "Kamaruddin", "Karim", "Khalid",
        "Lokman", "Mahmood", "Majid", "Mamat", "Manaf", "Mat Nor", "Md Noor",
        "Mokhtar", "Muhamad", "Nasir", "Nawawi", "Nawi", "Ngah", "Noor",
        "Nordin", "Omar", "Piah", "Ramli", "Rauf", "Razali", "Rejab", "Saad",
        "Sabri", "Saidin", "Salim", "Sallehuddin", "Sani", "Sarip",
        "Sepawi", "Sepri", "Shafie", "Shamsuddin", "Sidek", "Suhaimi",
        "Taib", "Tahir", "Tajuddin", "Tamby", "Uda", "Umar", "Wahid",
        "Wan Chik", "Yaakub", "Yahaya", "Yob", "Yunus", "Zabidi", "Zahari",
        "Zainuddin", "Zambri", "Zin", "Zulkarnain",
    )

    prefixes_male = ("Encik", "Tuan", "Dato'", "Datuk", "Tan Sri", "Dr.")
    prefixes_female = ("Puan", "Cik", "Datin", "Puan Sri", "Dr.")
    prefixes = prefixes_male + prefixes_female
