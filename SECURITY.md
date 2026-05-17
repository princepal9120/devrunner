# Security Policy

`devrunner` executes local development commands, reads project configuration, and performs optional update checks. Please report behavior that could cause unexpected command execution, unsafe path handling, supply-chain exposure, or credential disclosure.

## Supported versions

Security fixes are targeted at the latest released version and the `main` branch. If you are using an older release, upgrade before reporting unless the issue still reproduces on `main`.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting for this repository when available. If it is not available, contact the maintainers through the repository owner's GitHub profile and share only a high-level summary until a private channel is established.

Helpful details:

- Affected version or commit.
- Operating system and shell.
- Minimal project layout or config that reproduces the issue.
- Expected behavior and actual behavior.
- Whether the issue can trigger command execution or data exposure.

## Scope

In scope:

- Command construction or argument handling vulnerabilities.
- Path traversal or unsafe file handling.
- Update-check behavior that could leak sensitive data or trust the wrong source.
- Installation script issues that alter unexpected files.

Out of scope:

- Behavior that requires already running arbitrary local code.
- Vulnerabilities in third-party package managers invoked by user projects.
- Denial-of-service cases that only affect a local development command.
