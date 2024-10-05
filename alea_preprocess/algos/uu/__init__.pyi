# imports
from typing import Tuple

def encode_buffer(buffer: bytes, name: str, mode: int) -> str:
    """
    Returns the uuencoded representation of a buffer.

    Args:
        buffer (bytes): A buffer.
        name (str): The name of the file.
        mode (int): The file mode.

    Returns:
        str: The uuencoded representation of the buffer.
    """
    ...

def decode_buffer(buffer: str) -> Tuple[bytes, str, int]:
    """
    Returns the decoded buffer, name, and mode from a uuencoded buffer.

    Args:
        buffer (str): A uuencoded buffer.

    Returns:
        Tuple[bytes, str, int]: A tuple containing the decoded buffer, name, and mode.
    """
    ...
