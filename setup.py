# -*- coding: utf-8 -*-
import os
import re

from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension


def get_version() -> str:
    path = os.path.join(os.path.abspath(os.path.dirname(__file__)), "rsilk", "__init__.py")
    with open(path, "r", encoding="utf-8") as f:
        data = f.read()
    result = re.findall(r"(?<=__version__ = \")\S+(?=\")", data)
    return result[0]


def get_dis():
    with open("README.markdown", "r", encoding="utf-8") as f:
        return f.read()


packages = find_packages(exclude=('test', 'tests.*', "test*"))


def main():
    version: str = get_version()

    dis = get_dis()
    setup(
        name="rsilk",
        version=version,
        url="https://github.com/synodriver/rsilk",
        packages=packages,
        keywords=["silk", "pcm", "audio"],
        description="silk encoding and decoding for python",
        long_description_content_type="text/markdown",
        long_description=dis,
        author="synodriver",
        author_email="diguohuangjiajinweijun@gmail.com",
        python_requires=">=3.6",
        install_requires=[""],
        license='MIT',
        classifiers=[
            "Development Status :: 5 - Production/Stable",
            "Operating System :: OS Independent",
            "License :: OSI Approved :: MIT License",
            "Topic :: Multimedia :: Sound/Audio",
            "Programming Language :: Rust",
            "Programming Language :: Python",
            "Programming Language :: Python :: 3.7",
            "Programming Language :: Python :: 3.8",
            "Programming Language :: Python :: 3.9",
            "Programming Language :: Python :: 3.10",
            "Programming Language :: Python :: Implementation :: CPython"
        ],
        rust_extensions=[RustExtension("rsilk._silk", binding=Binding.PyO3)],
        zip_safe=False,
        include_package_data=True
    )


if __name__ == "__main__":
    main()
