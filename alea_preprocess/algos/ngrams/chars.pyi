"""
This module provides functions for extracting character n-grams from a string.
"""

from typing import Dict

def transform(input_data: str, n: int) -> list[list[str]]:
    """
    Transform a string into a list of character n-grams.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A list of character n-grams.
    """
    ...

def extract(input_data: str, n: int) -> Dict[str, int]:
    """
    Extract character n-grams from a string.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A dictionary of character n-grams and their counts.
    """
    ...
