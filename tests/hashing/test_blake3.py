# imports

# packages
import pytest

# extension module
import alea_preprocess

INPUT_STR = "Hello, world!"
OUTPUT_HASH = "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"


# test blake3 has
def test_blake3_has():
    assert hasattr(alea_preprocess.algos.hashing.blake3, "hash_str")
    assert hasattr(alea_preprocess.algos.hashing.blake3, "hash_bytes")


def test_blake3_hash_str():
    assert alea_preprocess.algos.hashing.blake3.hash_str(INPUT_STR) == OUTPUT_HASH


def test_blake3_hash_bytes():
    assert (
        alea_preprocess.algos.hashing.blake3.hash_bytes(INPUT_STR.encode())
        == OUTPUT_HASH
    )


def test_blake3_hash_str_exception():
    with pytest.raises(TypeError):
        alea_preprocess.algos.hashing.blake3.hash_str(INPUT_STR.encode())
