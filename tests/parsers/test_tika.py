import alea_preprocess
from pathlib import Path

TEST_SERVER_URL = "http://tika-alb-1540561742.us-east-2.elb.amazonaws.com"

FILE_PATH = Path(__file__)
PROJECT_PATH = FILE_PATH.parent.parent.parent
RESOURCE_PATH = PROJECT_PATH / "resources/"


EXAMPLE_1 = b"""
<!DOCTYPE html>
<html>
    <head>
        <title>Page Title</title>
    </head>
    <body>
        <h1>This is a Heading</h1>
        <p>This is a paragraph.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    </body>
</html>
"""


def test_recursive_metadata_buffer():
    results = alea_preprocess.parsers.tika.client.get_recursive_metadata_buffer(
        EXAMPLE_1, TEST_SERVER_URL
    )
    assert results[0]["Content-Type"] == "text/html; charset=ISO-8859-1"
    assert results[0]["Content-Length"] == "266"


def test_recursive_metadata_file():
    results = alea_preprocess.parsers.tika.client.get_recursive_metadata_file(
        str(RESOURCE_PATH / "file1.html"), TEST_SERVER_URL
    )
    assert results[0]["Content-Type"] == "text/html; charset=UTF-8"
    assert results[0]["Content-Length"] == "10594"


def test_recursive_content_html_buffer():
    results = alea_preprocess.parsers.tika.client.get_recursive_content_html_buffer(
        EXAMPLE_1, TEST_SERVER_URL
    )
    assert '<html xmlns="http://www.w3.org/1999/xhtml">' in results[0]


def test_recursive_content_html_file():
    results = alea_preprocess.parsers.tika.client.get_recursive_content_html_file(
        str(RESOURCE_PATH / "test1.docx"), TEST_SERVER_URL
    )
    assert "<p><i>COMMENT: EPA could use the rate of increase in" in results[0]


def test_recursive_content_markdown_buffer():
    results = alea_preprocess.parsers.tika.client.get_recursive_content_markdown_buffer(
        EXAMPLE_1, TEST_SERVER_URL, True, True
    )
    assert (
        results[0]
        == "# This is a Heading\n\nThis is a paragraph.\n\n- Item 1\n- Item 2\n"
    )


def test_recursive_content_markdown_file():
    results = alea_preprocess.parsers.tika.client.get_recursive_content_markdown_file(
        str(RESOURCE_PATH / "test1.docx"), TEST_SERVER_URL, True, True
    )
    assert "**Regulatory Impact Analysis**\n" in results[0]
