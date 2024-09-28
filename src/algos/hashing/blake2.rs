use crate::io::fs::files::{read_file_content, read_gz_file_content};
/// This module provides functions for hashing data using the Blake2b (512) algorithm.
///
use blake2::{Blake2b512, Digest};
use hex;

/// Hashes a buffer using the Blake2b512 algorithm.
///
/// Args:
///   buffer (bytes): The buffer to hash.
///
/// Returns:
///  str: The hash of the buffer.
pub fn hash_bytes(buffer: &[u8]) -> String {
    hex::encode(Blake2b512::digest(buffer))
}

/// Hashes a string using the Blake2b512 algorithm.
///
/// Args:
///  s (str): The string to hash.
///
/// Returns:
/// str: The hash of the string.
///
pub fn hash_str(s: &str) -> String {
    hash_bytes(s.as_bytes())
}

/// Hashes the content of a file using the Blake2b512 algorithm.
///
/// Args:
/// path (&str): The path to the file to hash.
///
/// Returns:
/// str: The hash of the file content.
pub fn hash_file(path: &str) -> String {
    hash_bytes(read_file_content(path).unwrap().as_slice())
}

/// Hashes the content of a Gzipped file using the Blake2b512 algorithm.
///
/// Args:
/// path (&str): The path to the file to hash.
///
/// Returns:
/// str: The hash of the file content.
pub fn hash_gz_file(path: &str) -> String {
    hash_bytes(read_gz_file_content(path).unwrap().as_slice())
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

    #[test]
    fn test_hash_bytes() {
        let input = b"Hello, world!";
        let hash = hash_bytes(input);
        assert_eq!(hash.len(), 128);
        assert_eq!(hash, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f".to_string());
    }

    #[test]
    fn test_hash_string() {
        let input = "Hello, world!";
        let hash = hash_str(input);
        assert_eq!(hash.len(), 128);
        assert_eq!(hash, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f".to_string());
    }

    #[test]
    fn test_hash_file() {
        let file_path = get_test_file_path();
        let hash = hash_file(file_path.to_str().unwrap());
        assert_eq!(hash.len(), 128);
        assert_eq!(hash, "80fe13860815f4a018ad5075bfb6844ca24b5963b6064a3b3912240a5824ba34ef71d2e32870af66b1054c94d65436446fff8ca844667de50ef8f700f9234301".to_string());
    }

    #[test]
    fn test_hash_gz_file() {
        let file_path = get_test_gz_file_path();
        let hash = hash_gz_file(file_path.to_str().unwrap());
        assert_eq!(hash.len(), 128);
        assert_eq!(hash, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f".to_string());
    }
}
