/// Context-Triggered Piecewise Hashing (CTPH) adapted for token arrays
/// Instead of working with bytes, this version works directly with token sequences
use std::collections::HashSet;
use crate::algos::hashing::token_rolling::RollingTokenHash;


pub struct TokenCTPH {
    window_size: usize,
    digest_size: usize,
}

impl TokenCTPH {
    pub fn new(window_size: usize, digest_size: usize) -> Self {
        TokenCTPH {
            window_size,
            digest_size,
        }
    }

    /// Hash a piece of the token sequence using blake3
    fn hash_piece(&self, tokens: &[i64]) -> String {
        let mut hasher = blake3::Hasher::new();
        // Convert tokens to bytes for hashing
        for &token in tokens {
            hasher.update(&token.to_le_bytes());
        }
        let mut result = [0; 8];  // Using 64-bit pieces since we're working with tokens
        hasher.finalize_xof().fill(&mut result);
        hex::encode(result)
    }

    pub fn compute(&self, tokens: &[i64]) -> String {
        let mut rolling_hash = RollingTokenHash::new(self.window_size);
        let mut blocks = vec![String::new()];
        let mut current_piece = Vec::new();
        let mut trigger_count = 0;

        for &token in tokens {
            rolling_hash.update(token);
            current_piece.push(token);

            // Trigger on rolling hash value or max piece size
            if rolling_hash.hash() % self.digest_size as u64 == (self.digest_size - 1) as u64
                || current_piece.len() >= self.window_size
            {
                /// debug print on trigger
                let piece_hash = self.hash_piece(&current_piece);
                blocks.last_mut().unwrap().push_str(&piece_hash);
                current_piece.clear();
                trigger_count += 1;

                // Start new block after digest_size triggers
                if trigger_count % self.digest_size == 0 {
                    blocks.push(String::new());
                }
            }
        }

        // Hash any remaining tokens
        if !current_piece.is_empty() {
            let piece_hash = self.hash_piece(&current_piece);
            blocks.last_mut().unwrap().push_str(&piece_hash);
        }

        // Remove empty blocks
        blocks.retain(|block| !block.is_empty());

        // Format output as window_size:digest_size:block1:block2:...
        let blocks_str = blocks.join(":");
        format!("{}:{}:{}", self.window_size, self.digest_size, blocks_str)
    }
}

/// Compute CTPH hash of a token sequence
pub fn hash_tokens(tokens: &[i64], window_size: usize, digest_size: usize) -> String {
    TokenCTPH::new(window_size, digest_size).compute(tokens)
}

/// Compare two token CTPH hashes using Jaccard similarity
pub fn similarity(hash1: &str, hash2: &str) -> f64 {
    // Split hashes into components
    let hash1_parts: Vec<&str> = hash1.split(':').collect();
    let hash2_parts: Vec<&str> = hash2.split(':').collect();

    // Verify minimum length and matching parameters
    if hash1_parts.len() < 3 || hash2_parts.len() < 3 {
        return 0.0;
    }

    let window_size1 = hash1_parts[0].parse::<usize>().unwrap_or(0);
    let window_size2 = hash2_parts[0].parse::<usize>().unwrap_or(0);
    if window_size1 != window_size2 {
        return 0.0;
    }

    let digest_size1 = hash1_parts[1].parse::<usize>().unwrap_or(0);
    let digest_size2 = hash2_parts[1].parse::<usize>().unwrap_or(0);
    if digest_size1 != digest_size2 {
        return 0.0;
    }

    // Compare blocks using Jaccard similarity
    let blocks1: HashSet<&str> = hash1_parts[2..].iter().copied().collect();
    let blocks2: HashSet<&str> = hash2_parts[2..].iter().copied().collect();

    let intersection = blocks1.intersection(&blocks2).count();
    let union = blocks1.len() + blocks2.len() - intersection;

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hash() {
        let tokens = vec![1, 2, 3, 4, 5];
        let ctph = TokenCTPH::new(2, 4);
        let hash = ctph.compute(&tokens);
        assert!(hash.starts_with("2:4:"));
    }

    #[test]
    fn test_empty_sequence() {
        let tokens = vec![];
        let ctph = TokenCTPH::new(2, 4);
        let hash = ctph.compute(&tokens);
        assert_eq!(hash, "2:4:");
    }

    #[test]
    fn test_single_token() {
        let tokens = vec![42];
        let ctph = TokenCTPH::new(2, 4);
        let hash = ctph.compute(&tokens);
        assert!(hash.starts_with("2:4:"));
        assert!(hash.len() > 4);  // Should have some hash content
    }

    #[test]
    fn test_identical_sequences() {
        let tokens1 = vec![1, 2, 3, 4, 5];
        let tokens2 = vec![1, 2, 3, 4, 5];
        let hash1 = hash_tokens(&tokens1, 2, 4);
        let hash2 = hash_tokens(&tokens2, 2, 4);
        assert_eq!(similarity(&hash1, &hash2), 1.0);
    }

    #[test]
    fn test_different_sequences() {
        let tokens1 = vec![1, 2, 3, 4, 5];
        let tokens2 = vec![6, 7, 8, 9, 10];
        let hash1 = hash_tokens(&tokens1, 2, 4);
        let hash2 = hash_tokens(&tokens2, 2, 4);
        assert!(dbg!(similarity(&hash1, &hash2)) < 0.5);  // Should be quite different
    }

    #[test]
    fn test_similar_sequences() {
        let tokens1 = vec![6153, 424, 24, 300, 281, 17938, 295, 281, 1032, 922, 377, 300,
                          281, 45261, 24, 2413, 24, 377, 38148, 295, 4639, 3184, 54456, 310, 2899,
                          295, 281, 1032, 922, 1171, 9018, 377, 777, 4845, 281, 1974, 1412, 15118,
                          295, 2507, 16228, 1228, 3156, 1974, 735, 517, 1727, 15549, 377, 35233,
                          4757, 1663, 18640, 24, 15549, 377, 35233, 4757, 3999, 3757, 24, 377,
                          15549, 377, 35233, 284, 519, 280, 295, 4757, 7873, 4305, 295, 37043,
                          334, 3778, 674, 295, 281, 10603, 11940, 1431, 4899, 643, 7500, 300,
                          270, 2979, 2781, 377, 965, 18880, 7369, 24, 300, 6344, 377, 300, 1173,
                          440, 281, 1285, 295, 281, 3238, 295, 3504, 1184, 475, 517, 10023, 295,
                          281, 1032, 922, 26];
        let tokens2 = vec![6153, 424, 24, 300, 281, 17938, 295, 281, 1032, 922, 377, 300,
                           281, 45261, 24, 2413, 24, 377, 38148, 295, 4639, 3184, 54456, 310, 2899,
                           295, 281, 1032, 922, 1171, 9018, 377, 777, 4845, 281, 1974, 1412, 15118,
                           295, 2507, 16228, 1228, 3156, 23805, 735, 517, 1727, 15549, 377, 35233,
                           4757, 1663, 18640, 24, 61109, 82, 11114, 377, 35233, 4757, 3999, 3757,
                           24, 377, 15549, 377, 35233, 284, 519, 280, 295, 4757, 7873, 4305, 295,
                           37043, 334, 3778, 674, 295, 281, 10603, 11940, 1431, 4899, 643, 7500,
                           300, 270, 2979, 2781, 377, 965, 18880, 7369, 24, 300, 6344, 377, 300,
                           1173, 440, 281, 1285, 295, 281, 3238, 295, 3504, 1184, 475, 517, 10023,
                           295, 281, 1032, 922, 26];
        let hash1 = dbg!(hash_tokens(&tokens1, 4, 8));
        let hash2 = dbg!(hash_tokens(&tokens2, 4, 8));
        let sim = dbg!(similarity(&hash1, &hash2));
        assert!(sim > 0.0 && sim < 1.0);  // Should be somewhat similar
    }

    #[test]
    fn test_different_window_sizes() {
        let tokens = vec![1, 2, 3, 4, 5];
        let hash1 = hash_tokens(&tokens, 2, 4);
        let hash2 = hash_tokens(&tokens, 3, 4);
        assert_eq!(similarity(&hash1, &hash2), 0.0);  // Different parameters should give 0
    }

    #[test]
    fn test_different_digest_sizes() {
        let tokens = vec![1, 2, 3, 4, 5];
        let hash1 = hash_tokens(&tokens, 2, 4);
        let hash2 = hash_tokens(&tokens, 2, 5);
        assert_eq!(similarity(&hash1, &hash2), 0.0);  // Different parameters should give 0
    }

    #[test]
    fn test_large_tokens() {
        let tokens = vec![i64::MAX, i64::MIN, 0];
        let ctph = TokenCTPH::new(2, 4);
        let hash = ctph.compute(&tokens);
        assert!(hash.starts_with("2:4:"));
    }

    #[test]
    fn test_subsequence_similarity() {
        let tokens1 = vec![1, 2, 3, 4, 5];
        let tokens2 = vec![0, 2, 2, 6, 1, 2, 3, 4, 5, 6, 7];  // Contains first sequence
        let hash1 = dbg!(hash_tokens(&tokens1, 2, 2));
        let hash2 = dbg!(hash_tokens(&tokens2, 2, 2));
        let sim = similarity(&hash1, &hash2);
        assert!(sim > 0.0);  // Should detect the similar subsequence
    }
}
