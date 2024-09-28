use flate2::read::GzDecoder;
/// Utility functions to process files.
///
/// This module contains utility functions to process files, such as iterating over lines of a file
/// without reading the whole file into memory.
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::{fs, io};

// set buffer size to 32KB
const BUFFER_SIZE: usize = 32 * 1024;

/// Shared method to iterate over lines of a file without reading the whole file into memory.
///
/// Args:
/// path (&str): The path to the file to process.
pub fn iter_lines(path: &str) -> Result<Lines<BufReader<File>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

/// Shared method to iterate over lines of a Gzipped file without reading the whole file into memory.
///
/// Args:
/// path (&str): The path to the file to process.
pub fn iter_gz_lines(path: &str) -> Result<Lines<BufReader<GzDecoder<File>>>, io::Error> {
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    Ok(reader.lines())
}

/// Shared method to read whole file content.
///
/// Args:
///   path (&str): The path to the file to process.
pub fn read_file_content(path: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(path)
}

/// Shared method to read whole Gzipped file content.
///
/// Args:
/// path (&str): The path to the file to process.
pub fn read_gz_file_content(path: &str) -> Result<Vec<u8>, io::Error> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Shared method to read all lines into a Vec of byte vectors.
///
/// Args:
/// path (&str): The path to the file to process.
/// Returns:
/// Result<Vec<Vec<u8>>, io::Error>: A vector of byte vectors, each representing a line, or an IO error.
pub fn read_lines(path: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut lines = Vec::new();

    for line in reader.split(b'\n') {
        lines.push(line?);
    }

    Ok(lines)
}

/// Shared method to read all lines of a Gzipped file into a Vec of byte vectors.
/// Args:
/// path (&str): The path to the file to process.
/// Returns:
/// Result<Vec<Vec<u8>>, io::Error>: A vector of byte vectors, each representing a line, or an IO error.
pub fn read_gz_lines(path: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::with_capacity(BUFFER_SIZE, decoder);
    let mut lines = Vec::new();

    for line in reader.split(b'\n') {
        lines.push(line?);
    }

    Ok(lines)
}

/// Shared method to read first N bytes of a file.
///
/// Args:
///  path (&str): The path to the file to process.
/// n (usize): The number of bytes to process.
pub fn read_first_n_bytes(path: &str, n: usize) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; n];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Shared method to read first N bytes of a Gzipped file.
///
/// Args:
/// path (&str): The path to the file to process.
/// n (usize): The number of bytes to process.
pub fn read_gz_first_n_bytes(path: &str, n: usize) -> Result<Vec<u8>, io::Error> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = vec![0u8; n];
    let bytes_read = decoder.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Shared method to read first N lines of a file as bytes, split into lines.
///
/// Args:
/// path (&str): The path to the file to process.
/// n (usize): The number of lines to process.
///
/// Returns:
/// Result<Vec<Vec<u8>>, io::Error>: A vector of byte vectors, each representing a line, or an IO error.
pub fn read_first_n_lines(path: &str, n: usize) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut lines = Vec::with_capacity(n);

    for line in reader.split(b'\n').take(n) {
        lines.push(line?);
    }

    Ok(lines)
}

/// Shared method to read first N lines of a Gzipped file as bytes, split into lines.
///
/// Args:
/// path (&str): The path to the file to process.
/// n (usize): The number of lines to process.
pub fn read_gz_first_n_lines(path: &str, n: usize) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::with_capacity(BUFFER_SIZE, decoder);
    let mut lines = Vec::with_capacity(n);

    for line in reader.split(b'\n').take(n) {
        lines.push(line?);
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_lines() {
        let path = format!("{}/resources/usc.100.jsonl", env!("CARGO_MANIFEST_DIR"));
        let lines = iter_lines(&path).unwrap();
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0].chars().next(), Some('{'));
    }

    #[test]
    fn test_iter_gz_lines() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt.gz
        let path = format!("{}/resources/usc.100.jsonl.gz", env!("CARGO_MANIFEST_DIR"));
        let lines = iter_gz_lines(&path).unwrap();
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0].chars().next(), Some('{'));
    }

    #[test]
    fn test_read_file_content() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt
        let path = format!("{}/resources/hello-world.txt", env!("CARGO_MANIFEST_DIR"));
        let content = read_file_content(&path).unwrap();
        assert_eq!(content, b"Hello, world!\n");
    }

    #[test]
    fn test_read_gz_file_content() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt.gz
        let path = format!(
            "{}/resources/hello-world.txt.gz",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = read_gz_file_content(&path).unwrap();
        assert_eq!(content, b"Hello, world!");
    }

    #[test]
    fn test_read_lines() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt
        let path = format!("{}/resources/hello-world.txt", env!("CARGO_MANIFEST_DIR"));
        let lines = read_lines(&path).unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], b"Hello, world!");
    }

    #[test]
    fn test_read_gz_lines() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt.gz
        let path = format!(
            "{}/resources/hello-world.txt.gz",
            env!("CARGO_MANIFEST_DIR")
        );
        let lines = read_gz_lines(&path).unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], b"Hello, world!");
    }

    #[test]
    fn test_read_first_n_bytes() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt
        let path = format!("{}/resources/hello-world.txt", env!("CARGO_MANIFEST_DIR"));
        let content = read_first_n_bytes(&path, 5).unwrap();
        assert_eq!(content, b"Hello");
    }

    #[test]
    fn test_read_gz_first_n_bytes() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt.gz
        let path = format!(
            "{}/resources/hello-world.txt.gz",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = read_gz_first_n_bytes(&path, 5).unwrap();
        assert_eq!(content, b"Hello");
    }

    #[test]
    fn test_read_first_n_lines() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt
        let path = format!("{}/resources/usc.100.jsonl", env!("CARGO_MANIFEST_DIR"));
        let lines = read_first_n_lines(&path, 5).unwrap();
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_read_gz_first_n_lines() {
        // get CARGO_MANIFEST_DIR/resources/hello-world.txt.gz
        let path = format!("{}/resources/usc.100.jsonl.gz", env!("CARGO_MANIFEST_DIR"));
        let lines = read_gz_first_n_lines(&path, 5).unwrap();
        assert_eq!(lines.len(), 5);
    }
}
