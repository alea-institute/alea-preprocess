"""

#[pyfunction]
pub fn extract_buffer_text(buffer: &[u8]) -> String {
    crate::parsers::pdf::conversion::extract_buffer_text(buffer)
}

#[pyfunction]
pub fn extract_file_text(file_path: &str) -> String {
    crate::parsers::pdf::conversion::extract_file_text(file_path)
}

#[pyfunction]
pub fn extract_buffer_markdown(buffer: &[u8]) -> String {
    crate::parsers::pdf::conversion::extract_buffer_markdown(buffer)
}

#[pyfunction]
pub fn extract_file_markdown(file_path: &str) -> String {
    crate::parsers::pdf::conversion::extract_file_markdown(file_path)
}
"""

def extract_buffer_text(buffer: bytes) -> str:
    """
    Extract the text from the buffer.

    Args:
        buffer: The buffer to extract text from.

    Returns:
        The text extracted from the buffer.
    """
    ...

def extract_file_text(file_path: str) -> str:
    """
    Extract the text from the file.

    Args:
        file_path: The path to the file to extract text from.

    Returns:
        The text extracted from the file.
    """
    ...

def extract_buffer_markdown(buffer: bytes) -> str:
    """
    Extract the text with position from the buffer.

    Args:
        buffer: The buffer to extract text from.

    Returns:
        The text with position extracted from the buffer.
    """
    ...

def extract_file_markdown(file_path: str) -> str:
    """
    Extract the text with position from the file.

    Args:
        file_path: The path to the file to extract text from.

    Returns:
        The text with position extracted from the file.
    """
    ...
