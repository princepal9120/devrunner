# Best practices

- Read the @README.md file to understand the project.
- Read the @AGENTS.md file to understand the best practices for this project.
- Every time you end a asked task, call the tool funcion "play_notification" to notify the user. Then make a commit. Only push when asked.

## Testing instructions

- Find the CI plan in the .github/workflows folder.
- Use `dr precommit` (or `devrunner precommit`) to run every check defined for that package.
- From the package root you can also call `make precommit` directly. The commit should pass all tests before you merge.
- Fix any test or type errors until the whole suite is green.
- After moving files or changing imports, use `dr precommit` to be sure rules still pass.
- Add or update tests for the code you change, even if nobody asked.

## Before PR

- Verify carefully if @README.md is updated or need to be updated.
- Based on last session, verify if @AGENTS.md ADR memories need to be updated or added new memories.

## PR instructions

- Title format: [<project_name>] <Title>
- Always use `dr precommit` before committing.

## Architecture Decision Records (ADR)
- **Zero-Install Toolchains (2026-05-29)**: Instead of reinventing package managers or shipping bloated binaries, devrunner integrates with `mise` and `proto` via subprocess (`mise exec` / `proto run`). This provides automatic, zero-config tool provisioning with minimal overhead (5-10ms). The `[toolchain]` section in config controls this behavior.
