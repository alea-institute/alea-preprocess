"""
Root package for the alea_preprocess module.
"""

# import submodules
from .alea_preprocess import algos, io, parsers, tasks

# export modules
__all__ = ["algos", "io", "parsers", "tasks"]

# metadata fields
__version__ = "0.1.13"
__author__ = "ALEA Institute (https://aleainstitute.ai)"
__license__ = "MIT"
