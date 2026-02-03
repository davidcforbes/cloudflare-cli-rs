.PHONY: help check fmt clippy test build clean install-tools quality-check

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

quality-check: ## Run all quality checks (format, clippy, tests, markdown)
	@echo "Running quality checks..."
	@cargo fmt --all -- --check
	@cargo clippy --all-targets --all-features -- -D warnings
	@cargo test --quiet
	@markdownlint "**/*.md" --ignore node_modules --ignore target || true
	@echo "✓ Quality checks complete"

fmt: ## Format code
	cargo fmt --all

clippy: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

test: ## Run tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test --verbose

build: ## Build debug binary
	cargo build

build-release: ## Build release binary
	cargo build --release

clean: ## Clean build artifacts
	cargo clean

install-tools: ## Install development tools
	@echo "Installing Rust tools..."
	@rustup component add clippy rustfmt
	@echo "Installing markdownlint (requires Node.js)..."
	@npm install -g markdownlint-cli || echo "Node.js not installed, skipping markdownlint"
	@echo "✓ Tools installed"

check: quality-check ## Alias for quality-check
