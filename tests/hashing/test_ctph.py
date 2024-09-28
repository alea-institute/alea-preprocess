# imports

# packages
import pytest

# extension module
import alea_preprocess


@pytest.mark.parametrize("window_size", [16, 32, 64, 128])
@pytest.mark.parametrize("precision", [8, 16, 32, 64])
def test_compare_html(window_size, precision):
    values = []
    for digest_size in [8, 16, 32, 64]:
        for _ in range(10):
            x1 = alea_preprocess.algos.hashing.ctph.hash_file(
                "resources/file1.html", window_size, digest_size, precision
            )
            x2 = alea_preprocess.algos.hashing.ctph.hash_file(
                "resources/file2.html", window_size, digest_size, precision
            )
            values.append(alea_preprocess.algos.hashing.ctph.compare(x1, x2))
    mean = sum(values) / len(values)

    # arbitrary based on testing; some value that is expected to be true based on architecture-independent testing
    assert mean > 0.075
