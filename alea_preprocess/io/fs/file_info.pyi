class FileInfo:
    name: str
    short_name: str
    media_type: str
    extension: str
    kind: str

def get_file_info_from_buffer(buffer: bytes) -> FileInfo:
    """
    Get file info from bytes buffer.

    Args:
        buffer: bytes buffer.

    Returns:
        List of FileInfo.
    """
    ...

def get_file_info_from_file(path: str) -> FileInfo:
    """
    Get file info from file.

    Args:
        path (str): File path.

    Returns:
        FileInfo.
    """
    ...
