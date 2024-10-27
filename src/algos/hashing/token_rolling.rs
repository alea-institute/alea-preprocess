/// Rolling hash function optimized for LLM token arrays

use base64::prelude::*;

pub struct RollingTokenHash {
    window: Vec<i64>,
    window_size: usize,
    hash: u64,
}

impl RollingTokenHash {
    pub fn new(window_size: usize) -> Self {
        RollingTokenHash {
            window: Vec::with_capacity(window_size),
            window_size,
            hash: 0,
        }
    }

    pub fn from(tokens: &[i64], window_size: usize) -> Self {
        let mut rolling_hash = RollingTokenHash::new(window_size);
        for &token in tokens {
            rolling_hash.update(token);
        }
        rolling_hash
    }

    pub fn update(&mut self, token: i64) {
        // Convert signed token to unsigned for hashing
        let token_unsigned = token as u64;

        if self.window.len() == self.window_size {
            let old_token = self.window.remove(0) as u64;
            // Use wrapping operations to handle overflow
            self.hash = self.hash.wrapping_sub(old_token);
        }

        self.window.push(token);

        // Combine previous hash with new token value using wrapping operations
        // Using rotate_left helps distribute token values across the hash space
        self.hash = self.hash
            .wrapping_add(token_unsigned)
            .rotate_left(1);
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn window_size(&self) -> usize {
        self.window_size
    }

    pub fn current_window(&self) -> &[i64] {
        &self.window
    }
}

pub fn hash_tokens(tokens: &[i64], window_size: usize) -> String {
    let hash = RollingTokenHash::from(tokens, window_size).hash();
    BASE64_STANDARD.encode(&hash.to_be_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hash() {
        let hash = RollingTokenHash::new(3);
        assert_eq!(hash.window_size(), 3);
        assert_eq!(hash.hash(), 0);
        assert!(hash.current_window().is_empty());
    }

    #[test]
    fn test_single_token_update() {
        let mut hash = RollingTokenHash::new(3);
        hash.update(42);
        assert_eq!(hash.current_window(), &[42]);
        assert_ne!(hash.hash(), 0);  // Hash should change after update
    }

    #[test]
    fn test_window_sliding() {
        let mut hash = RollingTokenHash::new(2);

        // Add first two tokens
        hash.update(1);
        hash.update(2);
        assert_eq!(hash.current_window(), &[1, 2]);

        // Add third token, should slide window
        let hash_before = hash.hash();
        hash.update(3);
        assert_eq!(hash.current_window(), &[2, 3]);
        assert_ne!(hash.hash(), hash_before);  // Hash should change
    }

    #[test]
    fn test_negative_tokens() {
        let mut hash = RollingTokenHash::new(2);
        hash.update(-1);
        hash.update(-2);
        // Just verify it doesn't panic and produces some hash
        assert_ne!(hash.hash(), 0);
    }

    #[test]
    fn test_from_slice() {
        let tokens = vec![1, 2, 3, 4, 5];
        let hash = RollingTokenHash::from(&tokens, 3);
        assert_eq!(hash.current_window(), &[3, 4, 5]);
    }

    #[test]
    fn test_hash_stability() {
        // Same input should produce same hash
        let tokens1 = vec![1, 2, 3];
        let tokens2 = vec![1, 2, 3];
        let hash1 = RollingTokenHash::from(&tokens1, 2);
        let hash2 = RollingTokenHash::from(&tokens2, 2);
        assert_eq!(hash1.hash(), hash2.hash());
    }

    #[test]
    fn test_different_window_sizes() {
        let tokens = vec![1, 2, 3, 4, 5];
        let hash1 = RollingTokenHash::from(&tokens, 2);
        let hash2 = RollingTokenHash::from(&tokens, 3);
        // Different window sizes should produce different hashes
        assert_ne!(hash1.hash(), hash2.hash());
    }

    #[test]
    fn test_hash_tokens_encoding() {
        let tokens = vec![1, 2, 3];
        let hash_str = hash_tokens(&tokens, 2);
        // Verify it produces valid base64
        assert!(BASE64_STANDARD.decode(&hash_str).is_ok());
    }

    #[test]
    fn test_large_tokens() {
        let mut hash = RollingTokenHash::new(2);
        // Test with tokens near i64 bounds
        hash.update(i64::MAX);
        hash.update(i64::MIN);
        // Just verify it doesn't panic and produces some hash
        assert_ne!(hash.hash(), 0);
    }

    #[test]
    fn test_empty_sequence() {
        let hash = RollingTokenHash::from(&[], 3);
        assert_eq!(hash.hash(), 0);
        assert!(hash.current_window().is_empty());
    }

    #[test]
    fn test_sequence_shorter_than_window() {
        let tokens = vec![1, 2];
        let hash = RollingTokenHash::from(&tokens, 3);
        assert_eq!(hash.current_window(), &[1, 2]);
    }

    #[test]
    fn test_similar_sequences_1() {
        let tokens1 = vec![1, 2, 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
        let tokens2 = vec![1, 2, 3, 4, 6, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
        let hash1 = dbg!(hash_tokens(&tokens1, 2));
        let hash2 = dbg!(hash_tokens(&tokens2, 2));
        assert!(hash1 != hash2);
    }

    #[test]
    fn test_similar_sequences_2() {
        let tokens1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
        let tokens2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
        let hash1 = dbg!(hash_tokens(&tokens1, 2));
        let hash2 = dbg!(hash_tokens(&tokens2, 2));
        assert!(hash1 != hash2);
    }

    // test two real tokenized sequences:
    // >>> p.tokenizer("The Department of Labor is responsible for the provision of unemployment benefits.")
    // {'input_ids': [671, 1485, 295, 3773, 400, 5515, 353, 281, 1519, 295, 13034, 2793, 26], 'token_type_ids': [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 'attention_mask': [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}
    // >>> p.tokenizer("The Department of Defense is responsible for the provision of conscription.")
    // {'input_ids': [671, 1485, 295, 3112, 400, 5515, 353, 281, 1519, 295, 713, 3003, 26], 'token_type_ids': [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 'attention_mask': [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}
    #[test]
    fn test_real_token_sequences() {
        let tokens1 = vec![671, 1485, 295, 3773, 400, 5515, 353, 281, 1519, 295, 13034, 2793, 26];
        let tokens2 = vec![671, 1485, 295, 3112, 400, 5515, 353, 281, 1519, 295, 713, 3003, 26];
        let hash1 = dbg!(hash_tokens(&tokens1, 2));
        let hash2 = dbg!(hash_tokens(&tokens2, 2));

        assert!(hash1 != hash2);
    }

}
