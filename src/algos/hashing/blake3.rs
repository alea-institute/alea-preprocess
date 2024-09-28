use crate::io::fs::files::{read_file_content, read_gz_file_content};
/// This file contains the implementation of the blake3 hashing algorithm.
use blake3;
use hex;

/// Hashes a buffer using the Blake3 algorithm.
///
/// Args:
///   buffer (bytes): The buffer to hash.
///
/// Returns:
///  str: The hash of the buffer.
pub fn hash_bytes(buffer: &[u8]) -> String {
    hex::encode(blake3::hash(buffer).as_bytes())
}

/// Hashes a string using the Blake3 algorithm.
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

/// Hashes the content of a file using the Blake3 algorithm.
///
/// Args:
/// path (&str): The path to the file to hash.
///
/// Returns:
/// str: The hash of the file content.
pub fn hash_file(path: &str) -> String {
    hash_bytes(read_file_content(path).unwrap().as_slice())
}

/// Hashes the content of a Gzipped file using the Blake3 algorithm.
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
        let buffer = b"Hello, world!";
        let hash = hash_bytes(buffer);
        assert_eq!(
            hash,
            "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"
        );
    }

    #[test]
    fn test_hash_str() {
        let s = "Hello, world!";
        let hash = hash_str(s);
        assert_eq!(
            hash,
            "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"
        );
    }

    #[test]
    fn test_hash_file() {
        let path = get_test_file_path();
        let hash = hash_file(path.to_str().unwrap());
        assert_eq!(
            hash,
            "94f1675bac4f8bc3c593c63dbf5fe78a0bfda01082af85d5b41a65096db56bff"
        );
    }

    #[test]
    fn test_hash_gz_file() {
        let path = get_test_gz_file_path();
        let hash = hash_gz_file(path.to_str().unwrap());
        assert_eq!(
            hash,
            "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"
        );
    }
}
