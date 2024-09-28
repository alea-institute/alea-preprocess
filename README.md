# alea-preprocess

[![PyPI version](https://badge.fury.io/py/alea-preprocess.svg)](https://badge.fury.io/py/alea-preprocess)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python Versions](https://img.shields.io/pypi/pyversions/alea-preprocess.svg)](https://pypi.org/project/alea-preprocess/)

## Description
Efficient, accessible preprocessing routines for pretrain, SFT, and DPO training data preparation.

This library is part of ALEA's open source large language model training pipeline, used in the research and development
of the [KL3M](https://kl3m.ai/) project.


## Installation

Note that this project is a work-in-progress and relies on compiled Rust code. As such, it is recommended to install
the package from GitHub source until a stable release is available.

You can install the latest release from PyPI using pip:
```
pip install alea-preprocess
```

You can install a development version of the package by running the following command:
```
poetry run maturin develop
```


## Examples
Example use cases are currently available under the `tests/` directory.

Additional documentation and examples will be provided in the future.

## License

This ALEA project is released under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have questions about using this ALEA project, please [open an issue](https://github.com/alea-institute/alea-preprocess/issues) on GitHub.

## Learn More

To learn more about ALEA and its software and research projects like KL3M, visit the [ALEA website](https://aleainstitute.ai/).
