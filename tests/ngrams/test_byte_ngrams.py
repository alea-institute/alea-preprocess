# imports

# extension module
import alea_preprocess


def test_transform():
    text = b"Hello, world"
    result = alea_preprocess.algos.ngrams.binary.transform(text, 2)
    assert result == [
        [ord(c) for c in "He"],
        [ord(c) for c in "el"],
        [ord(c) for c in "ll"],
        [ord(c) for c in "lo"],
        [ord(c) for c in "o,"],
        [ord(c) for c in ", "],
        [ord(c) for c in " w"],
        [ord(c) for c in "wo"],
        [ord(c) for c in "or"],
        [ord(c) for c in "rl"],
        [ord(c) for c in "ld"],
    ]

    result = alea_preprocess.algos.ngrams.binary.transform(text, 3)
    assert result == [
        [ord(c) for c in "Hel"],
        [ord(c) for c in "ell"],
        [ord(c) for c in "llo"],
        [ord(c) for c in "lo,"],
        [ord(c) for c in "o, "],
        [ord(c) for c in ", w"],
        [ord(c) for c in " wo"],
        [ord(c) for c in "wor"],
        [ord(c) for c in "orl"],
        [ord(c) for c in "rld"],
    ]


def test_extract():
    text = b"Hello, world"
    result = alea_preprocess.algos.ngrams.binary.extract(text, 2)
    assert result == {
        b"He": 1,
        b"el": 1,
        b"ll": 1,
        b"lo": 1,
        b"o,": 1,
        b", ": 1,
        b" w": 1,
        b"wo": 1,
        b"or": 1,
        b"rl": 1,
        b"ld": 1,
    }


def test_extract_empty():
    text = b""
    result = alea_preprocess.algos.ngrams.binary.extract(text, 2)
    assert result == {}


def test_spanish():
    text = "¡Hola, mundo!".encode("utf-8")
    result = alea_preprocess.algos.ngrams.binary.extract(text, 2)
    assert result.get(b"\xa1H") == 1
    assert result.get(b"nd") == 1


def test_kanji():
    text = "こんにちは".encode()
    result = alea_preprocess.algos.ngrams.binary.extract(text, 2)
    assert result.get(b"\x81\x93") == 1


def test_hindi():
    # NB: byte order
    text = "नमस्ते".encode()
    result = alea_preprocess.algos.ngrams.binary.extract(text, 2)
    assert result.get(b"\xa4\xb8") == 1
