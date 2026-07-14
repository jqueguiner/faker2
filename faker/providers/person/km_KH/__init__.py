from collections import OrderedDict

from .. import Provider as PersonProvider


class Provider(PersonProvider):
    # In Khmer usage the family name is written first, followed by the
    # given name. Weights are arbitrarily assigned.
    formats_male = OrderedDict(
        (
            ("{{last_name}} {{first_name_male}}", 0.97),
            ("{{prefix_male}}{{last_name}} {{first_name_male}}", 0.03),
        )
    )
    formats_female = OrderedDict(
        (
            ("{{last_name}} {{first_name_female}}", 0.97),
            ("{{prefix_female}}{{last_name}} {{first_name_female}}", 0.03),
        )
    )
    formats = OrderedDict(
        (
            ("{{last_name}} {{first_name_male}}", 0.48),
            ("{{last_name}} {{first_name_female}}", 0.48),
            ("{{prefix_male}}{{last_name}} {{first_name_male}}", 0.02),
            ("{{prefix_female}}{{last_name}} {{first_name_female}}", 0.02),
        )
    )

    # Authentic Khmer given names written in Khmer script.
    first_names_male = (
        "សុខា",
        "ចាន់",
        "រ៉ា",
        "ដារ៉ា",
        "វិសាល",
        "សំណាង",
        "ពិសិដ្ឋ",
        "សុវណ្ណ",
        "វុទ្ធី",
        "រតនៈ",
        "ចន្ទ្រា",
        "បញ្ញា",
        "សុផល",
        "ណារិន",
        "វិចិត្រ",
        "ធារ៉ា",
        "មករា",
        "បូរ៉ា",
        "វណ្ណៈ",
        "សំបូរ",
        "រិទ្ធី",
        "សុវត្ថិ",
        "ចំរើន",
        "សុគន្ធ",
        "ភក្តី",
        "វិរៈ",
        "មុនី",
        "ណារ៉ុង",
        "សុជាតិ",
        "វាសនា",
        "រស្មី",
        "ចន្ថា",
        "កុសល",
        "តារា",
        "ធានី",
        "សុវិទ្យា",
        "បញ្ញារិទ្ធ",
        "ចេស្តា",
        "ដារ៉ារិទ្ធ",
        "សុវណ្ណារ៉ា",
        "ពិសិដ្ឋា",
        "ចរិយា",
        "ស្រីនាថ",
    )

    first_names_female = (
        "សុភា",
        "ដាវី",
        "ស្រីមុំ",
        "សុខនី",
        "ស្រីនាង",
        "បូព្ណា",
        "វិមាន",
        "ចន្ថា",
        "សុវណ្ណារី",
        "ស្រីលក្ខណ៍",
        "រ៉ានី",
        "សុគន្ធា",
        "ចន្ទនី",
        "ណារី",
        "ស្រីនិច",
        "ពិសី",
        "ដានី",
        "វិចិត្រា",
        "សុវត្តី",
        "បូផា",
        "ស្រីពៅ",
        "ចន្ថនី",
        "មុន្នី",
        "សុផាណ្ណា",
        "ស្រីនាថ",
        "វាសនា",
        "រតនា",
        "សុជាតា",
        "ណារ៉ុម",
        "ស្រីនិត",
        "ភក្ត្រា",
        "ចាន់ធី",
        "សុវណ្ណី",
        "ដាលីស",
        "ស្រីមុន្នី",
        "បូព្ធា",
        "សុខលី",
        "ចន្ទ្រា",
        "មនីរ័ត្ន",
        "ណានី",
        "ពិម្ព័ណ្ណា",
        "សុខាវី",
        "ស្រីល័ក្ខ",
    )

    # Authentic Khmer (Cambodian) family names written in Khmer script.
    last_names = (
        "សុខ",
        "ចាន់",
        "ហេង",
        "លី",
        "អ៊ុំ",
        "ម៉ៅ",
        "ខៀវ",
        "សេង",
        "យ៉ង",
        "ពៅ",
        "ថាច់",
        "នូ",
        "ខាត់",
        "មាស",
        "វង្ស",
        "ចេង",
        "គឹម",
        "សៀង",
        "ទេព",
        "ប៊ុន",
        "ណុប",
        "ព្រំ",
        "អ៊ាង",
        "ឈិន",
        "ឈួន",
        "ស្រ៊ុន",
        "កែវ",
        "ហុក",
        "ជា",
        "ម៉ក់",
        "ស៊ុន",
        "ដួង",
        "ភី",
        "រស់",
        "ទី",
        "លឹម",
        "អ៊ូ",
        "សំ",
        "ជ័យ",
        "ពិន",
        "តាំង",
        "ស្រេង",
        "ង៉ែត",
    )

    prefixes_male = OrderedDict(
        (
            ("លោក ", 0.7),
            ("បណ្ឌិត ", 0.15),
            ("ឯកឧត្តម ", 0.1),
            ("អ្នកឧកញ៉ា ", 0.05),
        )
    )
    prefixes_female = OrderedDict(
        (
            ("លោកស្រី ", 0.5),
            ("កញ្ញា ", 0.35),
            ("បណ្ឌិត ", 0.1),
            ("លោកជំទាវ ", 0.05),
        )
    )

    prefixes = OrderedDict(
        (
            ("លោក ", 0.4),
            ("លោកស្រី ", 0.3),
            ("កញ្ញា ", 0.2),
            ("បណ្ឌិត ", 0.07),
            ("ឯកឧត្តម ", 0.03),
        )
    )
