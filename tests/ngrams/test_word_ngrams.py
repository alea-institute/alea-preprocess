# imports

# extension module
import alea_preprocess


def test_transform():
    text = "Hello, world!  What are you doing?"
    result = alea_preprocess.algos.ngrams.words.transform(text, 2)
    assert result == [
        ["Hello", ","],
        [",", "world"],
        ["world", "!"],
        ["!", "What"],
        ["What", "are"],
        ["are", "you"],
        ["you", "doing"],
        ["doing", "?"],
    ]

    result = alea_preprocess.algos.ngrams.words.transform(text, 3)
    assert result == [
        ["Hello", ",", "world"],
        [",", "world", "!"],
        ["world", "!", "What"],
        ["!", "What", "are"],
        ["What", "are", "you"],
        ["are", "you", "doing"],
        ["you", "doing", "?"],
    ]


def test_extract():
    text = "Hello, world"
    result = alea_preprocess.algos.ngrams.words.extract(text, 2)
    assert result == {("Hello", ","): 1, (",", "world"): 1}


def test_extract_empty():
    text = ""
    result = alea_preprocess.algos.ngrams.words.extract(text, 2)
    assert result == {}


def test_spanish():
    n = 100000
    text = "¡Hola, mundo!" * n
    result = alea_preprocess.algos.ngrams.words.extract(text, 2)

    assert result == {
        ("¡", "Hola"): n,
        ("Hola", ","): n,
        (",", "mundo"): n,
        ("mundo", "!"): n,
        ("!", "¡"): n - 1,
    }
