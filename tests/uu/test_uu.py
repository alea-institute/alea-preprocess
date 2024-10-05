# imports

# packages

# extension module
import alea_preprocess


ENCODED_SAMPLE = """begin 644 cat.txt
#0V%T
`
end
"""

DECODED_SAMPLE = b"Cat"


def test_encode():
    encoded = alea_preprocess.algos.uu.encode_buffer(DECODED_SAMPLE, "cat.txt", 644)
    assert encoded == ENCODED_SAMPLE


def test_decode():
    name, buffer = alea_preprocess.algos.uu.decode_buffer(ENCODED_SAMPLE)
    assert buffer == DECODED_SAMPLE


def test_edgar():
    with open("resources/edgar-uu-example.txt", "rt") as input_file:
        EDGAR_EXAMPLE = input_file.read()
    name, buffer = alea_preprocess.algos.uu.decode_buffer(EDGAR_EXAMPLE)
    assert name == "img_002.jpg"

    # check jpg header
    assert buffer[:2] == b"\xff\xd8"
    assert buffer[-2:] == b"\xff\xd9"

    # write to tmp to manually review
    with open("/tmp/tmp.jpg", "wb") as output_file:
        output_file.write(buffer)
