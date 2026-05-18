# devrunner

**One command. Any project. Zero configuration.**

[![CI](https://github.com/princepal9120/devrunner/workflows/CI/badge.svg)](https://github.com/princepal9120/devrunner/actions)
[![Release](https://img.shields.io/github/v/release/princepal9120/devrunner)](https://github.com/princepal9120/devrunner/releases)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue)](LICENSE)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen)](CONTRIBUTING.md)

```
devrunner test
# or the short alias:
dr test
```

That's it. Whether your project uses npm, yarn, pnpm, bun, cargo, poetry, gradle, or any of 20+ other tools — `devrunner` (or `dr`) figures it out.

## Why?

Every project has its own package manager. Every time you switch projects, you ask yourself:

> "Was this npm or yarn? pnpm? Does it have a Makefile?"

**devrunner** eliminates this friction. Just type `devrunner <command>` and it works.

## Install

```bash
# Linux/macOS
curl -fsSL install.cat/princepal9120/devrunner | bash

# Windows (PowerShell)
irm install.cat/princepal9120/devrunner | iex

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
devrunner completions zsh > ~/.zsh/completion/_devrunner

# Fish
devrunner completions fish > ~/.config/fish/completions/devrunner.fish

# PowerShell
devrunner completions powershell >> $PROFILE

# Alias completions are also available
dr completions bash > ~/.local/share/bash-completion/completions/dr
dr completions zsh > ~/.zsh/completion/_dr
```

## AI Agent Integration

The installer automatically configures AI coding assistants (Claude Code, OpenCode, Codex) to use `dr` as the universal task runner.

If you installed manually or want to reconfigure, add this to your AI tool's global config:

**Claude Code** (`~/.claude/CLAUDE.md`):
```markdown
## devrunner
Use `dr <command>` or `devrunner <command>` as the universal task runner.
Do NOT call npm run, cargo, make, etc. directly — use `dr <command>` instead.
Examples: `dr test`, `dr build`, `dr lint`, `dr dev`
```

**OpenCode / Codex** (`~/.config/opencode/AGENTS.md` or `~/.codex/AGENTS.md`): same content.

**npx skills** — install devrunner as an agent skill in any project:

```bash
npx skills add princepal9120/devrunner
```

This installs [`SKILL.md`](SKILL.md) into `.claude/skills/` (or `.agents/skills/`) so any agent in that project automatically knows to use `dr`.

## Development

```bash
git clone https://github.com/princepal9120/devrunner.git
cd devrunner
make precommit   # Format, lint, test, audit
cargo build --release
```

## Contributing

Contributions are welcome. Good first contributions include new ecosystem detection tests, clearer error messages, install-script fixes, and documentation improvements.

Before opening a PR, run:

```bash
make precommit
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, testing guidance, and the pull request checklist. Please also read the [Code of Conduct](CODE_OF_CONDUCT.md) and report vulnerabilities through the private process in [SECURITY.md](SECURITY.md).

## License

AGPL-3.0. See [LICENSE](LICENSE).

---

Made with mass production by [princepal9120](https://github.com/princepal9120)
