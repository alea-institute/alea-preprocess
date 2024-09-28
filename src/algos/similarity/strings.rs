/// String similarity measures and algorithms
///
/// This module provides functions for computing similarity between strings.
use strsim;

/// Compute similarity metric based on the distance between two strings
/// Return a bounded value regardless of the distance and string lengths.
fn edit_distance_to_similarity(distance: i64, a: &str, b: &str) -> f64 {
    (1.0 - (distance as f64 / a.len().max(b.len()) as f64))
        .max(0.0)
        .min(1.0)
}

/// Compute distance based on edit distance and string lengths
/// Return a bounded value regardless of the distance and string lengths.
/// Arguments:
/// - edit_distance: The similarity between the two strings
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The distance between the two strings
pub fn edit_distance_to_distance(edit_distance: i64, a: &str, b: &str) -> f64 {
    (edit_distance as f64) / a.len().max(b.len()) as f64
}

/// Compute similarity metric based on the distance between two strings
pub fn distance_to_similarity(distance: f64) -> f64 {
    (1.0 - distance).max(0.0).min(1.0)
}

/// Compute the Hamming distance between two strings
///
/// The Hamming distance is the number of positions at which the corresponding characters are different.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Hamming distance between the two strings divided by the maximum length of the two strings
pub fn hamming_distance(a: &str, b: &str) -> f64 {
    let edit_distance = match strsim::hamming(a, b) {
        Ok(d) => d as i64,
        Err(_) => a.len().max(b.len()) as i64,
    } as i64;
    edit_distance_to_distance(edit_distance, a, b) as f64
}

/// Compute the Hamming similarity between two strings
///
/// The Hamming similarity is the inverse of the Hamming distance.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Hamming similarity between the two strings
pub fn hamming_similarity(a: &str, b: &str) -> f64 {
    distance_to_similarity(hamming_distance(a, b))
}

/// Compute the Levenshtein distance between two strings
///
/// The Levenshtein distance is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one string into the other.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Levenshtein distance between the two strings divided by the maximum length of the two strings
/// (consistent with others, not normalized Levenshtein distance)
pub fn levenshtein_distance(a: &str, b: &str) -> f64 {
    edit_distance_to_distance(dbg!(strsim::levenshtein(a, b) as i64), a, b)
}

/// Compute the normalized Levenshtein distance between two strings
///
/// The normalized Levenshtein distance is the Levenshtein distance divided by the maximum length of the two strings.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The normalized Levenshtein distance between the two strings
pub fn normalized_levenshtein_distance(a: &str, b: &str) -> f64 {
    strsim::normalized_levenshtein(a, b)
}

/// Compute the Levenshtein similarity between two strings
///
/// The Levenshtein similarity is the inverse of the Levenshtein distance.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Levenshtein similarity between the two strings
pub fn levenshtein_similarity(a: &str, b: &str) -> f64 {
    distance_to_similarity(levenshtein_distance(a, b))
}

/// Compute the Optimal String Alignment distance between two strings
///
/// The Optimal String Alignment distance is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one string into the other, with the additional operation of transposing two adjacent characters.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Optimal String Alignment distance between the two strings
pub fn osa_distance(a: &str, b: &str) -> f64 {
    edit_distance_to_distance(strsim::osa_distance(a, b) as i64, a, b)
}

/// Compute the Optimal String Alignment similarity between two strings
///
/// The Optimal String Alignment similarity is the inverse of the Optimal String Alignment distance.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Optimal String Alignment similarity between the two strings
pub fn osa_similarity(a: &str, b: &str) -> f64 {
    distance_to_similarity(osa_distance(a, b))
}

/// Compute the Damerau-Levenshtein distance between two strings
///
/// The Damerau-Levenshtein distance is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one string into the other, with the additional operation of transposing two adjacent characters.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Damerau-Levenshtein distance between the two strings
pub fn damerau_levenshtein_distance(a: &str, b: &str) -> f64 {
    edit_distance_to_distance(strsim::damerau_levenshtein(a, b) as i64, a, b)
}

/// Compute the Damerau-Levenshtein similarity between two strings
///
/// The Damerau-Levenshtein similarity is the inverse of the Damerau-Levenshtein distance.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Damerau-Levenshtein similarity between the two strings
pub fn damerau_levenshtein_similarity(a: &str, b: &str) -> f64 {
    distance_to_similarity(damerau_levenshtein_distance(a, b))
}

/// Compute the normalized Damerau-Levenshtein distance between two strings
///
/// The normalized Damerau-Levenshtein distance is the Damerau-Levenshtein distance divided by the maximum length of the two strings.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The normalized Damerau-Levenshtein distance between the two strings
pub fn normalized_damerau_levenshtein_distance(a: &str, b: &str) -> f64 {
    strsim::normalized_damerau_levenshtein(a, b)
        .max(0.0)
        .min(1.0)
}

/// Compute the Jaro distance between two strings
/// Return a bounded value regardless of the distance and string lengths.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Jaro distance between the two strings
pub fn jaro_distance(a: &str, b: &str) -> f64 {
    distance_to_similarity(jaro_similarity(a, b))
}

/// Compute the Jaro similarity between two strings
///
/// The Jaro similarity is a measure of similarity between two strings that takes into account the number of matching characters and the number of transpositions.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Jaro similarity between the two strings
pub fn jaro_similarity(a: &str, b: &str) -> f64 {
    strsim::jaro(a, b).max(0.0).min(1.0)
}

/// Compute the Jaro-Winkler distance between two strings
/// Return a bounded value regardless of the distance and string lengths.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Jaro-Winkler distance between the two strings
pub fn jaro_winkler_distance(a: &str, b: &str) -> f64 {
    distance_to_similarity(jaro_winkler_similarity(a, b))
}

/// Compute the Jaro-Winkler similarity between two strings
///
/// The Jaro-Winkler similarity is a measure of similarity between two strings that takes into account the number of matching characters
/// and the number of transpositions, with a bonus for common prefixes.
/// Arguments:
/// - a: The first string
/// - b: The second string
/// Returns:
/// - The Jaro-Winkler similarity between the two strings
pub fn jaro_winkler_similarity(a: &str, b: &str) -> f64 {
    strsim::jaro_winkler(a, b).max(0.0).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test int_distance_to_similarity
    #[test]
    fn test_int_distance_to_similarity() {
        assert_eq!(edit_distance_to_similarity(0, "a", "a"), 1.0);
        assert_eq!(edit_distance_to_similarity(1, "a", "a"), 0.0);
        assert_eq!(edit_distance_to_similarity(1, "a", "b"), 0.0);
        assert_eq!(edit_distance_to_similarity(2, "a", "b"), 0.0);
    }

    // test float_distance_to_similarity
    #[test]
    fn test_float_distance_to_similarity() {
        assert_eq!(distance_to_similarity(0.0), 1.0);
        assert_eq!(distance_to_similarity(0.5), 0.5);
        assert_eq!(distance_to_similarity(1.0), 0.0);
    }

    #[test]
    fn test_hamming() {
        let a = "kittens";
        let b = "sitting";
        let c = "kitten";
        assert_eq!(hamming_distance(a, b), 3.0 / 7.0);
        assert_eq!(hamming_similarity(a, b), 4.0 / 7.0);
        // max distance for different lengths
        assert_eq!(hamming_distance(a, c), 1.0);
    }

    #[test]
    fn test_levenshtein() {
        let a = "kittens";
        let b = "sitting";
        let c = "kitten";
        assert_eq!(levenshtein_distance(a, b), 3.0 / 7.0);
        assert_eq!(levenshtein_similarity(a, b), 4.0 / 7.0);
        assert_eq!(levenshtein_distance(a, c), 1.0 / 7.0);
    }

    #[test]
    fn test_osa() {
        let a = "kittens";
        let b = "sitting";
        let c = "kitten";
        assert_eq!(osa_distance(a, b), 3.0 / 7.0);
        assert_eq!(osa_similarity(a, b), 4.0 / 7.0);
        assert_eq!(osa_distance(a, c), 1.0 / 7.0);
    }

    #[test]
    fn test_damerau_levenshtein() {
        let a = "kittens";
        let b = "sitting";
        let c = "kitten";
        assert_eq!(damerau_levenshtein_distance(a, b), 3.0 / 7.0);
        assert_eq!(damerau_levenshtein_similarity(a, b), 4.0 / 7.0);
        assert_eq!(damerau_levenshtein_distance(a, c), 1.0 / 7.0);
    }

    #[test]
    fn test_normalized_damerau_levenshtein() {
        let a = "kittens";
        let b = "sitting";
        assert_eq!(normalized_damerau_levenshtein_distance(a, b), 4.0 / 7.0);
    }

    #[test]
    fn test_jaro() {
        let a = "kittens";
        let b = "sitting";
        assert_eq!(jaro_distance(a, b), 2.0 / 7.0);
        assert_eq!(jaro_similarity(a, b), 5.0 / 7.0);
    }

    #[test]
    fn test_jaro_winkler() {
        let a = "kittens";
        let b = "sitting";
        assert_eq!(jaro_winkler_distance(a, b), 2.0 / 7.0);
        assert_eq!(jaro_winkler_similarity(a, b), 5.0 / 7.0);
    }
}
