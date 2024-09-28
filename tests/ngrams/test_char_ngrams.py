# imports

# extension module
import alea_preprocess


def test_transform():
    text = "Hello, world"
    result = alea_preprocess.algos.ngrams.chars.transform(text, 2)
    assert result == [
        ["H", "e"],
        ["e", "l"],
        ["l", "l"],
        ["l", "o"],
        ["o", ","],
        [",", " "],
        [" ", "w"],
        ["w", "o"],
        ["o", "r"],
        ["r", "l"],
        ["l", "d"],
    ]

    result = alea_preprocess.algos.ngrams.chars.transform(text, 3)
    assert result == [
        ["H", "e", "l"],
        ["e", "l", "l"],
        ["l", "l", "o"],
        ["l", "o", ","],
        ["o", ",", " "],
        [",", " ", "w"],
        [" ", "w", "o"],
        ["w", "o", "r"],
        ["o", "r", "l"],
        ["r", "l", "d"],
    ]


def test_extract():
    text = "Hello, world"
    result = alea_preprocess.algos.ngrams.chars.extract(text, 2)
    assert result == {
        "He": 1,
        "el": 1,
        "ll": 1,
        "lo": 1,
        "o,": 1,
        ", ": 1,
        " w": 1,
        "wo": 1,
        "or": 1,
        "rl": 1,
        "ld": 1,
    }


def test_extract_empty():
    text = ""
    result = alea_preprocess.algos.ngrams.chars.extract(text, 2)
    assert result == {}


def test_spanish():
    text = "¡Hola, mundo!"
    result = alea_preprocess.algos.ngrams.chars.extract(text, 2)
    assert result.get("¡H") == 1
    assert result.get("nd") == 1


def test_kanji():
    text = "こんにちは"
    result = alea_preprocess.algos.ngrams.chars.extract(text, 2)
    assert result.get("こん") == 1


def test_hindi():
    text = "नमस्ते"
    result = alea_preprocess.algos.ngrams.chars.extract(text, 2)
    assert result.get("नम") == 1
