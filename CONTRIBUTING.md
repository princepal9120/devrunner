# Contributing to devrunner

Thanks for helping make `devrunner` better. This project is a Rust CLI that detects a project's toolchain and runs a requested task with the right underlying command.

## Ways to contribute

- Add or improve runner detection for an ecosystem.
- Fix false positives, conflict handling, or platform-specific behavior.
- Improve install scripts, shell completions, docs, or examples.
- Add tests that capture real project layouts.

If you are not sure where to start, open an issue with your use case or look for issues labeled `good first issue` or `help wanted`.

## Development setup

Requirements:

- Rust stable toolchain
- `make`
- Optional: `cargo-audit` for local security checks

```bash
git clone https://github.com/princepal9120/devrunner.git
cd devrunner
cargo build
cargo test
make precommit
```

`make precommit` is the same local quality gate maintainers expect before a pull request: formatting, Clippy, tests, and a security audit when `cargo-audit` is installed.

## Project layout

- `src/cli.rs` — command-line interface definition.
- `src/runner.rs` — ecosystem detection and command resolution.
- `src/config.rs` — global and project config loading.
- `src/update.rs` — background update checks.
- `tests/` — integration, custom runner, and property tests.
- `.github/workflows/ci.yml` — CI, release builds, and release publishing.

## Pull request checklist

Before opening a PR:

- [ ] Keep the change focused and explain the user problem it solves.
- [ ] Add or update tests for changed behavior.
- [ ] Update `README.md`, `CHANGELOG.md`, or other docs when behavior changes.
- [ ] Run `make precommit` from the repository root.
- [ ] Do not include local state, generated build artifacts, or secrets.

## Testing guidance

Prefer integration tests that create a temporary project layout and assert the command selected by `devrunner --dry-devrunner`. For new ecosystem support, include at least:

1. Manifest-only detection when applicable.
2. Lockfile-priority detection when applicable.
3. Conflict or missing-tool behavior when the ecosystem can conflict with another runner.
4. Windows-safe assertions when shell behavior differs by platform.

## Commit and review expectations

Use clear commit messages, keep diffs reviewable, and call out any compatibility risks. Maintainers may ask for tests, docs, or a smaller PR if a change mixes unrelated concerns.

## License

By contributing, you agree that your contribution is provided under this repository's AGPL-3.0 license.
