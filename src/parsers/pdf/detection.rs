/// PDF type detection module, designed to detect whether each page of a document is:
/// - Digital-native PDF page
/// - Image PDF page that have not been OCR'd
/// - Image PDF page that have been OCR'd
/// Documents have:
/// An entry point to all the various object collections contained in a single PDF file. These collections include:
// PdfDocument::attachments(), an immutable collection of all the PdfAttachments in the document.
// PdfDocument::attachments_mut(), a mutable collection of all the PdfAttachments in the document.
// PdfDocument::bookmarks(), an immutable collection of all the PdfBookmarks in the document.
// PdfDocument::fonts(), an immutable collection of all the PdfFonts in the document.
// PdfDocument::fonts_mut(), a mutable collection of all the PdfFonts in the document.
// PdfDocument::form(), an immutable reference to the PdfForm embedded in the document, if any.
// PdfDocument::metadata(), an immutable collection of all the PdfMetadata tags in the document.
// PdfDocument::pages(), an immutable collection of all the PdfPages in the document.
// PdfDocument::pages_mut(), a mutable collection of all the PdfPages in the document.
// PdfDocument::permissions(), settings relating to security handlers and document permissions for the document.
// PdfDocument::signatures(), an immutable collection of all the PdfSignatures in the document.
/// Pages have:
/// An entry point to all the various objects contained in a single PDF page. These objects include:
/// A single page in a PdfDocument.
// In addition to its own intrinsic properties, a PdfPage serves as the entry point to all object collections related to a single page in a document. These collections include:
// PdfPage::annotations(), an immutable collection of all the user annotations attached to the PdfPage.
// PdfPage::annotations_mut(), a mutable collection of all the user annotations attached to the PdfPage.
// PdfPage::boundaries(), an immutable collection of the boundary boxes relating to the PdfPage.
// PdfPage::boundaries_mut(), a mutable collection of the boundary boxes relating to the PdfPage.
// PdfPage::links(), an immutable collection of the links on the PdfPage.
// PdfPage::links_mut(), a mutable collection of the links on the PdfPage.
// PdfPage::objects(), an immutable collection of all the displayable objects on the PdfPage.
// PdfPage::objects_mut(), a mutable collection of all the displayable objects on the PdfPage.
use pdfium_render::prelude::*;

// enum for different page types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Digital,
    ImagePreOCR,
    ImagePostOCR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    Unknown = 0,
    Mixed = 1,
    Text = 2,
    ImagePreOCR = 3,
    ImagePostOCR = 4,
}

// struct for object counts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectCounts {
    pub text: usize,
    pub image: usize,
    pub other: usize,
}

/// Get the count of text, image, and other objects on a page.
/// Args:
///    page: PdfPage object
/// Returns:
///   ObjectCounts object
pub fn get_page_object_count(page: &PdfPage) -> ObjectCounts {
    let mut text_count = 0;
    let mut image_count = 0;
    let mut other_count = 0;
    for object in page.objects().iter() {
        if object.object_type() == PdfPageObjectType::Text {
            text_count += 1;
        } else if object.object_type() == PdfPageObjectType::Image {
            image_count += 1;
        } else {
            other_count += 1;
        }
    }
    ObjectCounts {
        text: text_count,
        image: image_count,
        other: other_count,
    }
}

/// Determine the type of a page based on the number of text, image, and other objects.
/// Args:
///    page: PdfPage object
/// Returns:
///    PageType object
pub fn detect_page_type(page: &PdfPage, document: &PdfDocument) -> PageType {
    // get counts and ratios
    let object_counts = get_page_object_count(page);

    // check if we have no text or image objects
    if object_counts.image == 0 {
        return PageType::Digital;
    }

    // check if we have >0 images and nothing else
    if object_counts.other == 0 && object_counts.text == 0 {
        return PageType::ImagePreOCR;
    }

    // check if there is a single image that is the size of the entire page
    if object_counts.image == 1 {
        let page_width = page.page_size().width().value;
        let page_height = page.page_size().height().value;
        let image = page
            .objects()
            .iter()
            .filter(|obj| obj.object_type() == PdfPageObjectType::Image)
            .next()
            .unwrap();
        let image_size = image.bounds().unwrap();
        let image_width = image_size.width().value;
        let image_height = image_size.height().value;

        // check if the width and height are within 1%
        let width_diff = (page_width - image_width).abs() / page_width;
        let height_diff = (page_height - image_height).abs() / page_height;

        if width_diff < 0.01 && height_diff < 0.01 {
            return PageType::ImagePostOCR;
        }
    }

    // check if there are zero embedded fonts
    let mut font_count = 0;
    let mut font_embed_count = 0;
    for page in document.pages().iter() {
        for font in page.fonts().iter() {
            font_count += 1;
            if font.is_embedded().unwrap() {
                font_embed_count += 1;
            }
        }
    }

    // otherwise, use the ratio to catch intermediate cases
    let text_to_image_ratio = object_counts.text as f64 / object_counts.image as f64;
    let image_to_else_ratio = object_counts.image as f64 / object_counts.other as f64;

    // if no fonts, it's likely an image
    if font_count == 0 {
        return PageType::ImagePreOCR;
    }

    // if we have more text than images, it's digital
    if text_to_image_ratio > 1.0 {
        return PageType::Digital;
    }

    // if we have more images than text, it's image
    if text_to_image_ratio < 1.0 && font_embed_count == 0 {
        return PageType::ImagePreOCR;
    }

    // if we have more images than other objects, it's image
    if image_to_else_ratio > 1.0 {
        return PageType::ImagePreOCR;
    }

    // else, assume it's digital
    PageType::Digital
}

pub fn detect_document_type(pdf_document: &PdfDocument) -> DocumentType {
    let mut document_type = DocumentType::Unknown;
    for page in pdf_document.pages().iter() {
        // get the page type
        let page_type = detect_page_type(&page, &pdf_document);

        // set from first page
        if document_type == DocumentType::Unknown {
            document_type = match page_type {
                PageType::Digital => DocumentType::Text,
                PageType::ImagePreOCR => DocumentType::ImagePreOCR,
                PageType::ImagePostOCR => DocumentType::ImagePostOCR,
            };
        } else {
            // check if the page type is different from the document type
            if document_type == DocumentType::Text && page_type != PageType::Digital {
                document_type = DocumentType::Mixed;
            } else if document_type == DocumentType::ImagePreOCR
                && page_type != PageType::ImagePreOCR
            {
                document_type = DocumentType::Mixed;
            } else if document_type == DocumentType::ImagePostOCR
                && page_type != PageType::ImagePostOCR
            {
                document_type = DocumentType::Mixed;
            }
        }
    }

    document_type
}

/// Detect the type of a buffer by parsing all pages, detecting each
/// page type, and returning the resulting type.
/// Args:
///   buffer: byte array of the PDF file
/// Returns:
///  DocumentType object
pub fn detect_buffer_type(buffer: &[u8]) -> DocumentType {
    let pdf_parser = Pdfium::default();
    let pdf_file = pdf_parser.load_pdf_from_byte_slice(buffer, None).unwrap();

    detect_document_type(&pdf_file)
}

/// Detect the type of a file by parsing all pages, detecting each
/// page type, and returning the resulting type.
/// Args:
///  file_path: path to the PDF file
/// Returns:
/// DocumentType object
pub fn detect_file_type(file_path: &str) -> DocumentType {
    let pdf_parser = Pdfium::default();
    let pdf_file = pdf_parser.load_pdf_from_file(file_path, None).unwrap();

    detect_document_type(&pdf_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::*;

    fn get_test_file_path() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test1.pdf");
        path.to_str().unwrap().to_string()
    }

    fn get_test_ocr_pre_file_path() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test3.pdf");
        path.to_str().unwrap().to_string()
    }

    fn get_test_ocr_post_file_path() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test2.pdf");
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_detect_digital() {
        // load the file
        let document_type = detect_file_type(&get_test_file_path());

        assert_eq!(document_type, DocumentType::Text);
    }

    #[test]
    fn test_detect_ocr_pre() {
        // load the file
        let document_type = detect_file_type(&get_test_ocr_pre_file_path());

        assert_eq!(document_type, DocumentType::ImagePreOCR);
    }

    #[test]
    fn test_detect_ocr_post() {
        // load the file
        let document_type = detect_file_type(&get_test_ocr_post_file_path());

        assert_eq!(document_type, DocumentType::ImagePostOCR);
    }
}
