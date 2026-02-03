# Changelog

All notable changes to CFAD (Cloudflare Admin CLI) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive test suite with 68 tests (54 unit + 14 integration)
- Integration tests using wiremock for API mocking
- Test coverage for DNS parsers, output formatters, configuration, and error handling

### Changed
- Refactored project structure to support both library and binary targets
- Added `new_with_base_url()` method to CloudflareClient for testing

## [0.2.0] - 2026-02-02

### Added
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

### Changed
- **BREAKING:** DNS commands now require `<ZONE>` parameter
  - Old: `cfad dns show <record-id>`
  - New: `cfad dns show <zone> <record-id>`
- **BREAKING:** DNS update and delete commands updated with zone parameter
- Version bumped from 0.1.0 to 0.2.0
- Updated README with migration guide and new examples

### Fixed
- DNS show, update, and delete commands are now fully functional
- Proper error handling for DNS operations

### Documentation
- Added comprehensive usage examples for DNS import
- Added migration guide (v0.1.0 → v0.2.0)
- Updated feature matrix

## [0.1.0] - 2026-02-01

### Added
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

### Documentation
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
