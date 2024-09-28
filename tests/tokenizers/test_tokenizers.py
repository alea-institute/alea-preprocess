# imports
import json
from pathlib import Path

# packages

# extension module
import alea_preprocess


def get_jsonl_path() -> Path:
    return Path("resources/usc.100.jsonl")


def get_jsonl_text() -> list[str]:
    with open(get_jsonl_path(), "r") as f:
        return [json.loads(line)["text"] for line in f]


def test_encode_str():
    for row in get_jsonl_text():
        result = alea_preprocess.algos.tokenizers.encode_str("gpt2", row)
        assert len(result) > 0


def test_encode_str_list():
    rows = list(get_jsonl_text())
    result = alea_preprocess.algos.tokenizers.encode_str_list("gpt2", rows)
    assert len(result) > 0
    assert all(isinstance(x, list) for x in result)


def test_decode_str():
    for row in get_jsonl_text():
        encoded = alea_preprocess.algos.tokenizers.encode_str("gpt2", row)
        result = alea_preprocess.algos.tokenizers.decode_str("gpt2", encoded)
        assert result == row


def test_decode_str_list():
    rows = list(get_jsonl_text())
    encoded = alea_preprocess.algos.tokenizers.encode_str_list("gpt2", rows)
    result = alea_preprocess.algos.tokenizers.decode_str_list("gpt2", encoded)
    assert all([rows[i] == result[i] for i in range(len(encoded))])
