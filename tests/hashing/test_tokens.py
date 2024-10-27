"""
Test token rolling hash and token CTPH.
"""

# imports
from pathlib import Path

# packages

# extension module
import alea_preprocess


def test_rolling_hash():
    buffer1 = Path("resources/file1.html").read_text()
    buffer2 = Path("resources/file2.html").read_text()
    tokens1 = alea_preprocess.algos.tokenizers.encode_str(
        "alea-institute/kl3m-003-64k", buffer1
    )
    tokens2 = alea_preprocess.algos.tokenizers.encode_str(
        "alea-institute/kl3m-003-64k", buffer2
    )

    hash1 = alea_preprocess.algos.hashing.token_rolling.hash_tokens(
        tokens1, window_size=16
    )
    hash2 = alea_preprocess.algos.hashing.token_rolling.hash_tokens(
        tokens2, window_size=16
    )

    assert hash1 == "2qremtCcW9k="
    assert hash2 == "AWGm4dNvagk="


def test_compare_html():
    buffer1 = Path("resources/file1.html").read_text()
    buffer2 = Path("resources/file2.html").read_text()
    tokens1 = alea_preprocess.algos.tokenizers.encode_str(
        "alea-institute/kl3m-003-64k", buffer1
    )
    tokens2 = alea_preprocess.algos.tokenizers.encode_str(
        "alea-institute/kl3m-003-64k", buffer2
    )

    hash1 = alea_preprocess.algos.hashing.token_ctph.hash_tokens(
        tokens1, window_size=16, digest_size=16
    )
    hash2 = alea_preprocess.algos.hashing.token_ctph.hash_tokens(
        tokens2, window_size=16, digest_size=16
    )

    similarity = alea_preprocess.algos.hashing.token_ctph.compare(hash1, hash2)

    assert similarity > 0.1
