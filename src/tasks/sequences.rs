// Utility methods for sequences

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn extract_content(encoded_content: &str) -> Vec<u8> {
    // decode the content
    let decoded_content = BASE64_STANDARD.decode(encoded_content.as_bytes()).unwrap();

    // decompress the content
    let mut zlib_decoder = ZlibDecoder::new(&decoded_content[..]);
    let mut output_buffer = Vec::new();
    zlib_decoder.read_to_end(&mut output_buffer).unwrap();

    output_buffer
}

pub fn split_sequence_max(sequence: &[i32], max_size: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    // check case where sequence is less than max_size
    if sequence.len() <= max_size {
        result.push(sequence.to_vec());
        return result;
    }

    // check case where sequence is less than 2 * max_size
    if sequence.len() <= 2 * max_size {
        result.push(sequence[0..max_size].to_vec());
        result.push(sequence[max_size..].to_vec());
        return result;
    }

    // otherwise, handle general case
    let mut start = 0;
    while start < sequence.len() {
        let end = (start + max_size).min(sequence.len());
        result.push(sequence[start..end].to_vec());
        start += max_size;
    }

    result
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_sequence_max() {
        let sequence = vec![1, 2, 3, 4, 5];
        let result = split_sequence_max(&sequence, 2);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);

        let sequence = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = split_sequence_max(&sequence, 3);
        assert_eq!(
            result,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]]
        );

        let sequence = vec![1, 2, 3];
        let result = split_sequence_max(&sequence, 5);
        assert_eq!(result, vec![vec![1, 2, 3]]);

        let sequence = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let result = split_sequence_max(&sequence, 4);
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15]
            ]
        );
    }

    // test extract_content with eJwLycgsVgCiRIWS1OISACRzBPY=
    #[test]
    fn test_extract_content() {
        let encoded_content = "eJwLycgsVgCiRIWS1OISACRzBPY=";
        let content = extract_content(encoded_content);
        assert_eq!(content, b"This is a test");
    }
}
