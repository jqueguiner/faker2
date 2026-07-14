import json
import unittest

from importlib import import_module
from pathlib import Path

import faker2

from faker2.config import META_PROVIDERS_MODULES, PROVIDERS
from faker2.generator import random
from faker2.typing import OrderedDictType
from faker2.utils.checksums import calculate_luhn, luhn_checksum
from faker2.utils.datasets import add_ordereddicts
from faker2.utils.distribution import choices_distribution, choices_distribution_unique
from faker2.utils.loading import find_available_locales, find_available_providers, get_path

TEST_DIR = Path(__file__).resolve().parent


class UtilsTestCase(unittest.TestCase):
    def test_choice_distribution(self):
        a = ("a", "b", "c", "d")
        p = (0.5, 0.2, 0.2, 0.1)

        sample = choices_distribution(a, p)[0]
        assert sample in a

        random_state = json.loads((TEST_DIR / "random_state.json").read_text())
        random_state[1] = tuple(random_state[1])

        random.setstate(random_state)
        samples = choices_distribution(a, p, length=100)
        a_pop = len([i for i in samples if i == "a"])
        b_pop = len([i for i in samples if i == "b"])
        c_pop = len([i for i in samples if i == "c"])
        d_pop = len([i for i in samples if i == "d"])

        boundaries = []
        tolerance = 5
        for probability in p:
            boundaries.append([100 * probability + tolerance, 100 * probability - tolerance])

        assert boundaries[0][0] > a_pop > boundaries[0][1]
        assert boundaries[1][0] > b_pop > boundaries[1][1]
        assert boundaries[2][0] > c_pop > boundaries[2][1]
        assert boundaries[3][0] > d_pop > boundaries[3][1]

    def test_choices_distribution_unique(self):
        a = ("a", "b", "c", "d")
        p = (0.25, 0.25, 0.25, 0.25)
        with self.assertRaises(AssertionError):
            choices_distribution_unique(a, p, length=5)

        samples = choices_distribution_unique(a, p, length=4)
        assert len(set(samples)) == len(samples)

    def test_get_path(self):
        result = get_path(faker2)
        assert isinstance(result, str)

    def test_find_available_locales(self):
        result = find_available_locales(PROVIDERS)
        assert len(result) != 0

    def test_find_available_providers(self):
        modules = [import_module(path) for path in META_PROVIDERS_MODULES]
        providers = find_available_providers(modules)

        expected_providers = list(
            map(
                str,
                [
                    "faker2.providers.address",
                    "faker2.providers.automotive",
                    "faker2.providers.bank",
                    "faker2.providers.barcode",
                    "faker2.providers.color",
                    "faker2.providers.company",
                    "faker2.providers.credit_card",
                    "faker2.providers.currency",
                    "faker2.providers.date_time",
                    "faker2.providers.doi",
                    "faker2.providers.emoji",
                    "faker2.providers.file",
                    "faker2.providers.geo",
                    "faker2.providers.internet",
                    "faker2.providers.isbn",
                    "faker2.providers.job",
                    "faker2.providers.lorem",
                    "faker2.providers.misc",
                    "faker2.providers.passport",
                    "faker2.providers.person",
                    "faker2.providers.phone_number",
                    "faker2.providers.profile",
                    "faker2.providers.python",
                    "faker2.providers.sbn",
                    "faker2.providers.ssn",
                    "faker2.providers.user_agent",
                ],
            )
        )
        assert providers == expected_providers

    def test_luhn_checksum(self):
        """
        Tests if a valid checksum is generated
        Example from wiki: https://en.wikipedia.org/wiki/Luhn_algorithm
        """
        check_digit = calculate_luhn("7992739871")
        assert check_digit == 3

    def test_valid_luhn(self):
        """
        Tests if the number has a valid check digit
        Example from wiki https://en.wikipedia.org/wiki/Luhn_algorithm
        """
        assert luhn_checksum("79927398713") == 0

    def test_invalid_luhn(self):
        """
        Tests a number with an invalid check digit
        Example from wiki https://en.wikipedia.org/wiki/Luhn_algorithm
        """
        assert luhn_checksum("79927398714") != 0

    def test_add_ordereddicts(self):
        d1 = OrderedDictType([("a", 1), ("b", 2)])
        d2 = OrderedDictType([("c", 3), ("d", 4)])
        result = add_ordereddicts(d1, d2)
        assert result == OrderedDictType([("a", 1), ("b", 2), ("c", 3), ("d", 4)])
