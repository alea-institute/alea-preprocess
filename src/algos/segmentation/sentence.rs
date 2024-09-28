/// Sentence segmentation algorithms from ALEA Institute, including
///  - abbreviation collection routines
/// - extremely  fast, deterministic sentence boundary detection
/// - calibrated classifier for sentence boundary detection
use crate::algos::unicode::segmentations::get_word_indices;
use crate::algos::segmentation::abbreviations::ABBREVIATIONS;
use regex::Regex;

/**
Get abbreviations from a text string using a simple heuristic.

# Arguments
* `input_text` - A string slice that holds the input text.

# Returns
* A vector of strings that holds the abbreviations found in the input text.
**/
pub fn get_abbreviations_simple(input_text: &str) -> Vec<String> {
    let mut abbreviations = Vec::new();
    let token_indices = get_word_indices(input_text);

    let mut i = 0;
    while i < token_indices.len() {
        let (_, _, current_token) = &token_indices[i];

        if current_token == "." && i > 0 {
            let (_, _, previous_token) = &token_indices[i - 1];
            let mut combined_token = previous_token.to_string();

            let is_potential_abbrev = {
                let chars: Vec<char> = previous_token.chars().collect();
                (chars.len() == 1 && chars[0].is_alphabetic())
                    || (chars.len() > 1 && chars.iter().all(|&c| c.is_uppercase()))
                    || (previous_token.contains('.') && !previous_token.ends_with('.'))
                    || (chars.len() > 1
                        && chars.len() <= 4
                        && chars[0].is_uppercase()
                        && chars[1..].iter().all(|&c| c.is_lowercase()))
            };

            if is_potential_abbrev {
                combined_token.push('.');

                // Check the next token (if it exists)
                if i + 1 < token_indices.len() {
                    let (_, _, next_token) = &token_indices[i + 1];
                    let is_valid_next_token = next_token.trim().is_empty()
                        || (next_token.chars().count() == 1
                            && next_token.chars().next().unwrap().is_ascii_punctuation())
                        || (next_token.chars().next().unwrap().is_uppercase()
                            && next_token.chars().count() > 1);

                    if is_valid_next_token && !abbreviations.contains(&combined_token) {
                        abbreviations.push(combined_token);
                    }
                } else {
                    // If it's the last token, consider it an abbreviation
                    if !abbreviations.contains(&combined_token) {
                        abbreviations.push(combined_token);
                    }
                }
            }
        } else if current_token.contains('.') && !current_token.ends_with('.') {
            // Handle cases like "U.S.C" without a separate period token
            if i + 1 < token_indices.len() {
                let (_, _, next_token) = &token_indices[i + 1];
                let is_valid_next_token = next_token.trim().is_empty()
                    || (next_token.chars().count() == 1
                        && next_token.chars().next().unwrap().is_ascii_punctuation())
                    || (next_token.chars().next().unwrap().is_uppercase()
                        && next_token.chars().count() > 1);

                if is_valid_next_token {
                    let full_abbrev = format!("{}.", current_token);
                    if !abbreviations.contains(&full_abbrev) {
                        abbreviations.push(full_abbrev);
                    }
                }
            } else {
                // If it's the last token, consider it an abbreviation
                let full_abbrev = format!("{}.", current_token);
                if !abbreviations.contains(&full_abbrev) {
                    abbreviations.push(full_abbrev);
                }
            }
        }

        i += 1;
    }

    abbreviations
}

/**
regex-based abbreviation detection

# Arguments
* `input_text` - A string slice that holds the input text.

# Returns
* A vector of strings that holds the abbreviations found in the input text.
**/
pub fn get_abbreviations_regex(input_text: &str) -> Vec<String> {
    let abbreviation_regex = Regex::new(
        r"\b(?:[A-Z]\.){2,}|\b(?:[A-Z][a-z]*\.){1,}|\b[A-Z][a-z]+\.|\b[A-Z]{1,3}\.|\b[a-z]{1,3}\.",
    )
    .unwrap();
    let mut abbreviations = Vec::new();
    for caps in abbreviation_regex.captures_iter(input_text) {
        abbreviations.push(caps.get(0).unwrap().as_str().to_string());
    }

    abbreviations
}


#[allow(unused_variables)]
pub fn get_sentence_boundaries(input_text: &str) -> Vec<usize> {
    // get the start, end, token values for the input text.
    let word_indices = get_word_indices(input_text);

    // store boundaries
    let mut boundaries = Vec::new();

    // terminal punctuation list
    let terminal_punctuation = vec![".", "!", "?"];

    // when we have a period token, look forward to see if it is a sentence boundary:
    //  - if the next token is whitespace, and the next alphabetic token is uppercase, and the
    // prior token is not an abbreviation, then it is a sentence boundary.

    for i in 1..word_indices.len() - 2 {
        // check if the current token is terminal punctuation
        let (c_start, c_end, c_token) = &word_indices[i];
        if terminal_punctuation.contains(&c_token.as_str()) {
            let (p1_start, p1_end, p1_token) = &word_indices[i - 1];
            let (n1_start, n1_end, n1_token) = &word_indices[i + 1];
            let (n2_start, n2_end, n2_token) = &word_indices[i + 2];

            if n1_token.trim() == "" && n2_token.trim() == "" {
                // add as a boundary
                boundaries.push(*c_end);
            } else if n1_token.trim() == "" && n1_token.len() > 1 {
                boundaries.push(*c_end);
            } else if n1_token.trim() == "" && n2_token.chars().next().unwrap().is_uppercase() {
                // check for abbreviations
                if ABBREVIATIONS.contains(&p1_token.as_str()) {
                    continue;
                }

                boundaries.push(*c_end);
            } else if n1_token.trim() == "" && n2_token.chars().next().unwrap().is_ascii_punctuation() {
                boundaries.push(*c_end);
            } else if n1_token.trim() == "" && n2_token.trim() == "" {
                // check for abbreviations
                if ABBREVIATIONS.contains(&p1_token.as_str()) {
                    continue;
                }

                boundaries.push(*c_end);
            }

            // dbg!(p2_token, p1_token, c_token, n1_token, n2_token);
        }
    }

    // add the last token as a boundary
    boundaries.push(word_indices.last().unwrap().1);

    boundaries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::fs::files::read_file_content;

    // get test file from CARGO_MANIFEST_DIR at usc.100.jsonl
    fn get_test_data() -> String {
        let path = format!("{}/resources/10usc101.txt", env!("CARGO_MANIFEST_DIR"));
        String::from_utf8_lossy(read_file_content(&path).unwrap().as_slice()).to_string()
    }

    #[allow(unused_variables)]
    #[test]
    fn test_get_abbreviations() {
        let text = get_test_data();
        let abbrevs = get_abbreviations_simple(&text);
    }

    #[allow(unused_variables)]
    #[test]
    fn test_get_abbreviations_regex() {
        let text = get_test_data();
        let abbrevs = get_abbreviations_regex(&text);
    }

    #[test]
    fn test_get_sentence_boundaries() {
        //let text = get_test_data();
        let text = "10 U.S.C. 42 says that it's Dr. and Mrs. Smiths' birthday. Don't have a Cow.  It'll be fine.";
        let boundaries = dbg!(get_sentence_boundaries(&text));

        // print the sentences from the boundaries
        let mut sentences = Vec::new();
        let mut start = 0;
        for end in boundaries {
            sentences.push(dbg!(&text[start..end]));
            start = end;
        }
    }
}
