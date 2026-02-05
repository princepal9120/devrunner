# devrunner

Universal task runner for modern development.

`devrunner` detects your project tooling (npm, pnpm, cargo, poetry, go, maven, and more) and runs the right command without needing to remember ecosystem-specific syntax.

## Why devrunner

- Auto-detects 20+ runners across Node.js, Python, Rust, Go, Java, Ruby, PHP, .NET, Elixir, Swift, Zig, and Make.
- Works from nested directories by searching parent directories for project markers.
- Monorepo-aware for Node workspaces: prefers root lockfile manager (pnpm/yarn/bun/npm lock) over leaf `package.json` fallback.
- Handles lockfile/tool conflicts and returns CI-safe exit codes.
- Includes shell completions and optional background self-update.

## Installation

### Mac/Linux

```bash
curl -fsSL https://raw.githubusercontent.com/princepal9120/devrunner/main/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/princepal9120/devrunner/main/install.ps1 | iex
```

### From source

```bash
cargo install devrunner
```

## Usage

Basic syntax:

```bash
devrunner <command> [args...] [flags] [-- extra-args]
```

Examples:

```bash
# Run tests with detected tool
devrunner test

# Run from nested directory (searches parent folders)
devrunner build

# In monorepos, prefers workspace lockfile tool over leaf package fallback
devrunner test --levels=5

# Increase search depth
devrunner lint --levels=5

# Ignore specific tools
devrunner start --ignore=npm,yarn

# Print command without executing
devrunner test --dry-run

# Pass raw arguments through to underlying runner
devrunner test -- --coverage --verbose

# Force blocking update
devrunner --update

# Inspect detection behavior
devrunner why

# Diagnose tool installation and conflicts
devrunner doctor

# List available scripts/targets for supported project types
devrunner list
```

## Supported Ecosystems

| Ecosystem | Detected tools |
| --- | --- |
| JavaScript/TypeScript | bun, pnpm, yarn, npm |
| Python | uv, poetry, pipenv, pip |
| Rust | cargo |
| Go | task, go |
| Java | gradle, maven |
| Ruby | bundler, rake |
| PHP | composer |
| .NET | dotnet |
| Elixir | mix |
| Swift | swift |
| Zig | zig |
| Generic | make |

## Configuration

Config precedence is:

1. Built-in defaults
2. Global config: `~/.config/run/config.toml`
3. Local config: `./run.toml`
4. CLI flags

Example global/local config:

```toml
max_levels = 5
auto_update = true
ignore_tools = ["npm"]
verbose = false
quiet = false
show_timing = false

[aliases]
t = "test"
b = "build"
```

## Exit Codes

- `0`: Success
- `1`: Generic CLI/runtime error
- `2`: Runner not found
- `3`: Lockfile conflict
- `127`: Detected tool is not installed

## Shell Completions

Generate completion files:

```bash
devrunner completions bash > ~/.local/share/bash-completion/completions/devrunner
devrunner completions zsh > ~/.zsh/completion/_devrunner
devrunner completions fish > ~/.config/fish/completions/devrunner.fish
devrunner completions powershell > _devrunner.ps1
```

For zsh, ensure your `fpath` includes `~/.zsh/completion` and run `compinit`.

## Local Development

Run the same checks as CI:

```bash
./scripts/pre-push.sh
```

It runs formatting, clippy, tests, and security audit (when `cargo-audit` is installed).

## Release Artifacts

Tag pushes matching `v*` trigger cross-platform builds and GitHub release publishing via `.github/workflows/ci.yml`.

## Changelog

See `CHANGELOG.md` for unreleased and released changes.

## License

Licensed under AGPL-3.0. See `LICENSE`.
