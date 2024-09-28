from typing import Dict, Tuple

def transform_category(text: str, n: int) -> list[Tuple[str, ...]]:
    """
    Transform text into a list of Unicode character category n-grams.

    Args:
        text: The text to transform.
        n: The length of the n-grams.

    Returns:
        A list of n-grams.
    """
    ...

def transform_category_group(text: str, n: int) -> list[Tuple[str, ...]]:
    """
    Transform text into a list of Unicode character category groups.

    Args:
        text: The text to transform.
        n: The length of the n-grams.

    Returns:
        A list of n-grams.
    """
    ...

def extract_category(text: str, n: int) -> Dict[Tuple[str, ...], int]:
    """
    Extract Unicode character category n-grams from text.

    Args:
        text: The text to extract n-grams from.
        n: The length of the n-grams to extract.

    Returns:
        A dictionary mapping n-grams to their frequency in the text.
    """
    ...

def extract_category_group(text: str, n: int) -> Dict[Tuple[str, ...], int]:
    """
    Extract Unicode character category groups from text.

    Args:
        text: The bytes to extract n-grams from.
        n: The length of the n-grams to extract.

    Returns:
        A dictionary mapping n-grams to their frequency in the text.
    """
    ...
