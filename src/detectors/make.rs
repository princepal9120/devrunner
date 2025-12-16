

use super::{DetectedRunner, Ecosystem};
use std::path::Path;

/// Detect Makefile projects
/// Priority: 21 (last, as it's the most generic)
pub fn detect(dir: &Path) -> Vec<DetectedRunner> {
    let mut runners = Vec::new();

    // Use read_dir to get exact filename (case-sensitive on all platforms)
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name == "Makefile" || name == "makefile" {
                    runners.push(DetectedRunner::new("make", name, Ecosystem::Generic, 21));
                    break;
                }
            }
        }
    }

    runners
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_makefile() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Makefile")).unwrap();

        let runners = detect(dir.path());
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "make");
        assert_eq!(runners[0].detected_file, "Makefile");
    }

    #[test]
    fn test_detect_makefile_lowercase() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("makefile")).unwrap();

        let runners = detect(dir.path());
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "make");
        assert_eq!(runners[0].detected_file, "makefile");
    }

    #[test]
    fn test_no_makefile() {
        let dir = tempdir().unwrap();

        let runners = detect(dir.path());
        assert!(runners.is_empty());
    }
}
