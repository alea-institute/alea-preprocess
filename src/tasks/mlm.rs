// Masked Language Modeling task

// external imports
use rand::Rng;

// internal imports
use crate::algos::tokenizers::tokenizers::encode_str;
use crate::tasks::sequences::{extract_content, split_sequence_max};

/// Get a masked sample
/// Returns the input_ids, attention_mask, token_type_ids, and labels
pub fn get_masked_sample(
    tokens: &[i32],
    cls_token_id: i32,
    mask_token_id: i32,
    sep_token_id: i32,
    pad_token_id: i32,
    label_mask_id: i32,
    max_seq_length: usize,
    prob_mask: f64,
) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    // check that tokens fit in max_seq_length
    assert!(
        tokens.len() <= max_seq_length - 2,
        "Tokens exceed max sequence length"
    );

    // get rng
    let mut rng = rand::thread_rng();

    // initialize for fewest touches
    let mut input_ids = vec![0; max_seq_length];
    let mut attention_mask = vec![0; max_seq_length];
    let token_type_ids = vec![0; max_seq_length];
    let mut labels = vec![label_mask_id; max_seq_length];

    // iterate over max seq length
    // note: we only touch labels for the masked tokens
    // note: we  don't need to touch token_type_ids since we are only using a single segment
    for i in 0..max_seq_length {
        if i == 0 {
            input_ids[i] = cls_token_id;
            attention_mask[i] = 1;
        } else if i < tokens.len() + 1 {
            if rng.gen::<f64>() < prob_mask {
                input_ids[i] = mask_token_id;
                labels[i] = tokens[i - 1];
            } else {
                input_ids[i] = tokens[i - 1];
            }
            attention_mask[i] = 1;
        } else if i == tokens.len() + 1 {
            // need sep token here
            input_ids[i] = sep_token_id;
            attention_mask[i] = 1;
        } else {
            // pad tokens
            input_ids[i] = pad_token_id;
        }
    }

    (input_ids, attention_mask, token_type_ids, labels)
}

/// Get masked samples from tokens
pub fn get_masked_samples_from_tokens(
    tokens: &[i32],
    max_seq_length: usize,
    cls_token_id: i32,
    mask_token_id: i32,
    sep_token_id: i32,
    pad_token_id: i32,
    label_mask_id: i32,
    prob_mask: f64,
) -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)> {
    // we first use split max to get the longest sequence lengths, then provide them to get_masked_sample
    // note that we use convention where each sample should have at least <cls> and <sep> tokens,
    // though we don't require any eos equivalent or padding.
    let sequences = split_sequence_max(&tokens, max_seq_length - 2);

    // return the masked samples
    sequences
        .iter()
        .map(|seq| {
            get_masked_sample(
                seq,
                cls_token_id,
                mask_token_id,
                sep_token_id,
                pad_token_id,
                label_mask_id,
                max_seq_length,
                prob_mask,
            )
        })
        .collect()
}

/// Get masked samples from content
/// Need to return a vector of tuples containing input_ids, attention_mask, token_type_ids, and labels
pub fn get_masked_samples_from_content(
    encoded_content: &str,
    max_seq_length: usize,
    tokenizer_name: &str,
    cls_token_id: i32,
    mask_token_id: i32,
    sep_token_id: i32,
    pad_token_id: i32,
    label_mask_id: i32,
    prob_mask: f64,
) -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)> {
    // extract text
    let content = String::from_utf8(extract_content(encoded_content))
        .expect("Failed to convert content to string");

    // we need to cast u32 tokens to i32
    let tokens: Vec<i32> = encode_str(&tokenizer_name, &content)
        .iter()
        .map(|&x| x as i32)
        .collect();

    // get masked samples from tokens
    get_masked_samples_from_tokens(
        &tokens,
        max_seq_length,
        cls_token_id,
        mask_token_id,
        sep_token_id,
        pad_token_id,
        label_mask_id,
        prob_mask,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAD_TOKEN_ID: i32 = 2;
    const SEP_TOKEN_ID: i32 = 4;
    const CLS_TOKEN_ID: i32 = 5;
    const MASK_TOKEN_ID: i32 = 6;
    const LABEL_MASK_ID: i32 = -100;

    #[test]
    fn test_get_masked_sample() {
        let tokens = vec![33, 34, 35, 36];
        let (input_ids, attention_mask, token_type_ids, labels) = get_masked_sample(
            &tokens,
            CLS_TOKEN_ID,
            MASK_TOKEN_ID,
            SEP_TOKEN_ID,
            PAD_TOKEN_ID,
            LABEL_MASK_ID,
            10,
            0.15,
        );

        // check the length of the output vectors
        assert_eq!(input_ids.len(), 10);
        assert_eq!(attention_mask.len(), 10);
        assert_eq!(token_type_ids.len(), 10);
        assert_eq!(labels.len(), 10);

        // check that the CLS token is at the start
        assert_eq!(input_ids[0], CLS_TOKEN_ID);

        // check that the SEP token is at the correct position
        assert_eq!(input_ids[tokens.len() + 1], SEP_TOKEN_ID);

        // check that we end with a pad
        for i in (tokens.len() + 2)..10 {
            assert_eq!(input_ids[i], PAD_TOKEN_ID);
        }
    }

    #[test]
    fn test_get_masked_samples_from_content_p0() {
        let encoded_content = "eJwLycgsVgCiRIWS1OISACRzBPY=";
        let (input_ids, attention_mask, token_type_ids, labels) = get_masked_samples_from_content(
            encoded_content,
            10,
            "alea-institute/kl3m-003-64k",
            CLS_TOKEN_ID,
            MASK_TOKEN_ID,
            SEP_TOKEN_ID,
            PAD_TOKEN_ID,
            LABEL_MASK_ID,
            0.0,
        )
        // get only first
        .into_iter()
        .next()
        .unwrap();

        // input_ids should be: cls, 2556, 400, 270, 2329, sep, pad, pad, pad, pad
        assert_eq!(input_ids[0], CLS_TOKEN_ID);
        assert_eq!(input_ids[1], 2556);
        assert_eq!(input_ids[2], 400);
        assert_eq!(input_ids[3], 270);
        assert_eq!(input_ids[4], 2329);
        assert_eq!(input_ids[5], SEP_TOKEN_ID)
    }

    #[test]
    #[test]
    fn test_get_masked_samples_from_content_p1() {
        let encoded_content = "eJwLycgsVgCiRIWS1OISACRzBPY=";
        let (input_ids, attention_mask, token_type_ids, labels) = get_masked_samples_from_content(
            encoded_content,
            10,
            "alea-institute/kl3m-003-64k",
            CLS_TOKEN_ID,
            MASK_TOKEN_ID,
            SEP_TOKEN_ID,
            PAD_TOKEN_ID,
            LABEL_MASK_ID,
            1.0,
        )
        // get only first
        .into_iter()
        .next()
        .unwrap();

        // input_ids should be: cls, 2556, 400, 270, 2329, sep, pad, pad, pad, pad
        assert_eq!(input_ids[0], CLS_TOKEN_ID);
        assert_eq!(input_ids[1], 6);
        assert_eq!(input_ids[2], 6);
        assert_eq!(input_ids[3], 6);
        assert_eq!(input_ids[4], 6);
        assert_eq!(input_ids[5], SEP_TOKEN_ID);

        // check the length of the output vectors
        assert_eq!(input_ids.len(), 10);
        assert_eq!(attention_mask.len(), 10);
        assert_eq!(token_type_ids.len(), 10);
        assert_eq!(labels.len(), 10);
    }
}
