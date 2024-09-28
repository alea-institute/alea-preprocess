def nfd_str(buffer: str) -> str:
    """
    Returns a buffer normalized to NFD (Normalization Form Decomposition).

    Transforms the buffer into Normalization Form D (NFD).

    Args:
        buffer (str): A string to be normalized.

    Returns:
        str: A string representing the buffer normalized to NFD.
    """
    ...

def nfkd_str(buffer: str) -> str:
    """
    Returns a buffer normalized to NFKD (Normalization Form Compatibility Decomposition).

    Transforms the buffer into Normalization Form KD (NFKD).

    Args:
        buffer (str): A string to be normalized.

    Returns:
        str: A string representing the buffer normalized to NFKD.
    """
    ...

def nfc_str(buffer: str) -> str:
    """
    Returns a buffer normalized to NFC (Normalization Form Composition).

    Transforms the buffer into Normalization Form C (NFC).

    Args:
        buffer (str): A string to be normalized.

    Returns:
        str: A string representing the buffer normalized to NFC.
    """
    ...

def nfkc_str(buffer: str) -> str:
    """
    Returns a buffer normalized to NFKC (Normalization Form Compatibility Composition).

    Transforms the buffer into Normalization Form KC (NFKC).

    Args:
        buffer (str): A string to be normalized.

    Returns:
        str: A string representing the buffer normalized to NFKC.
    """
    ...
