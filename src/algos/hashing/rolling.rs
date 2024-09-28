/// Rolling hash is a hash function that can be updated in O(1) time complexity.
/// It is used in the Context-Triggered Piecewise Hashing (CTPH) algorithm.
/// The algorithm is used to identify similar pieces of data in a large dataset,
/// such as a file or a stream of data.
use crate::io::fs::files::{read_file_content, read_gz_file_content};
use base64::prelude::*;

/// Rolling hash function for 8-bit integers.
pub struct RollingHash8 {
    window: Vec<u8>,
    window_size: usize,
    hash: u8,
}

/// Rolling hash function for 16-bit integers.
pub struct RollingHash16 {
    window: Vec<u16>,
    window_size: usize,
    hash: u16,
}

/// Rolling hash function for 32-bit integers.
pub struct RollingHash32 {
    window: Vec<u32>,
    window_size: usize,
    hash: u32,
}

/// Rolling hash function for 64-bit integers.
pub struct RollingHash64 {
    window: Vec<u64>,
    window_size: usize,
    hash: u64,
}

/// Rolling hash function for 8-bit integers.
impl RollingHash8 {
    /// Create a new rolling hash with the given window size.
    pub fn new(window_size: usize) -> Self {
        RollingHash8 {
            window: Vec::with_capacity(window_size),
            window_size,
            hash: 0,
        }
    }

    pub fn from(data: &[u8], window_size: usize) -> Self {
        let mut rolling_hash = RollingHash8::new(window_size);
        for &byte in data {
            rolling_hash.update(byte);
        }
        rolling_hash
    }

    pub fn update(&mut self, byte: u8) {
        if self.window.len() == self.window_size {
            let old_byte = self.window.remove(0);
            self.hash = self.hash.wrapping_sub(old_byte);
        }
        self.window.push(byte);
        self.hash = self.hash.wrapping_add(byte).rotate_left(1);
    }

    pub fn hash(&self) -> u8 {
        self.hash
    }
}

/// Rolling hash function for 16-bit integers.
impl RollingHash16 {
    pub fn new(window_size: usize) -> Self {
        RollingHash16 {
            window: Vec::with_capacity(window_size),
            window_size,
            hash: 0,
        }
    }

    pub fn from(data: &[u8], window_size: usize) -> Self {
        let mut rolling_hash = RollingHash16::new(window_size);
        for &byte in data {
            rolling_hash.update(byte as u16);
        }
        rolling_hash
    }

    pub fn update(&mut self, byte: u16) {
        if self.window.len() == self.window_size {
            let old_byte = self.window.remove(0);
            self.hash = self.hash.wrapping_sub(old_byte);
        }
        self.window.push(byte);
        self.hash = self.hash.wrapping_add(byte).rotate_left(1);
    }

    pub fn hash(&self) -> u16 {
        self.hash
    }
}

/// Rolling hash function for 32-bit integers.
impl RollingHash32 {
    pub fn new(window_size: usize) -> Self {
        RollingHash32 {
            window: Vec::with_capacity(window_size),
            window_size,
            hash: 0,
        }
    }

    pub fn from(data: &[u8], window_size: usize) -> Self {
        let mut rolling_hash = RollingHash32::new(window_size);
        for &byte in data {
            rolling_hash.update(byte as u32);
        }
        rolling_hash
    }

    pub fn update(&mut self, byte: u32) {
        if self.window.len() == self.window_size {
            let old_byte = self.window.remove(0);
            self.hash = self.hash.wrapping_sub(old_byte);
        }
        self.window.push(byte);
        self.hash = self.hash.wrapping_add(byte).rotate_left(1);
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }
}

/// Rolling hash function for 64-bit integers.
impl RollingHash64 {
    pub fn new(window_size: usize) -> Self {
        RollingHash64 {
            window: Vec::with_capacity(window_size),
            window_size,
            hash: 0,
        }
    }

    pub fn from(data: &[u8], window_size: usize) -> Self {
        let mut rolling_hash = RollingHash64::new(window_size);
        for &byte in data {
            rolling_hash.update(byte as u64);
        }
        rolling_hash
    }

    pub fn update(&mut self, byte: u64) {
        if self.window.len() == self.window_size {
            let old_byte = self.window.remove(0);
            self.hash = self.hash.wrapping_sub(old_byte);
        }
        self.window.push(byte);
        self.hash = self.hash.wrapping_add(byte).rotate_left(1);
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

pub fn hash_bytes(bytes: &[u8], window_size: usize, precision: u8) -> String {
    let hash = match precision {
        8 => RollingHash8::from(bytes, window_size).hash() as u64,
        16 => RollingHash16::from(bytes, window_size).hash() as u64,
        32 => RollingHash32::from(bytes, window_size).hash() as u64,
        64 => RollingHash64::from(bytes, window_size).hash(),
        _ => panic!("Invalid precision"),
    };
    BASE64_STANDARD.encode(&hash.to_be_bytes())
}

/// Hashes a string using the Rolling hash algorithm.
///
/// Args:
///  s (str): The string to hash.
/// window_size (usize): The size of the rolling hash window.
/// precision (u8): The precision of the rolling hash.
///
/// Returns:
/// str: The hash of the string.
///
pub fn hash_str(s: &str, window_size: usize, precision: u8) -> String {
    hash_bytes(s.as_bytes(), window_size, precision)
}

/// Hashes the content of a file using the Rolling hash algorithm.
///
/// Args:
/// path (&str): The path to the file to hash.
/// window_size (usize): The size of the rolling hash window.
/// precision (u8): The precision of the rolling hash.
///
/// Returns:
/// str: The hash of the file content.
pub fn hash_file(path: &str, window_size: usize, precision: u8) -> String {
    hash_bytes(
        read_file_content(path).unwrap().as_slice(),
        window_size,
        precision,
    )
}

/// Hashes the content of a Gzipped file using the Rolling hash algorithm.
///
/// Args:
/// path (&str): The path to the file to hash.
/// window_size (usize): The size of the rolling hash window.
/// precision (u8): The precision of the rolling hash.
///
/// Returns:
/// str: The hash of the file content.
pub fn hash_gz_file(path: &str, window_size: usize, precision: u8) -> String {
    hash_bytes(
        read_gz_file_content(path).unwrap().as_slice(),
        window_size,
        precision,
    )
}
