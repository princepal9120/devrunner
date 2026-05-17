use super::{DetectedRunner, Ecosystem};
use std::path::Path;

/// Detect Swift Package Manager projects
/// Priority: 19
pub fn detect(dir: &Path) -> Vec<DetectedRunner> {
    let mut runners = Vec::new();

    let package_swift = dir.join("Package.swift");
    if package_swift.exists() {
        runners.push(DetectedRunner::new(
            "swift",
            "Package.swift",
            Ecosystem::Swift,
            19,
        ));
    }

    runners
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_swift() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Package.swift")).unwrap();

        let runners = detect(dir.path());
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "swift");
    }

    #[test]
    fn test_no_swift() {
        let dir = tempdir().unwrap();

        let runners = detect(dir.path());
        assert!(runners.is_empty());
    }
}
