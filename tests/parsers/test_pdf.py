import alea_preprocess


def test_pdf_file_1():
    result = alea_preprocess.parsers.pdf.detection.detect_file_type(
        "resources/test1.pdf"
    )
    assert result == alea_preprocess.parsers.pdf.detection.PyDocumentType.Text


def test_pdf_file_2():
    result = alea_preprocess.parsers.pdf.detection.detect_file_type(
        "resources/test2.pdf"
    )
    assert result == alea_preprocess.parsers.pdf.detection.PyDocumentType.ImagePostOCR


def test_pdf_file_3():
    result = alea_preprocess.parsers.pdf.detection.detect_file_type(
        "resources/test3.pdf"
    )
    assert result == alea_preprocess.parsers.pdf.detection.PyDocumentType.ImagePreOCR


def test_pdf_file_4():
    result = alea_preprocess.parsers.pdf.detection.detect_file_type(
        "resources/test4.pdf"
    )
    assert result == alea_preprocess.parsers.pdf.detection.PyDocumentType.Text


def test_pdf_extract_text_simple():
    text = alea_preprocess.parsers.pdf.conversion.extract_file_text(
        "resources/test1.pdf"
    )
    assert "Fair and Competitive Livestock" in text


def test_pdf_extract_markdown():
    text = alea_preprocess.parsers.pdf.conversion.extract_file_markdown(
        "resources/test1.pdf"
    )
    assert "**AGENCY :**  Agricultural Marketing Service, USDA." in text
