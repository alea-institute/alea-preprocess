"""
Bytes n-gram extraction.
"""

from typing import Dict

def transform(input_data: bytes, n: int) -> list[list[bytes]]:
    """
    Transform a bytes object into a list of n-grams.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A list of n-grams.
    """
    ...

def extract(input_data: bytes, n: int) -> Dict[bytes, int]:
    """
    Extract byte n-grams from a bytes object.

    Args:
        input_data: The input data.
        n: The n-gram length.

    Returns:
        A dictionary of n-grams and their counts.
    """
    ...
