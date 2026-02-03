# Development Guide

This guide covers local development setup and code quality checks.

## Quick Start

**Run all quality checks before pushing:**

```bash
# Windows PowerShell
.\scripts\quality-check.ps1

# Linux/macOS
./scripts/quality-check.sh

# Or using Make (all platforms)
make quality-check
```

## Setup Development Tools

### Required Tools

1. **Rust toolchain** (already installed)

   ```bash
   rustup component add clippy rustfmt
   ```

2. **Node.js** (for markdown linting)
   - Download from <https://nodejs.org>
   - Or use a package manager:

     ```bash
     # Windows (winget)
     winget install OpenJS.NodeJS

     # macOS
     brew install node

     # Linux
     sudo apt install nodejs npm
     ```

3. **Markdown linter**

   ```bash
   npm install -g markdownlint-cli
   ```

### Optional Tools

**For advanced complexity analysis:**

```bash
# Tokei - code statistics
cargo install tokei

# Cargo geiger - security and complexity metrics
cargo install cargo-geiger
```

## Code Quality Checks

### 1. Code Formatting

**Check formatting:**

```bash
cargo fmt --all -- --check
```

**Auto-fix formatting:**

```bash
cargo fmt --all
```

### 2. Clippy Lints

**Run clippy:**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Complexity warnings:**

- Cyclomatic complexity limit: 8
- Function length limit: 50 lines
- These are enforced by Codacy, not clippy

### 3. Tests

**Run all tests:**

```bash
cargo test

# Verbose output
cargo test --verbose

# Specific test
cargo test test_name
```

**Current test coverage:**

- 68 tests (54 unit + 14 integration)

### 4. Markdown Linting

**Check all markdown files:**

```bash
markdownlint "**/*.md" --ignore node_modules --ignore target
```

**Auto-fix markdown issues:**

```bash
markdownlint "**/*.md" --ignore node_modules --ignore target --fix
```

**Configuration:** `.markdownlint.json`

### 5. Build Verification

**Debug build:**

```bash
cargo build
```

**Release build:**

```bash
cargo build --release
```

## Complexity Analysis

### Check Cyclomatic Complexity

Codacy enforces complexity limits:

- **Cyclomatic complexity:** ≤ 8
- **Function length:** ≤ 50 lines

**Manual check with tokei:**

```bash
tokei --files src/
```

**Tips to reduce complexity:**

1. Extract methods (break large functions into smaller ones)
2. Use early returns
3. Isolate boolean logic into predicate functions
4. Apply Single Responsibility Principle

## Git Hooks (Optional)

### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
# Run quality checks before commit

echo "Running pre-commit quality checks..."

# Format check
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
    echo "Error: Code is not formatted. Run 'cargo fmt --all'"
    exit 1
fi

# Clippy
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "Error: Clippy found issues"
    exit 1
fi

# Tests
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "Error: Tests failed"
    exit 1
fi

echo "✓ Pre-commit checks passed"
```

**Make it executable:**

```bash
chmod +x .git/hooks/pre-commit
```

### Pre-push Hook

Create `.git/hooks/pre-push`:

```bash
#!/bin/sh
# Run full quality check before push

./scripts/quality-check.sh
```

**Make it executable:**

```bash
chmod +x .git/hooks/pre-push
```

## Makefile Commands

```bash
make help              # Show all available commands
make quality-check     # Run all quality checks
make fmt               # Format code
make clippy            # Run clippy
make test              # Run tests
make build             # Build debug binary
make build-release     # Build release binary
make clean             # Clean build artifacts
make install-tools     # Install development tools
```

## CI/CD Pipeline

GitHub Actions runs these checks automatically:

- **CI workflow** (`.github/workflows/ci.yml`):
  - Tests on Windows, Linux, macOS
  - Clippy lints
  - Format check
  - Code coverage

- **Release workflow** (`.github/workflows/release.yml`):
  - Multi-platform builds
  - Binary creation and checksums

## Code Quality Services

- **Codacy**: Automated code review
  - Configuration: `.codacy.yml`
  - Checks: complexity, length, markdown, security

- **Codecov**: Test coverage tracking
  - Target: Maintain current coverage
  - Integration: Automatic via CI

## Troubleshooting

### Clippy warnings won't go away

```bash
# Clean and rebuild
cargo clean
cargo clippy --all-targets --all-features -- -D warnings
```

### Markdown linting errors

Check `.markdownlint.json` for current rules. Common fixes:

- Line length: 80 characters (code blocks: 120)
- Blank lines around headings and lists
- No bare URLs (use angle brackets: `<url>`)

### Tests failing locally but passing in CI

Check Rust version:

```bash
rustc --version
rustup update
```

## Best Practices

1. **Run quality checks before every commit**
2. **Fix clippy warnings immediately**
3. **Keep functions under 50 lines**
4. **Keep cyclomatic complexity ≤ 8**
5. **Run full test suite before pushing**
6. **Use `make quality-check` as final verification**

## Editor Integration

### VS Code

Install extensions:

- rust-analyzer
- Even Better TOML
- markdownlint

Settings (`.vscode/settings.json`):

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "[rust]": {
    "editor.formatOnSave": true
  }
}
```

### IntelliJ IDEA / RustRover

- Enable Clippy: Settings → Rust → External Linters
- Enable format on save: Settings → Rust → Rustfmt

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Markdown Style Guide](https://www.markdownguide.org/)
- [Keep a Changelog](https://keepachangelog.com/)
