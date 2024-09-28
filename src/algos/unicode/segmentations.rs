use icu::properties::{maps, LineBreak};
use icu::segmenter::*;

/// Generates segments with their start and end indices from a list of breakpoints.
///
/// # Arguments
/// * `buffer` - The input string to segment
/// * `breakpoints` - A vector of indices where the string should be split
///
/// # Returns
/// A vector of tuples, each containing the start index, end index, and the segment as a String
pub fn get_segment_indices_from_breakpoints(
    buffer: &str,
    breakpoints: &Vec<usize>,
) -> Vec<(usize, usize, String)> {
    let mut result: Vec<(usize, usize, String)> = Vec::with_capacity(breakpoints.len() + 1);
    let mut start = 0;

    for &end in breakpoints.iter().chain(std::iter::once(&buffer.len())) {
        if start != end {
            let segment = String::from(&buffer[start..end]);
            result.push((start, end, segment));
        }
        start = end;
    }

    result
}

/// Generates segments from a list of breakpoints.
///
/// # Arguments
/// * `buffer` - The input string to segment
/// * `breakpoints` - A vector of indices where the string should be split
///
/// # Returns
/// A vector of Strings, each representing a segment
pub fn get_segments_from_breakpoints(buffer: &str, breakpoints: &Vec<usize>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(breakpoints.len() + 1);
    let mut start = 0;

    for &end in breakpoints.iter().chain(std::iter::once(&buffer.len())) {
        if start != end {
            result.push(String::from(&buffer[start..end]));
        }
        start = end;
    }

    result
}

/// Segments the input string into grapheme clusters with their indices.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of tuples, each containing the start index, end index, and the grapheme cluster as a String
pub fn get_grapheme_indices(buffer: &str) -> Vec<(usize, usize, String)> {
    let segmenter = GraphemeClusterSegmenter::new();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segment_indices_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into words with their indices.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of tuples, each containing the start index, end index, and the word as a String
pub fn get_word_indices(buffer: &str) -> Vec<(usize, usize, String)> {
    let segmenter = WordSegmenter::new_auto();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segment_indices_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into sentences with their indices.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of tuples, each containing the start index, end index, and the sentence as a String
pub fn get_sentence_indices(buffer: &str) -> Vec<(usize, usize, String)> {
    let segmenter = SentenceSegmenter::new();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segment_indices_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into lines with their indices.
///
/// This function uses a custom filter to determine line breaks based on specific Unicode properties.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of tuples, each containing the start index, end index, and the line as a String
pub fn get_line_indices(buffer: &str) -> Vec<(usize, usize, String)> {
    let segmenter = LineSegmenter::new_auto();
    let breakpoints: Vec<usize> = segmenter
        .segment_str(buffer)
        .into_iter()
        .filter(|&i| {
            buffer[..i].chars().next_back().map_or(false, |c| {
                matches!(
                    maps::line_break().get(c),
                    LineBreak::MandatoryBreak
                        | LineBreak::CarriageReturn
                        | LineBreak::LineFeed
                        | LineBreak::NextLine
                ) || i == buffer.len()
            })
        })
        .collect();

    get_segment_indices_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into grapheme clusters.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of Strings, each representing a grapheme cluster
pub fn segment_graphemes(buffer: &str) -> Vec<String> {
    let segmenter = GraphemeClusterSegmenter::new();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segments_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into words.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of Strings, each representing a word
pub fn segment_words(buffer: &str) -> Vec<String> {
    let segmenter = WordSegmenter::new_auto();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segments_from_breakpoints(buffer, &breakpoints)
        // but filter out any that are just whitespace
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect()
}

/// Segments the input string into sentences.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of Strings, each representing a sentence
pub fn segment_sentences(buffer: &str) -> Vec<String> {
    let segmenter = SentenceSegmenter::new();
    let breakpoints: Vec<usize> = segmenter.segment_str(buffer).collect();

    get_segments_from_breakpoints(buffer, &breakpoints)
}

/// Segments the input string into lines.
///
/// This function uses a custom filter to determine line breaks based on specific Unicode properties.
///
/// # Arguments
/// * `buffer` - The input string to segment
///
/// # Returns
/// A vector of Strings, each representing a line
pub fn segment_lines(buffer: &str) -> Vec<String> {
    let segmenter = LineSegmenter::new_auto();
    let breakpoints: Vec<usize> = segmenter
        .segment_str(buffer)
        .into_iter()
        .filter(|&i| {
            buffer[..i].chars().next_back().map_or(false, |c| {
                matches!(
                    maps::line_break().get(c),
                    LineBreak::MandatoryBreak
                        | LineBreak::CarriageReturn
                        | LineBreak::LineFeed
                        | LineBreak::NextLine
                ) || i == buffer.len()
            })
        })
        .collect();

    get_segments_from_breakpoints(buffer, &breakpoints)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_segment_indices_from_breakpoints() {
        let buffer = "a b c";
        let breakpoints = vec![1, 3];
        let result = get_segment_indices_from_breakpoints(buffer, &breakpoints);
        assert_eq!(
            result,
            vec![
                (0, 1, "a".to_string()),
                (1, 3, " b".to_string()),
                (3, 5, " c".to_string())
            ]
        );
    }

    #[test]
    fn test_get_segments_from_breakpoints() {
        let buffer = "a b c";
        let breakpoints = vec![1, 3];
        let result = get_segments_from_breakpoints(buffer, &breakpoints);
        assert_eq!(result, vec!["a", " b", " c"]);
    }

    #[test]
    fn test_get_grapheme_indices() {
        let buffer = "a b c d\u{0300}";
        let result = get_grapheme_indices(buffer);
        assert_eq!(
            result,
            vec![
                (0, 1, "a".to_string()),
                (1, 2, " ".to_string()),
                (2, 3, "b".to_string()),
                (3, 4, " ".to_string()),
                (4, 5, "c".to_string()),
                (5, 6, " ".to_string()),
                (6, 9, "d\u{300}".to_string())
            ]
        );
    }

    #[test]
    fn test_get_word_indices() {
        let buffer = "a b c";
        let result = get_word_indices(buffer);
        assert_eq!(
            result,
            vec![
                (0, 1, "a".to_string()),
                (1, 2, " ".to_string()),
                (2, 3, "b".to_string()),
                (3, 4, " ".to_string()),
                (4, 5, "c".to_string())
            ]
        );
    }

    #[test]
    fn test_get_sentence_indices() {
        let buffer = "Hello, world.  How are you?";
        let result = get_sentence_indices(buffer);
        assert_eq!(
            result,
            vec![
                (0, 15, "Hello, world.  ".to_string()),
                (15, 27, "How are you?".to_string())
            ]
        );
    }

    #[test]
    fn test_get_line_indices() {
        let buffer = "Hello, world.\nHow are you?";
        let result = get_line_indices(buffer);
        assert_eq!(
            result,
            vec![
                (0, 14, "Hello, world.\n".to_string()),
                (14, 26, "How are you?".to_string())
            ]
        );
    }

    // now test words and sentences
    #[test]
    fn test_words() {
        let buffer = "Hello, world; how are you?!";
        let result = segment_words(buffer);
        //assert_eq!(result, vec!["Hello", ",", " ", "world", ";", " ", "how", " ", "are", " ", "you", "?", "!"]);
        assert_eq!(
            result,
            vec!["Hello", ",", "world", ";", "how", "are", "you", "?", "!"]
        );
    }

    #[test]
    fn test_sentences() {
        let buffer = "Hello, world.  How are you?!";
        let result = segment_sentences(buffer);
        assert_eq!(result, vec!["Hello, world.  ", "How are you?!",]);
    }

    #[test]
    fn test_lines() {
        let buffer = "Hello, world.\nHow are you?";
        let result = segment_lines(buffer);
        assert_eq!(result, vec!["Hello, world.\n", "How are you?"]);
    }
}
