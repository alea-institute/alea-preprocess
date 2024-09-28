use rayon::prelude::*;
use std::ops::Range;

/// Represents a pattern used for splitting, which can be either raw text or tokenized.
#[derive(Clone)]
enum SplitPattern {
    /// Raw text pattern as a string.
    Text(String),
    /// Tokenized pattern as a vector of token IDs.
    Tokenized(Vec<u32>),
}

/// Defines the constraints for splitting the input.
#[derive(Clone)]
struct SplitConstraints {
    /// The minimum size of a split chunk.
    min_size: usize,
    /// The maximum size of a split chunk.
    max_size: usize,
    /// The patterns to use for splitting.
    split_patterns: Vec<SplitPattern>,
}

/// Finds the largest possible split point within the given range for text.
fn find_largest_split_text(
    text: &str,
    start: usize,
    end: usize,
    patterns: &[String],
) -> Option<usize> {
    for pattern in patterns {
        if let Some(index) = text[start..end].rfind(pattern) {
            return Some(start + index + pattern.len());
        }
    }
    None
}

/// Finds the largest possible split point within the given range for tokens.
fn find_largest_split_tokens(
    tokens: &[u32],
    start: usize,
    end: usize,
    patterns: &[Vec<u32>],
) -> Option<usize> {
    for pattern in patterns {
        for i in (start..=end.saturating_sub(pattern.len())).rev() {
            if tokens[i..].starts_with(pattern) {
                return Some(i + pattern.len());
            }
        }
    }
    None
}

/// Splits the text input based on the given constraints.
fn split_input_text(text: &str, constraints: &SplitConstraints) -> Vec<Range<usize>> {
    let mut splits = Vec::new();
    let mut start = 0;
    let char_indices: Vec<(usize, char)> = text.char_indices().collect();
    let total_chars = char_indices.len();

    while start < total_chars {
        let mut end = (start + constraints.max_size).min(total_chars);

        if end - start < constraints.min_size {
            splits.push(
                char_indices[start].0..char_indices[end - 1].0 + char_indices[end - 1].1.len_utf8(),
            );
            break;
        }

        if let Some(SplitPattern::Text(ref patterns)) = constraints.split_patterns.first() {
            if let Some(split_point) = find_largest_split_text(
                text,
                char_indices[start].0,
                char_indices[end - 1].0 + char_indices[end - 1].1.len_utf8(),
                &[patterns.clone()],
            ) {
                end = char_indices
                    .binary_search_by_key(&split_point, |&(i, _)| i)
                    .unwrap_or_else(|e| e);
            }
        }

        splits.push(
            char_indices[start].0..char_indices[end - 1].0 + char_indices[end - 1].1.len_utf8(),
        );
        start = end;
    }

    splits
}

/// Splits the tokenized input based on the given constraints.
fn split_input_tokens(tokens: &[u32], constraints: &SplitConstraints) -> Vec<Range<usize>> {
    let mut splits = Vec::new();
    let mut start = 0;
    let total_tokens = tokens.len();

    while start < total_tokens {
        let mut end = (start + constraints.max_size).min(total_tokens);

        if end - start < constraints.min_size {
            splits.push(start..end);
            break;
        }

        if let Some(SplitPattern::Tokenized(ref patterns)) = constraints.split_patterns.first() {
            if let Some(split_point) =
                find_largest_split_tokens(tokens, start, end, &[patterns.clone()])
            {
                end = split_point;
            }
        }

        splits.push(start..end);
        start = end;
    }

    splits
}

/// Splits a string into chunks based on the given constraints.
pub fn split_str(
    text: &str,
    min_size: usize,
    max_size: usize,
    split_patterns: Vec<String>,
) -> Vec<String> {
    let constraints = SplitConstraints {
        min_size,
        max_size,
        split_patterns: split_patterns.into_iter().map(SplitPattern::Text).collect(),
    };
    split_input_text(text, &constraints)
        .into_iter()
        .map(|range| text[range].to_string())
        .collect()
}

/// Splits a sequence of tokens into chunks based on the given constraints.
pub fn split_tokens(
    tokens: &[u32],
    min_size: usize,
    max_size: usize,
    split_patterns: Vec<u32>,
) -> Vec<Vec<u32>> {
    let constraints = SplitConstraints {
        min_size,
        max_size,
        split_patterns: split_patterns
            .into_iter()
            .map(|p| SplitPattern::Tokenized(vec![p]))
            .collect(),
    };
    split_input_tokens(tokens, &constraints)
        .into_iter()
        .map(|range| tokens[range].to_vec())
        .collect()
}

/// Splits a list of strings into chunks based on the given constraints, using parallel processing.
/// This function is useful when you have a large list of strings to split.
/// # Arguments
/// - `texts`: A list of strings to split.
/// - `min_size`: The minimum size of a split chunk.
/// - `max_size`: The maximum size of a split chunk.
/// - `split_patterns`: A list of patterns to use for splitting.
/// # Returns
/// A list of split chunks for each input string.
pub fn split_str_list(
    texts: &[String],
    min_size: usize,
    max_size: usize,
    split_patterns: Vec<String>,
) -> Vec<Vec<String>> {
    texts
        .par_iter()
        .map(|text| {
            let constraints = SplitConstraints {
                min_size,
                max_size,
                split_patterns: split_patterns
                    .clone()
                    .into_iter()
                    .map(SplitPattern::Text)
                    .collect(),
            };
            split_input_text(text, &constraints)
                .into_iter()
                .map(|range| text[range].to_string())
                .collect()
        })
        .collect()
}

/// Splits a list of token sequences into chunks based on the given constraints, using parallel processing.
/// This function is useful when you have a large list of token sequences to split.
/// # Arguments
/// - `token_lists`: A list of token sequences to split.
/// - `min_size`: The minimum size of a split chunk.
/// - `max_size`: The maximum size of a split chunk.
/// - `split_patterns`: A list of patterns to use for splitting.
/// # Returns
/// A list of split chunks for each input token sequence.
pub fn split_token_list(
    token_lists: &[Vec<u32>],
    min_size: usize,
    max_size: usize,
    split_patterns: Vec<u32>,
) -> Vec<Vec<Vec<u32>>> {
    token_lists
        .par_iter()
        .map(|tokens| {
            let constraints = SplitConstraints {
                min_size,
                max_size,
                split_patterns: split_patterns
                    .clone()
                    .into_iter()
                    .map(|p| SplitPattern::Tokenized(vec![p]))
                    .collect(),
            };
            split_input_tokens(tokens, &constraints)
                .into_iter()
                .map(|range| tokens[range].to_vec())
                .collect()
        })
        .collect()
}
