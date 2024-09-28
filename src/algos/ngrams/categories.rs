/// This module contains functions to extract n-grams of Unicode categories and Unicode category groups.
use crate::algos::unicode::categories::{
    category_to_group, char_to_category, UnicodeCategory, UnicodeCategoryGroup,
};
use rayon::prelude::*;
use std::collections::HashMap;

/// Transform the input text into a sequence of n-grams of Unicode categories.
/// Arguments:
/// - `text`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A Vec of n-grams.
pub fn transform_category<const N: usize>(text: &str) -> Vec<Vec<UnicodeCategory>> {
    text.chars()
        .map(|c| char_to_category(c))
        .collect::<Vec<UnicodeCategory>>()
        .par_windows(N)
        .map(|window| window.to_vec())
        .collect()
}

/// Transform the input text into a sequence of n-grams of Unicode category groups.
/// Arguments:
/// - `text`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A Vec of n-grams.
pub fn transform_category_group<const N: usize>(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    text.chars()
        .map(|c| category_to_group(char_to_category(c)))
        .collect::<Vec<UnicodeCategoryGroup>>()
        .par_windows(N)
        .map(|window| window.to_vec())
        .collect()
}

pub fn transform_category_1(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<1>(text)
}

pub fn transform_category_2(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<2>(text)
}

pub fn transform_category_3(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<3>(text)
}

pub fn transform_category_4(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<4>(text)
}

pub fn transform_category_5(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<5>(text)
}

pub fn transform_category_6(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<6>(text)
}

pub fn transform_category_7(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<7>(text)
}

pub fn transform_category_8(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<8>(text)
}

pub fn transform_category_9(text: &str) -> Vec<Vec<UnicodeCategory>> {
    transform_category::<9>(text)
}

pub fn transform_category_group_1(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<1>(text)
}

pub fn transform_category_group_2(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<2>(text)
}

pub fn transform_category_group_3(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<3>(text)
}

pub fn transform_category_group_4(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<4>(text)
}

pub fn transform_category_group_5(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<5>(text)
}

pub fn transform_category_group_6(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<6>(text)
}

pub fn transform_category_group_7(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<7>(text)
}

pub fn transform_category_group_8(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<8>(text)
}

pub fn transform_category_group_9(text: &str) -> Vec<Vec<UnicodeCategoryGroup>> {
    transform_category_group::<9>(text)
}

/// Extracts n-grams of Unicode categories from the given text.
/// Arguments:
/// - `text`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A HashMap containing the n-grams as keys and their counts as values.
pub fn extract_category<const N: usize>(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    text.chars()
        .map(|c| char_to_category(c))
        .collect::<Vec<UnicodeCategory>>()
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

/// Extracts n-grams of Unicode category groups from the given text.
/// Arguments:
/// - `text`: The input text.
/// - `N`: The size of the n-grams.
/// Returns:
/// - A HashMap containing the n-grams as keys and their counts as values.
pub fn extract_category_group<const N: usize>(
    text: &str,
) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    text.chars()
        .map(|c| category_to_group(char_to_category(c)))
        .collect::<Vec<UnicodeCategoryGroup>>()
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

pub fn extract_category_1(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    extract_category::<1>(text)
}

pub fn extract_category_2(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    extract_category::<2>(text)
}

pub fn extract_category_3(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    extract_category::<3>(text)
}

pub fn extract_category_4(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    extract_category::<4>(text)
}

pub fn extract_category_5(text: &str) -> HashMap<Vec<UnicodeCategory>, u32> {
    extract_category::<5>(text)
}

pub fn extract_category_group_1(text: &str) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    extract_category_group::<1>(text)
}

pub fn extract_category_group_2(text: &str) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    extract_category_group::<2>(text)
}

pub fn extract_category_group_3(text: &str) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    extract_category_group::<3>(text)
}

pub fn extract_category_group_4(text: &str) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    extract_category_group::<4>(text)
}

pub fn extract_category_group_5(text: &str) -> HashMap<Vec<UnicodeCategoryGroup>, u32> {
    extract_category_group::<5>(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::unicode::categories::{UnicodeCategory, UnicodeCategoryGroup};

    #[test]
    fn test_transform_category_1() {
        let input_data = "Hi !";
        let expected = vec![
            vec![UnicodeCategory::Lu],
            vec![UnicodeCategory::Ll],
            vec![UnicodeCategory::Zs],
            vec![UnicodeCategory::Po],
        ];

        // test equality
        let result = transform_category_1(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_2() {
        let input_data = "Hi !";
        let expected = vec![
            vec![UnicodeCategory::Lu, UnicodeCategory::Ll],
            vec![UnicodeCategory::Ll, UnicodeCategory::Zs],
            vec![UnicodeCategory::Zs, UnicodeCategory::Po],
        ];

        // test equality
        let result = transform_category_2(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_3() {
        let input_data = "Hi !";
        let expected = vec![
            vec![
                UnicodeCategory::Lu,
                UnicodeCategory::Ll,
                UnicodeCategory::Zs,
            ],
            vec![
                UnicodeCategory::Ll,
                UnicodeCategory::Zs,
                UnicodeCategory::Po,
            ],
        ];

        // test equality
        let result = transform_category_3(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_4() {
        let input_data = "Hi !";
        let expected = vec![vec![
            UnicodeCategory::Lu,
            UnicodeCategory::Ll,
            UnicodeCategory::Zs,
            UnicodeCategory::Po,
        ]];

        // test equality
        let result = transform_category_4(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_5() {
        let input_data = "Hi  !";
        let expected = vec![vec![
            UnicodeCategory::Lu,
            UnicodeCategory::Ll,
            UnicodeCategory::Zs,
            UnicodeCategory::Zs,
            UnicodeCategory::Po,
        ]];

        // test equality
        let result = transform_category_5(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_group_1() {
        let input_data = "Hi !";
        let expected = vec![
            vec![UnicodeCategoryGroup::L],
            vec![UnicodeCategoryGroup::L],
            vec![UnicodeCategoryGroup::Z],
            vec![UnicodeCategoryGroup::P],
        ];

        // test equality
        let result = transform_category_group_1(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_group_2() {
        let input_data = "Hi !";
        let expected = vec![
            vec![UnicodeCategoryGroup::L, UnicodeCategoryGroup::L],
            vec![UnicodeCategoryGroup::L, UnicodeCategoryGroup::Z],
            vec![UnicodeCategoryGroup::Z, UnicodeCategoryGroup::P],
        ];

        // test equality
        let result = transform_category_group_2(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_group_3() {
        let input_data = "Hi !";
        let expected = vec![
            vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::Z,
            ],
            vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::Z,
                UnicodeCategoryGroup::P,
            ],
        ];

        // test equality
        let result = transform_category_group_3(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_group_4() {
        let input_data = "Hi !";
        let expected = vec![vec![
            UnicodeCategoryGroup::L,
            UnicodeCategoryGroup::L,
            UnicodeCategoryGroup::Z,
            UnicodeCategoryGroup::P,
        ]];

        // test equality
        let result = transform_category_group_4(input_data);
        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_transform_category_group_5() {
        let input_data = "Hi  !";
        let expected = vec![vec![
            UnicodeCategoryGroup::L,
            UnicodeCategoryGroup::L,
            UnicodeCategoryGroup::Z,
            UnicodeCategoryGroup::Z,
            UnicodeCategoryGroup::P,
        ]];

        // test equality
        let result = transform_category_group_5(input_data);
        assert_eq!(result.len(), expected.len());
    }

    // test them with "Hello, world!"
    #[test]
    fn test_extract_category_1() {
        let input_data = "Hello, world!";
        let expected = vec![
            (UnicodeCategory::Lu, 1),
            (UnicodeCategory::Ll, 9),
            (UnicodeCategory::Po, 2),
            (UnicodeCategory::Zs, 1),
        ];

        let result = extract_category_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&vec![key]], value);
        }
    }

    #[test]
    fn test_extract_category_2() {
        let input_data = "Hello, world!";
        let result = extract_category_2(input_data);

        // check that we have "He" = 1 and "lo" = 1
        assert_eq!(result[&vec![UnicodeCategory::Lu, UnicodeCategory::Ll]], 1);
        assert_eq!(result[&vec![UnicodeCategory::Ll, UnicodeCategory::Ll]], 7);
    }

    #[test]
    fn test_extract_category_3() {
        let input_data = "Hello, world!";
        let result = extract_category_3(input_data);

        // check that we have "Hel" = 1 and "llo" = 1
        assert_eq!(
            result[&vec![
                UnicodeCategory::Lu,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll
            ]],
            1
        );
        assert_eq!(
            result[&vec![
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll
            ]],
            5
        );
    }

    #[test]
    fn test_extract_category_4() {
        let input_data = "Hello, world!";
        let result = extract_category_4(input_data);

        // check that we have "Hell" = 1 and "ello" = 1
        assert_eq!(
            result[&vec![
                UnicodeCategory::Lu,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll
            ]],
            1
        );
        assert_eq!(
            result[&vec![
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll
            ]],
            3
        );
    }

    #[test]
    fn test_extract_category_5() {
        let input_data = "Hello, world!";
        let result = extract_category_5(input_data);

        // check that we have "Hello" = 1 and "ello," = 1
        assert_eq!(
            result[&vec![
                UnicodeCategory::Lu,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll
            ]],
            1
        );
        assert_eq!(
            result[&vec![
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Ll,
                UnicodeCategory::Po,
                UnicodeCategory::Zs
            ]],
            1
        );
    }

    #[test]
    fn test_extract_category_group_1() {
        let input_data = "Hello, world!";
        let expected = vec![
            (UnicodeCategoryGroup::L, 10),
            (UnicodeCategoryGroup::P, 2),
            (UnicodeCategoryGroup::Z, 1),
        ];

        let result = extract_category_group_1(input_data);
        assert_eq!(result.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(result[&vec![key]], value);
        }
    }

    #[test]
    fn test_extract_category_group_2() {
        let input_data = "Hello, world!";
        let result = extract_category_group_2(input_data);

        // check that we have "He" = 1 and "lo" = 1
        assert_eq!(
            result[&vec![UnicodeCategoryGroup::L, UnicodeCategoryGroup::L]],
            8
        );
        assert_eq!(
            result[&vec![UnicodeCategoryGroup::L, UnicodeCategoryGroup::P]],
            2
        );
    }

    #[test]
    fn test_extract_category_group_3() {
        let input_data = "Hello, world!";
        let result = extract_category_group_3(input_data);

        // check that we have "Hel" = 1 and "llo" = 1
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L
            ]],
            6
        );
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::P
            ]],
            2
        );
    }

    #[test]
    fn test_extract_category_group_4() {
        let input_data = "Hello, world!";
        let result = extract_category_group_4(input_data);

        // check that we have "Hell" = 1 and "ello" = 1
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L
            ]],
            4
        );
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::P,
                UnicodeCategoryGroup::Z
            ]],
            1
        );
    }

    #[test]
    fn test_extract_category_group_5() {
        let input_data = "Hello, world!";
        let result = extract_category_group_5(input_data);

        // check that we have "Hello" = 1 and "ello," = 1
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::P
            ]],
            2
        );
        assert_eq!(
            result[&vec![
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::L,
                UnicodeCategoryGroup::P,
                UnicodeCategoryGroup::Z,
                UnicodeCategoryGroup::L
            ]],
            1
        );
    }
}
