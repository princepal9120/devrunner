# devrunner

**One command. Any project. Zero configuration.**

[![CI](https://github.com/verseles/devrunner/workflows/CI/badge.svg)](https://github.com/verseles/devrunner/actions)
[![Release](https://img.shields.io/github/v/release/verseles/devrunner)](https://github.com/verseles/devrunner/releases)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue)](LICENSE)

```
devrunner test
```

That's it. Whether your project uses npm, yarn, pnpm, bun, cargo, poetry, gradle, or any of 20+ other tools — `devrunner` figures it out.

## Why?

Every project has its own package manager. Every time you switch projects, you ask yourself:

> "Was this npm or yarn? pnpm? Does it have a Makefile?"

**devrunner** eliminates this friction. Just type `devrunner <command>` and it works.

## Install

```bash
# Linux/macOS
curl -fsSL install.cat/verseles/devrunner | bash

# Windows (PowerShell)
irm install.cat/verseles/devrunner | iex

# Or via Cargo
cargo install devrunner-cli
```

## Usage

```bash
devrunner test              # Runs test with detected tool
devrunner build             # Runs build
devrunner lint              # Runs lint
devrunner dev               # Runs dev server

# Pass arguments after --
devrunner test -- --coverage --watch

# Works from any subdirectory
cd src/components && devrunner test    # Finds package.json in parent dirs
```

## Supported Tools

| Ecosystem | Tools (priority order) |
|-----------|----------------------|
| **Monorepo** | nx → turbo → lerna |
| **Node.js** | bun → pnpm → yarn → npm |
| **Deno** | deno |
| **Python** | uv → poetry → pipenv → pip |
| **Rust** | cargo |
| **PHP** | composer |
| **Go** | task → go |
| **Ruby** | bundler → rake |
| **Java** | gradle → maven |
| **.NET** | dotnet |
| **Elixir** | mix |
| **Swift** | swift (SPM) |
| **Zig** | zig |
| **Generic** | just → make |

Detection is based on lockfiles first (more specific), then manifest files.

## Options

```bash
devrunner test --dry-devrunner         # Show command without executing
devrunner test --verbose         # Show detection details
devrunner test --quiet           # Suppress output except errors
devrunner test --levels=5        # Search up to 5 parent directories (default: 3)
devrunner test --ignore=npm,yarn # Skip specific runners
devrunner --update               # Force update check
```

## Configuration

Create `~/.config/devrunner/config.toml` for global settings:

```toml
max_levels = 5
auto_update = true
ignore_tools = ["npm"]

# Advanced update settings (optional)
[update]
enabled = true              # Enable auto-update (default: true)
check_interval_hours = 2    # Hours between update checks (default: 2)
```

Or `devrunner.toml` in your project for local overrides.

**Precedence:** CLI args > local config > global config > defaults

## Conflict Resolution

When multiple lockfiles exist (e.g., `package-lock.json` + `yarn.lock`):

1. **Corepack** — If `package.json` has a `packageManager` field, uses that tool
2. If only one tool is installed → uses it with a warning
3. If multiple tools installed → error with suggested action
4. If no tools installed → shows installation instructions

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (passes through original exit code) |
| 1 | Generic error |
| 2 | No runner found |
| 3 | Lockfile conflict |
| 127 | Tool not installed |

## Auto-Update

Updates happen silently in the background after commands complete (every 2 hours by default). 

Disable with:
- Environment variable: `RUN_NO_UPDATE=1`
- Legacy config: `auto_update = false`
- New config section: `[update] enabled = false`

## Shell Completions

```bash
# Bash
devrunner completions bash > ~/.local/share/bash-completion/completions/devrunner

# Zsh
devrunner completions zsh > ~/.zsh/completion/_run

# Fish
devrunner completions fish > ~/.config/fish/completions/devrunner.fish

# PowerShell
devrunner completions powershell >> $PROFILE
```

## Development

```bash
git clone https://github.com/verseles/devrunner.git
cd devrunner
make precommit   # Format, lint, test, audit
cargo build --release
```

## License

AGPL-3.0. See [LICENSE](LICENSE).

---

Made with mass production by [Verseles](https://github.com/verseles)
