#!/usr/bin/env python

from pathlib import Path

from setuptools import find_packages, setup

here = Path(__file__).resolve().parent
README = (here / "README.rst").read_text(encoding="utf-8")
VERSION = (here / "VERSION").read_text(encoding="utf-8").strip()

excluded_packages = ["docs", "tests", "tests.*"]


# this module can be zip-safe if the zipimporter implements iter_modules or if
# pkgutil.iter_importer_modules has registered a dispatch for the zipimporter.
try:
    import pkgutil
    import zipimport

    zip_safe = (
        hasattr(zipimport.zipimporter, "iter_modules")
        or zipimport.zipimporter in pkgutil.iter_importer_modules.registry.keys()
    )
except AttributeError:
    zip_safe = False

setup(
    name="faker2",
    version=VERSION,
    description="faker2 - fork of Faker with 100-language (Whisper) locale coverage and large name pools for PII pseudonymization.",
    long_description=README,
    long_description_content_type="text/x-rst",
    entry_points={
        "console_scripts": ["faker2=faker2.cli:execute_from_command_line"],
        "pytest11": ["faker = faker2.contrib.pytest.plugin"],
    },
    classifiers=[
        # See https://pypi.org/pypi?%3Aaction=list_classifiers
        "Development Status :: 5 - Production/Stable",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Python :: 3.13",
        "Programming Language :: Python :: 3.14",
        "Programming Language :: Python :: Implementation :: CPython",
        "Programming Language :: Python :: Implementation :: PyPy",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Software Development :: Testing",
        "Topic :: Utilities",
        "License :: OSI Approved :: MIT License",
    ],
    keywords="faker fixtures data test mock generator",
    author="joke2k",
    author_email="joke2k@gmail.com",
    url="https://github.com/joke2k/faker",
    project_urls={
        "Bug Tracker": "https://github.com/joke2k/faker/issues",
        "Changes": "https://github.com/joke2k/faker/blob/master/CHANGELOG.md",
        "Documentation": "http://faker.rtfd.org/",
        "Source Code": "https://github.com/joke2k/faker",
    },
    license="MIT License",
    packages=find_packages(exclude=excluded_packages),
    package_data={
        "faker2": ["py.typed", "proxy.pyi"],
    },
    platforms=["any"],
    zip_safe=zip_safe,
    install_requires=['tzdata; platform_system=="Windows"'],
    extras_require={
        "tzdata": ["tzdata"],
    },
    python_requires=">=3.10",
)
