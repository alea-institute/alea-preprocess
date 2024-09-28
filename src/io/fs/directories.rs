use walkdir::WalkDir;

pub fn get_files(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect()
}

pub fn get_directories(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().display().to_string())
        .collect()
}

pub fn get_entries(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().display().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // list files in CARGO_MANIFEST_DIR
    #[test]
    fn test_get_files() {
        // get CARGO_MANIFEST_DIR/resources/
        let path = format!("{}/resources/", env!("CARGO_MANIFEST_DIR"));
        let files = get_files(&path);
        assert!(files.len() > 10);
        assert!(files.contains(&format!(
            "{}/resources/file1.html",
            env!("CARGO_MANIFEST_DIR")
        )));
    }

    // list directories in CARGO_MANIFEST_DIR
    #[test]
    fn test_get_directories() {
        // get CARGO_MANIFEST_DIR/resources/
        let path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        let directories = get_directories(&path);
        assert!(directories.len() > 2);
        assert!(directories.contains(&format!("{}/resources", env!("CARGO_MANIFEST_DIR"))));
    }

    // list entries in CARGO_MANIFEST_DIR
    #[test]
    fn test_get_entries() {
        // get CARGO_MANIFEST_DIR/resources/
        let path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        let entries = get_entries(&path);
        assert!(entries.len() > 10);
        assert!(entries.contains(&format!("{}/resources", env!("CARGO_MANIFEST_DIR"))));
        assert!(entries.contains(&format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"))));
    }
}
