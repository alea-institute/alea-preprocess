from typing import List, Any

def get_recursive_metadata_buffer(buffer: bytes, server_url: str) -> List[Any]:
    """
    Use the Tika server to extract metadata from a buffer.

    Args:
        buffer: The buffer to extract metadata from.
        server_url: The URL of the Tika server.

    Returns:
        A list of metadata dictionaries.
    """
    ...

def get_recursive_metadata_file(file_path: str, server_url: str) -> List[Any]:
    """
    Use the Tika server to extract metadata from a file.

    Args:
        file_path: The path to the file to extract metadata from.
        server_url: The URL of the Tika server.

    Returns:
        A list of metadata dictionaries.
    """
    ...

def get_recursive_content_html_buffer(buffer: bytes, server_url: str) -> List[Any]:
    """
    Use the Tika server to extract content from an HTML buffer.

    Args:
        buffer: The buffer to extract content from.
        server_url: The URL of the Tika server.

    Returns:
        A list of content dictionaries.
    """
    ...

def get_recursive_content_html_file(file_path: str, server_url: str) -> List[Any]:
    """
    Use the Tika server to extract content from an HTML file.

    Args:
        file_path: The path to the file to extract content from.
        server_url: The URL of the Tika server.

    Returns:
        A list of content dictionaries.
    """
    ...

def get_recursive_content_markdown_buffer(
    buffer: bytes, server_url: str, output_links: bool, output_images: bool
) -> List[Any]:
    """
    Use the Tika server to extract content from a Markdown buffer.

    Args:
        buffer: The buffer to extract content from.
        server_url: The URL of the Tika server.
        output_links: Whether to output links.
        output_images: Whether to output images.

    Returns:
        A list of content dictionaries.
    """
    ...

def get_recursive_content_markdown_file(
    file_path: str, server_url: str, output_links: bool, output_images: bool
) -> List[Any]:
    """
    Use the Tika server to extract content from a Markdown file.

    Args:
        file_path: The path to the file to extract content from.
        server_url: The URL of the Tika server.
        output_links: Whether to output links.
        output_images: Whether to output images.

    Returns:
        A list of content dictionaries.
    """
    ...
