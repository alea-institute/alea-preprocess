/// Extracts byte n-grams from a byte slices.
use rayon::prelude::*;
use std::collections::HashMap;

/// Transform the input_data into a sequence of n-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A Vec of n-grams.
pub fn transform<const N: usize>(input_data: &[u8]) -> Vec<Vec<u8>> {
    input_data
        .par_windows(N)
        .map(|window| window.to_vec())
        .collect()
}

/// Extracts byte n-grams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A HashMap with the n-grams as keys and their counts as values.
pub fn extract<const N: usize>(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    {
        input_data
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
}

/// Transform the input_data into a sequence of byte unigrams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte unigrams.
pub fn transform_1(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<1>(input_data)
}

/// Transform the input_data into a sequence of byte bigrams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte bigrams.
pub fn transform_2(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<2>(input_data)
}

/// Transform the input_data into a sequence of byte trigrams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte trigrams.
pub fn transform_3(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<3>(input_data)
}

/// Transform the input_data into a sequence of byte 4-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 4-grams.
pub fn transform_4(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<4>(input_data)
}

/// Transform the input_data into a sequence of byte 5-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 5-grams.
pub fn transform_5(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<5>(input_data)
}

/// Transform the input_data into a sequence of byte 5-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 5-grams.
pub fn transform_6(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<6>(input_data)
}

/// Transform the input_data into a sequence of byte 5-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 5-grams.
pub fn transform_7(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<7>(input_data)
}

/// Transform the input_data into a sequence of byte 5-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 5-grams.
pub fn transform_8(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<8>(input_data)
}

/// Transform the input_data into a sequence of byte 5-grams.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A Vec of byte 5-grams.
pub fn transform_9(input_data: &[u8]) -> Vec<Vec<u8>> {
    transform::<9>(input_data)
}

/// Extracts byte unigrams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A HashMap with the unigrams as keys and their counts as values.
pub fn extract_1(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    extract::<1>(input_data)
}

/// Extracts byte bigrams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A HashMap with the bigrams as keys and their counts as values.
pub fn extract_2(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    extract::<2>(input_data)
}

/// Extracts byte trigrams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A HashMap with the trigrams as keys and their counts as values.
pub fn extract_3(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    extract::<3>(input_data)
}

/// Extracts byte 4-grams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A HashMap with the 4-grams as keys and their counts as values.
pub fn extract_4(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    extract::<4>(input_data)
}

/// Extracts byte 5-grams from a byte slice.
/// Arguments:
/// - `input_data`: A byte slice.
/// Returns:
/// - A HashMap with the 5-grams as keys and their counts as values.
pub fn extract_5(input_data: &[u8]) -> HashMap<Vec<u8>, u32> {
    extract::<5>(input_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_1() {
        let input_data = b"hello world";
        let expected = vec![
            vec![b'h'],
            vec![b'e'],
            vec![b'l'],
            vec![b'l'],
            vec![b'o'],
            vec![b' '],
            vec![b'w'],
            vec![b'o'],
            vec![b'r'],
            vec![b'l'],
            vec![b'd'],
        ];

        let result = transform_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_2() {
        let input_data = b"hello world";
        let expected = vec![
            vec![b'h', b'e'],
            vec![b'e', b'l'],
            vec![b'l', b'l'],
            vec![b'l', b'o'],
            vec![b'o', b' '],
            vec![b' ', b'w'],
            vec![b'w', b'o'],
            vec![b'o', b'r'],
            vec![b'r', b'l'],
            vec![b'l', b'd'],
        ];

        let result = transform_2(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_3() {
        let input_data = b"hello world";
        let expected = vec![
            vec![b'h', b'e', b'l'],
            vec![b'e', b'l', b'l'],
            vec![b'l', b'l', b'o'],
            vec![b'l', b'o', b' '],
            vec![b'o', b' ', b'w'],
            vec![b' ', b'w', b'o'],
            vec![b'w', b'o', b'r'],
            vec![b'o', b'r', b'l'],
            vec![b'r', b'l', b'd'],
        ];

        let result = transform_3(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_4() {
        let input_data = b"hello world";
        let expected = vec![
            vec![b'h', b'e', b'l', b'l'],
            vec![b'e', b'l', b'l', b'o'],
            vec![b'l', b'l', b'o', b' '],
            vec![b'l', b'o', b' ', b'w'],
            vec![b'o', b' ', b'w', b'o'],
            vec![b' ', b'w', b'o', b'r'],
            vec![b'w', b'o', b'r', b'l'],
            vec![b'o', b'r', b'l', b'd'],
        ];

        let result = transform_4(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_transform_5() {
        let input_data = b"hello world";
        let expected = vec![
            vec![b'h', b'e', b'l', b'l', b'o'],
            vec![b'e', b'l', b'l', b'o', b' '],
            vec![b'l', b'l', b'o', b' ', b'w'],
            vec![b'l', b'o', b' ', b'w', b'o'],
            vec![b'o', b' ', b'w', b'o', b'r'],
            vec![b' ', b'w', b'o', b'r', b'l'],
            vec![b'w', b'o', b'r', b'l', b'd'],
        ];

        let result = transform_5(input_data);
        assert_eq!(result.len(), expected.len());
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(result[index], *value);
        }
    }

    #[test]
    fn test_extract_1() {
        let input_data = b"hello world";
        let expected = vec![
            (b'h', 1),
            (b'e', 1),
            (b'l', 3),
            (b'o', 2),
            (b' ', 1),
            (b'w', 1),
            (b'r', 1),
            (b'd', 1),
        ];

        let result = extract_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&vec![key]], value);
        }
    }

    #[test]
    fn test_extract_2() {
        let input_data = b"hello world";
        let expected = vec![
            (b"he".to_vec(), 1),
            (b"el".to_vec(), 1),
            (b"ll".to_vec(), 1),
            (b"lo".to_vec(), 1),
            (b"o ".to_vec(), 1),
            (b" w".to_vec(), 1),
            (b"wo".to_vec(), 1),
            (b"or".to_vec(), 1),
            (b"rl".to_vec(), 1),
            (b"ld".to_vec(), 1),
        ];

        let result = extract_2(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&key.to_vec()], value);
        }
    }

    #[test]
    fn test_extract_3() {
        let input_data = b"hello world";
        let expected = vec![
            (b"hel".to_vec(), 1),
            (b"ell".to_vec(), 1),
            (b"llo".to_vec(), 1),
            (b"lo ".to_vec(), 1),
            (b"o w".to_vec(), 1),
            (b" wo".to_vec(), 1),
            (b"wor".to_vec(), 1),
            (b"orl".to_vec(), 1),
            (b"rld".to_vec(), 1),
        ];

        let result = extract_3(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&key.to_vec()], value);
        }
    }

    #[test]
    fn test_extract_4() {
        let input_data = b"hello world";
        let expected = vec![
            (b"hell".to_vec(), 1),
            (b"ello".to_vec(), 1),
            (b"llo ".to_vec(), 1),
            (b"lo w".to_vec(), 1),
            (b"o wo".to_vec(), 1),
            (b" wor".to_vec(), 1),
            (b"worl".to_vec(), 1),
            (b"orld".to_vec(), 1),
        ];

        let result = extract_4(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&key.to_vec()], value);
        }
    }

    #[test]
    fn test_extract_5() {
        let input_data = b"hello world";
        let expected = vec![
            (b"hello".to_vec(), 1),
            (b"ello ".to_vec(), 1),
            (b"llo w".to_vec(), 1),
            (b"lo wo".to_vec(), 1),
            (b"o wor".to_vec(), 1),
            (b" worl".to_vec(), 1),
            (b"world".to_vec(), 1),
        ];

        let result = extract_5(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&key.to_vec()], value);
        }
    }
}
