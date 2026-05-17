# Copyright (C) 2025 princepal9120
# SPDX-License-Identifier: AGPL-3.0

.PHONY: precommit fmt clippy test audit build release clean

# devrunner all CI checks locally before committing
precommit: fmt clippy test audit
	@echo ""
	@echo "✅ All checks passed!"
	@echo ""

# Check formatting
fmt:
	@echo "📝 Checking formatting..."
	@cargo fmt --check
	@echo "✓ Formatting OK"
	@echo ""

# devrunner Clippy linter
clippy:
	@echo "🔬 Running Clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✓ Clippy OK"
	@echo ""

# devrunner tests
test:
	@echo "🧪 Running tests..."
	@cargo test --all-features
	@echo "✓ Tests OK"
	@echo ""

# Security audit (skips if cargo-audit not installed)
audit:
	@echo "🔒 Running security audit..."
	@if command -v cargo-audit >/dev/null 2>&1; then \
		cargo audit && echo "✓ Security audit OK"; \
	else \
		echo "⚠ cargo-audit not installed, skipping security audit"; \
		echo "  Install with: cargo install cargo-audit"; \
	fi
	@echo ""

# Build debug version
build:
	@echo "🔨 Building debug..."
	@cargo build

# Build release version
release:
	@echo "🚀 Building release..."
	@cargo build --release

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@cargo clean
