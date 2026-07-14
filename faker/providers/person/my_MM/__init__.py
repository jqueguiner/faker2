from collections import OrderedDict

from .. import Provider as PersonProvider


class Provider(PersonProvider):
    """Implement person provider for the ``my_MM`` locale (Burmese, Myanmar script)."""

    # weights are arbitrarily assigned
    formats_male = OrderedDict(
        (
            ("{{first_name_male}} {{last_name}}", 0.45),
            ("{{prefix_male}} {{first_name_male}} {{last_name}}", 0.30),
            ("{{first_name_male}} {{first_name_male}} {{last_name}}", 0.15),
            ("{{prefix_male}} {{first_name_male}} {{first_name_male}} {{last_name}}", 0.10),
        )
    )

    formats_female = OrderedDict(
        (
            ("{{first_name_female}} {{last_name}}", 0.45),
            ("{{prefix_female}} {{first_name_female}} {{last_name}}", 0.30),
            ("{{first_name_female}} {{first_name_female}} {{last_name}}", 0.15),
            ("{{prefix_female}} {{first_name_female}} {{first_name_female}} {{last_name}}", 0.10),
        )
    )

    formats = OrderedDict(
        (
            ("{{first_name_male}} {{last_name}}", 0.225),
            ("{{prefix_male}} {{first_name_male}} {{last_name}}", 0.15),
            ("{{first_name_male}} {{first_name_male}} {{last_name}}", 0.075),
            ("{{prefix_male}} {{first_name_male}} {{first_name_male}} {{last_name}}", 0.05),
            ("{{first_name_female}} {{last_name}}", 0.225),
            ("{{prefix_female}} {{first_name_female}} {{last_name}}", 0.15),
            ("{{first_name_female}} {{first_name_female}} {{last_name}}", 0.075),
            ("{{prefix_female}} {{first_name_female}} {{first_name_female}} {{last_name}}", 0.05),
        )
    )

    # Authentic Burmese given-name elements (Myanmar script)
    first_names_male = (
        "အောင်",
        "မြင့်",
        "ကျော်",
        "ဇော်",
        "ထွန်း",
        "ဝင်း",
        "စိုး",
        "နိုင်",
        "ထက်",
        "မင်း",
        "ကို",
        "သန်း",
        "လှ",
        "တင်",
        "မောင်",
        "ဟန်",
        "ဖြိုး",
        "ရဲ",
        "ဟိန်း",
        "ဆန်း",
        "နန္ဒ",
        "ဌေး",
        "ဉာဏ်",
        "ဇေယျာ",
        "သီဟ",
        "ကောင်း",
        "ခန့်",
        "ပြည့်",
        "ဝဏ္ဏ",
        "ရာဇာ",
        "ပိုင်",
        "စွမ်း",
        "ချစ်",
        "ဆွေ",
        "အေး",
        "ဗိုလ်",
        "တင့်",
        "ဟေမာန်",
        "ကြည်",
        "နေ",
        "ရန်",
        "သူ",
        "ထူး",
        "လွင်",
    )

    first_names_female = (
        "စု",
        "မြ",
        "နှင်း",
        "နီလာ",
        "သဇင်",
        "ခင်",
        "မို့မို့",
        "ဇာ",
        "ခိုင်",
        "နန္ဒာ",
        "ယမင်း",
        "သန္တာ",
        "နွယ်",
        "ဆု",
        "ဖြူ",
        "ဝါ",
        "မွန်",
        "မာ",
        "စန္ဒာ",
        "ထားထား",
        "ချော",
        "ရွှေ",
        "ငြိမ်း",
        "ဝတ်ရည်",
        "ယွန်း",
        "သီရိ",
        "ဧက",
        "မေ",
        "ပွင့်",
        "ယဉ်",
        "အိ",
        "ရတနာ",
        "နန်း",
        "ဂျူး",
        "ဆုမွန်",
        "မိုး",
        "ဖြူဖြူ",
        "ချယ်ရီ",
        "နှင်းဆီ",
        "ခေမာ",
        "ဝေဝေ",
        "ဇင်",
        "ခိုင်ဇာ",
        "မြတ်",
    )

    first_names = first_names_male + first_names_female

    # Burmese naming has no true surnames; these are common trailing
    # name-elements that function as the final part of a full name.
    last_names = (
        "အောင်",
        "ဝင်း",
        "ထွန်း",
        "မောင်",
        "ကျော်",
        "စိုး",
        "မြင့်",
        "ဌေး",
        "လွင်",
        "ဇော်",
        "နိုင်",
        "ထူး",
        "ဝေ",
        "ဟန်",
        "သန်း",
        "အေး",
        "လှ",
        "ကြည်",
        "ဦး",
        "ဖြိုး",
        "ထက်",
        "မင်း",
        "ဟိန်း",
        "နန္ဒ",
        "သူ",
        "ဇင်",
        "နွယ်",
        "မွန်",
        "ခိုင်",
        "ဇာ",
        "ဆန်း",
        "ဝဏ္ဏ",
        "ရီ",
        "ချို",
        "ညွန့်",
        "တင်",
        "ဆွေ",
        "ပိုင်",
        "ဝါ",
        "နီ",
        "မာ",
        "ထိုက်",
        "ရဲ",
        "စံ",
    )

    # Burmese honorific prefixes
    prefixes_male = (
        "ဦး",
        "ကို",
        "မောင်",
        "ဆရာ",
        "ဗိုလ်",
    )

    prefixes_female = (
        "ဒေါ်",
        "မ",
        "ဆရာမ",
    )

    prefixes = prefixes_male + prefixes_female
