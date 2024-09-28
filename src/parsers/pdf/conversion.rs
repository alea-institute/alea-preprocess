/// PDF text extraction module
/// This module provides functions to extract text from PDF files using
/// different methods, such as simple text extraction and position-aware
/// text extraction.
use crate::algos::unicode::normalizations::nfkc_printable_str;
use crate::parsers::pdf::gaps::{calculate_percentiles, gap_to_string, get_gaps};
use crate::parsers::pdf::utils::{get_font_size, get_font_weight};
use pdfium_render::prelude::*;
use regex::Regex;

/// Normalize PDF text by removing leading and trailing whitespace and
/// converting to NFC form.
/// Arguments:
/// - text: The text to normalize.
/// Returns:
/// - The normalized text.
pub fn normalize_pdf_text(text: &str) -> String {
    // get clean NFKC printable form
    let clean_text = text.replace("\n\n", "");
    let clean_text = clean_text.replace("\n", "");
    let clean_text = clean_text.replace(" ", "");
    let clean_text = clean_text.replace("", "");

    // build regex to match bold blocks like:
    // Original: **AGENCY** **and** Agricultural Marketing Service, USDA.
    // New: **AGENCY and** Agricultural Marketing Service, USDA.
    let bold_merge_regex = Regex::new(r"\*\*([^\*]+)\*\*[ ]*\*\*([^\*]+)\*\*").unwrap();
    let clean_text = bold_merge_regex
        .replace_all(&clean_text, "**$1 $2** ")
        .to_string();

    // normalize at end
    let clean_text = nfkc_printable_str(&clean_text);

    clean_text
}

/// Extract text from a PDF file using the most simple method, which
/// concatenates all text on each page with a single newline character
/// regardless of position or style.
/// Arguments:
/// - pdf_path: The path to the PDF file to extract text from.
/// Returns:
/// - The extracted text.
pub fn extract_text_simple(pdf_document: &PdfDocument) -> String {
    normalize_pdf_text(
        pdf_document
            .pages()
            .iter()
            .map(|page| page.text().unwrap().all())
            .collect::<Vec<String>>()
            .join("\n")
            .trim(),
    )
}

/// Extract text from a PDF file using a position-aware method without return
/// breaks between non-paragraph breaks.  This approximates "un-word-wrapping"
/// PDF text back to their natural paragraph breaks.
/// If the gap is more than 5x the median gap, it is considered a page or column break,
/// and no newline is inserted.
/// Arguments:
/// - pdf_path: The path to the PDF file to extract text from.
/// Returns:
/// - The extracted text.
pub fn extract_text_positions(pdf_document: &PdfDocument) -> String {
    // keep because we need to do multiple passes
    let pages: Vec<PdfPage> = pdf_document.pages().iter().collect();

    let mut x_gaps: Vec<f32> = Vec::new();
    let mut x_last = 0.0;
    let mut y_gaps: Vec<f32> = Vec::new();
    let mut y_last = 0.0;
    let mut font_sizes = Vec::new();

    for page in pages.iter() {
        // get the distribution of gaps between text blocks
        for object in page.objects().iter() {
            // get object info
            if let Some(text_object) = object.as_text_object() {
                // add unscaled font size
                font_sizes.push(get_font_size(&text_object));

                // calculate gaps
                let (gap_x, gap_y) = get_gaps(&text_object, x_last, y_last);
                if gap_x >= 0.0 {
                    x_gaps.push(gap_x);
                }
                if gap_y >= 0.0 {
                    y_gaps.push(gap_y);
                }
                x_last = text_object.bounds().unwrap().right.value;
                y_last = text_object.bounds().unwrap().top.value;
            }
        }
    }

    let x_percentiles = calculate_percentiles(&x_gaps);
    let y_percentiles = calculate_percentiles(&y_gaps);
    let font_percentiles = calculate_percentiles(&font_sizes);

    // now extract the text
    let mut text = String::new();
    for page in pages.iter() {
        x_last = 0.0;
        y_last = 0.0;
        let mut x_last_neg = 0.0;
        let mut last_font_size = 0.0;
        for object in page.objects().iter() {
            if let Some(text_object) = object.as_text_object() {
                let text_bbox = object.bounds().unwrap();
                let text_object_text = &text_object.text();

                // skip if it's empty
                if text_object_text.trim().is_empty() {
                    continue;
                }

                // get the gap between the current and last text block
                let (x_gap, y_gap) = get_gaps(&text_object, x_last, y_last);
                let font_size = get_font_size(&text_object);
                let font_weight = get_font_weight(&text_object);

                // get the text with the appropriate space
                let gap_str =
                    gap_to_string(x_gap, y_gap, &x_percentiles, &y_percentiles, &text_object);

                if x_gap < 0.0 {
                    // check if the new left position is within 1.01 and 1.2 of the last negative x;
                    // if so, it's probably an indented tight paragraph.
                    // otherwise, use the gap string as is.
                    let font_x_width = text_object.unscaled_font_size().value;
                    let left_width_diff = text_bbox.left.value - x_last_neg;
                    if left_width_diff > 4.0 * font_x_width && left_width_diff < 10.0 * font_x_width
                    {
                        text.push_str("\n\n");
                    } else {
                        text.push_str(&gap_str);
                    }

                    // update
                    x_last_neg = text_bbox.left.value;
                } else {
                    text.push_str(&gap_str);
                }

                // set proper markdown formatting based on font weight and size
                if font_size > font_percentiles.p50 * 1.5 && font_size > last_font_size {
                    // markdown heading #
                    text.push_str(&format!("\n\n# {}", text_object_text.trim()));
                } else if font_size > font_percentiles.p50 * 1.25 && font_size > last_font_size {
                    // markdown heading ##
                    text.push_str(&format!("\n\n## {}", text_object_text.trim()));
                } else if font_size > font_percentiles.p50 * 1.01 && font_size > last_font_size {
                    // markdown heading ###
                    text.push_str(&format!("\n\n### {}", text_object_text.trim()));
                } else {
                    // check on the font weight
                    if font_weight > 400 {
                        // bold
                        text.push_str(&format!("**{}**", text_object_text.trim()));
                    } else {
                        // normal
                        text.push_str(text_object_text.trim());
                    }
                }

                // update last values
                x_last = text_bbox.right.value;
                y_last = text_bbox.top.value;
                last_font_size = font_size;
            }
        }
    }

    // return the extracted text
    normalize_pdf_text(&text)
}

pub fn extract_buffer_text(buffer: &[u8]) -> String {
    let pdf_parser = Pdfium::default();
    let pdf_document = pdf_parser.load_pdf_from_byte_slice(buffer, None).unwrap();
    extract_text_simple(&pdf_document)
}

pub fn extract_file_text(file_path: &str) -> String {
    let pdf_parser = Pdfium::default();
    let pdf_document = pdf_parser.load_pdf_from_file(file_path, None).unwrap();
    extract_text_simple(&pdf_document)
}

pub fn extract_buffer_markdown(buffer: &[u8]) -> String {
    let pdf_parser = Pdfium::default();
    let pdf_document = pdf_parser.load_pdf_from_byte_slice(buffer, None).unwrap();
    extract_text_positions(&pdf_document)
}

pub fn extract_file_markdown(file_path: &str) -> String {
    let pdf_parser = Pdfium::default();
    let pdf_document = pdf_parser.load_pdf_from_file(file_path, None).unwrap();
    extract_text_positions(&pdf_document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test1.pdf");
        path
    }

    fn get_test_ocr_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test2.pdf");
        path
    }

    fn get_test_long_file_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test4.pdf");
        path
    }

    #[test]
    fn test_extract_text() {
        // get text
        let text = extract_file_text(get_test_file_path().to_str().unwrap());

        // check for "Fair and Competitive Livestock"
        assert!(text.contains("Fair and Competitive Livestock"));
    }

    #[test]
    fn test_extract_markdown() {
        // get text with position
        let text = extract_file_markdown(get_test_file_path().to_str().unwrap());

        // check for "AGENCY: Agricultural Marketing Service,
        assert!(text.contains("**AGENCY :**  Agricultural Marketing Service, USDA."));
    }

    #[test]
    fn test_extract_text_long() {
        let text = extract_file_text(get_test_long_file_path().to_str().unwrap());

        assert!(text.contains("[Name of Money Market Mutual Fund]"));
    }

    #[test]
    fn test_extract_markdown_long() {
        let text = extract_file_markdown(get_test_long_file_path().to_str().unwrap());

        assert!(text.contains("### PART 31—LEVERAGE TRANSACTIONS\n"));
    }

    #[test]
    fn test_extract_text_ocr() {
        // get text with position
        let text = extract_file_text(get_test_ocr_file_path().to_str().unwrap());

        // garbage output with this method
        assert!(!text.contains("AGENCY"));
    }
}
