from typing import List

def char_to_category(ch: str) -> str:
    """
    Returns the Unicode general category for a given character.

    Args:
        ch (str): A single character.

    Returns:
        str: A string representing the Unicode general category.
    """
    ...

def char_to_category_group(ch: str) -> str:
    """
    Returns the Unicode general category group for a given character.

    Args:
        ch (str): A single character.

    Returns:
        str: A string representing the Unicode general category group.
    """
    ...

def to_category_vector(input_data: str) -> List[str]:
    """
    Returns the Unicode general category for each character in a given string.

    This method automatically switches between single-threaded and multi-threaded execution
    when the input is longer than 1024 characters.

    Args:
        input_data (str): A string.

    Returns:
        List[str]: A list of strings representing the Unicode general category for each character in the input string.
    """
    ...

def to_category_group_vector(input_data: str) -> List[str]:
    """
    Returns the Unicode general category group for each character in a given string.

    This method automatically switches between single-threaded and multi-threaded execution
    when the input is longer than 1024 characters.

    Args:
        input_data (str): A string.

    Returns:
        List[str]: A list of strings representing the Unicode general category group for each character in the input string.
    """
    ...
