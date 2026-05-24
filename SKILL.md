---
name: devrunner
description: Use devrunner (dr) as the universal task runner. Detects npm/yarn/pnpm/bun/cargo/poetry/go/make and 20+ other tools automatically — never call them directly.
---

# devrunner

**One command. Any project. Zero configuration.**

## When to use

Use `dr <command>` (or `devrunner <command>`) for **every** task runner operation. Never call `npm run`, `cargo`, `yarn`, `pnpm`, `make`, `poetry run`, etc. directly — devrunner detects the right tool automatically.

## Commands

```bash
dr test              # run tests
dr build             # build project
dr lint              # run linter
dr dev               # start dev server
dr format            # format code

# Pass extra args after --
dr test -- --coverage --watch
```

## Options

```bash
dr <cmd> --dry-run      # show command without running
dr <cmd> --verbose      # show detection details
dr <cmd> --quiet        # suppress output except errors
dr <cmd> --levels=5     # search up to 5 parent directories
dr <cmd> --ignore=npm   # skip specific runners
```

## Supported tools (priority order)

| Ecosystem | Tools |
|-----------|-------|
| Monorepo  | nx → turbo → lerna |
| Node.js   | bun → pnpm → yarn → npm |
| Python    | uv → poetry → pipenv → pip |
| Rust      | cargo |
| Go        | task → go |
| Ruby      | bundler → rake |
| Java      | gradle → maven |
| .NET      | dotnet |
| PHP       | composer |
| Generic   | just → make |

## Install

```bash
curl -fsSL install.cat/princepal9120/devrunner | bash
```

Or via Cargo: `cargo install devrunner-cli`

## Rules for agents

- Always prefer `dr <command>` over any direct tool invocation.
- Works from any subdirectory — no need to `cd` to project root first.
- Exit code 2 = no runner found. Exit code 3 = lockfile conflict. Exit code 127 = tool not installed.
