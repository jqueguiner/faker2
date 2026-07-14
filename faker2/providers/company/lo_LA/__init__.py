from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = (
        "ບໍລິສັດ {{last_name}} {{company_suffix}}",
        "{{last_name}} {{company_suffix}}",
        "ຫ້າງຮ້ານ {{last_name}}",
        "{{last_name}}-{{last_name}} {{company_suffix}}",
    )

    # Realistic Lao company suffixes written in the Lao script
    company_suffixes = (
        "ຈຳກັດ",
        "ຈຳກັດຜູ້ດຽວ",
        "ມະຫາຊົນ",
        "ກຸ່ມ",
        "ການຄ້າ",
        "ກໍ່ສ້າງ",
        "ບໍລິການ",
        "ຂາເຂົ້າ-ຂາອອກ",
        "ພັດທະນາ",
        "ອຸດສາຫະກຳ",
        "ວິສາຫະກິດ",
        "ແລະ ຫມູ່",
    )
