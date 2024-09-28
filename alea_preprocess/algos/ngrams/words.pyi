"""
This module provides functions for extracting traditional "word" n-grams.
"""

from typing import Dict, Tuple

def transform(input_data: str, n: int) -> list[Tuple[str, ...]]:
    """
    Transform a string into a list of word n-grams.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A list of word n-grams.
    """
    ...

def extract(input_data: str, n: int) -> Dict[Tuple[str, ...], int]:
    """
    Extract word n-grams from a string.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A dictionary of n-grams and their counts.
    """
    ...
