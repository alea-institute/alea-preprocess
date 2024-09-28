/// This module provides a function to extract n-grams of characters from a given text.
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Transform the input_data into a sequence of n-grams.
/// Arguments:
/// - `input_data`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A Vec of n-grams.
pub fn transform<const N: usize>(input_data: &str) -> Vec<Vec<char>> {
    input_data
        .chars()
        .collect::<Vec<char>>()
        .par_windows(N)
        .map(|window| window.to_vec())
        .collect()
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_1(input_data: &str) -> Vec<Vec<char>> {
    transform::<1>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_2(input_data: &str) -> Vec<Vec<char>> {
    transform::<2>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_3(input_data: &str) -> Vec<Vec<char>> {
    transform::<3>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_4(input_data: &str) -> Vec<Vec<char>> {
    transform::<4>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_5(input_data: &str) -> Vec<Vec<char>> {
    transform::<5>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_6(input_data: &str) -> Vec<Vec<char>> {
    transform::<6>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_7(input_data: &str) -> Vec<Vec<char>> {
    transform::<7>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_8(input_data: &str) -> Vec<Vec<char>> {
    transform::<8>(input_data)
}

/// Transform the input_data into a sequence of character unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of character unigrams.
pub fn transform_9(input_data: &str) -> Vec<Vec<char>> {
    transform::<9>(input_data)
}

/// Extracts n-grams of characters from the given text.
/// Arguments:
/// - `input_data`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A HashMap containing the n-grams as keys and their counts as values.
pub fn extract<const N: usize>(input_data: &str) -> HashMap<Vec<char>, u32>
where
    [char; N]: Hash + Eq,
{
    input_data
        .chars()
        .collect::<Vec<char>>()
        .par_windows(N)
        .fold(HashMap::new, |mut acc, window| {
            *acc.entry(window.to_vec()).or_insert(0) += 1;
            acc
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        })
}

// Inlined methods for n-gram sizes 1-5
pub fn extract_1(input_data: &str) -> HashMap<Vec<char>, u32> {
    extract::<1>(input_data)
}

pub fn extract_2(input_data: &str) -> HashMap<Vec<char>, u32> {
    extract::<2>(input_data)
}

pub fn extract_3(input_data: &str) -> HashMap<Vec<char>, u32> {
    extract::<3>(input_data)
}

pub fn extract_4(input_data: &str) -> HashMap<Vec<char>, u32> {
    extract::<4>(input_data)
}

pub fn extract_5(input_data: &str) -> HashMap<Vec<char>, u32> {
    extract::<5>(input_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_1() {
        let input_data = "hello world";
        let expected = vec![
            vec!['h'],
            vec!['e'],
            vec!['l'],
            vec!['l'],
            vec!['o'],
            vec![' '],
            vec!['w'],
            vec!['o'],
            vec!['r'],
            vec!['l'],
            vec!['d'],
        ];

        let result = transform_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_2() {
        let input_data = "hello world";
        let expected = vec![
            vec!['h', 'e'],
            vec!['e', 'l'],
            vec!['l', 'l'],
            vec!['l', 'o'],
            vec!['o', ' '],
            vec![' ', 'w'],
            vec!['w', 'o'],
            vec!['o', 'r'],
            vec!['r', 'l'],
            vec!['l', 'd'],
        ];

        let result = transform_2(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_3() {
        let input_data = "hello world";
        let expected = vec![
            vec!['h', 'e', 'l'],
            vec!['e', 'l', 'l'],
            vec!['l', 'l', 'o'],
            vec!['l', 'o', ' '],
            vec!['o', ' ', 'w'],
            vec![' ', 'w', 'o'],
            vec!['w', 'o', 'r'],
            vec!['o', 'r', 'l'],
            vec!['r', 'l', 'd'],
        ];

        let result = transform_3(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_4() {
        let input_data = "hello world";
        let expected = vec![
            vec!['h', 'e', 'l', 'l'],
            vec!['e', 'l', 'l', 'o'],
            vec!['l', 'l', 'o', ' '],
            vec!['l', 'o', ' ', 'w'],
            vec!['o', ' ', 'w', 'o'],
            vec![' ', 'w', 'o', 'r'],
            vec!['w', 'o', 'r', 'l'],
            vec!['o', 'r', 'l', 'd'],
        ];

        let result = transform_4(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_5() {
        let input_data = "hello world";
        let expected = vec![
            vec!['h', 'e', 'l', 'l', 'o'],
            vec!['e', 'l', 'l', 'o', ' '],
            vec!['l', 'l', 'o', ' ', 'w'],
            vec!['l', 'o', ' ', 'w', 'o'],
            vec!['o', ' ', 'w', 'o', 'r'],
            vec![' ', 'w', 'o', 'r', 'l'],
            vec!['w', 'o', 'r', 'l', 'd'],
        ];

        let result = transform_5(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_extract_1() {
        let input_data = "hello world";
        let expected = vec![
            ('h', 1),
            ('e', 1),
            ('l', 3),
            ('o', 2),
            (' ', 1),
            ('w', 1),
            ('r', 1),
            ('d', 1),
        ];

        let result = extract_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&vec![key]], value);
        }
    }

    #[test]
    fn test_extract_2() {
        let input_data = "hello world";
        let result = extract_2(input_data);

        // check that we have "he" = 1 and "ow" = 1
        assert_eq!(result[&vec!['h', 'e']], 1);
        assert_eq!(result[&vec![' ', 'w']], 1);
    }

    #[test]
    fn test_extract_3() {
        let input_data = "hello world";
        let result = extract_3(input_data);

        // check that we have "hel" = 1 and "llo" = 1
        assert_eq!(result[&vec!['h', 'e', 'l']], 1);
        assert_eq!(result[&vec!['l', 'l', 'o']], 1);
    }

    #[test]
    fn test_extract_4() {
        let input_data = "hello world";
        let result = extract_4(input_data);

        // check that we have "hell" = 1 and "llo " = 1
        assert_eq!(result[&vec!['h', 'e', 'l', 'l']], 1);
        assert_eq!(result[&vec!['l', 'l', 'o', ' ']], 1);
    }

    #[test]
    fn test_extract_5() {
        let input_data = "hello world";
        let result = extract_5(input_data);

        // check that we have "hello" = 1 and "lo wo" = 1
        assert_eq!(result[&vec!['h', 'e', 'l', 'l', 'o']], 1);
        assert_eq!(result[&vec!['l', 'o', ' ', 'w', 'o']], 1);
    }
}
