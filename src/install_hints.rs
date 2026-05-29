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

//! Actionable install hints for tools detected by devrunner.
//!
//! When a tool is detected (e.g. via `Cargo.toml`) but not installed on the
//! user's system, this module provides OS-aware installation instructions so
//! users immediately know how to fix the problem.

/// Returns an OS-aware install hint for the given tool name, or `None` if
/// no hint is available.
pub fn get_install_hint(tool: &str) -> Option<InstallHint> {
    let hint = match tool {
        // ── Rust ─────────────────────────────────────────────────────────
        "cargo" => InstallHint {
            tool: "cargo",
            ecosystem: "Rust",
            url: "https://rustup.rs",
            steps: os_steps!(
                macos: &[
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                    "source ~/.cargo/env   # or restart your terminal",
                ],
                linux: &[
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                    "source ~/.cargo/env   # or restart your terminal",
                ],
                windows: &[
                    "# Download and run: https://win.rustup.rs/x86_64",
                    "# Or via winget: winget install Rustlang.Rustup",
                ]
            ),
        },

        // ── Node.js ecosystem ─────────────────────────────────────────────
        "npm" | "node" => InstallHint {
            tool: "npm / node",
            ecosystem: "Node.js",
            url: "https://nodejs.org",
            steps: os_steps!(
                macos: &[
                    "brew install node",
                    "# Or use nvm: curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash",
                    "#              nvm install --lts",
                ],
                linux: &[
                    "# Via nvm (recommended):",
                    "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash",
                    "nvm install --lts",
                    "# Or via apt: sudo apt install nodejs npm",
                ],
                windows: &[
                    "winget install OpenJS.NodeJS.LTS",
                    "# Or download from https://nodejs.org",
                ]
            ),
        },
        "yarn" => InstallHint {
            tool: "yarn",
            ecosystem: "Node.js",
            url: "https://yarnpkg.com",
            steps: os_steps!(
                macos: &["npm install -g yarn", "# Or: brew install yarn"],
                linux: &["npm install -g yarn"],
                windows: &["npm install -g yarn", "# Or: winget install Yarn.Yarn"]
            ),
        },
        "pnpm" => InstallHint {
            tool: "pnpm",
            ecosystem: "Node.js",
            url: "https://pnpm.io",
            steps: os_steps!(
                macos: &[
                    "npm install -g pnpm",
                    "# Or: brew install pnpm",
                    "# Or: curl -fsSL https://get.pnpm.io/install.sh | sh -",
                ],
                linux: &[
                    "npm install -g pnpm",
                    "# Or: curl -fsSL https://get.pnpm.io/install.sh | sh -",
                ],
                windows: &[
                    "npm install -g pnpm",
                    "# Or via winget: winget install pnpm.pnpm",
                ]
            ),
        },
        "bun" => InstallHint {
            tool: "bun",
            ecosystem: "Node.js / Bun",
            url: "https://bun.sh",
            steps: os_steps!(
                macos: &[
                    "curl -fsSL https://bun.sh/install | bash",
                    "# Or: brew install oven-sh/bun/bun",
                ],
                linux: &["curl -fsSL https://bun.sh/install | bash"],
                windows: &[r#"powershell -c "irm bun.sh/install.ps1 | iex""#]
            ),
        },

        // ── Deno ──────────────────────────────────────────────────────────
        "deno" => InstallHint {
            tool: "deno",
            ecosystem: "Deno",
            url: "https://deno.land",
            steps: os_steps!(
                macos: &[
                    "curl -fsSL https://deno.land/install.sh | sh",
                    "# Or: brew install deno",
                ],
                linux: &["curl -fsSL https://deno.land/install.sh | sh"],
                windows: &[r#"irm https://deno.land/install.ps1 | iex"#]
            ),
        },

        // ── Python ecosystem ──────────────────────────────────────────────
        "python" | "python3" | "pip" | "pip3" => InstallHint {
            tool: "python / pip",
            ecosystem: "Python",
            url: "https://www.python.org",
            steps: os_steps!(
                macos: &[
                    "brew install python",
                    "# python3 and pip3 are included",
                ],
                linux: &[
                    "sudo apt install python3 python3-pip   # Debian/Ubuntu",
                    "# Or: sudo dnf install python3 python3-pip   # Fedora/RHEL",
                ],
                windows: &["winget install Python.Python.3"]
            ),
        },
        "uv" => InstallHint {
            tool: "uv",
            ecosystem: "Python",
            url: "https://docs.astral.sh/uv",
            steps: os_steps!(
                macos: &["curl -LsSf https://astral.sh/uv/install.sh | sh"],
                linux: &["curl -LsSf https://astral.sh/uv/install.sh | sh"],
                windows: &[r#"powershell -c "irm https://astral.sh/uv/install.ps1 | iex""#]
            ),
        },
        "poetry" => InstallHint {
            tool: "poetry",
            ecosystem: "Python",
            url: "https://python-poetry.org",
            steps: os_steps!(
                macos: &[
                    "curl -sSL https://install.python-poetry.org | python3 -",
                    "# Or: brew install poetry",
                ],
                linux: &["curl -sSL https://install.python-poetry.org | python3 -"],
                windows: &[
                    r#"(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | python -"#,
                ]
            ),
        },
        "pipenv" => InstallHint {
            tool: "pipenv",
            ecosystem: "Python",
            url: "https://pipenv.pypa.io",
            steps: os_steps!(
                macos: &["brew install pipenv", "# Or: pip install pipenv"],
                linux: &["pip install pipenv", "# Or: sudo apt install pipenv"],
                windows: &["pip install pipenv"]
            ),
        },

        // ── Go ────────────────────────────────────────────────────────────
        "go" => InstallHint {
            tool: "go",
            ecosystem: "Go",
            url: "https://go.dev/dl",
            steps: os_steps!(
                macos: &["brew install go"],
                linux: &[
                    "# Download from https://go.dev/dl and extract to /usr/local",
                    "wget https://go.dev/dl/go1.22.0.linux-amd64.tar.gz",
                    "sudo tar -C /usr/local -xzf go1.22.0.linux-amd64.tar.gz",
                    r#"echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile"#,
                ],
                windows: &["winget install GoLang.Go"]
            ),
        },
        "task" => InstallHint {
            tool: "task",
            ecosystem: "Go / Task",
            url: "https://taskfile.dev",
            steps: os_steps!(
                macos: &["brew install go-task/tap/go-task"],
                linux: &["sh -c \"$(curl --location https://taskfile.dev/install.sh)\" -- -d -b ~/.local/bin"],
                windows: &["winget install Task.Task"]
            ),
        },

        // ── Java ecosystem ────────────────────────────────────────────────
        "gradle" => InstallHint {
            tool: "gradle",
            ecosystem: "Java / Kotlin",
            url: "https://gradle.org/install",
            steps: os_steps!(
                macos: &["brew install gradle"],
                linux: &[
                    "sdk install gradle   # via SDKMAN: https://sdkman.io",
                    "# Or: sudo apt install gradle",
                ],
                windows: &["winget install Gradle.Gradle"]
            ),
        },
        "mvn" => InstallHint {
            tool: "mvn (Maven)",
            ecosystem: "Java",
            url: "https://maven.apache.org",
            steps: os_steps!(
                macos: &["brew install maven"],
                linux: &[
                    "sudo apt install maven   # Debian/Ubuntu",
                    "# Or: sdk install maven  # via SDKMAN",
                ],
                windows: &["winget install Apache.Maven"]
            ),
        },

        // ── PHP ───────────────────────────────────────────────────────────
        "composer" => InstallHint {
            tool: "composer",
            ecosystem: "PHP",
            url: "https://getcomposer.org",
            steps: os_steps!(
                macos: &["brew install composer"],
                linux: &[
                    "curl -sS https://getcomposer.org/installer | php",
                    "sudo mv composer.phar /usr/local/bin/composer",
                ],
                windows: &["# Download Composer-Setup.exe from https://getcomposer.org/download"]
            ),
        },

        // ── Ruby ecosystem ────────────────────────────────────────────────
        "ruby" | "bundler" | "bundle" | "rake" => InstallHint {
            tool: "ruby / bundler / rake",
            ecosystem: "Ruby",
            url: "https://www.ruby-lang.org",
            steps: os_steps!(
                macos: &[
                    "brew install ruby",
                    "gem install bundler rake",
                ],
                linux: &[
                    "sudo apt install ruby ruby-dev   # Debian/Ubuntu",
                    "gem install bundler rake",
                ],
                windows: &[
                    "winget install RubyInstallerTeam.RubyWithDevKit",
                    "gem install bundler rake",
                ]
            ),
        },

        // ── .NET ──────────────────────────────────────────────────────────
        "dotnet" => InstallHint {
            tool: "dotnet",
            ecosystem: ".NET",
            url: "https://dotnet.microsoft.com/download",
            steps: os_steps!(
                macos: &[
                    "brew install dotnet",
                    "# Or download from https://dotnet.microsoft.com/download",
                ],
                linux: &[
                    "# Ubuntu/Debian:",
                    "sudo apt install dotnet-sdk-8.0",
                    "# Or use the install script: https://dot.net/v1/dotnet-install.sh",
                ],
                windows: &["winget install Microsoft.DotNet.SDK.8"]
            ),
        },

        // ── Elixir ───────────────────────────────────────────────────────
        "mix" | "elixir" => InstallHint {
            tool: "elixir / mix",
            ecosystem: "Elixir",
            url: "https://elixir-lang.org/install.html",
            steps: os_steps!(
                macos: &["brew install elixir"],
                linux: &[
                    "sudo apt install elixir   # Debian/Ubuntu",
                    "# Or via asdf: asdf plugin add elixir && asdf install elixir latest",
                ],
                windows: &["winget install Elixir.Elixir"]
            ),
        },

        // ── Swift ─────────────────────────────────────────────────────────
        "swift" => InstallHint {
            tool: "swift",
            ecosystem: "Swift",
            url: "https://swift.org/download",
            steps: os_steps!(
                macos: &[
                    "xcode-select --install   # installs Swift via Xcode Command Line Tools",
                    "# Or install full Xcode from the App Store",
                ],
                linux: &[
                    "# Install swiftly: https://swift.org/install/linux",
                    "curl -L https://swift.org/install/swiftly-install.sh | bash",
                ],
                windows: &["# Download from https://swift.org/download (experimental Windows support)"]
            ),
        },

        // ── Zig ───────────────────────────────────────────────────────────
        "zig" => InstallHint {
            tool: "zig",
            ecosystem: "Zig",
            url: "https://ziglang.org/download",
            steps: os_steps!(
                macos: &["brew install zig"],
                linux: &[
                    "snap install zig --classic --beta",
                    "# Or download a tarball from https://ziglang.org/download",
                ],
                windows: &["winget install zig.zig"]
            ),
        },

        // ── Build tools ───────────────────────────────────────────────────
        "make" => InstallHint {
            tool: "make",
            ecosystem: "Build Tools",
            url: "https://www.gnu.org/software/make",
            steps: os_steps!(
                macos: &["xcode-select --install   # includes make"],
                linux: &["sudo apt install make   # Debian/Ubuntu", "# Or: sudo dnf install make   # Fedora/RHEL"],
                windows: &[
                    "winget install GnuWin32.Make",
                    "# Or install via WSL: sudo apt install make",
                ]
            ),
        },
        "just" => InstallHint {
            tool: "just",
            ecosystem: "Build Tools",
            url: "https://just.systems",
            steps: os_steps!(
                macos: &["brew install just"],
                linux: &[
                    "cargo install just   # requires cargo",
                    "# Or: curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/.local/bin",
                ],
                windows: &["winget install Casey.Just"]
            ),
        },

        // ── Monorepo tools ────────────────────────────────────────────────
        "nx" => InstallHint {
            tool: "nx",
            ecosystem: "Monorepo (Node.js)",
            url: "https://nx.dev",
            steps: os_steps!(
                macos: &["npm install -g nx"],
                linux: &["npm install -g nx"],
                windows: &["npm install -g nx"]
            ),
        },
        "turbo" => InstallHint {
            tool: "turbo",
            ecosystem: "Monorepo (Node.js)",
            url: "https://turbo.build",
            steps: os_steps!(
                macos: &["npm install -g turbo"],
                linux: &["npm install -g turbo"],
                windows: &["npm install -g turbo"]
            ),
        },
        "lerna" => InstallHint {
            tool: "lerna",
            ecosystem: "Monorepo (Node.js)",
            url: "https://lerna.js.org",
            steps: os_steps!(
                macos: &["npm install -g lerna"],
                linux: &["npm install -g lerna"],
                windows: &["npm install -g lerna"]
            ),
        },

        // Unknown tool — no hint available
        _ => return None,
    };

    Some(hint)
}

/// Print a formatted install hint to stderr.
/// Call this right after printing a `ToolNotInstalled` error.
///
/// If neither mise nor proto is installed, also suggests installing mise as a
/// universal tool manager so future missing tools are handled automatically.
pub fn print_install_hint(tool: &str) {
    if let Some(hint) = get_install_hint(tool) {
        hint.print();
    } else {
        // Generic fallback for tools not in the registry
        eprintln!();
        eprintln!(
            "💡 Search online for how to install '{}', then re-run devrunner.",
            tool
        );
    }

    // If neither mise nor proto is installed, suggest mise as a one-time setup
    // that makes ALL future missing tools auto-install transparently.
    let has_mise = which::which("mise").is_ok();
    let has_proto = which::which("proto").is_ok();

    if !has_mise && !has_proto {
        use owo_colors::OwoColorize;
        let colors_off = crate::output::colors_disabled();

        eprintln!();
        if colors_off {
            eprintln!("⚡ Zero-install tip: Install mise once and devrunner will auto-install");
            eprintln!("   any missing tools for you — no more manual installs:");
            eprintln!();
            eprintln!("   curl https://mise.run | sh    # https://mise.jdx.dev");
        } else {
            eprintln!(
                "{}  {} Install {} once and devrunner will auto-install",
                "⚡".yellow(),
                "Zero-install tip:".bold(),
                "mise".cyan().bold()
            );
            eprintln!("   any missing tools for you automatically — no more manual installs:");
            eprintln!();
            eprintln!("   {}", "curl https://mise.run | sh".green());
            eprintln!("   {}", "# https://mise.jdx.dev".dimmed());
        }
    }
}

/// Structured install hint for a tool.
pub struct InstallHint {
    /// Display name of the tool (e.g. "cargo", "npm / node")
    pub tool: &'static str,
    /// Human-readable ecosystem name (e.g. "Rust", "Node.js")
    pub ecosystem: &'static str,
    /// Official website / download URL
    pub url: &'static str,
    /// OS-specific install steps (lines to display)
    pub steps: &'static [&'static str],
}

impl InstallHint {
    /// Print the hint to stderr in a friendly, colorized format.
    pub fn print(&self) {
        use owo_colors::OwoColorize;

        let colors_off = crate::output::colors_disabled();

        eprintln!();
        if colors_off {
            eprintln!("💡 To install {} ({}):", self.tool, self.ecosystem);
        } else {
            eprintln!(
                "{}  To install {} ({}):",
                "💡".yellow(),
                self.tool.bold(),
                self.ecosystem.cyan()
            );
        }
        eprintln!();
        for step in self.steps {
            if colors_off {
                eprintln!("   {}", step);
            } else {
                eprintln!("   {}", step.green());
            }
        }
        eprintln!();
        if colors_off {
            eprintln!("   More info: {}", self.url);
        } else {
            eprintln!("   More info: {}", self.url.underline());
        }
    }
}

/// Macro to produce OS-aware step slices at compile time.
/// Expands to the right `&[&str]` for the current target OS.
macro_rules! os_steps {
    (macos: $mac:expr, linux: $linux:expr, windows: $win:expr) => {{
        #[cfg(target_os = "macos")]
        {
            $mac
        }
        #[cfg(target_os = "linux")]
        {
            $linux
        }
        #[cfg(target_os = "windows")]
        {
            $win
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            $linux
        } // fallback: use Linux instructions on unknown OS
    }};
}
use os_steps;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_hint_exists() {
        let hint = get_install_hint("cargo");
        assert!(hint.is_some(), "cargo should have an install hint");
        let h = hint.unwrap();
        assert_eq!(h.ecosystem, "Rust");
        assert!(!h.steps.is_empty());
        assert!(!h.url.is_empty());
    }

    #[test]
    fn test_npm_hint_exists() {
        let hint = get_install_hint("npm");
        assert!(hint.is_some());
        assert_eq!(hint.unwrap().ecosystem, "Node.js");
    }

    #[test]
    fn test_node_alias_hint_exists() {
        // "node" should map to the same Node.js hint as "npm"
        let hint = get_install_hint("node");
        assert!(hint.is_some());
    }

    #[test]
    fn test_all_supported_tools_have_hints() {
        let tools = [
            "cargo", "npm", "node", "yarn", "pnpm", "bun", "deno", "python", "python3", "pip",
            "pip3", "uv", "poetry", "pipenv", "go", "task", "gradle", "mvn", "composer", "ruby",
            "bundler", "bundle", "rake", "dotnet", "mix", "elixir", "swift", "zig", "make", "just",
            "nx", "turbo", "lerna",
        ];
        for tool in &tools {
            assert!(
                get_install_hint(tool).is_some(),
                "Missing install hint for tool: {}",
                tool
            );
        }
    }

    #[test]
    fn test_unknown_tool_returns_none() {
        assert!(get_install_hint("nonexistent-tool-xyz").is_none());
        assert!(get_install_hint("").is_none());
    }

    #[test]
    fn test_hint_has_non_empty_steps() {
        let tools = ["cargo", "npm", "pnpm", "bun", "uv", "go", "make"];
        for tool in &tools {
            let hint = get_install_hint(tool).unwrap();
            assert!(!hint.steps.is_empty(), "Tool '{}' has empty steps", tool);
        }
    }

    #[test]
    fn test_hint_has_valid_url() {
        let tools = ["cargo", "npm", "deno", "go", "zig"];
        for tool in &tools {
            let hint = get_install_hint(tool).unwrap();
            assert!(
                hint.url.starts_with("https://"),
                "Tool '{}' URL should start with https://",
                tool
            );
        }
    }
}
