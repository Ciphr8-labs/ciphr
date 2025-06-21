use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Creates a file with the given content at the specified path.
///
/// This is a helper for setting up test cases that require files.
///
/// # Panics
///
/// This function will panic if it fails to create the file or write
/// the content. Tests should fail if their setup fails.
pub fn create_file_with_content(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::TestHarness;

    #[test]
    fn test_create_file_with_content() {
        let harness = TestHarness::new();
        let file_path = harness.root_path.join("test_file.txt");
        let content = "hello, world!";

        create_file_with_content(&file_path, content).unwrap();

        assert!(file_path.exists());
        let read_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(read_content, content);
    }
} 