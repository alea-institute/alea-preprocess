pub mod tokenizers {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    use tokenizers::Tokenizer;

    static TOKENIZER_CACHE: LazyLock<Mutex<HashMap<String, Tokenizer>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    fn get_tokenizer(tokenizer: &str) -> Tokenizer {
        let mut cache = TOKENIZER_CACHE.lock().unwrap();
        cache
            .entry(tokenizer.to_string())
            .or_insert_with(|| {
                Tokenizer::from_pretrained(tokenizer, None).expect("Failed to load tokenizer")
            })
            .clone()
    }

    pub fn encode_str(tokenizer: &str, text: &str) -> Vec<u32> {
        let tokenizer = get_tokenizer(tokenizer);
        let encoding = tokenizer
            .encode(text, false)
            .expect("Failed to encode text");
        encoding.get_ids().to_vec()
    }

    pub fn decode_str(tokenizer: &str, ids: Vec<u32>) -> String {
        let tokenizer = get_tokenizer(tokenizer);
        tokenizer
            .decode(ids.as_slice(), false)
            .expect("Failed to decode text")
    }

    pub fn encode_str_list(tokenizer: &str, texts: Vec<String>) -> Vec<Vec<u32>> {
        let tokenizer = get_tokenizer(tokenizer);
        tokenizer
            .encode_batch(texts, false)
            .expect("Failed to encode text")
            .iter()
            .map(|encoding| encoding.get_ids().to_vec())
            .collect()
    }

    pub fn decode_str_list(tokenizer: &str, ids: Vec<Vec<u32>>) -> Vec<String> {
        let tokenizer = get_tokenizer(tokenizer);
        ids.into_iter()
            .map(|ids| {
                tokenizer
                    .decode(ids.as_slice(), false)
                    .expect("Failed to decode text")
            })
            .collect()
    }
}
