# imports

# packages

# extension module
import alea_preprocess


def test_get_segment_indices_from_breakpoints():
    buffer = "a b c"
    breakpoints = [1, 3]
    result = alea_preprocess.algos.unicode.segmentations.get_segment_indices_from_breakpoints(
        buffer, breakpoints
    )
    assert result == [(0, 1, "a"), (1, 3, " b"), (3, 5, " c")]


def test_get_segments_from_breakpoints():
    buffer = "a b c"
    breakpoints = [1, 3]
    result = alea_preprocess.algos.unicode.segmentations.get_segments_from_breakpoints(
        buffer, breakpoints
    )
    assert result == ["a", " b", " c"]


def test_get_grapheme_indices():
    buffer = "a b c"
    result = alea_preprocess.algos.unicode.segmentations.get_grapheme_indices(buffer)
    assert result == [(0, 1, "a"), (1, 2, " "), (2, 3, "b"), (3, 4, " "), (4, 5, "c")]


def test_get_word_indices():
    buffer = "a b  c"
    result = alea_preprocess.algos.unicode.segmentations.get_word_indices(buffer)
    assert result == [(0, 1, "a"), (1, 2, " "), (2, 3, "b"), (3, 5, "  "), (5, 6, "c")]


def test_get_sentence_indices():
    buffer = "Hello, world.  How are you?"
    result = alea_preprocess.algos.unicode.segmentations.get_sentence_indices(buffer)
    assert result == [(0, 15, "Hello, world.  "), (15, 27, "How are you?")]


def test_get_line_indices():
    buffer = "Hello, world.\nHow are you?"
    result = alea_preprocess.algos.unicode.segmentations.get_line_indices(buffer)
    assert result == [(0, 14, "Hello, world.\n"), (14, 26, "How are you?")]


def test_segment_graphemes():
    buffer = "a b c"
    result = alea_preprocess.algos.unicode.segmentations.segment_graphemes(buffer)
    assert result == ["a", " ", "b", " ", "c"]


def test_segment_words():
    buffer = "a b  c"
    result = alea_preprocess.algos.unicode.segmentations.segment_words(buffer)
    assert result == ["a", "b", "c"]


def test_segment_sentences():
    buffer = "Hello, world.  How are you?"
    result = alea_preprocess.algos.unicode.segmentations.segment_sentences(buffer)
    assert result == ["Hello, world.  ", "How are you?"]


def test_segment_lines():
    buffer = "Hello, world.\nHow are you?"
    result = alea_preprocess.algos.unicode.segmentations.segment_lines(buffer)
    assert result == ["Hello, world.\n", "How are you?"]
