# imports

# packages
import pytest

# extension module
import alea_preprocess


TEST_STRING_LIST = ["kittens", "sitting", "kitten", "sittin", "saturday", "sunday"]


# hamming distance
@pytest.mark.parametrize(
    "a, b, expected",
    [
        ("kittens", "sitting", 3.0 / 7.0),
        ("kittens", "kitten", 1.0),
        ("kittens", "sittin", 1.0),
    ],
)
def test_hamming_distance(a, b, expected):
    assert alea_preprocess.algos.similarity.strings.hamming_distance(a, b) == expected
    assert alea_preprocess.algos.similarity.strings.hamming_similarity(b, a) == (
        1.0 - expected
    )


# levenshtein distance
@pytest.mark.parametrize(
    "a, b, expected",
    [
        ("kittens", "sitting", 3.0 / 7.0),
        ("kittens", "kitten", 1.0 / 7.0),
        ("kittens", "sittin", 3.0 / 7.0),
    ],
)
def test_levenshtein_distance(a, b, expected):
    assert (
        alea_preprocess.algos.similarity.strings.levenshtein_distance(a, b) == expected
    )
    assert alea_preprocess.algos.similarity.strings.levenshtein_similarity(b, a) == (
        1.0 - expected
    )


# osa distance
@pytest.mark.parametrize(
    "a, b, expected",
    [
        ("kittens", "sitting", 3.0 / 7.0),
        ("kittens", "kitten", 1.0 / 7.0),
        ("kittens", "sittin", 3.0 / 7.0),
    ],
)
def test_osa_distance(a, b, expected):
    assert alea_preprocess.algos.similarity.strings.osa_distance(a, b) == expected
    assert alea_preprocess.algos.similarity.strings.osa_similarity(b, a) == (
        1.0 - expected
    )


# damerau levenshtein distance
@pytest.mark.parametrize(
    "a, b, expected",
    [
        ("kittens", "sitting", 3.0 / 7.0),
        ("kittens", "kitten", 1.0 / 7.0),
        ("kittens", "sittin", 3.0 / 7.0),
    ],
)
def test_damerau_levenshtein_distance(a, b, expected):
    assert (
        alea_preprocess.algos.similarity.strings.damerau_levenshtein_distance(a, b)
        == expected
    )
    assert alea_preprocess.algos.similarity.strings.damerau_levenshtein_similarity(
        b, a
    ) == (1.0 - expected)
