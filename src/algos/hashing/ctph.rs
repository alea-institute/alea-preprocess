/// Context-Triggered Piecewise Hashing (CTPH) is a hashing algorithm that
/// identifies similar pieces of data by hashing them in a sliding window.
/// The algorithm is used to identify similar pieces of data in a large
/// dataset, such as a file or a stream of data.
use crate::algos::hashing::rolling::*;
use crate::io::fs::files::read_gz_file_content;

pub struct CTPH {
    window_size: usize,
    digest_size: usize,
    precision: u8,
}

/// Context-Triggered Piecewise Hashing (CTPH) is a hashing algorithm that
impl CTPH {
    /// Create a new CTPH instance with the given window size and digest size.
    /// The window size is the size of the sliding window used to hash the data.
    /// The digest size is the size of the hash digest used to identify similar
    /// pieces of data.
    ///
    /// Arguments:
    /// - `window_size`: The size of the sliding window used to hash the data.
    /// - `digest_size`: The size of the hash digest used to identify similar
    ///  pieces of data.
    ///
    /// Returns:
    /// - A new CTPH instance.
    pub fn new(window_size: usize, digest_size: usize, precision: u8) -> Self {
        CTPH {
            window_size,
            digest_size,
            precision,
        }
    }

    /// Blake3 hash piece with 8-bit integers
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The Blake3 hash of the data.
    fn hash_piece8(&self, data: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let mut result = [0; 1];
        hasher.finalize_xof().fill(&mut result);
        hex::encode(result)
    }

    /// Blake3 hash piece with 16-bit integers
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The Blake3 hash of the data.
    fn hash_piece16(&self, data: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let mut result = [0; 2];
        hasher.finalize_xof().fill(&mut result);
        hex::encode(result)
    }

    /// Blake3 hash piece with 32-bit integers
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The Blake3 hash of the data.
    fn hash_piece32(&self, data: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let mut result = [0; 4];
        hasher.finalize_xof().fill(&mut result);
        hex::encode(result)
    }

    /// Blake3 hash piece with 64-bit integers
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The Blake3 hash of the data.
    fn hash_piece64(&self, data: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let mut result = [0; 4];
        hasher.finalize_xof().fill(&mut result);
        hex::encode(result)
    }

    /// Compute with RollingHash8
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The CTPH hash of the data.
    pub fn compute8(&self, data: &[u8]) -> String {
        let mut rolling_hash = RollingHash8::new(self.window_size);
        let mut blocks = vec![String::new()];
        let mut current_piece = Vec::new();
        let mut trigger_count = 0;

        for &byte in data {
            rolling_hash.update(byte);
            current_piece.push(byte);

            if rolling_hash.hash() % self.digest_size as u8 == (self.digest_size - 1) as u8
                || current_piece.len() >= 64 * self.window_size
            {
                let piece_hash = self.hash_piece8(&current_piece);
                blocks.last_mut().unwrap().push_str(&piece_hash);
                current_piece.clear();
                trigger_count += 1;

                if trigger_count % self.digest_size == 0 {
                    blocks.push(String::new());
                }
            }
        }

        if !current_piece.is_empty() {
            let piece_hash = self.hash_piece8(&current_piece);
            blocks.last_mut().unwrap().push_str(&piece_hash);
        }

        // Remove any empty blocks
        blocks.retain(|block| !block.is_empty());

        let blocks_str = blocks.join(":");
        format!("{}:{}:{}", self.window_size, self.digest_size, blocks_str)
    }

    /// Compute with RollingHash16
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The CTPH hash of the data.
    pub fn compute16(&self, data: &[u8]) -> String {
        let mut rolling_hash = RollingHash16::new(self.window_size);
        let mut blocks = vec![String::new()];
        let mut current_piece = Vec::new();
        let mut trigger_count = 0;

        for &byte in data {
            rolling_hash.update(byte as u16);
            current_piece.push(byte);

            if rolling_hash.hash() % self.digest_size as u16 == (self.digest_size - 1) as u16
                || current_piece.len() >= 64 * self.window_size
            {
                let piece_hash = self.hash_piece16(&current_piece);
                blocks.last_mut().unwrap().push_str(&piece_hash);
                current_piece.clear();
                trigger_count += 1;

                if trigger_count % self.digest_size == 0 {
                    blocks.push(String::new());
                }
            }
        }

        if !current_piece.is_empty() {
            let piece_hash = self.hash_piece16(&current_piece);
            blocks.last_mut().unwrap().push_str(&piece_hash);
        }

        // Remove any empty blocks
        blocks.retain(|block| !block.is_empty());

        let blocks_str = blocks.join(":");
        format!("{}:{}:{}", self.window_size, self.digest_size, blocks_str)
    }

    /// Compute with RollingHash32
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The CTPH hash of the data.
    pub fn compute32(&self, data: &[u8]) -> String {
        let mut rolling_hash = RollingHash32::new(self.window_size);
        let mut blocks = vec![String::new()];
        let mut current_piece = Vec::new();
        let mut trigger_count = 0;

        for &byte in data {
            rolling_hash.update(byte as u32);
            current_piece.push(byte);

            if rolling_hash.hash() % self.digest_size as u32 == (self.digest_size - 1) as u32
                || current_piece.len() >= 64 * self.window_size
            {
                let piece_hash = self.hash_piece32(&current_piece);
                blocks.last_mut().unwrap().push_str(&piece_hash);
                current_piece.clear();
                trigger_count += 1;

                if trigger_count % self.digest_size == 0 {
                    blocks.push(String::new());
                }
            }
        }

        if !current_piece.is_empty() {
            let piece_hash = self.hash_piece32(&current_piece);
            blocks.last_mut().unwrap().push_str(&piece_hash);
        }

        // Remove any empty blocks
        blocks.retain(|block| !block.is_empty());

        let blocks_str = blocks.join(":");
        format!("{}:{}:{}", self.window_size, self.digest_size, blocks_str)
    }

    /// Compute with RollingHash64
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The CTPH hash of the data.
    pub fn compute64(&self, data: &[u8]) -> String {
        let mut rolling_hash = RollingHash64::new(self.window_size);
        let mut blocks = vec![String::new()];
        let mut current_piece = Vec::new();
        let mut trigger_count = 0;

        for &byte in data {
            rolling_hash.update(byte as u64);
            current_piece.push(byte);

            if rolling_hash.hash() % self.digest_size as u64 == (self.digest_size - 1) as u64
                || current_piece.len() >= 64 * self.window_size
            {
                let piece_hash = self.hash_piece64(&current_piece);
                blocks.last_mut().unwrap().push_str(&piece_hash);
                current_piece.clear();
                trigger_count += 1;

                if trigger_count % self.digest_size == 0 {
                    blocks.push(String::new());
                }
            }
        }

        if !current_piece.is_empty() {
            let piece_hash = self.hash_piece64(&current_piece);
            blocks.last_mut().unwrap().push_str(&piece_hash);
        }

        // Remove any empty blocks
        blocks.retain(|block| !block.is_empty());

        let blocks_str = blocks.join(":");
        format!("{}:{}:{}", self.window_size, self.digest_size, blocks_str)
    }

    /// Compute with the given data type
    /// Arguments:
    /// - `data`: The data to hash.
    /// Returns:
    /// - The CTPH hash of the data.
    pub fn compute(&self, data: &[u8]) -> String {
        match self.precision {
            8 => self.compute8(data),
            16 => self.compute16(data),
            32 => self.compute32(data),
            64 => self.compute64(data),
            _ => self.compute8(data),
        }
    }
}

/// Compute the CTPH hash of the given data.
/// Arguments:
/// - `data`: The data to hash.
/// - `window_size`: The size of the sliding window used to hash the data.
/// - `digest_size`: The size of the hash digest used to identify similar pieces of data.
/// - `precision`: The precision of the rolling hash.
/// Returns:
/// - The CTPH hash of the data.
pub fn hash_bytes(bytes: &[u8], window_size: usize, digest_size: usize, precision: u8) -> String {
    CTPH::new(window_size, digest_size, precision).compute(bytes)
}

/// Compute the CTPH hash of the given string.
/// Arguments:
/// - `s`: The string to hash.
/// - `window_size`: The size of the sliding window used to hash the data.
/// - `digest_size`: The size of the hash digest used to identify similar pieces of data.
/// - `precision`: The precision of the rolling hash.
/// Returns:
/// - The CTPH hash of the string.
pub fn hash_str(s: &str, window_size: usize, digest_size: usize, precision: u8) -> String {
    hash_bytes(s.as_bytes(), window_size, digest_size, precision)
}

/// Compute the CTPH hash of the given file.
/// Compute the CTPH hash of the given string.
/// Arguments:
/// - `path`: The path to the file to hash.
/// - `window_size`: The size of the sliding window used to hash the data.
/// - `digest_size`: The size of the hash digest used to identify similar pieces of data.
/// - `precision`: The precision of the rolling hash.
/// Returns:
/// - The CTPH hash of the string.
pub fn hash_file(path: &str, window_size: usize, digest_size: usize, precision: u8) -> String {
    let data = std::fs::read(path).unwrap();
    hash_bytes(&data, window_size, digest_size, precision)
}

/// Compute the CTPH hash of the given Gzipped file.
/// Arguments:
/// - `path`: The path to the file to hash.
/// - `window_size`: The size of the sliding window used to hash the data.
/// - `digest_size`: The size of the hash digest used to identify similar pieces of data.
/// - `precision`: The precision of the rolling hash.
/// Returns:
/// - The CTPH hash of the string.
pub fn hash_gz_file(path: &str, window_size: usize, digest_size: usize, precision: u8) -> String {
    hash_bytes(
        read_gz_file_content(path).unwrap().as_slice(),
        window_size,
        digest_size,
        precision,
    )
}

/// Compare two CTPH hashes and return the similarity score as the Jacccard similarity.
/// If the digest and window sizes are not the same, the similarity score is 0.
/// Arguments:
/// - `hash1`: The first CTPH hash.
/// - `hash2`: The second CTPH hash.
/// Returns:
/// - The Jaccard similarity score between the two hashes.
pub fn similarity(hash1: &str, hash2: &str) -> f64 {
    // Split hash1 and hash2 into block size, digest size, and blocks split by :
    let hash1_tokens: Vec<&str> = hash1.split(':').collect();
    let hash2_tokens: Vec<&str> = hash2.split(':').collect();

    // compare the window size elements
    if hash1_tokens.len() < 3 || hash2_tokens.len() < 3 {
        return 0.0;
    }

    let window_size1 = hash1_tokens[0].parse::<usize>().unwrap_or(0);
    let window_size2 = hash2_tokens[0].parse::<usize>().unwrap_or(0);

    if window_size1 != window_size2 {
        return 0.0;
    }

    // compare the digest size elements
    let digest_size1 = hash1_tokens[1].parse::<usize>().unwrap_or(0);
    let digest_size2 = hash2_tokens[1].parse::<usize>().unwrap_or(0);

    if digest_size1 != digest_size2 {
        return 0.0;
    }

    // compare the blocks as sets
    let blocks1: std::collections::HashSet<&str> =
        hash1_tokens[2..].to_vec().iter().copied().collect();
    let blocks2: std::collections::HashSet<&str> =
        hash2_tokens[2..].to_vec().iter().copied().collect();

    // get jaccard similarity
    let intersection = blocks1.intersection(&blocks2).count();
    let union = (blocks1.len() + blocks2.len() - intersection) as f64;
    intersection as f64 / union
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("hello-world.txt");
        path
    }

    fn get_test_gz_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("hello-world.txt.gz");
        path
    }

    fn get_test_jsonl_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("usc.100.jsonl");
        path
    }

    #[test]
    fn test_ctph_compute() {
        let ctph = CTPH::new(8, 4, 8);
        let data = b"hello world";
        let hash = dbg!(ctph.compute(data));
        assert_eq!(&hash[0..5], "8:4:e");
    }

    #[test]
    fn test_ctph_compute_file() {
        let ctph = CTPH::new(8, 4, 16);
        let path = get_test_file_path();
        let data = std::fs::read(path).unwrap();
        let hash = dbg!(ctph.compute(&data));
        assert_eq!(&hash[0..5], "8:4:1");
    }

    #[test]
    fn test_ctph_compute_jsonl_file() {
        let ctph = CTPH::new(8, 4, 16);
        let path = get_test_jsonl_file_path();
        let hash = dbg!(ctph.compute(&std::fs::read(path).unwrap()));
        assert_eq!(&hash[0..5], "8:4:4");
    }

    #[test]
    fn test_ctph_compute_gz_file() {
        let ctph = CTPH::new(8, 4, 16);
        let path = get_test_gz_file_path();
        let data = read_gz_file_content(path.to_str().unwrap()).unwrap();
        let hash = dbg!(ctph.compute(&data));
        assert_eq!(&hash[0..5], "8:4:1");
    }
}
