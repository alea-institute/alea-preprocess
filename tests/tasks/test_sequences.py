"""
Test sequenec methods
"""

# imports
from pathlib import Path

# extension module
import alea_preprocess

TOKENIZER_ID = "alea-institute/kl3m-003-64k"

"""
alea 003-64k constants:
const PAD_TOKEN_ID: i32 = 2;
const SEP_TOKEN_ID: i32 = 4;
const CLS_TOKEN_ID: i32 = 5;
const MASK_TOKEN_ID: i32 = 6;
const LABEL_MASK_ID: i32 = -100;
"""
PAD_TOKEN_ID = 2
SEP_TOKEN_ID = 4
CLS_TOKEN_ID = 5
MASK_TOKEN_ID = 6
LABEL_MASK_ID = -100


def test_split_sequence_max():
    buffer1 = Path("resources/10usc101.txt").read_text()
    tokens1 = alea_preprocess.algos.tokenizers.encode_str(TOKENIZER_ID, buffer1)
    token_sequences = alea_preprocess.tasks.sequences.split_sequence_max(
        tokens1, max_size=32
    )
    assert len(token_sequences) == 86


def test_mlm_tokens():
    buffer1 = Path("resources/10usc101.txt").read_text()
    tokens1 = alea_preprocess.algos.tokenizers.encode_str(TOKENIZER_ID, buffer1)
    samples = alea_preprocess.tasks.mlm.get_masked_samples_from_tokens(
        tokens1,
        max_seq_length=8,
        cls_token_id=CLS_TOKEN_ID,
        mask_token_id=MASK_TOKEN_ID,
        sep_token_id=SEP_TOKEN_ID,
        pad_token_id=PAD_TOKEN_ID,
        label_mask_id=LABEL_MASK_ID,
        prob_mask=0.0,
    )

    assert samples[0] == (
        [5, 3179, 3087, 26, 14024, 211, 61354, 4],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [-100, -100, -100, -100, -100, -100, -100, -100],
    )


def test_mlm_encoded_content():
    encoded_content = "eJwLycgsVgCiRIWS1OISACRzBPY="
    samples = alea_preprocess.tasks.mlm.get_masked_samples_from_content(
        encoded_content,
        tokenizer_name=TOKENIZER_ID,
        max_seq_length=8,
        cls_token_id=CLS_TOKEN_ID,
        mask_token_id=MASK_TOKEN_ID,
        sep_token_id=SEP_TOKEN_ID,
        pad_token_id=PAD_TOKEN_ID,
        label_mask_id=LABEL_MASK_ID,
        prob_mask=0.0,
    )

    assert samples[0] == (
        [5, 2556, 400, 270, 2329, 4, 2, 2],
        [1, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [-100, -100, -100, -100, -100, -100, -100, -100],
    )
