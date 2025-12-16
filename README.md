# 🚀 devrunner

> **Universal task runner for modern development.**
> Detects your project's tools (npm, cargo, maven, etc.) and runs commands automatically.


```
 ██████╗ ███████╗██╗   ██╗██████╗ ██╗   ██╗███╗   ██╗███╗   ██╗███████╗██████╗ 
 ██╔══██╗██╔════╝██║   ██║██╔══██╗██║   ██║████╗  ██║████╗  ██║██╔════╝██╔══██╗
 ██║  ██║█████╗  ██║   ██║██████╔╝██║   ██║██╔██╗ ██║██╔██╗ ██║█████╗  ██████╔╝
 ██║  ██║██╔══╝  ╚██╗ ██╔╝██╔══██╗██║   ██║██║╚██╗██║██║╚██╗██║██╔══╝  ██╔══██╗
 ██████╔╝███████╗ ╚████╔╝ ██║  ██║╚██████╔╝██║ ╚████║██║ ╚████║███████╗██║  ██║
 ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝
 
                     Universal Task Runner

```

**devrunner** eliminates the mental overhead of switching between languages and project structures. Stop remembering if it's `npm run`, `yarn run`, `cargo run`, `mvn exec`, or `python -m`. Just type `devrunner run`.

## ✨ Why devrunner?

- 🧠 **Smart Detection**: Automatically identifies 20+ build tools (Node.js, Rust, Python, Go, Java, PHP, etc.).
- � **Recursive Power**: Run commands from *any* subdirectory; `devrunner` finds the root config.
- ⚡ **Blazing Fast**: Written in Rust. Cold start < 50ms.
- 🔧 **Zero Config**: No setup required. It just works.
- 🆕 **Self-Updating**: Keeps itself up to date silently in the background.

## 📦 Installation

### Quick Install (Mac/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/princepal9120/devrunner/main/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/princepal9120/devrunner/main/install.ps1 | iex
```

### From Source (Rust)

```bash
cargo install devrunner
```

## 🚀 Usage

Basic syntax: `devrunner <command> [args...]`

| Goal | Traditional Command | With devrunner |
| :--- | :--- | :--- |
| **Run Tests** | `npm test` OR `cargo test` OR `go test` | `devrunner test` |
| **Build App** | `npm run build` OR `cargo build` | `devrunner build` |
| **Start Server** | `npm start` OR `python manage.py runserver` | `devrunner start` |
| **Lint Code** | `npm run lint` OR `cargo clippy` | `devrunner lint` |

### Passing Arguments
Use `--` to pass flags directly to the underlying tool:
```bash
# equivalent to: npm run test -- --verbose
devrunner test -- --verbose
```

## 🎯 Supported Ecosystems

| Language | Tools Detected |
| :--- | :--- |
| **JavaScript/TS** | `npm`, `yarn`, `pnpm`, `bun` |
| **Rust** | `cargo` |
| **Python** | `pip`, `poetry`, `pipenv`, `uv` |
| **Go** | `go mod`, `task` |
| **Java** | `maven`, `gradle` |
| **PHP** | `composer` |
| **Ruby** | `bundler`, `rake` |
| **.NET** | `dotnet` |
| **Others** | `make`, `zig`, `swift`, `elixir` |

## ⚙️ Configuration (Optional)

You can configure global preferences in `~/.config/devrunner/config.toml` or per-project in `devrunner.toml`.

```toml
[config]
auto_update = true      # Enable/disable background updates
verbose = false         # Show detailed detection logs
ignore_tools = ["npm"]  # Tools to skip during detection
```

## 🐚 Shell Architecture

Enable tab completions for your shell:

```bash
# Zsh
devrunner completions zsh > ~/.zsh/completion/_devrunner

# Bash
devrunner completions bash > ~/.local/share/bash-completion/completions/devrunner

# Fish
devrunner completions fish > ~/.config/fish/completions/devrunner.fish
```

## 🤝 Contributing

We love contributors! 
1. Fork the repo.
2. Clone it: `git clone https://github.com/princepal9120/devrunner.git`
3. Create a branch: `git checkout -b feature/cool-thing`
4. Submit a PR.

## 📄 License

Licensed under **AGPL-3.0**. See [LICENSE](LICENSE) for details.

---
*Built with ❤️ for developers who value their keystrokes.*