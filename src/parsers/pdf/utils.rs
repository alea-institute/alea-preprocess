use pdfium_render::prelude::{PdfFontWeight, PdfPageObjectCommon, PdfPageTextObject};

/// Get PDF font weight to a u32 value.
/// Arguments:
/// - font_weight: The PDF font weight.
/// Returns:
/// - The font weight as a u32 value.
/// Weight200,
//     Weight300,
//     Weight400Normal,
//     Weight500,
//     Weight600,
//     Weight700Bold,
//     Weight800,
//     Weight900,
pub fn get_font_weight(text_object: &PdfPageTextObject) -> u32 {
    match text_object.font().weight().unwrap() {
        PdfFontWeight::Weight100 => 100,
        PdfFontWeight::Weight200 => 200,
        PdfFontWeight::Weight300 => 300,
        PdfFontWeight::Weight400Normal => 400,
        PdfFontWeight::Weight500 => 500,
        PdfFontWeight::Weight600 => 600,
        PdfFontWeight::Weight700Bold => 700,
        PdfFontWeight::Weight800 => 800,
        PdfFontWeight::Weight900 => 900,
        _ => 400,
    }
}

/// Get PDF font size as a floating-point value.
pub fn get_font_size(text_object: &PdfPageTextObject) -> f32 {
    text_object.scaled_font_size().value
}

/// Get font height as a floating-point value.
/// Arguments:
/// - text_object: The PDF text object.
/// Returns:
/// - The font height as a floating-point value.
pub fn get_font_height(text_object: &PdfPageTextObject) -> f32 {
    text_object.height().unwrap().value
}
