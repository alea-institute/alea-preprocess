/// Extracts n-grams from a list of words.
use crate::algos::unicode::segmentations::segment_words;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Transform the input_data into a sequence of n-grams.
/// Arguments:
/// - `input_data`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A Vec of n-grams.
pub fn transform<const N: usize>(input_data: &str) -> Vec<Vec<String>> {
    segment_words(input_data)
        .par_windows(N)
        .map(|window| window.to_vec())
        .collect()
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_1(input_data: &str) -> Vec<Vec<String>> {
    transform::<1>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_2(input_data: &str) -> Vec<Vec<String>> {
    transform::<2>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_3(input_data: &str) -> Vec<Vec<String>> {
    transform::<3>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_4(input_data: &str) -> Vec<Vec<String>> {
    transform::<4>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_5(input_data: &str) -> Vec<Vec<String>> {
    transform::<5>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_6(input_data: &str) -> Vec<Vec<String>> {
    transform::<6>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_7(input_data: &str) -> Vec<Vec<String>> {
    transform::<7>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_8(input_data: &str) -> Vec<Vec<String>> {
    transform::<8>(&input_data)
}

/// Transform the input_data into a sequence of word unigrams.
/// Arguments:
/// - `input_data`: The input text.
/// Returns:
/// - A Vec of word unigrams.
pub fn transform_9(input_data: &str) -> Vec<Vec<String>> {
    transform::<9>(&input_data)
}

/// Extracts n-grams of words from the given text.
/// Arguments:
/// - `words`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A HashMap containing the n-grams as keys and their counts as values.
pub fn extract<const N: usize>(input_data: &str) -> HashMap<Vec<String>, u32>
where
    [String; N]: Hash + Eq,
{
    segment_words(input_data)
        .par_windows(N)
        .fold(HashMap::new, |mut acc, window| {
            let key = window.to_vec();
            *acc.entry(key).or_insert(0) += 1;
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
pub fn extract_1(input_data: &str) -> HashMap<Vec<String>, u32> {
    extract::<1>(&input_data)
}

pub fn extract_2(input_data: &str) -> HashMap<Vec<String>, u32> {
    extract::<2>(&input_data)
}

pub fn extract_3(input_data: &str) -> HashMap<Vec<String>, u32> {
    extract::<3>(&input_data)
}

pub fn extract_4(input_data: &str) -> HashMap<Vec<String>, u32> {
    extract::<4>(&input_data)
}

pub fn extract_5(input_data: &str) -> HashMap<Vec<String>, u32> {
    extract::<5>(&input_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_1() {
        let input_data = "hello world";
        let expected = vec![vec!["hello".to_string()], vec!["world".to_string()]];
        assert_eq!(transform_1(input_data), expected);
    }

    #[test]
    fn test_transform_2() {
        let input_data = "hello world, this is a test!";
        let expected = vec![
            vec!["hello".to_string(), "world".to_string()],
            vec!["world".to_string(), ",".to_string()],
            vec![",".to_string(), "this".to_string()],
            vec!["this".to_string(), "is".to_string()],
            vec!["is".to_string(), "a".to_string()],
            vec!["a".to_string(), "test".to_string()],
            vec!["test".to_string(), "!".to_string()],
        ];
        assert_eq!(transform_2(input_data), expected);
    }

    #[test]
    fn test_transform_3() {
        let input_data = "hello world, this is a test!";
        let expected = vec![
            vec!["hello".to_string(), "world".to_string(), ",".to_string()],
            vec!["world".to_string(), ",".to_string(), "this".to_string()],
            vec![",".to_string(), "this".to_string(), "is".to_string()],
            vec!["this".to_string(), "is".to_string(), "a".to_string()],
            vec!["is".to_string(), "a".to_string(), "test".to_string()],
            vec!["a".to_string(), "test".to_string(), "!".to_string()],
        ];
        assert_eq!(transform_3(input_data), expected);
    }

    #[test]
    fn test_transform_4() {
        let input_data = "hello world, this is a test!";
        let expected = vec![
            vec![
                "hello".to_string(),
                "world".to_string(),
                ",".to_string(),
                "this".to_string(),
            ],
            vec![
                "world".to_string(),
                ",".to_string(),
                "this".to_string(),
                "is".to_string(),
            ],
            vec![
                ",".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
            ],
            vec![
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            vec![
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
                "!".to_string(),
            ],
        ];
        assert_eq!(transform_4(input_data), expected);
    }

    #[test]
    fn test_transform_5() {
        let input_data = "hello world, this is a test!";
        let expected = vec![
            vec![
                "hello".to_string(),
                "world".to_string(),
                ",".to_string(),
                "this".to_string(),
                "is".to_string(),
            ],
            vec![
                "world".to_string(),
                ",".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
            ],
            vec![
                ",".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            vec![
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
                "!".to_string(),
            ],
        ];
        assert_eq!(transform_5(input_data), expected);
    }

    #[test]
    fn test_extract_1() {
        let input_data = "hello world";
        let expected = vec![
            (vec!["hello".to_string()], 1),
            (vec!["world".to_string()], 1),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        assert_eq!(extract_1(input_data), expected);
    }

    #[test]
    fn test_extract_2() {
        let input_data = "hello world";
        let expected = vec![(vec!["hello".to_string(), "world".to_string()], 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        assert_eq!(extract_2(input_data), expected);
    }

    #[test]
    fn test_extract_3() {
        let input_data = "hello, world!  how are you?";
        let result = extract_3(input_data);
        // cehck that ("hello", ",", "world") == 1
        assert_eq!(
            result[&vec!["hello".to_string(), ",".to_string(), "world".to_string()]],
            1
        );
    }

    #[test]
    fn test_extract_4() {
        let input_data = "hello, world!  how are you?";
        let result = extract_4(input_data);
        // cehck that ("hello", ",", "world", "!") == 1
        assert_eq!(
            result[&vec![
                "hello".to_string(),
                ",".to_string(),
                "world".to_string(),
                "!".to_string()
            ]],
            1
        );
    }

    #[test]
    fn test_extract_5() {
        let input_data = "hello, world!  how are you?";
        let result = extract_5(input_data);
        // cehck that ("hello", ",", "world", "!", "how") == 1
        assert_eq!(
            result[&vec![
                "hello".to_string(),
                ",".to_string(),
                "world".to_string(),
                "!".to_string(),
                "how".to_string()
            ]],
            1
        );
    }
}
