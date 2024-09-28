# imports

# extension module
import alea_preprocess


def test_transform():
    text = "Hello, world"
    result = alea_preprocess.algos.ngrams.categories.transform_category(text, 2)
    assert result == [
        ["Lu", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Po"],
        ["Po", "Zs"],
        ["Zs", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Ll"],
        ["Ll", "Ll"],
    ]

    result = alea_preprocess.algos.ngrams.categories.transform_category(text, 3)
    assert result == [
        ["Lu", "Ll", "Ll"],
        ["Ll", "Ll", "Ll"],
        ["Ll", "Ll", "Ll"],
        ["Ll", "Ll", "Po"],
        ["Ll", "Po", "Zs"],
        ["Po", "Zs", "Ll"],
        ["Zs", "Ll", "Ll"],
        ["Ll", "Ll", "Ll"],
        ["Ll", "Ll", "Ll"],
        ["Ll", "Ll", "Ll"],
    ]


def test_extract_category():
    text = "Hello, world!"
    result = alea_preprocess.algos.ngrams.categories.extract_category(text, 2)
    assert result.get(("Lu", "Ll")) == 1
    assert result.get(("Ll", "Ll")) == 7
    assert result.get(("Ll", "Po")) == 2


def test_extract_category_empty():
    text = ""
    result = alea_preprocess.algos.ngrams.categories.extract_category(text, 2)
    assert result == {}


def test_extract_category_groups():
    text = "Hello, world!"
    result = alea_preprocess.algos.ngrams.categories.extract_category_group(text, 2)
    assert result.get(("L", "L")) == 8
    assert result.get(("L", "P")) == 2


def test_extract_category_groups_empty():
    text = ""
    result = alea_preprocess.algos.ngrams.categories.extract_category_group(text, 2)
    assert result == {}


def test_spanish():
    text = "¡Hola, mundo!"
    result = alea_preprocess.algos.ngrams.categories.extract_category(text, 2)
    assert result.get(("Po", "Lu")) == 1
    assert result.get(("Lu", "Ll")) == 1


def test_spanish_groups():
    text = "¡Hola, mundo!"
    result = alea_preprocess.algos.ngrams.categories.extract_category_group(text, 2)
    assert result.get(("P", "L")) == 1
    assert result.get(("L", "L")) == 7


def test_kanji():
    text = "こんにちは、世界！"
    result = alea_preprocess.algos.ngrams.categories.extract_category(text, 2)
    assert result.get(("Lo", "Lo")) == 5
    assert result.get(("Lo", "Po")) == 2


def test_kanji_groups():
    text = "こんにちは、世界！"
    result = alea_preprocess.algos.ngrams.categories.extract_category_group(text, 2)
    assert result.get(("L", "L")) == 5
    assert result.get(("L", "P")) == 2
