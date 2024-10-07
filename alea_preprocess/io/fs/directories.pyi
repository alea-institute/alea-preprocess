from typing import List, Tuple

from alea_preprocess.io.fs.file_info import FileInfo

def get_files(path: str) -> List[str]:
    """
    Get the files in the given directory.

    Args:
        path (str): The path to the directory.

    Returns:
        List[str]: A list of file paths.
    """
    ...

def get_directories(path: str) -> List[str]:
    """
    Get the directories in the given directory.

    Args:
        path (str): The path to the directory.

    Returns:
        List[str]: A list of directory paths.
    """
    ...

def get_entries(path: str) -> List[str]:
    """
    Get the files and directories in the given directory.

    Args:
        path (str): The path to the directory.

    Returns:
        List[str]: A list of file and directory paths.
    """
    ...

def get_all_file_info(path: str) -> List[Tuple[str, FileInfo]]:
    """
    Get all file info from bytes buffer.

    Args:
        path (str): File path.

    Returns:
        List of FileInfo.
    """
    ...
