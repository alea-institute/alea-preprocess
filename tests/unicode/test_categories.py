# imports

# packages

# extension module
import alea_preprocess


def test_to_category_vector():
    input_data = "a1! "
    expected = ["Ll", "Nd", "Po", "Zs"]
    result = alea_preprocess.algos.unicode.categories.to_category_vector(input_data)
    assert result == expected


def test_to_category_group_vector():
    input_data = "a1! "
    expected = ["L", "N", "P", "Z"]
    result = alea_preprocess.algos.unicode.categories.to_category_group_vector(
        input_data
    )
    assert result == expected


def test_to_category_vector_long():
    input_data = "a" * 2048  # Equivalent to RAYON_THRESHOLD * 2
    expected = ["Ll"] * 2048
    result = alea_preprocess.algos.unicode.categories.to_category_vector(input_data)
    assert result == expected


# Additional tests for char_to_category and char_to_category_group
def test_char_to_category():
    assert alea_preprocess.algos.unicode.categories.char_to_category("a") == "Ll"
    assert alea_preprocess.algos.unicode.categories.char_to_category("A") == "Lu"
    assert alea_preprocess.algos.unicode.categories.char_to_category("1") == "Nd"
    assert alea_preprocess.algos.unicode.categories.char_to_category(" ") == "Zs"


def test_char_to_category_group():
    assert alea_preprocess.algos.unicode.categories.char_to_category_group("a") == "L"
    assert alea_preprocess.algos.unicode.categories.char_to_category_group("A") == "L"
    assert alea_preprocess.algos.unicode.categories.char_to_category_group("1") == "N"
    assert alea_preprocess.algos.unicode.categories.char_to_category_group(" ") == "Z"
