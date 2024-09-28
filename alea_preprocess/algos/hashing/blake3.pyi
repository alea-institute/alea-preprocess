def hash_bytes(bytes: bytes) -> str:
    """
    Hash bytes using blake3.

    Args:
        bytes: The bytes to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_str(s: str) -> str:
    """
    Hash string using blake3.

    Args:
        s: The string to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_file(path: str) -> str:
    """
    Hash file using blake3.

    Args:
        path: The path to the file to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_gz_file(path: str) -> str:
    """
    Hash gz file using blake3.

    Args:
        path: The path to the gz file to hash.

    Returns:
        A string representation of the hash.
    """
    ...
