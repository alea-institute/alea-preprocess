from pathlib import Path

import alea_preprocess

FILE_PATH = Path(__file__)
PROJECT_PATH = FILE_PATH.parent.parent.parent
RESOURCE_PATH = PROJECT_PATH / "resources/"


def test_get_html_file_info_from_buffer():
    test_path = RESOURCE_PATH / "file1.html"
    buffer = test_path.read_bytes()
    file_info = alea_preprocess.io.fs.file_info.get_file_info_from_buffer(buffer)
    assert file_info.name == "HyperText Markup Language"
    assert file_info.short_name == "HTML"
    assert file_info.media_type == "text/html"
    assert file_info.extension == "html"
    assert file_info.kind == "Other"


def test_get_html_file_info_from_file():
    test_path = RESOURCE_PATH / "file1.html"
    file_info = alea_preprocess.io.fs.file_info.get_file_info_from_file(str(test_path))
    assert file_info.name == "HyperText Markup Language"
    assert file_info.short_name == "HTML"
    assert file_info.media_type == "text/html"
    assert file_info.extension == "html"
    assert file_info.kind == "Other"


def test_get_pdf_file_info_from_buffer():
    test_path = RESOURCE_PATH / "test1.pdf"
    buffer = test_path.read_bytes()
    file_info = alea_preprocess.io.fs.file_info.get_file_info_from_buffer(buffer)
    assert file_info.name == "Portable Document Format"
    assert file_info.short_name == "PDF"
    assert file_info.media_type == "application/pdf"
    assert file_info.extension == "pdf"
    assert file_info.kind == "Document"


def test_get_pdf_file_info_from_file():
    test_path = RESOURCE_PATH / "test1.pdf"
    file_info = alea_preprocess.io.fs.file_info.get_file_info_from_file(str(test_path))
    assert file_info.name == "Portable Document Format"
    assert file_info.short_name == "PDF"
    assert file_info.media_type == "application/pdf"
    assert file_info.extension == "pdf"
    assert file_info.kind == "Document"


def test_random_file_info_from_buffer():
    test_path = RESOURCE_PATH / "random1"
    buffer = test_path.read_bytes()
    file_info = alea_preprocess.io.fs.file_info.get_file_info_from_buffer(buffer)
    assert file_info.name == "Arbitrary Binary Data"
    assert file_info.short_name == "BIN"
    assert file_info.media_type == "application/octet-stream"
    assert file_info.extension == "bin"
    assert file_info.kind == "Other"
