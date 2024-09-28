use crate::parsers::pdf::utils::{get_font_height, get_font_size};
/// This module contains functions for parsing the layout of a PDF file.
/// The layout parser is responsible for determining the structure of the PDF file,
/// including paragraphs, columns, and sections.
use pdfium_render::prelude::{PdfPageObjectCommon, PdfPageTextObject};

/// Percentile struct
/// This struct is used to store the 10th, 25th, 50th, 75th, and 90th percentiles
/// of a list of values.
#[derive(Debug)]
pub struct GapPercentiles {
    pub p10: f32,
    pub p25: f32,
    pub p50: f32,
    pub p75: f32,
    pub p90: f32,
}

/// Calculate the 10th, 25th, 50th, 75th, and 90th percentiles of a list of values.
/// This function is used to determine the distribution of gaps between text blocks
/// in a PDF file.
/// Arguments:
/// - values: A list of floating-point values.
/// Returns:
/// - A tuple of the 10th, 25th, 50th, 75th, and 90th percentiles of the input values.
pub fn calculate_percentiles(values: &[f32]) -> GapPercentiles {
    let mut sorted_values = values.iter().map(|x| x.abs()).collect::<Vec<f32>>();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted_values.len();

    let percentile_10 = sorted_values[(len as f32 * 0.1) as usize];
    let percentile_25 = sorted_values[(len as f32 * 0.25) as usize];
    let percentile_50 = sorted_values[(len as f32 * 0.5) as usize];
    let percentile_75 = sorted_values[(len as f32 * 0.75) as usize];
    let percentile_90 = sorted_values[(len as f32 * 0.9) as usize];

    GapPercentiles {
        p10: percentile_10,
        p25: percentile_25,
        p50: percentile_50,
        p75: percentile_75,
        p90: percentile_90,
    }
}

/// Determine the type of space to insert based on the horizontal and vertical gaps
/// between text blocks in a PDF file.
///
/// This function uses percentiles of x and y gaps to make intelligent decisions about
/// spacing between text blocks. It aims to preserve the original document structure
/// as much as possible, including paragraphs, columns, and sections.
///
/// # Arguments
/// * `x_gap`: The horizontal gap between two text blocks.
/// * `y_gap`: The vertical gap between two text blocks.
/// * `x_percentiles`: Percentiles of horizontal gaps between text blocks.
/// * `y_percentiles`: Percentiles of vertical gaps between text blocks.
///
/// # Returns
/// A string representing the type of space to insert between the two text blocks.
///
/// # Priority order and reasoning:
/// 1. Large vertical gaps (new sections or paragraphs)
/// 2. Negative x-gaps (possible new lines or columns)
/// 3. Large horizontal gaps (possible new columns or indentation)
/// 4. Medium vertical gaps (possible new lines within paragraphs)
/// 5. Small gaps (normal text flow)
pub fn gap_to_string(
    x_gap: f32,
    y_gap: f32,
    x_percentiles: &GapPercentiles,
    y_percentiles: &GapPercentiles,
    text_object: &PdfPageTextObject,
) -> String {
    // check if empty
    if text_object.text().trim().len() == 0 {
        return "".to_string();
    }

    // 1. Large vertical gaps (new sections or paragraphs)
    if y_gap > y_percentiles.p75 * 2.0 {
        return "\n\n".to_string(); // New section or paragraph
    }

    if y_gap > y_percentiles.p90 * 1.25 {
        return "\n\n".to_string(); // New section or paragraph
    }

    if y_gap > y_percentiles.p75 * 1.5 {
        return "\n".to_string(); // New line
    }

    if y_gap > y_percentiles.p25 && x_gap > x_percentiles.p25 {
        return " ".to_string(); // New line
    }

    if x_gap > x_percentiles.p10 {
        return " ".to_string(); // New line
    }

    if text_object.text().contains("\u{2}") {
        return "".to_string();
    } else if x_gap < 0.0 {
        return " ".to_string(); // New line
    }

    // Default case: no extra space
    "".to_string()
}

/// Get the horizontal and vertical gaps between two text blocks.
/// Arguments:
/// - text_object: The PDF text object.
/// - last_x: The x-coordinate of the last text block.
/// - last_y: The y-coordinate of the last text block.
/// Returns:
/// - A tuple of the horizontal and vertical gaps between the two text blocks.
/// # Priority order and reasoning:
pub fn get_gaps(text_object: &PdfPageTextObject, last_x: f32, last_y: f32) -> (f32, f32) {
    let gap_x = (text_object.bounds().unwrap().left.value - last_x) / get_font_size(&text_object);
    let gap_y = (last_y - text_object.bounds().unwrap().top.value - get_font_height(&text_object))
        / get_font_height(&text_object);
    (gap_x, gap_y)
}
