# DEVELOPMENT PLAN: "run" CLI

## OBJECTIVE

Create a command-line tool (CLI) in Rust called `devrunner` that abstracts the execution of project commands, automatically detecting the development environment (Node.js, Python, Rust, PHP, Go, Ruby, Java, .NET, Elixir, Swift, Zig, Make) and delegating to the appropriate tool, eliminating the need to memorize which manager each project uses.

**Repository**: https://github.com/princepal9120/devrunner

***

## CRITICAL DEVELOPMENT GUIDANCE

**BEFORE starting any implementation**, conduct web research for up-to-date information on:
- Current lockfile conventions for each ecosystem
- Standard execution commands for each tool
- Recent breaking changes in package managers
- Best practices for Rust project structure for CLIs
- Latest and stable Rust crates for: CLI parsing, self-update, async runtime, colorization
- GitHub Releases API structure and authentication
- Cross-compilation best practices for Rust

Conduct incremental research during development when there are doubts about specific implementation. Do not assume outdated knowledge.

***

## DETECTION ARCHITECTURE

### Global Priority Hierarchy

Check for the presence of key files in the following order of precedence:

**Node.js Ecosystem**:
1. **Bun**: `bun.lockb` OR `bun.lock` + `package.json` → `bun run <command>`
2. **PNPM**: `pnpm-lock.yaml` + `package.json` → `pnpm run <command>`
3. **Yarn**: `yarn.lock` + `package.json` → `yarn run <command>`
4. **NPM**: `package-lock.json` + `package.json` OR just `package.json` → `npm run <command>`

**Python Ecosystem**:
5. **UV**: `uv.lock` + `pyproject.toml` → `uv run <command>`
6. **Poetry**: `poetry.lock` + `pyproject.toml` → `poetry run <command>`
7. **Pipenv**: `Pipfile.lock` + `Pipfile` → `pipenv run <command>`
8. **Pip**: `requirements.txt` OR `pyproject.toml` (without poetry/uv lock) → `python -m <command>`

**Rust Ecosystem**:
9. **Cargo**: `Cargo.toml` + `Cargo.lock` → `cargo <command>`

**PHP Ecosystem**:
10. **Composer**: `composer.lock` + `composer.json` → `composer run <command>`

**Go Ecosystem**:
11. **Taskfile**: `Taskfile.yml` OR `Taskfile.yaml` → `task <command>`
12. **Go Modules**: `go.mod` + `go.sum` → `go run <command>` (if command looks like a path) OR `go <command>`

**Ruby Ecosystem**:
13. **Bundler**: `Gemfile.lock` + `Gemfile` → `bundle exec <command>`
14. **Rake**: `Rakefile` → `rake <command>`

**Java/JVM Ecosystem**:
15. **Gradle**: `build.gradle` OR `build.gradle.kts` + `gradle.lockfile` (optional) → `gradle <command>`
16. **Maven**: `pom.xml` → `mvn <command>`

**.NET Ecosystem**:
17. **.NET**: `*.csproj` OR `*.sln` → `dotnet <command>`

**Elixir Ecosystem**:
18. **Mix**: `mix.exs` + `mix.lock` → `mix <command>`

**Swift Ecosystem**:
19. **Swift Package Manager**: `Package.swift` → `swift run <command>`

**Zig Ecosystem**:
20. **Zig Build**: `build.zig` → `zig build <command>`

**Generic Utility**:
21. **Make**: `Makefile` OR `makefile` → `make <command>`

**Rationale for Order**:
- Prioritize more specific tools before generic ones (lockfiles before simple manifests)
- Within each ecosystem, prioritize modern tools over legacy ones
- Make is last as it is more generic and used as a universal fallback

### Recursive Search Strategy

1. Check current directory (`./`)
2. If no runner is found, go up one level (`../`)
3. Repeat up to **3 levels up** by default (configurable via `--levels=N`)
4. If nothing is found after limit, return formatted error:
   ```
   Error: No runner found in 3 levels above the current directory.
   Hint: Use --levels=N to increase search or --ignore=<tool> to adjust detection.
   ```

Implement smart cache: if the directory has already been scanned in the same execution, reuse result.

### Lockfile Conflict Resolution

When multiple lockfiles of the **same ecosystem** are found (example: `package-lock.json` + `yarn.lock`):

1. Check which corresponding tools are installed globally using `which` (Unix) or `where` (Windows)
2. **If only one tool is installed**: use that one and emit a colored warning (yellow):
   ```
   ⚠ Warning: Found package-lock.json and yarn.lock, but only npm is installed.
   Using npm. Consider removing yarn.lock if not in use.
   ```
3. **If both are installed**: stop with error (red):
   ```
   ❌ Error: Detected package-lock.json and yarn.lock.
   Both tools (npm, yarn) are installed globally.
   Action required: Remove the outdated lockfile or use --ignore=npm (or --ignore=yarn).
   ```

4. **If neither is installed**: informative error suggesting installation

Apply similar logic for other ecosystems (Poetry vs UV, Gradle vs Maven when both present).

***

## COMMAND LINE INTERFACE

### Base Syntax
```
run <command> [arguments] [flags] [-- extra-arguments]
```

### Mandatory Flags

Implement the following flags with robust parsing:

- `--levels=N`: Defines how many levels up from the current directory to search (default: 3, min: 0, max: 10)
- `--ignore=tool1,tool2`: Ignores specific runners in detection (accepts comma-separated list)
- `--ignore tool1 --ignore tool2`: Alternative syntax, multiple flags (both syntaxes must work)
- `-v, --verbose`: Displays detailed detection info, executed command, found files
- `-q, --quiet`: Suppresses all messages from the CLI itself (warnings, info), keeps only output of the executed command and critical errors
- `--dry-run`: Displays the full command that would be executed without executing (useful for debug and scripts)
- `--update`: Forces check and immediate update installation, blocking (overrides default async behavior)
- `-h, --help`: Displays full help with list of all supported runners, usage examples
- `-V, --version`: Displays current CLI version

### Argument Separator

Implement support for the standard Unix `--` separator:
```
run test -- --coverage --verbose --reporter=json
```

All content after `--` must be passed literally to the underlying command, without parsing or modification. Preserve spaces, quotes, and special characters.

### Exit Code Behavior

Capture and return the **original exit code** of the executed command, without modification. Essential for integration with CI/CD and bash scripts that depend on `$?`.

Exception: if the CLI itself fails before execution (command not found, parsing error), return specific exit codes:
- `1`: Generic error
- `2`: Runner not found
- `3`: Lockfile conflict
- `127`: Detected tool not installed

***

## CONFIGURATION

### Global File: `~/.config/run/config.toml`

Create directory structure if it doesn't exist. TOML format:

```toml
max_levels = 5
auto_update = true
ignore_tools = ["npm"]
verbose = false
quiet = false
```

### Local File: `./run.toml`

Allows per-project override:

```toml
max_levels = 2
ignore_tools = ["yarn", "pip"]
verbose = true
```

### Configuration Precedence

Apply in order (last one overwrites previous):
1. Hardcoded defaults
2. `~/.config/run/config.toml` (global)
3. `./run.toml` (project local)
4. CLI arguments

Implement robust parsing with type and value validation. Silently ignore unknown keys for future compatibility.

***

## AUTO-UPDATE

### Async Update Strategy

**Critical timing**: Execute update process **after** the requested command finishes, **before** the main process exit.

**Execution flow**:
1. CLI receives `run test`
2. Detects appropriate runner
3. Executes command immediately (stdout/stderr/exit code connected to terminal)
4. Command finishes
5. **Before exiting**, spawn detached/daemon child process that:
   - Queries GitHub Releases API: `GET https://api.github.com/repos/verseles/run/releases/latest`
   - Compares remote `tag_name` with local version (semver parsing)
   - If remote version > local:
     - Detects current platform/architecture
     - Downloads appropriate asset (e.g., `run-linux-x86_64`, `run-macos-aarch64`, `run-windows-x86_64.exe`)
     - Verifies asset SHA256 checksum
     - Replaces existing binary atomically (rename temp → target)
     - Saves update metadata in `~/.config/run/update.json`:
       ```json
       {
         "updated_at": "2025-12-14T03:00:00Z",
         "from_version": "0.1.0",
         "to_version": "0.2.0",
         "changelog_url": "https://github.com/verseles/run/releases/tag/v0.2.0"
       }
       ```
   - Daemon process terminates silently
6. Main CLI exits with executed command code

**Timeout**: Download process has a **5-second** timeout. If exceeded, abort silently without affecting UX.

**Failures**: Any error in the update process (network, permissions, invalid checksum) must be silent. Do not impact user experience.

### Applied Update Notification

On the **next execution** after a successful update:

1. Check existence of `~/.config/run/update.json`
2. If it exists and `updated_at` is recent (< 24h), display colored message (green):
   ```
   ✓ run was updated: v0.1.0 → v0.2.0
   
   Key changes:
   - Added support for Zig and Swift
   - Improved conflict detection
   - Fixed bug in Windows auto-update
   
   See full changelog: https://github.com/verseles/run/releases/tag/v0.2.0
   ```
3. Extract changelog: fetch release via API and use `body` field (summarize first 3-5 lines if too long)
4. Delete `update.json` after displaying (to avoid repeating)

Message must respect `--quiet` flag (do not display if quiet active).

### Update Control

- Auto-update is generic **default** (opt-out via config `auto_update = false`)
- Flag `--update` forces check and **synchronous/blocking** installation before command execution
- Environment variable `RUN_NO_UPDATE=1` temporarily disables

### Technology

Use:
- Async runtime: **Tokio** (most mature and adopted)
- HTTP client: **reqwest** with TLS features
- Crate for self-update: research and evaluate `self_update` crate or custom implementation based on GitHub API
- Semver parsing: `semver` crate

***

## BINARY OPTIMIZATION

### `Cargo.toml` Configuration

Add optimized release profile:

```toml
[profile.release]
lto = true              # Link-Time Optimization (cross-crate)
strip = true            # Remove debug symbols
panic = "abort"         # Remove stack unwinding
opt-level = "z"         # Optimize for size
codegen-units = 1       # Maximum optimizations (slower compilation)
```

### Size Goal

Final binary must be **< 5MB** for all platforms (x86_64, aarch64).

After release build, run additional `strip` if necessary. Consider `upx` compression for distribution (test if it causes issues with Windows antivirus).

### Performance

- Cold start (time to first detection): **< 50ms**
- 3-level recursive search: **< 10ms**
- Command execution should not add perceptible overhead (< 5ms)

Do profiling with `cargo flamegraph` during development to identify bottlenecks.

***

## QUALITY AND TESTING

### Testing Structure

Organize tests into three categories:

#### 1. Unit Tests (`#[test]`)

For each detection module implement:

**Node.js Module** (`src/detectors/node.rs`):
- Correctly detect each lockfile type (bun.lockb, pnpm-lock.yaml, yarn.lock, package-lock.json)
- Prioritization when multiple lockfiles exist
- Fallback to package.json without lock
- Parsing of package.json to extract scripts (if necessary)

**Python Module** (`src/detectors/python.rs`):
- Detect uv.lock, poetry.lock, Pipfile.lock, requirements.txt
- Prioritization UV > Poetry > Pipenv > Pip
- Validate generated command for each tool

Replicate similar structure for Go, Ruby, Java, .NET, Elixir, Swift, Zig, Make.

**Config Module** (`src/config.rs`):
- Parsing of valid and invalid TOML
- Precedence between global/local/CLI args
- Correct defaults when files don't exist

**CLI Module** (`src/cli.rs`):
- Argument parsing with clap
- `--` separator working correctly
- Multiple flags (repeated --ignore)

#### 2. Integration Tests (`tests/`)

Create real project fixtures in `tests/fixtures/`:
```
tests/fixtures/
├── node-bun/          # project with bun.lockb
├── node-pnpm/         # project with pnpm-lock.yaml
├── python-poetry/     # project with poetry.lock
├── rust-cargo/        # Rust project
├── mixed-lockfiles/   # intentional conflict
├── nested/
│   └── deep/
│       └── project/   # test recursive search
└── ...
```

**Scenarios to test**:
- End-to-end execution with mock command in each project type
- Recursive search: run from subdirectory and verify it finds runner N levels up
- Lockfile conflict: verify appropriate error
- Flag `--dry-run`: verify output without executing
- Flag `--ignore`: verify runner is skipped
- Correct exit codes

Use `assert_cmd` crate to test CLI.

#### 3. Cross-platform Tests

Configure CI to run tests on:
- **Linux**: Ubuntu latest (x86_64)
- **macOS**: latest (x86_64 and aarch64 if possible)
- **Windows**: latest (x86_64)

Special attention to:
- Path separators (`/` vs `\`)
- Commands `which` vs `where`
- Line endings (LF vs CRLF)
- Filesystem case sensitivity
- File permissions (executable on Unix)

### Code Coverage

**Minimum goal**: 80% coverage for core logic (detection, config, CLI parsing).

Exclude from coverage: output formatting, specific error codes, update module (hard to test).

Use `cargo-tarpaulin` or `cargo-llvm-cov` to generate reports. Integrate with CI.

### Property-Based Testing

Consider using `proptest` to:
- Parsing of file paths with special characters
- Semver validation in update checker
- Recursive search invariants (never go up more than max_levels)

***

## CI/CD PIPELINE

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

**Triggers**:
- Push on `main` and `develop`
- Pull requests
- `v*` tags (for releases)

**Jobs**:

#### Job 1: Lint (`lint`)

Run on Ubuntu latest:
```yaml
- cargo fmt --check
- cargo clippy --all-targets --all-features -- -D warnings
```

Fail build if there are clippy warnings.

#### Job 2: Test (`test`)

Matrix strategy:
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable]
```

Steps:
```yaml
- cargo test --all-features --verbose
- cargo test --release --all-features  # test optimizations
```

#### Job 3: Security Audit (`security`)

Run on Ubuntu latest:
```yaml
- cargo install cargo-audit
- cargo audit
```

Fail if there are HIGH or CRITICAL vulnerabilities.

#### Job 4: Build Release (`build`)

**Trigger**: Only on `v*` tags

Matrix for multiple platforms:
```yaml
strategy:
  matrix:
    include:
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
      - os: ubuntu-latest
        target: aarch64-unknown-linux-gnu
      - os: macos-latest
        target: x86_64-apple-darwin
      - os: macos-latest
        target: aarch64-apple-darwin
      - os: windows-latest
        target: x86_64-pc-windows-msvc
```

Steps:
```yaml
- Install cross if necessary
- cargo build --release --target $TARGET
- Generate SHA256 checksum of binary
- Compress (tar.gz for Unix, zip for Windows)
- Upload as artifact
```

#### Job 5: Release (`release`)

**Dependency**: After `build` job completes successfully

**Trigger**: Only `v*` tags

Steps:
```yaml
- Download all artifacts
- Create GitHub Release using tag
- Upload all binaries + checksums
- Generate and include shell completions (bash, zsh, fish, powershell)
```

Use `softprops/action-gh-release` action or similar.

### Local CI Simulation

Create script `scripts/pre-push.sh`:

```bash
#!/usr/bin/env bash
set -e

echo "🔍 Running CI checks locally..."

echo "📝 Checking formatting..."
cargo fmt --check

echo "🔬 Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🧪 Running tests..."
cargo test --all-features

echo "🔒 Checking vulnerabilities..."
cargo audit

echo "✅ All checks passed!"
```

Make executable: `chmod +x scripts/pre-push.sh`

**Instructions in README**: Suggest installation as git hook:
```bash
ln -s ../../scripts/pre-push.sh .git/hooks/pre-push
```

***

## USER EXPERIENCE

### Color and Icon System

Use `owo-colors` or `colored` library for stylized output.

**Color Palette**:
- 🟢 Green (`#00ff00`): Success, successful detection, update applied
- 🟡 Yellow (`#ffff00`): Warnings, automatically resolved conflicts
- 🔴 Red (`#ff0000`): Critical errors, failures
- 🔵 Blue/Cyan (`#00ffff`): Information in `--verbose` mode
- ⚪ White/Gray: Neutral output

**Unicode Icons**:
- ✓ (U+2713): Success
- ⚠ (U+26A0): Warning
- ❌ (U+274C): Error
- 🔍 (U+1F50D): Detection in progress (verbose)
- 📦 (U+1F4E6): Runner detected (verbose)
- ⬆ (U+2B06): Update available/applied

**Example output**:
```
🔍 Searching for runner in ./src/components...
📦 Detected: pnpm (pnpm-lock.yaml)
✓ Executing: pnpm run test

[command output...]

✓ Command completed successfully (exit code: 0)
```

Respect `NO_COLOR` environment variable (Unix convention) to disable colors.

### Shell Completions

Generate completions using `clap_complete`:

**Targets**:
- Bash: `run.bash`
- Zsh: `_run`
- Fish: `run.fish`
- PowerShell: `_run.ps1`

Include in releases. Add instructions in README for installation:

**Bash**:
```bash
sudo cp run.bash /usr/share/bash-completion/completions/run
```

**Zsh**:
```bash
cp _run ~/.zsh/completion/
```

**Fish**:
```bash
cp run.fish ~/.config/fish/completions/
```

**PowerShell**:
```powershell
# Add to $PROFILE
```

Completions should suggest:
- Available flags (`--levels`, `--ignore`, etc.)
- Values for `--ignore` (list of runners: npm, yarn, pnpm, etc.)
- Scripts from current project's `package.json` (advanced feature, optional)

***

## DISTRIBUTION

### Priority 1: Install Script

Create `install.sh` in repository root:

**Responsibilities**:
1. Automatically detect OS and architecture (`uname -s`, `uname -m`)
2. Map to correct asset name in GitHub Release
3. Download latest release from `https://github.com/verseles/run/releases/latest`
4. Verify SHA256 checksum (download corresponding `.sha256` file)
5. Install in appropriate directory:
   - Preference: `$HOME/.local/bin` (if exists or create)
   - Fallback: `/usr/local/bin` (if has sudo permission)
   - Windows: `%USERPROFILE%\.local\bin` or `C:\Program Files\run\`
6. Make executable (`chmod +x` in Unix)
7. Check if directory is in PATH, warn if not
8. If run again: detect existing installation and update

**Update behavior**:
```bash
curl -fsSL https://raw.githubusercontent.com/verseles/run/main/install.sh | bash
```

Expected output:
```
🔍 Detecting system: Linux x86_64
📦 Downloading run v0.2.0...
✓ Checksum verified
✓ Installed in ~/.local/bin/run
⚠ Add ~/.local/bin to your PATH:
  export PATH="$HOME/.local/bin:$PATH"
```

Also create `install.ps1` for Windows (PowerShell).

### Distribution Roadmap (Phase 2+)

Add support for package managers after stable MVP:

**Priority 2**:
- `cargo install run-cli` (publish to crates.io)
- Homebrew tap: `brew install verseles/tap/run`

**Priority 3**:
- Scoop (Windows): add to bucket
- Chocolatey (Windows): publish package
- AUR (Arch Linux): create PKGBUILD

**Priority 4**:
- Snap (Ubuntu/Linux): publish to snapcraft
- Flatpak: publish to Flathub
- APT repository: for Debian/Ubuntu
- RPM repository: for Fedora/RHEL

***

## DOCUMENTATION

### README.md

Mandatory structure:

#### 1. Hero Section
```markdown
# 🚀 run

> Universal task runner for modern development

[![CI](https://github.com/verseles/run/workflows/CI/badge.svg)](...)
[![Release](https://img.shields.io/github/v/release/verseles/run)](...)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue)](...)
```

Include ASCII art logo or image.

#### 2. Quick Demo

Animated GIF or Asciinema showing:
- Running `run test` in Node.js project (auto-detects pnpm)
- Running `run build` in Python project (detects poetry)
- Running from subdirectory (recursive search)
- Lockfile conflict + resolution

Use tool like `asciinema` or `vhs` to record.

#### 3. Why run?

List problems it solves:
- Eliminates "which command do I use in this project?" (npm vs yarn vs pnpm vs bun)
- Works in subdirectories (no need to cd to root)
- One command to rule them all (Node, Python, Rust, Go, Ruby, Java, etc.)
- Automatic auto-update (always on latest version)
- Zero configuration necessary

#### 4. Installation

```bash
curl -fsSL https://raw.githubusercontent.com/verseles/run/main/install.sh | bash
```

List alternative methods (cargo install, homebrew, etc. as available).

#### 5. Supported Runners

Visual table:

| Ecosystem | Detection | Executed Command |
|-----------|-----------|------------------|
| Bun | `bun.lockb` + `package.json` | `bun run <cmd>` |
| PNPM | `pnpm-lock.yaml` + `package.json` | `pnpm run <cmd>` |
| ... | ... | ... |

Include all 20+ supported runners.

#### 6. Usage Examples

```bash
# Run project script
run test

# Pass extra arguments
run build -- --verbose --production

# Run from subdirectory (automatic recursive search)
cd src/components
run lint

# Search more levels up
run deploy --levels=5

# Ignore specific runners
run start --ignore=npm,yarn

# Dry-run mode (see command without executing)
run build --dry-run

# Quiet mode
run test -q

# Force update
run --update
```

#### 7. Configuration

Examples of `~/.config/run/config.toml` and `./run.toml` with explanatory comments.

#### 8. Shell Completions

Step-by-step instructions for each shell.

#### 9. Advanced Features

- Background auto-update
- Conflict resolution
- Smart recursive search
- Cross-platform

#### 10. Roadmap

List of planned features:
- [x] MVP with 20+ runners
- [x] Auto-update
- [ ] Opt-out telemetry
- [ ] Detection cache
- [ ] Plugin system
- [ ] VS Code extension

#### 11. Contributing

Link to contribution guide (create when necessary).

#### 12. License

```
Licensed under GNU Affero General Public License v3.0 (AGPL-3.0)
See LICENSE file for details.
```

### Other Files

**LICENSE**: Include full text of AGPL-3.0

**CHANGELOG.md**: Keep updated with each release following Keep a Changelog format

**CONTRIBUTING.md**: Add when there is external interest in contribution

***

## LICENSE

**AGPL-3.0** (GNU Affero General Public License v3.0)

Include `LICENSE` file in root with full license text.

**Headers in source files**: Add header in each Rust file:
```rust
// Copyright (C) 2025 [Author Name]
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
```

**Implications**:
- Code must remain open source
- Modifications must be shared under same license
- If used in network service, source code must be made available
- Allows commercial use provided code remains open

***

## ROADMAP

### Phase 1: MVP (Version 0.1.0)

**Mandatory Deliverables**:
- ✅ Detection of 20+ runners (Node/Python/Rust/PHP/Go/Ruby/Java/.NET/Elixir/Swift/Zig/Make)
- ✅ Configurable recursive search (default 3 levels)
- ✅ Lockfile conflict resolution
- ✅ Essential flags (--levels, --ignore, -v, -q, --dry-run, --help, --version)
- ✅ Argument separator (--)
- ✅ Auto-update via GitHub Releases (async post-execution)
- ✅ Update notification with changelog
- ✅ Global + local configuration (TOML)
- ✅ Complete CI/CD (Linux/macOS/Windows)
- ✅ Pre-push script for local validation
- ✅ Automatic releases on v* tags
- ✅ Install script (curl-to-bash)
- ✅ Shell completions (bash/zsh/fish/powershell)
- ✅ Modern README with visual demos
- ✅ Unit + integration tests (> 80% coverage)
- ✅ Optimized binary (< 5MB)
- ✅ Appropriate exit codes
- ✅ Harmonious colors and icons

**Launch Criteria**:
- All tests passing on 3 platforms
- Complete documentation
- At least 5 languages manually tested in real projects

### Phase 2: Adoption and Polish (Version 0.2.0 - 0.5.0)

**Features**:
- Publication to crates.io (`cargo install run-cli`)
- Official Homebrew tap
- Scoop/Chocolatey for Windows
- Detection cache (avoid re-scan in multiple consecutive executions)
- Workspace/monorepo support (Nx, Turborepo, Lerna)
- `package.json` detection → `packageManager` field (Corepack)
- Anonymous usage statistics (opt-out via config)
- Performance improvements (checks parallelization)
- Support for more architectures (ARM, RISC-V)

**Success Metrics**:
- 100+ stars on GitHub
- 1000+ installations
- 5+ external contributors

### Phase 3: Extensibility (Version 1.0.0+)

**Advanced Features**:
- Plugin system (users can add custom runners via `.run-plugins/`)
- Integration with IDEs (VS Code extension)
- Container support (detect Dockerfile/docker-compose, run via docker)
- AI-powered: suggest commands when script doesn't exist
- Detailed telemetry with web dashboard (opt-in)
- Custom alias support (`run t` → `run test`)
- Pre/post-execution hooks (run setup before command)
- Interactive mode (TUI to choose between multiple scripts)

**Criteria for 1.0.0**:
- Stable API (breaking changes require major bump)
- Production-ready in corporate environments
- 1000+ stars
- 10000+ active installations

***

## SUCCESS METRICS

### Technical (Automated)

**Performance**:
- Cold start < 50ms (measured in CI)
- 3-level recursive search < 10ms
- Binary size < 5MB all platforms
- Zero performance regressions between releases

**Quality**:
- Test coverage > 80%
- Zero Clippy warnings
- All tests passing on 3 OSs
- Cargo audit without HIGH/CRITICAL vulnerabilities

**Reliability**:
- CI green > 95% of time
- Releases without rollback
- Critical issues resolved < 48h

### Adoption (Tracked)

**Short term (3 months)**:
- 100 GitHub stars
- 500 installations via install.sh
- 10 issues/discussions created by users
- 3 external contributors

**Medium term (6 months)**:
- 500 GitHub stars
- 5000 installations
- 1000 daily executions (via opt-in telemetry)
- Mentioned in 3+ articles/tutorials

**Long term (12 months)**:
- 1000+ stars
- 20000+ installations
- Adopted by known open source project
- Packaged in mainstream Linux distro

***

## FINAL IMPLEMENTATION INSTRUCTIONS

### Before Starting

1. Web research on modern Rust project structures for CLIs (2024-2025)
2. Evaluate most up-to-date crates for each functionality
3. Review recent lockfile conventions (may have changed)
4. Verify GitHub Actions best practices for Rust cross-compilation

### During Development

- Make atomic commits with descriptive messages (Conventional Commits)
- Manually test on at least 2 different OSs before PR
- Run `scripts/pre-push.sh` before every push
- Document important architectural decisions (ADRs if necessary)
- Keep CHANGELOG.md updated

### Suggested Implementation Order

1. **Basic Setup**: Cargo project structure, basic CI, linting
2. **CLI Parsing**: Implement flags with clap, parsing tests
3. **Core Detection**: Start with 3-4 runners (npm, pnpm, cargo, make), recursive search
4. **Execution**: Spawn process, connect I/O, capture exit code
5. **Configuration**: TOML parsing, precedence
6. **Runner Expansion**: Add other languages incrementally
7. **Conflicts**: Multiple lockfiles resolution logic
8. **Auto-update**: Implement async after command
9. **Optimization**: Profile release, reduce binary size
10. **Completions**: Generate shell completions
11. **Documentation**: Complete README, visual demos
12. **Release**: CI workflow for multi-platform builds

### MVP Delivery Checklist

- [ ] Code compiles without warnings
- [ ] All tests passing (unit + integration)
- [ ] Coverage > 80%
- [ ] CI green on 3 platforms
- [ ] Complete README with examples
- [ ] LICENSE included
- [ ] Functional install script
- [ ] Shell completions generated
- [ ] Binaries < 5MB
- [ ] Auto-update manually tested
- [ ] At least 5 runners tested in real projects
- [ ] Tag v0.1.0 created
- [ ] Release published on GitHub with assets

***

**END OF PLAN**
