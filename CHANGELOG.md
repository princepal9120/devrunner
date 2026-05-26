# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.1] - 2026-05-26

### Changed
- Fixed CLI/config precedence so `devrunner.toml` and global config `max_levels` are applied when `--levels` is not provided.
- Aligned `list`, `why`, and `doctor` runner selection with the same conflict-resolution path used by command execution.
- Added monorepo-aware Node detection behavior: when a leaf package only has `package.json`, devrunner continues searching upward for workspace/root lockfile-based Node runners.
- Hardened Rust self-update flow to require SHA256 verification against release `.sha256` assets before replacing binaries.
- Hardened install scripts to enforce checksum validation and fail fast on mismatch.
- Improved Unix installer directory selection with `/usr/local/bin` fallback when user-local install path is not writable.
- Updated README to reflect actual config file locations, command usage, and local validation flow.

### Added
- Added integration tests for `max_levels` precedence (config default + CLI override).
- Added tests for monorepo runner selection and for avoiding cross-ecosystem parent override.
- Added update module unit tests for checksum parsing and SHA256 hashing.
- Added missing `LICENSE` file (AGPL-3.0).
