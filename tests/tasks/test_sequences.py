"""
Test sequenec methods
"""

# imports
from pathlib import Path

# packages

# extension module
import alea_preprocess


def test_split_sequence_max():
    buffer1 = Path("resources/10usc101.txt").read_text()
    tokens1 = alea_preprocess.algos.tokenizers.encode_str(
        "alea-institute/kl3m-003-64k", buffer1
    )
    token_sequences = alea_preprocess.tasks.sequences.split_sequence_max(
        tokens1, max_size=32
    )
    assert len(token_sequences) == 86
