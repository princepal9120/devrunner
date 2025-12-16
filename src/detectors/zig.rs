

use super::{DetectedRunner, Ecosystem};
use std::path::Path;

/// Detect Zig Build projects
/// Priority: 20
pub fn detect(dir: &Path) -> Vec<DetectedRunner> {
    let mut runners = Vec::new();

    let build_zig = dir.join("build.zig");
    if build_zig.exists() {
        runners.push(DetectedRunner::new("zig", "build.zig", Ecosystem::Zig, 20));
    }

    runners
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_zig() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("build.zig")).unwrap();

        let runners = detect(dir.path());
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "zig");
    }

    #[test]
    fn test_no_zig() {
        let dir = tempdir().unwrap();

        let runners = detect(dir.path());
        assert!(runners.is_empty());
    }
}
