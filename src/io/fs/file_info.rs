use file_format::FileFormat;
use pyo3::prelude::*;

/// A struct representing comprehensive file format information.
#[pyclass]
#[derive(Debug, Clone)]
pub struct FileInfo {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub short_name: String,
    #[pyo3(get)]
    pub media_type: String,
    #[pyo3(get)]
    pub extension: String,
    #[pyo3(get)]
    pub kind: String,
}

/// A struct representing comprehensive file format information.
#[pymethods]
impl FileInfo {
    #[new]
    #[pyo3(signature = (name, short_name, media_type, extension, kind))]
    fn new(
        name: String,
        short_name: String,
        media_type: String,
        extension: String,
        kind: String,
    ) -> Self {
        FileInfo {
            name,
            short_name,
            media_type,
            extension,
            kind,
        }
    }

    /// Get a string representation of the FileInfo.
    ///
    /// Returns:
    ///    str: A string representation of the FileInfo.
    ///
    /// Example:
    ///    >>> file_info = FileInfo("Portable Network Graphics", "PNG", "image/png", "png", "Image")
    ///   >>> str(file_info)
    ///  "FileInfo(name='Portable Network Graphics', short_name='PNG', media_type='image/png', extension='png', kind='Image')"
    ///
    fn __str__(&self) -> String {
        format!(
            "FileInfo(name='{}', short_name='{}', media_type='{}', extension='{}', kind='{}')",
            self.name, self.short_name, self.media_type, self.extension, self.kind
        )
    }
}

/// Convert a FileFormat object to a FileInfo object.
/// Args:
/// fmt (FileFormat): The FileFormat object to convert.
/// Returns:
/// FileInfo: The FileInfo object.
fn file_format_to_file_info(fmt: FileFormat) -> FileInfo {
    FileInfo {
        name: fmt.name().to_string(),
        short_name: fmt.short_name().map(|s| s.to_string()).unwrap_or_default(),
        media_type: fmt.media_type().to_string(),
        extension: fmt.extension().to_string(),
        kind: format!("{:?}", fmt.kind()),
    }
}

/// Get file information from a buffer.
/// Args:
/// buffer (bytes): The buffer to get file information from.
/// Returns:
/// FileInfo: The file information.
pub fn get_file_info_from_buffer(buffer: &[u8]) -> FileInfo {
    file_format_to_file_info(FileFormat::from_bytes(buffer))
}

/// Get file information from a file.
/// Args:
/// path (str): The path to the file to get information from.
/// Returns:
/// FileInfo: The file information.
pub fn get_file_info_from_file(path: &str) -> FileInfo {
    let file_format_result = FileFormat::from_file(path);
    match file_format_result {
        Ok(file_format) => file_format_to_file_info(file_format),
        Err(_) => FileInfo::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //test files from CARGO_MANIFEST_DIR
    // resources/file1.html => FileInfo(name='HTML', short_name='HTML', media_type='text/html', extension='html', kind='Text')
    // resources/test1.pdf => FileInfo(name='Portable Document Format', short_name='PDF', media_type='application/pdf', extension='pdf', kind='Document')
    // usc.100.jsonl.gz => ?

    #[test]
    fn test_get_html_file_info_from_buffer() {
        // get CARGO_MANIFEST_DIR/resources/file1.html
        let path = format!("{}/resources/file1.html", env!("CARGO_MANIFEST_DIR"));
        let buffer = std::fs::read(&path).unwrap();
        let file_info = get_file_info_from_buffer(&buffer);
        assert_eq!(file_info.name, "HyperText Markup Language");
        assert_eq!(file_info.short_name, "HTML");
        assert_eq!(file_info.media_type, "text/html");
        assert_eq!(file_info.extension, "html");
        assert_eq!(file_info.kind, "Other");
    }

    #[test]
    fn test_get_html_file_info_from_file() {
        // get CARGO_MANIFEST_DIR/resources/file1.html
        let path = format!("{}/resources/file1.html", env!("CARGO_MANIFEST_DIR"));
        let file_info = get_file_info_from_file(&path);
        assert_eq!(file_info.name, "HyperText Markup Language");
        assert_eq!(file_info.short_name, "HTML");
        assert_eq!(file_info.media_type, "text/html");
        assert_eq!(file_info.extension, "html");
        assert_eq!(file_info.kind, "Other");
    }

    #[test]
    fn test_get_pdf_file_info_from_buffer() {
        // get CARGO_MANIFEST_DIR/resources/test1.pdf
        let path = format!("{}/resources/test1.pdf", env!("CARGO_MANIFEST_DIR"));
        let buffer = std::fs::read(&path).unwrap();
        let file_info = get_file_info_from_buffer(&buffer);
        assert_eq!(file_info.name, "Portable Document Format");
        assert_eq!(file_info.short_name, "PDF");
        assert_eq!(file_info.media_type, "application/pdf");
        assert_eq!(file_info.extension, "pdf");
        assert_eq!(file_info.kind, "Document");
    }

    #[test]
    fn test_get_pdf_file_info_from_file() {
        // get CARGO_MANIFEST_DIR/resources/test1.pdf
        let path = format!("{}/resources/test1.pdf", env!("CARGO_MANIFEST_DIR"));
        let file_info = get_file_info_from_file(&path);
        assert_eq!(file_info.name, "Portable Document Format");
        assert_eq!(file_info.short_name, "PDF");
        assert_eq!(file_info.media_type, "application/pdf");
        assert_eq!(file_info.extension, "pdf");
        assert_eq!(file_info.kind, "Document");
    }

    // test random1, which is random bytes
    #[test]
    fn test_get_random_file_info_from_buffer() {
        // get CARGO_MANIFEST_DIR/resources/random1
        let path = format!("{}/resources/random1", env!("CARGO_MANIFEST_DIR"));
        let buffer = std::fs::read(&path).unwrap();
        let file_info = get_file_info_from_buffer(&buffer);
        assert_eq!(file_info.name, "Arbitrary Binary Data");
        assert_eq!(file_info.short_name, "BIN");
        assert_eq!(file_info.media_type, "application/octet-stream");
        assert_eq!(file_info.extension, "bin");
        assert_eq!(file_info.kind, "Other");
    }
}
