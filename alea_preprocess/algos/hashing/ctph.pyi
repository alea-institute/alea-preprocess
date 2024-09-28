def hash_bytes(bytes: bytes) -> str:
    """
    Hash bytes using ctph.

    Args:
        bytes: The bytes to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_str(s: str) -> str:
    """
    Hash string using ctph.

    Args:
        s: The string to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_file(path: str) -> str:
    """
    Hash file using ctph.

    Args:
        path: The path to the file to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def hash_gz_file(path: str) -> str:
    """
    Hash gz file using ctph.

    Args:
        path: The path to the gz file to hash.

    Returns:
        A string representation of the hash.
    """
    ...

def compare(hash1: str, hash2: str) -> float:
    """
    Compare two hashes using ctph.

    Args:
        hash1: The first hash to compare.
        hash2: The second hash to compare.

    Returns:
        A float representing the similarity of the hashes.
    """
    ...
