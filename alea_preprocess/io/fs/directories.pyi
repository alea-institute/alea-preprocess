from typing import List

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
