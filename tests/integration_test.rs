#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::tempdir;

fn run_cmd() -> Command {
    Command::cargo_bin("devrunner").unwrap()
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_help() {
    run_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Universal task runner"));
}

#[test]
fn test_version() {
    run_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_no_runner_found() {
    let dir = tempdir().unwrap();

    run_cmd()
        .current_dir(dir.path())
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No runner found"));
}

#[test]
fn test_dry_run_npm() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("npm run test"));
}

#[test]
fn test_dry_run_cargo() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Cargo.toml")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["build", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cargo build"));
}

#[test]
fn test_dry_run_pnpm() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    File::create(dir.path().join("pnpm-lock.yaml")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("pnpm run test"));
}

#[test]
fn test_dry_run_yarn() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    File::create(dir.path().join("yarn.lock")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("yarn run test"));
}

#[test]
fn test_dry_run_bun() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    File::create(dir.path().join("bun.lockb")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bun run test"));
}

#[test]
fn test_dry_run_poetry() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("pyproject.toml")).unwrap();
    File::create(dir.path().join("poetry.lock")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("poetry run test"));
}

#[test]
fn test_ignore_flag() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--ignore=npm", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No runner found"));
}

#[test]
fn test_recursive_search() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    let subdir = dir.path().join("src").join("components");
    fs::create_dir_all(&subdir).unwrap();

    run_cmd()
        .current_dir(&subdir)
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("npm run test"));
}

#[test]
fn test_levels_limit() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    let deep_subdir = dir.path().join("a").join("b").join("c").join("d");
    fs::create_dir_all(&deep_subdir).unwrap();

    // With levels=0, should not find it
    run_cmd()
        .current_dir(&deep_subdir)
        .args(["test", "--levels=0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No runner found"));

    // With levels=5, should find it
    run_cmd()
        .current_dir(&deep_subdir)
        .args(["test", "--levels=5", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("npm run test"));
}

#[test]
fn test_levels_from_local_config() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    let deep_subdir = dir.path().join("a").join("b").join("c").join("d");
    fs::create_dir_all(&deep_subdir).unwrap();
    fs::write(deep_subdir.join("run.toml"), "max_levels = 5\n").unwrap();

    run_cmd()
        .current_dir(&deep_subdir)
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("npm run test"));
}

#[test]
fn test_cli_levels_override_local_config() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    let deep_subdir = dir.path().join("a").join("b").join("c").join("d");
    fs::create_dir_all(&deep_subdir).unwrap();
    fs::write(deep_subdir.join("run.toml"), "max_levels = 5\n").unwrap();

    run_cmd()
        .current_dir(&deep_subdir)
        .args(["test", "--levels=0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No runner found"));
}

#[test]
fn test_extra_args() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run", "--", "--coverage", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "npm run test --coverage --verbose",
        ));
}

#[test]
fn test_verbose_mode() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run", "--verbose"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Detected"));
}

#[test]
fn test_completions_bash() {
    run_cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_devrunner"));
}

#[test]
fn test_completions_zsh() {
    run_cmd()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef devrunner"));
}

#[test]
fn test_completions_fish() {
    run_cmd()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_completions_powershell() {
    run_cmd()
        .args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Register-ArgumentCompleter"));
}

#[test]
fn test_makefile_detection() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Makefile")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["build", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("make build"));
}

#[test]
fn test_gradle_detection() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("build.gradle")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["build", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("gradle build"));
}

#[test]
fn test_maven_detection() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("pom.xml")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["compile", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mvn compile"));
}

#[test]
fn test_mix_detection() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("mix.exs")).unwrap();

    run_cmd()
        .current_dir(dir.path())
        .args(["test", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mix test"));
}

#[test]
fn test_list_subcommand_with_fixture() {
    run_cmd()
        .current_dir(fixture_path("node-npm"))
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available scripts"));
}

#[test]
fn test_why_subcommand_with_fixture() {
    run_cmd()
        .current_dir(fixture_path("node-npm"))
        .arg("why")
        .assert()
        .success()
        .stdout(predicate::str::contains("Runner Selection Analysis"));
}

#[test]
fn test_doctor_subcommand_with_fixture() {
    run_cmd()
        .current_dir(fixture_path("node-npm"))
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Project Detection"));
}

#[test]
fn test_monorepo_prefers_workspace_lockfile_runner() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    File::create(dir.path().join("pnpm-lock.yaml")).unwrap();
    File::create(dir.path().join("pnpm-workspace.yaml")).unwrap();

    let subdir = dir.path().join("apps").join("web");
    fs::create_dir_all(&subdir).unwrap();
    File::create(subdir.join("package.json")).unwrap();

    run_cmd()
        .current_dir(&subdir)
        .args(["test", "--dry-run", "--levels=5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("pnpm run test"));
}

#[test]
fn test_node_leaf_not_overridden_by_parent_non_node_project() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Cargo.toml")).unwrap();

    let subdir = dir.path().join("frontend");
    fs::create_dir_all(&subdir).unwrap();
    File::create(subdir.join("package.json")).unwrap();

    run_cmd()
        .current_dir(&subdir)
        .args(["test", "--dry-run", "--levels=5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("npm run test"));
}
