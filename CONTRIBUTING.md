# Contributing to CFAD (Cloudflare Admin CLI)

Thank you for your interest in contributing to CFAD! This document
provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a Code of Conduct (see CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid
duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, cfad version)
- **Error messages** or logs if applicable

Use the bug report template when creating a new issue.

### Suggesting Features

Feature suggestions are welcome! When suggesting a feature:

- **Explain the use case** - why is this feature needed?
- **Describe the solution** - what would you like to happen?
- **Consider alternatives** - have you thought of other approaches?

Use the feature request template when creating a new issue.

### Pull Requests

1. **Fork** the repository and create your branch from `main`
2. **Make your changes** with clear, focused commits
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Run tests** - ensure all tests pass (`cargo test`)
6. **Run clippy** - ensure no warnings (`cargo clippy -- -D warnings`)
7. **Format code** - run `cargo fmt`
8. **Submit PR** with a clear description

#### Pull Request Guidelines

- **One feature per PR** - keep PRs focused
- **Write good commit messages** - use conventional commits format
- **Include tests** - minimum 80% coverage for new code
- **Update CHANGELOG.md** - add entry under "Unreleased"
- **Reference issues** - link related issues in PR description

## Development Setup

### Prerequisites

- **Rust** 1.70 or later ([rustup.rs](https://rustup.rs))
- **Git** for version control
- **Cloudflare account** with API token (for testing against real API)

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/cloudflare-cli-rs.git
cd cloudflare-cli-rs

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build release binary
cargo build --release
```

### Project Structure

```text
cloudflare-cli-rs/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports for testing
│   ├── api/             # API type definitions
│   ├── cli/             # CLI command definitions
│   ├── client/          # HTTP client and auth
│   ├── config/          # Configuration management
│   ├── error/           # Error types
│   ├── ops/             # API operations (DNS, Zone, Cache)
│   ├── output/          # Output formatting
│   └── utils/           # Utility functions
├── tests/
│   ├── integration/     # Integration tests with wiremock
│   └── common/          # Test helpers
├── docs/
│   └── examples/        # Example files (CSV, BIND)
└── Cargo.toml
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Writing Tests

- **Unit tests** - Add `#[cfg(test)]` modules in source files
- **Integration tests** - Use `tests/integration/` directory
- **Use wiremock** - Mock Cloudflare API responses
- **Test coverage** - Aim for 80%+ on new code

Example unit test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_valid() {
        let csv = "type,name,content\nA,www,203.0.113.1";
        let records = parse_csv_format(csv).unwrap();
        assert_eq!(records.len(), 1);
    }
}
```

Example integration test:

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_list_zones() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(/* ... */))
        .mount(&mock_server)
        .await;

    // Test implementation
}
```

## Code Style

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (enforced in CI)
- Use `cargo clippy` for linting (no warnings allowed)
- Write clear, self-documenting code
- Add comments for complex logic only

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build/tooling changes

Examples:

```text
feat(dns): add BIND zone file import support
fix(client): handle rate limiting correctly
docs(readme): update installation instructions
test(dns): add CSV parser edge case tests
```

## Documentation

### Code Documentation

- **Public APIs** - Must have doc comments (`///`)
- **Examples** - Include usage examples in doc comments
- **Modules** - Add module-level documentation

```rust
/// Lists DNS records for a zone.
///
/// # Arguments
///
/// * `client` - Authenticated Cloudflare client
/// * `zone_id` - Zone identifier
/// * `record_type` - Optional filter by record type (e.g., "A", "CNAME")
///
/// # Example
///
/// ```
/// let records = list_records(&client, "zone123", Some("A")).await?;
/// ```
pub async fn list_records(
    client: &CloudflareClient,
    zone_id: &str,
    record_type: Option<&str>,
) -> Result<Vec<DnsRecord>> {
    // Implementation
}
```

### User Documentation

- Update **README.md** for user-facing changes
- Add examples to **docs/examples/** if needed
- Update **CHANGELOG.md** for all changes

## Release Process

Releases are managed by maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` - move "Unreleased" to version section
3. Commit: `chore(release): prepare v0.3.0`
4. Tag: `git tag -a v0.3.0 -m "Release v0.3.0"`
5. Push: `git push && git push --tags`
6. GitHub Actions builds and creates release

## Getting Help

- **Questions** - Open a discussion or issue
- **Chat** - (Coming soon)
- **Email** - <davidcforbes@aol.com>

## Recognition

Contributors are recognized in:

- GitHub contributors page
- Release notes
- CHANGELOG.md (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed
under the MIT License.

---

**Thank you for contributing to CFAD!**
