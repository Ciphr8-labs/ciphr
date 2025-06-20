use tempfile::{tempdir, TempDir};
use std::path::PathBuf;

/// A test harness for creating isolated test environments.
///
/// This harness manages a temporary directory, ensuring that tests
/// that require file system access do not interfere with each other.
pub struct TestHarness {
    _temp_dir: TempDir,
    pub root_path: PathBuf,
}

impl TestHarness {
    /// Creates a new `TestHarness` with a new temporary directory.
    pub fn new() -> Self {
        let temp_dir = tempdir().expect("Failed to create temporary directory for test harness");
        let root_path = temp_dir.path().to_path_buf();
        Self {
            _temp_dir: temp_dir,
            root_path,
        }
    }
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_creation() {
        let harness = TestHarness::new();
        assert!(harness.root_path.exists());
        assert!(harness.root_path.is_dir());
    }
} 