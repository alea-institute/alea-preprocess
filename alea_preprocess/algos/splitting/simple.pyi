from typing import List

def split_str(
    text: str, min_size: int, max_size: int, split_patterns: List[str]
) -> List[str]:
    """
    Split a string into substrings based on the given parameters.

    Args:
        text: The input string.
        min_size: The minimum size of the substrings.
        max_size: The maximum size of the substrings.
        split_patterns: A list of patterns to split on.

    Returns:
        A list of substrings.
    """
    ...

def split_tokens(
    tokens: List[int], min_size: int, max_size: int, split_patterns: List[List[int]]
) -> List[List[int]]:
    """
    Split a list of tokens into sublists based on the given parameters.

    Args:
        tokens: The input list of tokens.
        min_size: The minimum size of the sublists.
        max_size: The maximum size of the sublists.
        split_patterns: A list of patterns to split on.

    Returns:
        A list of sublists.
    """
    ...

def split_str_list(
    text_list: List[str], min_size: int, max_size: int, split_patterns: List[str]
) -> List[str]:
    """
    Split a list of strings into substrings based on the given parameters.

    Args:
        text_list: The input list of strings.
        min_size: The minimum size of the substrings.
        max_size: The maximum size of the substrings.
        split_patterns: A list of patterns to split on.

    Returns:
        A list of substrings.
    """
    ...

def split_token_list(
    tokens_list: List[List[int]],
    min_size: int,
    max_size: int,
    split_patterns: List[List[int]],
) -> List[List[int]]:
    """
    Split a list of lists of tokens into sublists based on the given parameters.

    Args:
        tokens_list: The input list of lists of tokens.
        min_size: The minimum size of the sublists.
        max_size: The maximum size of the sublists.
        split_patterns: A list of patterns to split on.

    Returns:
        A list of sublists.
    """
    ...
