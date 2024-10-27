def hash_tokens(tokens: list[int], window_size: int, digest_size: int) -> str:
    """
    Hash bytes using ctph.

    Args:
        tokens: A list of tokens to hash.
        window_size: The size of the window for the rolling hash.
        digest_size: The size of the digest.

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
