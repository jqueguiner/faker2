from collections import OrderedDict

from .. import Provider as CompanyProvider


class Provider(CompanyProvider):
    formats = OrderedDict(
        (
            ("ក្រុមហ៊ុន {{last_name}} {{company_suffix}}", 0.5),
            ("{{last_name}} {{company_suffix}}", 0.3),
            ("{{last_name}}-{{last_name}} {{company_suffix}}", 0.15),
            ("ក្រុមហ៊ុន {{last_name}}", 0.05),
        )
    )

    # Realistic Cambodian company legal-form designators in Khmer script.
    company_suffixes = (
        "ខូអិលធីឌី",          # Co., Ltd
        "អិលធីឌី",            # Ltd
        "ភីអិលស៊ី",           # PLC
        "សហគ្រាស",            # Enterprise
        "ចំកាត់",             # Limited
        "គ្រុប",              # Group
        "អ៊ិនធើណេសិនណល",     # International
        "ថ្រេឌីង",           # Trading
    )
