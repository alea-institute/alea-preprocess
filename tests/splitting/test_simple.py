# imports
import json
from pathlib import Path

# packages
import pytest

# extension module
import alea_preprocess


def get_jsonl_path() -> Path:
    return Path("resources/usc.100.jsonl")


def get_jsonl_text() -> list[str]:
    with open(get_jsonl_path(), "r") as f:
        return [json.loads(line)["text"] for line in f]


# parameterize min and max sizes
@pytest.mark.parametrize("min_size", [8, 16, 64])
@pytest.mark.parametrize("max_size", [32, 128, 1024])
@pytest.mark.parametrize(
    "split_patterns",
    [
        [" ", ".", ",", ";", ":", "!", "?"],
        ["\n\n", "\n", "."],
    ],
)
def test_split_str(min_size, max_size, split_patterns):
    for row in get_jsonl_text():
        result = alea_preprocess.algos.splitting.simple.split_str(
            row * 100, min_size, max_size, split_patterns
        )
        assert len(result) > 0


@pytest.mark.parametrize("min_size", [8, 16, 64])
@pytest.mark.parametrize("max_size", [32, 128, 1024])
@pytest.mark.parametrize(
    "split_patterns",
    [
        [" ", ".", ",", ";", ":", "!", "?"],
        ["\n\n", "\n", "."],
    ],
)
def test_split_str_list(min_size, max_size, split_patterns):
    rows = list(get_jsonl_text())
    result = alea_preprocess.algos.splitting.simple.split_str_list(
        rows, min_size, max_size, split_patterns
    )
    assert len(result) > 0
    assert all(isinstance(x, list) for x in result)
