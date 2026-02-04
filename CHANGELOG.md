# Changelog

All notable changes to CFAD (Cloudflare Admin CLI) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-02-04

### Added in 0.3.0

- **D1 Database Support** - Full management of Cloudflare D1 serverless SQLite databases
  - `cfad d1 list` - List all D1 databases
  - `cfad d1 show` - Show database details (tables, size, version)
  - `cfad d1 create` - Create new database with optional location hint
  - `cfad d1 update` - Update database name
  - `cfad d1 delete` - Delete database with confirmation
  - `cfad d1 query` - Execute SQL queries with object or raw array output
  - `cfad d1 query-file` - Execute SQL from file
  - `cfad d1 export` - Export database to SQL
  - `cfad d1 import` - Import SQL into database
  - `cfad d1 bookmark` - Get Time Travel bookmark
  - `cfad d1 restore` - Restore database to point in time

- **R2 Storage Support** - Full management of Cloudflare R2 object storage
  - `cfad r2 list` - List all buckets
  - `cfad r2 show` - Show bucket details
  - `cfad r2 create` - Create bucket with location hint and storage class
  - `cfad r2 delete` - Delete bucket with confirmation
  - `cfad r2 cors` - CORS configuration management (show, set, delete)
  - `cfad r2 domain` - Custom domain management (list, show, add, update, delete)
  - `cfad r2 public-access` - Managed r2.dev domain (show, enable, disable)
  - `cfad r2 lifecycle` - Lifecycle rules (show, set)
  - `cfad r2 lock` - Bucket lock / Object Lock (show, enable, disable)
  - `cfad r2 metrics` - Storage metrics across all buckets
  - `cfad r2 sippy` - Incremental migration from S3/GCS (show, enable, disable)
  - `cfad r2 notifications` - Event notifications (list, show, create, delete)
  - `cfad r2 migrate` - Super Slurper bulk migration (list, show, create, pause, resume, abort, progress, logs)
  - `cfad r2 temp-creds` - Generate scoped temporary credentials

- New API models: `src/api/d1.rs`, `src/api/r2.rs`
- New operations: `src/ops/d1.rs`, `src/ops/r2.rs` (40+ functions)
- New CLI commands: `src/cli/d1.rs`, `src/cli/r2.rs`
- New table formatters for D1 databases, R2 buckets, metrics, domains, notifications, migration jobs

### Changed in 0.3.0

- **Account ID Configuration** - `--account-id` flag is now optional for D1 and R2 commands
  - Supports `CLOUDFLARE_ACCOUNT_ID` environment variable
  - Supports `account_id` field in config profile
  - Resolution order: CLI flag > Environment variable > Config file
- Updated CLI to include D1 and R2 subcommands
- Extended output/table.rs with D1 and R2 formatters
- Expanded main.rs with 11 new command handlers

### Technical in 0.3.0

- Zero compilation errors/warnings
- Zero clippy warnings
- All 126 tests passing
- Full Cloudflare API v4 coverage for D1 and R2

---

### Added in Unreleased (2026-02-03)
- **CodeCov Integration** - Automated test coverage reporting
  - Configured codecov-action@v5 with token authentication
  - Coverage badge in README
  - Coverage reports uploaded on every CI run
- **Quality Check Claude Skill** - `/quality` command for comprehensive checks
  - Runs formatting, linting, tests, security audit, build verification
  - Validates cyclomatic complexity ≤8
  - Maintains GitHub badge integrity
- Comprehensive test suite with 68 tests (54 unit + 14 integration)
- Integration tests using wiremock for API mocking
- Test coverage for DNS parsers, output formatters, configuration, and error handling
- cargo-audit security scanning in quality checks

### Changed in Unreleased (2026-02-03)
- **Dependencies: 30% Reduction** (20 → 14 production dependencies)
  - Removed unused: chrono, futures, anyhow, colored, clap_complete, indicatif
  - Removed unused dev deps: assert_cmd, predicates, tempfile, serial_test
  - Updated to latest versions: reqwest 0.13, toml 0.9, dirs 6.0, indicatif 0.18, colored 3.1
  - Eliminated security warnings (RUSTSEC-2020-0159, RUSTSEC-2025-0055)
- **GitHub Actions: Modernized Workflows**
  - Release workflow: Replaced deprecated actions/create-release@v1 with softprops/action-gh-release@v2
  - Upgraded actions/download-artifact from v3 to v4
  - Upgraded actions/upload-artifact to v4
  - Fixed set-output deprecation warnings (11 warnings eliminated)
  - Removed musl build target (4 working platforms: Windows, Linux, macOS Intel/ARM)
- Refactored project structure to support both library and binary targets
- Added `new_with_base_url()` method to CloudflareClient for testing

### Fixed in Unreleased (2026-02-03)
- Cyclomatic complexity in `run()` function reduced from 12 to <8
- Release workflow permissions issue (added `contents: write`)
- All GitHub Actions deprecation warnings resolved
- deps.rs security false positives eliminated

### Documentation in Unreleased (2026-02-03)
- Updated README with current dependency list
- Removed musl installation instructions
- Added `/quality` skill documentation
- Updated all dependency version numbers
- Listed removed unused dependencies

## [0.2.0] - 2026-02-02

### Added in 0.2.0
- **DNS Import** - Import DNS records from CSV or BIND zone files
  - Auto-detects file format (CSV vs BIND)
  - Supports A, AAAA, CNAME, MX, TXT, NS record types
  - Progress tracking with success/failure statistics
  - Handles partial failures gracefully
- **DNS Show** - Display detailed information for a single DNS record
- **Zone Parameter** - All DNS commands now require zone parameter for clarity
- CSV parsing with validation
- BIND zone file parsing with $ORIGIN and $TTL directive support
- Example files in `docs/examples/` (dns-records.csv, zone.bind)

### Changed in 0.2.0
- **BREAKING:** DNS commands now require `<ZONE>` parameter
  - Old: `cfad dns show <record-id>`
  - New: `cfad dns show <zone> <record-id>`
- **BREAKING:** DNS update and delete commands updated with zone parameter
- Version bumped from 0.1.0 to 0.2.0
- Updated README with migration guide and new examples

### Fixed in 0.2.0
- DNS show, update, and delete commands are now fully functional
- Proper error handling for DNS operations

### Documentation in 0.2.0
- Added comprehensive usage examples for DNS import
- Added migration guide (v0.1.0 → v0.2.0)
- Updated feature matrix

## [0.1.0] - 2026-02-01

### Added in 0.1.0
- **Initial Release** - Core Cloudflare CLI functionality
- **DNS Management**
  - List DNS records with filters (type, name)
  - Create DNS records
  - Placeholder commands for show, update, delete (non-functional)
- **Zone Management**
  - List zones
  - Get zone by name
  - Create zones
  - Delete zones
  - Update zone settings
- **Cache Management**
  - Purge all cache
  - Purge by URLs
  - Purge by tags
- **Configuration**
  - Multi-profile support via config file (`~/.config/cfad/config.toml`)
  - Environment variable support
  - API token and API key + email authentication
- **Output Formatting**
  - Colored table output
  - JSON output support
- **Built-in Features**
  - Rate limiting (4 req/s default)
  - Retry with exponential backoff
  - Structured logging with tracing
  - Shell completion generation (bash, zsh, fish)

### Documentation in 0.1.0
- Initial README with installation and usage instructions
- Example configuration file
- License (MIT)

---

## Version History

- **v0.2.0** - DNS completion (import, show, update, delete)
- **v0.1.0** - Initial release

## Migration Guides

### v0.1.0 → v0.2.0

**DNS Command Changes (Breaking):**

All DNS commands now require the `<ZONE>` parameter:

```bash
# v0.1.0 (broken)
cfad dns show <record-id>
cfad dns update <record-id> --content 203.0.113.2
cfad dns delete <record-id>

# v0.2.0 (working)
cfad dns show example.com <record-id>
cfad dns update example.com <record-id> --content 203.0.113.2
cfad dns delete example.com <record-id>
```

**Unchanged Commands:**
- `cfad dns list <zone>` - No changes
- `cfad dns create <zone>` - No changes
- All zone and cache commands - No changes

**New Commands:**
- `cfad dns import <zone> <file>` - Import from CSV or BIND zone files

**Why the change?**
The Cloudflare API requires both zone_id and record_id for DNS operations. Requiring the zone parameter:
- Matches the API structure
- Provides clear user intent
- Avoids expensive cross-zone searches
- Follows industry standards (similar to other Cloudflare CLIs)

---

## Future Roadmap

### v0.3.0 (Planned)
- Firewall rules management
- Rate limiting rules
- Page rules management
- Improved test coverage (target: 70%+)
- CI/CD with GitHub Actions

### v0.4.0 (Planned)
- Analytics and metrics
- Workers management
- KV namespace operations

### v1.0.0 (Planned)
- R2 storage integration
- Stream API support
- Shell completion improvements
- Comprehensive documentation site

---

[Unreleased]: https://github.com/davidcforbes/cloudflare-cli-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/davidcforbes/cloudflare-cli-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/davidcforbes/cloudflare-cli-rs/releases/tag/v0.1.0
