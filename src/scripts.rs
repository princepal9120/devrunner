use crate::detectors::{DetectedRunner, Ecosystem};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Represents a script/command available in a project
#[derive(Debug, Clone)]
pub struct ProjectScript {
    pub name: String,
    pub command: String,
}

/// Result of script discovery
#[derive(Debug)]
pub struct ScriptList {
    pub scripts: Vec<ProjectScript>,
    pub source_file: String,
}

/// Parse scripts from a package.json file
pub fn parse_package_json_scripts(project_dir: &Path) -> Option<ScriptList> {
    let package_json_path = project_dir.join("package.json");
    
    if !package_json_path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&package_json_path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    
    let scripts_obj = json.get("scripts")?.as_object()?;
    
    let scripts: Vec<ProjectScript> = scripts_obj
        .iter()
        .map(|(name, cmd)| ProjectScript {
            name: name.clone(),
            command: cmd.as_str().unwrap_or("").to_string(),
        })
        .collect();
    
    Some(ScriptList {
        scripts,
        source_file: "package.json".to_string(),
    })
}

/// Parse targets from a Makefile
pub fn parse_makefile_targets(project_dir: &Path) -> Option<ScriptList> {
    let makefile_path = if project_dir.join("Makefile").exists() {
        project_dir.join("Makefile")
    } else if project_dir.join("makefile").exists() {
        project_dir.join("makefile")
    } else {
        return None;
    };
    
    let content = fs::read_to_string(&makefile_path).ok()?;
    
    let scripts: Vec<ProjectScript> = content
        .lines()
        .filter(|line| !line.starts_with('\t') && !line.starts_with(' ') && !line.starts_with('#'))
        .filter_map(|line| {
            // Match lines like "target:" or "target: deps"
            if let Some(colon_pos) = line.find(':') {
                let target = line[..colon_pos].trim();
                // Skip special targets and variables
                if !target.is_empty() 
                    && !target.starts_with('.') 
                    && !target.contains('=')
                    && !target.contains('$')
                {
                    return Some(ProjectScript {
                        name: target.to_string(),
                        command: format!("make {}", target),
                    });
                }
            }
            None
        })
        .collect();
    
    if scripts.is_empty() {
        return None;
    }
    
    Some(ScriptList {
        scripts,
        source_file: "Makefile".to_string(),
    })
}

/// Parse binary targets from Cargo.toml
pub fn parse_cargo_targets(project_dir: &Path) -> Option<ScriptList> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    
    if !cargo_toml_path.exists() {
        return None;
    }
    
    // Common cargo commands
    let scripts = vec![
        ProjectScript { name: "build".to_string(), command: "cargo build".to_string() },
        ProjectScript { name: "test".to_string(), command: "cargo test".to_string() },
        ProjectScript { name: "run".to_string(), command: "cargo run".to_string() },
        ProjectScript { name: "check".to_string(), command: "cargo check".to_string() },
        ProjectScript { name: "clippy".to_string(), command: "cargo clippy".to_string() },
        ProjectScript { name: "fmt".to_string(), command: "cargo fmt".to_string() },
        ProjectScript { name: "doc".to_string(), command: "cargo doc".to_string() },
        ProjectScript { name: "bench".to_string(), command: "cargo bench".to_string() },
    ];
    
    Some(ScriptList {
        scripts,
        source_file: "Cargo.toml".to_string(),
    })
}

/// Parse scripts from pyproject.toml (Poetry/UV)
pub fn parse_pyproject_scripts(project_dir: &Path) -> Option<ScriptList> {
    let pyproject_path = project_dir.join("pyproject.toml");
    
    if !pyproject_path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&pyproject_path).ok()?;
    let toml_value: toml::Value = toml::from_str(&content).ok()?;
    
    let mut scripts = Vec::new();
    
    // Check for poetry scripts
    if let Some(poetry) = toml_value.get("tool").and_then(|t| t.get("poetry")).and_then(|p| p.get("scripts")) {
        if let Some(scripts_table) = poetry.as_table() {
            for (name, cmd) in scripts_table {
                scripts.push(ProjectScript {
                    name: name.clone(),
                    command: cmd.as_str().unwrap_or("").to_string(),
                });
            }
        }
    }
    
    // Check for project.scripts (PEP 621)
    if let Some(project) = toml_value.get("project").and_then(|p| p.get("scripts")) {
        if let Some(scripts_table) = project.as_table() {
            for (name, cmd) in scripts_table {
                scripts.push(ProjectScript {
                    name: name.clone(),
                    command: cmd.as_str().unwrap_or("").to_string(),
                });
            }
        }
    }
    
    if scripts.is_empty() {
        return None;
    }
    
    Some(ScriptList {
        scripts,
        source_file: "pyproject.toml".to_string(),
    })
}

/// Get scripts for a detected runner
pub fn get_scripts_for_runner(runner: &DetectedRunner, project_dir: &Path) -> Option<ScriptList> {
    match runner.ecosystem {
        Ecosystem::NodeJs => parse_package_json_scripts(project_dir),
        Ecosystem::Rust => parse_cargo_targets(project_dir),
        Ecosystem::Python => parse_pyproject_scripts(project_dir),
        Ecosystem::Generic => parse_makefile_targets(project_dir),
        _ => None, // Other ecosystems can be added later
    }
}

/// Get all available scripts from a project directory
pub fn discover_all_scripts(project_dir: &Path) -> Vec<ScriptList> {
    let mut results = Vec::new();
    
    if let Some(scripts) = parse_package_json_scripts(project_dir) {
        results.push(scripts);
    }
    if let Some(scripts) = parse_cargo_targets(project_dir) {
        results.push(scripts);
    }
    if let Some(scripts) = parse_pyproject_scripts(project_dir) {
        results.push(scripts);
    }
    if let Some(scripts) = parse_makefile_targets(project_dir) {
        results.push(scripts);
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_package_json_scripts() {
        let dir = tempdir().unwrap();
        let package_json = dir.path().join("package.json");
        
        let mut file = File::create(&package_json).unwrap();
        file.write_all(br#"{
            "name": "test-project",
            "scripts": {
                "dev": "vite",
                "build": "vite build",
                "test": "vitest"
            }
        }"#).unwrap();
        
        let result = parse_package_json_scripts(dir.path()).unwrap();
        assert_eq!(result.scripts.len(), 3);
        assert_eq!(result.source_file, "package.json");
        
        let names: Vec<&str> = result.scripts.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"dev"));
        assert!(names.contains(&"build"));
        assert!(names.contains(&"test"));
    }

    #[test]
    fn test_parse_makefile_targets() {
        let dir = tempdir().unwrap();
        let makefile = dir.path().join("Makefile");
        
        let mut file = File::create(&makefile).unwrap();
        file.write_all(br#"
.PHONY: all clean

build:
	cargo build

test:
	cargo test

clean:
	rm -rf target
"#).unwrap();
        
        let result = parse_makefile_targets(dir.path()).unwrap();
        assert!(result.scripts.len() >= 3);
        
        let names: Vec<&str> = result.scripts.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"build"));
        assert!(names.contains(&"test"));
        assert!(names.contains(&"clean"));
    }

    #[test]
    fn test_parse_cargo_targets() {
        let dir = tempdir().unwrap();
        let cargo_toml = dir.path().join("Cargo.toml");
        
        File::create(&cargo_toml).unwrap();
        
        let result = parse_cargo_targets(dir.path()).unwrap();
        assert!(!result.scripts.is_empty());
        
        let names: Vec<&str> = result.scripts.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"build"));
        assert!(names.contains(&"test"));
        assert!(names.contains(&"run"));
    }

    #[test]
    fn test_no_scripts_found() {
        let dir = tempdir().unwrap();
        
        // Empty directory should return None
        assert!(parse_package_json_scripts(dir.path()).is_none());
        assert!(parse_makefile_targets(dir.path()).is_none());
        assert!(parse_cargo_targets(dir.path()).is_none());
    }
}
