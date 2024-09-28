from typing import List, Tuple

def get_segment_indices_from_breakpoints(
    buffer: str, breakpoints: List[int]
) -> List[Tuple[int, int, str]]:
    """
    Generates segments with their start and end indices from a list of breakpoints.

    Args:
        buffer (str): The input string to segment.
        breakpoints (List[int]): A list of indices where the string should be split.

    Returns:
        List[Tuple[int, int, str]]: A list of tuples, each containing the start index, end index, and the segment as a string.
    """
    ...

def get_segments_from_breakpoints(buffer: str, breakpoints: List[int]) -> List[str]:
    """
    Generates segments from a list of breakpoints.

    Args:
        buffer (str): The input string to segment.
        breakpoints (List[int]): A list of indices where the string should be split.

    Returns:
        List[str]: A list of strings, each representing a segment.
    """
    ...

def get_grapheme_indices(buffer: str) -> List[Tuple[int, int, str]]:
    """
    Segments the input string into grapheme clusters with their indices.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[Tuple[int, int, str]]: A list of tuples, each containing the start index, end index, and the grapheme cluster as a string.
    """
    ...

def get_word_indices(buffer: str) -> List[Tuple[int, int, str]]:
    """
    Segments the input string into words with their indices.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[Tuple[int, int, str]]: A list of tuples, each containing the start index, end index, and the word as a string.
    """
    ...

def get_sentence_indices(buffer: str) -> List[Tuple[int, int, str]]:
    """
    Segments the input string into sentences with their indices.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[Tuple[int, int, str]]: A list of tuples, each containing the start index, end index, and the sentence as a string.
    """
    ...

def get_line_indices(buffer: str) -> List[Tuple[int, int, str]]:
    """
    Segments the input string into lines with their indices.

    This function uses a custom filter to determine line breaks based on specific Unicode properties.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[Tuple[int, int, str]]: A list of tuples, each containing the start index, end index, and the line as a string.
    """
    ...

def segment_graphemes(buffer: str) -> List[str]:
    """
    Segments the input string into grapheme clusters.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[str]: A list of strings, each representing a grapheme cluster.
    """
    ...

def segment_words(buffer: str) -> List[str]:
    """
    Segments the input string into words.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[str]: A list of strings, each representing a word.
    """
    ...

def segment_sentences(buffer: str) -> List[str]:
    """
    Segments the input string into sentences.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[str]: A list of strings, each representing a sentence.
    """
    ...

def segment_lines(buffer: str) -> List[str]:
    """
    Segments the input string into lines.

    This function uses a custom filter to determine line breaks based on specific Unicode properties.

    Args:
        buffer (str): The input string to segment.

    Returns:
        List[str]: A list of strings, each representing a line.
    """
    ...
