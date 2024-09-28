# imports

# packages
import pytest

# extension module
import alea_preprocess

INPUT_STR = "Hello, world!"
OUTPUT_HASH = "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f"


# test blake2b has
def test_blake2b_has():
    assert hasattr(alea_preprocess.algos.hashing.blake2, "hash_str")
    assert hasattr(alea_preprocess.algos.hashing.blake2, "hash_bytes")


def test_blake2b_hash_str():
    assert alea_preprocess.algos.hashing.blake2.hash_str(INPUT_STR) == OUTPUT_HASH


def test_blake2b_hash_bytes():
    assert (
        alea_preprocess.algos.hashing.blake2.hash_bytes(INPUT_STR.encode())
        == OUTPUT_HASH
    )


def test_blake2b_hash_str_exception():
    with pytest.raises(TypeError):
        alea_preprocess.algos.hashing.blake2.hash_str(INPUT_STR.encode())
