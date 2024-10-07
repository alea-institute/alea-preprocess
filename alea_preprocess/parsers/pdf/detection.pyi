from typing import Any
from enum import Enum

class PyDocumentType(Enum):
    Malformed = 0
    Unknown = 1
    Mixed = 2
    Text = 3
    ImagePreOCR = 4
    ImagePostOCR = 5

def detect_buffer_type(buffer: bytes) -> Any:
    """
    Detect the type of document in the buffer.

    Args:
        buffer: The buffer to analyze.

    Returns:
        The type of document detected.
    """
    ...

def detect_file_type(file_path: str) -> Any:
    """
    Detect the type of document in the file.

    Args:
        file_path: The path to the file to analyze.

    Returns:
        The type of document detected.
    """
    ...
