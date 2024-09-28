use super::categories::*;
use icu::normalizer::{ComposingNormalizer, DecomposingNormalizer};

/// Returns a buffer normalized to NFD.
///
/// Transforms the buffer into Normalization Form D (NFD).
/// Arguments:
/// - `buffer` - A string.
/// Returns:
/// - A string representing the buffer normalized to NFD.
pub fn nfd_str(buffer: &str) -> String {
    let normalizer = DecomposingNormalizer::new_nfd();
    normalizer.normalize(buffer)
}

/// Returns a buffer normalized to NFKD.
///
/// Transforms the buffer into Normalization Form KD (NFKD).
/// Arguments:
/// - `buffer` - A string.
/// Returns:
/// - A string representing the buffer normalized to NFKD.
pub fn nfkd_str(buffer: &str) -> String {
    let normalizer = DecomposingNormalizer::new_nfkd();
    normalizer.normalize(buffer)
}

/// Returns a buffer normalized to NFC.
///
/// Transforms the buffer into Normalization Form C (NFC).
/// Arguments:
/// - `buffer` - A string.
/// Returns:
/// - A string representing the buffer normalized to NFC.
pub fn nfc_str(buffer: &str) -> String {
    let normalizer = ComposingNormalizer::new_nfc();
    normalizer.normalize(buffer)
}

/// Returns a buffer normalized to NFKC.
///
/// Transforms the buffer into Normalization Form KC (NFKC).
/// Arguments:
/// - `buffer` - A string.
/// Returns:
/// - A string representing the buffer normalized to NFKC.
pub fn nfkc_str(buffer: &str) -> String {
    let normalizer = ComposingNormalizer::new_nfkc();
    normalizer.normalize(buffer)
}

/// Returns a buffer of normalized and printable NFKC characters only.
/// Arguments:
/// - `buffer` - A string.
/// Returns:
/// - A string representing the buffer of normalized and printable NFKC characters only.
pub fn nfkc_printable_str(buffer: &str) -> String {
    nfkc_str(buffer)
        .chars()
        .filter_map(|c| {
            if char_to_category(c) == UnicodeCategory::Cc {
                if c == '\n' {
                    Some('\n')
                } else if c == '\r' {
                    Some('\n')
                } else if c == '\t' {
                    Some('\t')
                } else {
                    None
                }
            } else {
                Some(c)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfd_str() {
        let buffer = "a\u{0300}";
        let result = nfd_str(buffer);
        assert_eq!(result, "a\u{0300}");
    }

    #[test]
    fn test_nfkd_str() {
        let buffer = "a\u{0300}";
        let result = nfkd_str(buffer);
        assert_eq!(result, "à");
    }

    #[test]
    fn test_nfc_str() {
        let buffer = "a\u{0300}";
        let result = nfc_str(buffer);
        assert_eq!(result, "à");
    }

    #[test]
    fn test_nfkc_str() {
        let buffer = "a\u{0300}";
        let result = nfkc_str(buffer);
        assert_eq!(result, "à");
    }

    #[test]
    fn test_nfkc_printable_str() {
        let buffer = "recreation\nand parks";
        let result = nfkc_printable_str(buffer);
        assert_eq!(result, "recreation\nand parks");
    }
}
