"""
Sequence splitting and convenience functions.
"""

# imports
from typing import List

def get_masked_sample(
    tokens: List[int],
    cls_token_id: int,
    mask_token_id: int,
    sep_token_id: int,
    pad_token_id: int,
    label_mask_id: int,
    max_seq_length: int,
    prob_mask: float,
) -> tuple[List[int], List[int], List[int], List[int]]:
    """
    Get a masked sample from the tokens.

    Args:
        tokens: The input tokens.
        cls_token_id: The ID of the CLS token.
        mask_token_id: The ID of the MASK token.
        sep_token_id: The ID of the SEP token.
        pad_token_id: The ID of the PAD token.
        label_mask_id: The ID of the label mask.
        max_seq_length: The maximum sequence length.
        prob_mask: The probability of masking a token.

    Returns:
        A tuple containing the input IDs, attention mask, token type IDs, and label IDs.
    """
    ...

def get_masked_samples_from_tokens(
    tokens: List[int],
    max_seq_length: int,
    cls_token_id: int,
    mask_token_id: int,
    sep_token_id: int,
    pad_token_id: int,
    label_mask_id: int,
    prob_mask: float,
) -> List[tuple[List[int], List[int], List[int], List[int]]]:
    """
    Get masked samples from a list of tokens.

    Args:
        tokens: The input tokens.
        max_seq_length: The maximum sequence length.
        cls_token_id: The ID of the CLS token.
        mask_token_id: The ID of the MASK token.
        sep_token_id: The ID of the SEP token.
        pad_token_id: The ID of the PAD token.
        label_mask_id: The ID of the label mask.
        prob_mask: The probability of masking a token.

    Returns:
        A list of tuples containing the input IDs, attention mask, token type IDs, and label IDs.
    """
    ...

def get_masked_samples_from_content(
    encoded_content: str,
    max_seq_length: int,
    tokenizer_name: str,
    cls_token_id: int,
    mask_token_id: int,
    sep_token_id: int,
    pad_token_id: int,
    label_mask_id: int,
    prob_mask: float,
) -> List[tuple[List[int], List[int], List[int], List[int]]]:
    """
    Get masked samples from encoded content.

    Args:
        encoded_content: The encoded content as a string.
        max_seq_length: The maximum sequence length.
        tokenizer_name: The name of the tokenizer.
        cls_token_id: The ID of the CLS token.
        mask_token_id: The ID of the MASK token.
        sep_token_id: The ID of the SEP token.
        pad_token_id: The ID of the PAD token.
        label_mask_id: The ID of the label mask.
        prob_mask: The probability of masking a token.

    Returns:
        A list of tuples containing the input IDs, attention mask, token type IDs, and label IDs.
    """
    ...
