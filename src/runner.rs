// Copyright (C) 2025 princepal9120
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.

use crate::config::{ToolchainBackend, ToolchainConfig};
use crate::detectors::{
    detect_all, is_tool_installed, node, CommandSupport, DetectedRunner, Ecosystem,
};
use crate::output;
use crate::RunError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

/// Result of running a command
pub struct RunResult {
    pub exit_status: ExitStatus,
    pub runner: DetectedRunner,
    pub working_dir: PathBuf,
}

/// Search for runners in the directory hierarchy
pub fn search_runners(
    start_dir: &Path,
    max_levels: u8,
    ignore_list: &[String],
    verbose: bool,
) -> Result<(Vec<DetectedRunner>, PathBuf), RunError> {
    let mut current_dir = start_dir.to_path_buf();

    for level in 0..=max_levels {
        if verbose {
            output::info(&format!("Searching in {:?} (level {})", current_dir, level));
        }

        let runners = detect_all(&current_dir, ignore_list);
        if !runners.is_empty() {
            return Ok((runners, current_dir));
        }

        // Move up one directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    Err(RunError::RunnerNotFound(max_levels))
}

/// Check for lockfile conflicts within the same ecosystem
/// Uses Corepack (packageManager field) to resolve Node.js conflicts if available
/// Resolve conflicts and return the validated runner list for `select_runner`.
///
/// - Custom runners always override everything else.
/// - Node.js conflicts are resolved via Corepack or installed-tool check; errors on
///   unresolvable conflicts.
/// - Non-Node ecosystems with multiple runners emit a warning but keep all runners so
///   `select_runner` can choose by command support.
pub fn check_conflicts(
    runners: &[DetectedRunner],
    working_dir: &Path,
    verbose: bool,
) -> Result<Vec<DetectedRunner>, RunError> {
    if runners.is_empty() {
        return Err(RunError::RunnerNotFound(0));
    }

    // Custom runners (priority 0) override everything — return only them.
    if runners.iter().any(|r| r.ecosystem == Ecosystem::Custom) {
        if verbose {
            output::info("Using custom runner (highest priority)");
        }
        return Ok(runners
            .iter()
            .filter(|r| r.ecosystem == Ecosystem::Custom)
            .cloned()
            .collect());
    }

    if runners.len() == 1 {
        return Ok(runners.to_vec());
    }

    // Group runners by ecosystem (preserve Vec order = priority order)
    let mut by_ecosystem: HashMap<Ecosystem, Vec<&DetectedRunner>> = HashMap::new();
    for runner in runners {
        by_ecosystem
            .entry(runner.ecosystem)
            .or_default()
            .push(runner);
    }

    let mut resolved: Vec<DetectedRunner> = Vec::new();

    for (ecosystem, eco_runners) in &by_ecosystem {
        if eco_runners.len() == 1 {
            resolved.push((*eco_runners[0]).clone());
            continue;
        }

        match ecosystem {
            Ecosystem::NodeJs => {
                // Monorepo tools (priority 0: nx, turbo, lerna) always win — they
                // are workspace orchestrators, not package managers, so skip the
                // installed-tool check that applies to npm/yarn/pnpm conflicts.
                if let Some(mono) = eco_runners.iter().find(|r| r.priority == 0) {
                    if verbose {
                        output::info(&format!(
                            "Using monorepo tool {} (highest priority)",
                            mono.name
                        ));
                    }
                    resolved.push((*mono).clone());
                    continue;
                }

                // Try Corepack resolution first
                if let Some(corepack_pm) = node::get_corepack_manager(working_dir) {
                    if let Some(runner) = eco_runners.iter().find(|r| r.name == corepack_pm) {
                        if verbose {
                            output::info(&format!(
                                "Using {} (specified by packageManager in package.json)",
                                corepack_pm
                            ));
                        }
                        resolved.push((*runner).clone());
                        continue;
                    } else if verbose {
                        output::warning(&format!(
                            "packageManager specifies '{}' but no matching lockfile found",
                            corepack_pm
                        ));
                    }
                }

                // Fall back to installed-tool check
                let installed: Vec<&&DetectedRunner> = eco_runners
                    .iter()
                    .filter(|r| is_tool_installed(&r.name))
                    .collect();

                if installed.is_empty() {
                    let names: Vec<&str> = eco_runners.iter().map(|r| r.name.as_str()).collect();
                    return Err(RunError::ToolNotInstalled(format!(
                        "None of the detected {} tools are installed: {}. Please install one.",
                        ecosystem.as_str(),
                        names.join(", ")
                    )));
                } else if installed.len() == 1 {
                    let runner = installed[0];
                    let others: Vec<&str> = eco_runners
                        .iter()
                        .filter(|r| r.name != runner.name)
                        .map(|r| r.detected_file.as_str())
                        .collect();
                    output::warning(&format!(
                        "Found {} but only {} is installed. Consider removing: {}",
                        eco_runners
                            .iter()
                            .map(|r| r.detected_file.as_str())
                            .collect::<Vec<_>>()
                            .join(" and "),
                        runner.name,
                        others.join(", ")
                    ));
                    resolved.push((*runner).clone());
                } else {
                    let lockfiles: Vec<&str> = eco_runners
                        .iter()
                        .map(|r| r.detected_file.as_str())
                        .collect();
                    let tools: Vec<&str> = installed.iter().map(|r| r.name.as_str()).collect();
                    return Err(RunError::LockfileConflict(format!(
                        "Detected {} with multiple lockfiles ({}) and multiple tools installed ({}).\nAction needed: Remove the outdated lockfile or use --ignore=<tool>",
                        ecosystem.as_str(),
                        lockfiles.join(", "),
                        tools.join(", ")
                    )));
                }
            }
            _ => {
                // Non-Node ecosystems: warn and keep all (select_runner picks by command support)
                output::warning(&format!(
                    "Multiple {} runners detected ({}). Using highest priority: {}.",
                    ecosystem.as_str(),
                    eco_runners
                        .iter()
                        .map(|r| r.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    eco_runners[0].name
                ));
                for runner in eco_runners {
                    resolved.push((*runner).clone());
                }
            }
        }
    }

    // Restore priority order
    resolved.sort_by_key(|r| r.priority);
    Ok(resolved)
}

pub fn select_runner(
    runners: &[DetectedRunner],
    command: &str,
    working_dir: &Path,
    verbose: bool,
) -> Result<DetectedRunner, RunError> {
    if runners.is_empty() {
        return Err(RunError::RunnerNotFound(0));
    }

    let mut supported_runners: Vec<&DetectedRunner> = Vec::new();
    let mut unknown_runners: Vec<&DetectedRunner> = Vec::new();

    for runner in runners {
        match runner.supports_command(command, working_dir) {
            CommandSupport::Supported => {
                if verbose {
                    output::info(&format!("{} supports command '{}'", runner.name, command));
                }
                supported_runners.push(runner);
            }
            CommandSupport::NotSupported => {
                if verbose {
                    output::info(&format!(
                        "{} does not support command '{}'",
                        runner.name, command
                    ));
                }
            }
            CommandSupport::Unknown => {
                unknown_runners.push(runner);
            }
        }
    }

    if let Some(runner) = supported_runners.first() {
        return Ok((*runner).clone());
    }

    if let Some(runner) = unknown_runners.first() {
        return Ok((*runner).clone());
    }

    Err(RunError::CommandNotSupported(
        command.to_string(),
        runners.iter().map(|r| r.name.clone()).collect(),
    ))
}

/// Options for command execution
#[derive(Debug, Clone)]
pub struct ExecuteOptions<'a> {
    /// Show command without executing
    pub dry_run: bool,
    /// Show verbose output
    pub verbose: bool,
    /// Suppress normal output
    pub quiet: bool,
    /// Toolchain config for auto-install
    pub toolchain: &'a ToolchainConfig,
}

/// Execute a command with the detected runner.
///
/// If the runner's tool is not installed and `toolchain` allows it,
/// we transparently delegate to `mise exec` or `proto run` so the user
/// never has to install the tool manually.
pub fn execute(
    runner: &DetectedRunner,
    task: &str,
    extra_args: &[String],
    working_dir: &Path,
    opts: &ExecuteOptions,
) -> Result<RunResult, RunError> {
    // Skip tool-installed check for dry-run and custom runners.
    if !opts.dry_run && runner.ecosystem != Ecosystem::Custom && !is_tool_installed(&runner.name) {
        // Try auto-install via mise / proto before giving up.
        if opts.toolchain.get_auto_install() {
            return execute_via_toolchain(
                runner,
                task,
                extra_args,
                working_dir,
                opts,
                opts.toolchain.get_backend(),
            );
        }
        return Err(RunError::ToolNotInstalled(format!(
            "{} is not installed. Please install it to continue.",
            runner.name
        )));
    }

    // Build the command
    let cmd_parts = runner.build_command(task, extra_args);
    let cmd_string = cmd_parts.join(" ");

    if opts.verbose {
        output::detected(&runner.name, &runner.detected_file);
    }

    if opts.dry_run {
        if !opts.quiet {
            println!("{}", cmd_string);
        }
        // Return a fake success for dry-run
        return Ok(RunResult {
            exit_status: std::process::ExitStatus::default(),
            runner: runner.clone(),
            working_dir: working_dir.to_path_buf(),
        });
    }

    if !opts.quiet {
        output::executing(&cmd_string);
    }

    // Execute the command
    let program = &cmd_parts[0];
    let args = &cmd_parts[1..];

    let status = Command::new(program)
        .args(args)
        .current_dir(working_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| RunError::CommandFailed(format!("Failed to execute {}: {}", program, e)))?;

    Ok(RunResult {
        exit_status: status,
        runner: runner.clone(),
        working_dir: working_dir.to_path_buf(),
    })
}

/// Try to run the command via a toolchain manager (mise or proto).
/// Falls through to `ToolNotInstalled` if no suitable manager is found.
fn execute_via_toolchain(
    runner: &DetectedRunner,
    task: &str,
    extra_args: &[String],
    working_dir: &Path,
    opts: &ExecuteOptions,
    backend: ToolchainBackend,
) -> Result<RunResult, RunError> {
    match backend {
        ToolchainBackend::Mise => {
            if is_tool_installed("mise") {
                return execute_via_mise(runner, task, extra_args, working_dir, opts);
            }
            // mise requested but not installed
            Err(RunError::ToolNotInstalled(format!(
                "{} is not installed and mise is not available to auto-install it. \
                 Install mise from https://mise.jdx.dev or install {} directly.",
                runner.name, runner.name
            )))
        }
        ToolchainBackend::Proto => {
            if is_tool_installed("proto") {
                return execute_via_proto(runner, task, extra_args, working_dir, opts);
            }
            Err(RunError::ToolNotInstalled(format!(
                "{} is not installed and proto is not available to auto-install it. \
                 Install proto from https://moonrepo.dev/proto or install {} directly.",
                runner.name, runner.name
            )))
        }
        ToolchainBackend::Auto => {
            // Try mise first, then proto.
            if is_tool_installed("mise") {
                return execute_via_mise(runner, task, extra_args, working_dir, opts);
            }
            if is_tool_installed("proto") {
                return execute_via_proto(runner, task, extra_args, working_dir, opts);
            }
            // Neither found → propagate as ToolNotInstalled so the hint layer fires.
            Err(RunError::ToolNotInstalled(format!(
                "{} is not installed. Please install it to continue.",
                runner.name
            )))
        }
        ToolchainBackend::None => Err(RunError::ToolNotInstalled(format!(
            "{} is not installed. Please install it to continue.",
            runner.name
        ))),
    }
}

/// Run `mise exec <tool> -- <cmd> [args]`.
///
/// mise will auto-download the tool if missing (requires a `.tool-versions`
/// or `mise.toml` declaring the version, or a globally configured version).
fn execute_via_mise(
    runner: &DetectedRunner,
    task: &str,
    extra_args: &[String],
    working_dir: &Path,
    opts: &ExecuteOptions,
) -> Result<RunResult, RunError> {
    let cmd_parts = runner.build_command(task, extra_args);
    // mise exec -- <program> <args>
    let mut mise_args: Vec<String> = vec!["exec".to_string(), "--".to_string()];
    mise_args.extend(cmd_parts.iter().cloned());

    let cmd_string = format!("mise exec -- {}", cmd_parts.join(" "));

    if !opts.quiet {
        output::warning(&format!(
            "{} not found — using mise to auto-install and run it",
            runner.name
        ));
    }
    if opts.verbose {
        output::info(&format!("Delegating to: {}", cmd_string));
    }

    if opts.dry_run {
        if !opts.quiet {
            println!("{}", cmd_string);
        }
        return Ok(RunResult {
            exit_status: std::process::ExitStatus::default(),
            runner: runner.clone(),
            working_dir: working_dir.to_path_buf(),
        });
    }

    if !opts.quiet {
        output::executing(&cmd_string);
    }

    let status = Command::new("mise")
        .args(&mise_args)
        .env("MISE_YES", "1") // non-interactive: auto-accept all prompts
        .env("MISE_AUTO_INSTALL", "true") // auto-install missing tool versions
        .current_dir(working_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| RunError::CommandFailed(format!("Failed to execute mise: {}", e)))?;

    Ok(RunResult {
        exit_status: status,
        runner: runner.clone(),
        working_dir: working_dir.to_path_buf(),
    })
}

/// Run `proto run <tool> -- <args>`.
///
/// proto will auto-download the tool version declared in `.prototools`.
fn execute_via_proto(
    runner: &DetectedRunner,
    task: &str,
    extra_args: &[String],
    working_dir: &Path,
    opts: &ExecuteOptions,
) -> Result<RunResult, RunError> {
    let cmd_parts = runner.build_command(task, extra_args);
    // proto run <tool> -- <args_after_tool>
    let tool_args = &cmd_parts[1..];
    let mut proto_args: Vec<String> =
        vec!["run".to_string(), runner.name.clone(), "--".to_string()];
    proto_args.extend(tool_args.iter().cloned());

    let cmd_string = format!("proto run {} -- {}", runner.name, tool_args.join(" "));

    if !opts.quiet {
        output::warning(&format!(
            "{} not found — using proto to auto-install and run it",
            runner.name
        ));
    }
    if opts.verbose {
        output::info(&format!("Delegating to: {}", cmd_string));
    }

    if opts.dry_run {
        if !opts.quiet {
            println!("{}", cmd_string);
        }
        return Ok(RunResult {
            exit_status: std::process::ExitStatus::default(),
            runner: runner.clone(),
            working_dir: working_dir.to_path_buf(),
        });
    }

    if !opts.quiet {
        output::executing(&cmd_string);
    }

    let status = Command::new("proto")
        .args(&proto_args)
        .env("PROTO_AUTO_INSTALL", "true") // auto-install missing tool versions
        .current_dir(working_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| RunError::CommandFailed(format!("Failed to execute proto: {}", e)))?;

    Ok(RunResult {
        exit_status: status,
        runner: runner.clone(),
        working_dir: working_dir.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RunError;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_search_runners_current_dir() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();

        let (runners, found_dir) = search_runners(dir.path(), 3, &[], false).unwrap();
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "npm");
        assert_eq!(found_dir, dir.path());
    }

    #[test]
    fn test_search_runners_parent_dir() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();

        let subdir = dir.path().join("src");
        std::fs::create_dir(&subdir).unwrap();

        let (runners, found_dir) = search_runners(&subdir, 3, &[], false).unwrap();
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].name, "npm");
        assert_eq!(found_dir, dir.path());
    }

    #[test]
    fn test_search_runners_not_found() {
        let dir = tempdir().unwrap();
        let result = search_runners(dir.path(), 3, &[], false);
        assert!(matches!(result, Err(RunError::RunnerNotFound(3))));
    }

    #[test]
    fn test_search_runners_with_ignore() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();

        let result = search_runners(dir.path(), 3, &["npm".to_string()], false);
        assert!(matches!(result, Err(RunError::RunnerNotFound(3))));
    }

    #[test]
    fn test_check_conflicts_single_runner() {
        let dir = tempdir().unwrap();
        let runners = vec![DetectedRunner::new(
            "npm",
            "package.json",
            Ecosystem::NodeJs,
            4,
        )];
        let result = check_conflicts(&runners, dir.path(), false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "npm");
    }

    #[test]
    fn test_check_conflicts_different_ecosystems() {
        let dir = tempdir().unwrap();
        let runners = vec![
            DetectedRunner::new("npm", "package.json", Ecosystem::NodeJs, 4),
            DetectedRunner::new("cargo", "Cargo.toml", Ecosystem::Rust, 9),
        ];
        let result = check_conflicts(&runners, dir.path(), false).unwrap();
        // Both ecosystems kept; sorted by priority (npm=4 < cargo=9)
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "npm");
        assert_eq!(result[1].name, "cargo");
    }

    #[test]
    fn test_check_conflicts_corepack_resolves() {
        use std::io::Write;

        let dir = tempdir().unwrap();
        // Create package.json with packageManager specifying pnpm
        let mut file = File::create(dir.path().join("package.json")).unwrap();
        writeln!(file, r#"{{"packageManager": "pnpm@9.0.0"}}"#).unwrap();

        let runners = vec![
            DetectedRunner::new("yarn", "yarn.lock", Ecosystem::NodeJs, 3),
            DetectedRunner::new("pnpm", "pnpm-lock.yaml", Ecosystem::NodeJs, 2),
            DetectedRunner::new("npm", "package-lock.json", Ecosystem::NodeJs, 4),
        ];

        // Corepack should resolve to pnpm; only pnpm returned
        let result = check_conflicts(&runners, dir.path(), false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "pnpm");
    }

    #[test]
    fn test_check_conflicts_non_nodejs_ecosystem() {
        let dir = tempdir().unwrap();
        let runners = vec![
            DetectedRunner::new("bundler", "Gemfile.lock", Ecosystem::Ruby, 13),
            DetectedRunner::new("rake", "Rakefile", Ecosystem::Ruby, 14),
        ];

        // Non-Node: both returned (with a warning), sorted by priority
        let result = check_conflicts(&runners, dir.path(), false).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "bundler"); // priority 13 wins
    }

    // ── Toolchain auto-install tests ───────────────────────────────────

    #[test]
    fn test_execute_tool_not_installed_no_autoinstall_returns_error() {
        let dir = tempdir().unwrap();
        // Use a fake tool name guaranteed not to be in PATH.
        let runner =
            DetectedRunner::new("devrunner-fake-tool-xyz", "Cargo.toml", Ecosystem::Rust, 9);
        let toolchain = crate::config::ToolchainConfig {
            auto_install: Some(false), // disable auto-install
            ..Default::default()
        };

        let opts = ExecuteOptions {
            dry_run: false,
            verbose: false,
            quiet: false,
            toolchain: &toolchain,
        };

        let result = execute(&runner, "build", &[], dir.path(), &opts);
        assert!(
            matches!(result, Err(RunError::ToolNotInstalled(_))),
            "Expected ToolNotInstalled when tool missing and auto-install disabled"
        );
    }

    #[test]
    fn test_execute_backend_none_returns_error() {
        let dir = tempdir().unwrap();
        let runner =
            DetectedRunner::new("devrunner-fake-tool-xyz", "Cargo.toml", Ecosystem::Rust, 9);
        let toolchain = crate::config::ToolchainConfig {
            auto_install: Some(true),
            backend: Some("none".to_string()),
        };

        let opts = ExecuteOptions {
            dry_run: false,
            verbose: false,
            quiet: false,
            toolchain: &toolchain,
        };

        let result = execute(&runner, "build", &[], dir.path(), &opts);
        assert!(
            matches!(result, Err(RunError::ToolNotInstalled(_))),
            "Expected ToolNotInstalled when backend=none"
        );
    }

    #[test]
    fn test_execute_auto_install_auto_backend_no_managers_returns_error() {
        // When no mise/proto is installed and backend is Auto, we should still
        // get ToolNotInstalled (the hint layer handles UX).
        // We can only run this test if mise AND proto are both absent.
        if is_tool_installed("mise") || is_tool_installed("proto") {
            // Skip — at least one manager is available, so we can't test this path.
            return;
        }

        let dir = tempdir().unwrap();
        let runner =
            DetectedRunner::new("devrunner-fake-tool-xyz", "Cargo.toml", Ecosystem::Rust, 9);
        let toolchain = crate::config::ToolchainConfig::default(); // auto_install=true, backend=Auto

        let opts = ExecuteOptions {
            dry_run: false,
            verbose: false,
            quiet: false,
            toolchain: &toolchain,
        };

        let result = execute(&runner, "build", &[], dir.path(), &opts);
        assert!(
            matches!(result, Err(RunError::ToolNotInstalled(_))),
            "Expected ToolNotInstalled when no managers available"
        );
    }

    #[test]
    fn test_execute_dry_run_with_toolchain_enabled() {
        // dry-run skips the is_tool_installed check entirely, so it should
        // always succeed even with a fake tool and auto-install enabled.
        let dir = tempdir().unwrap();
        let runner =
            DetectedRunner::new("devrunner-fake-tool-xyz", "Cargo.toml", Ecosystem::Rust, 9);
        let toolchain = crate::config::ToolchainConfig::default();

        let opts = ExecuteOptions {
            dry_run: true,
            verbose: false,
            quiet: true,
            toolchain: &toolchain,
        };

        let result = execute(&runner, "build", &[], dir.path(), &opts);
        assert!(
            result.is_ok(),
            "dry-run should succeed regardless of tool installation"
        );
    }
}
