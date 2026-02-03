# CFAD - CloudFlare Admin CLI

A fast, type-safe Rust CLI for managing Cloudflare DNS, zones, and cache from the command line.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

> **Current Status:** v0.2.0 - DNS features complete (show, update, delete, import)

---

## Features

### ✅ Implemented Features

- 🚀 **Fast & Efficient** - Built in Rust for optimal performance
- 🔒 **Type-Safe** - Leverages Rust's type system for reliability
- 🎨 **Beautiful Output** - Colored tables and formatted output
- 🔄 **Smart Retries** - Automatic retry with exponential backoff
- ⚡ **Rate Limited** - Respects Cloudflare API rate limits (4 req/s)
- 🔐 **Secure** - Multiple authentication methods with credential redaction
- 📊 **Progress Tracking** - Visual feedback for operations

### 🎯 Core Capabilities

| Feature | Status | Commands |
|---------|--------|----------|
| **DNS Management** | ✅ Complete | list, show, add, update, delete, import |
| **Zone Management** | ✅ Complete | list, show, create, delete, settings, update |
| **Cache Management** | ✅ Complete | purge (all, files, tags, hosts, prefixes) |
| **Config Management** | ✅ Complete | init, show, profiles |
| **Firewall Rules** | 🔮 Planned | Firewall rule CRUD, IP access rules |
| **Analytics** | 🔮 Planned | Dashboard queries, metrics export |
| **Workers** | 🔮 Planned | Worker deployment and management |
| **R2 Integration** | 🔮 Planned | Integrate cfr2 functionality |

---

## Installation

### From Release Binary

```bash
# Copy to cargo bin directory
cp target/release/cfad ~/.cargo/bin/

# Verify installation
cfad --version
```

### From Source

```bash
git clone https://github.com/yourusername/cfad
cd cfad
cargo build --release
cp target/release/cfad ~/.cargo/bin/
```

---

## Quick Start

1. **Initialize configuration:**
   ```bash
   cfad config init
   ```

2. **Add your API token:**

   Edit your config file (`~/.config/cfad/config.toml` on Linux/Mac or `%APPDATA%\cfad\config.toml` on Windows):

   ```toml
   default_profile = "default"

   [profiles.default]
   api_token = "your_cloudflare_api_token"
   default_zone = "example.com"
   output_format = "table"
   ```

3. **Start managing your Cloudflare resources:**
   ```bash
   cfad zone list
   cfad dns list example.com
   ```

---

## Authentication

CFAD supports multiple authentication methods with the following priority:

1. **CLI Flags** (highest priority)
   ```bash
   cfad --api-token <token> zone list
   ```

2. **Environment Variables**
   ```bash
   export CLOUDFLARE_API_TOKEN="your_token"
   cfad zone list
   ```

3. **Configuration File**
   ```bash
   cfad --profile production zone list
   ```

### API Token (Recommended)

Create an API token at https://dash.cloudflare.com/profile/api-tokens

Required permissions:
- Zone:Read (for zone list/show)
- Zone:Edit (for zone create/update/delete)
- DNS:Read (for DNS list/show)
- DNS:Edit (for DNS create/update/delete)
- Cache Purge (for cache operations)

### Legacy API Key + Email

```bash
export CLOUDFLARE_API_KEY="your_key"
export CLOUDFLARE_API_EMAIL="your@email.com"
```

Or in config file:
```toml
[profiles.default]
api_key = "your_api_key"
api_email = "your@email.com"
```

---

## Usage

### DNS Management

#### List DNS Records
```bash
# List all DNS records for a zone
cfad dns list example.com

# Filter by record type
cfad dns list example.com --type A

# Filter by name
cfad dns list example.com --name www
```

**Output:**
```
DNS Records for example.com:

╔══════╦═══════════════════╦════════════════╦══════╦═════════╦══════════╗
║ Type ║ Name              ║ Content        ║ TTL  ║ Proxied ║ ID       ║
╠══════╬═══════════════════╬════════════════╬══════╬═════════╬══════════╣
║ A    ║ example.com       ║ 203.0.113.1    ║ Auto ║ ✓       ║ abc12345 ║
║ A    ║ www.example.com   ║ 203.0.113.1    ║ Auto ║ ✓       ║ def67890 ║
║ MX   ║ example.com       ║ mail.example.  ║ Auto ║ ✗       ║ ghi11213 ║
╚══════╩═══════════════════╩════════════════╩══════╩═════════╩══════════╝

Total: 3 records
```

#### Create DNS Record
```bash
# Create an A record
cfad dns add example.com A www 203.0.113.1

# With TTL and proxied
cfad dns add example.com A www 203.0.113.1 --ttl 3600 --proxied

# Create MX record with priority
cfad dns add example.com MX @ mail.example.com --priority 10
```

#### Show DNS Record
```bash
# Show DNS record details
cfad dns show example.com <record-id>
```

**Output:**
```
DNS Record Details:

  ID: abc123...
  Type: A
  Name: www.example.com
  Content: 203.0.113.1
  TTL: Auto
  Proxied: ✓
  Created: 2026-01-15T10:30:00Z
  Modified: 2026-01-20T14:22:00Z
```

#### Update DNS Record
```bash
# Update record content
cfad dns update example.com <record-id> --content 203.0.113.2

# Update TTL and proxy status
cfad dns update example.com <record-id> --ttl 7200 --proxied true

# Update name
cfad dns update example.com <record-id> --name api.example.com
```

#### Delete DNS Record
```bash
# Delete with confirmation
cfad dns delete example.com <record-id> --confirm
```

#### Import DNS Records
```bash
# Import from CSV file
cfad dns import example.com dns-records.csv

# Import from BIND zone file
cfad dns import example.com zone.bind
```

**CSV Format:**
```csv
type,name,content,ttl,proxied,priority
A,@,203.0.113.1,3600,true,
A,www,203.0.113.1,3600,true,
MX,@,mail.example.com,3600,false,10
TXT,@,"v=spf1 mx ~all",3600,false,
```

**BIND Format:**
```bind
$ORIGIN example.com.
$TTL 3600
@       IN  A       203.0.113.1
www     IN  A       203.0.113.1
mail    IN  A       203.0.113.2
@       IN  MX  10  mail.example.com.
@       IN  TXT     "v=spf1 mx ~all"
```

---

### Zone Management

#### List Zones
```bash
# List all zones
cfad zone list

# Filter by status
cfad zone list --status active
```

**Output:**
```
Zones:

╔═══════════════════╦════════╦══════════╗
║ Name              ║ Status ║ ID       ║
╠═══════════════════╬════════╬══════════╣
║ example.com       ║ active ║ abc12345 ║
║ example.org       ║ active ║ def67890 ║
║ pending.com       ║ pending║ ghi11213 ║
╚═══════════════════╩════════╩══════════╝

Total: 3 zones
```

#### Show Zone Details
```bash
# Show by name or ID
cfad zone show example.com
cfad zone show <zone-id>
```

**Output:**
```
Zone: example.com
  ID: abc123...
  Status: active
  Name Servers: ["ns1.cloudflare.com", "ns2.cloudflare.com"]
```

#### Create Zone
```bash
# Create a new zone
cfad zone create newdomain.com --account-id <account-id>
```

#### Delete Zone
```bash
# Delete with confirmation
cfad zone delete <zone-id> --confirm
```

#### Show Zone Settings
```bash
cfad zone settings example.com
```

#### Update Zone Settings
```bash
# Update SSL mode
cfad zone update example.com --ssl strict

# Update multiple settings
cfad zone update example.com \
  --ssl strict \
  --always-https on \
  --security-level high \
  --cache-level aggressive

# Available options:
# --security-level: off, low, medium, high, under_attack
# --cache-level: aggressive, basic, simplified
# --dev-mode: on, off
# --ipv6: on, off
# --ssl: off, flexible, full, strict
# --always-https: on, off
```

---

### Cache Management

#### Purge All Cache
```bash
cfad cache purge example.com --all
```

#### Purge Specific Files
```bash
# Single file
cfad cache purge example.com --files https://example.com/page.html

# Multiple files (comma-separated)
cfad cache purge example.com --files https://example.com/page1.html,https://example.com/page2.html
```

#### Purge by Cache Tags
```bash
# Requires Cloudflare Enterprise
cfad cache purge example.com --tags tag1,tag2,tag3
```

#### Purge by Hosts
```bash
cfad cache purge example.com --hosts cdn.example.com,assets.example.com
```

#### Purge by Prefixes
```bash
# Requires Cloudflare Enterprise
cfad cache purge example.com --prefixes /static/,/images/
```

---

### Configuration Management

#### Initialize Config
```bash
cfad config init
```

#### Show Configuration
```bash
# Show default profile
cfad config show

# Show specific profile
cfad config show production
```

**Output:**
```
Profile configuration:
  API Token: Some("abcd****")
  API Key: None
  API Email: None
  Default Zone: Some("example.com")
  Output Format: Some("table")
```

#### Manage Profiles

```bash
# List all profiles
cfad config profiles list

# Add a new profile
cfad config profiles add production

# Set default profile
cfad config profiles set-default production
```

---

## Global Options

All commands support these global options:

```bash
--profile <name>         # Use specific profile
--api-token <token>      # Override API token
--api-key <key>          # Override API key
--api-email <email>      # Override API email
--format <format>        # Output format: table, json, csv
--quiet                  # Minimal output
--verbose                # Debug logging
```

### Examples

```bash
# Use production profile
cfad --profile production zone list

# Override with API token
cfad --api-token <token> dns list example.com

# JSON output for scripting
cfad --format json zone list | jq '.[0].name'

# Verbose mode for debugging
cfad --verbose dns add example.com A www 203.0.113.1

# Quiet mode
cfad --quiet cache purge example.com --all
```

---

## Output Formats

### Table (Default)
Beautiful formatted tables with colors

### JSON
Machine-readable output for scripting:
```bash
cfad --format json zone list | jq
```

### CSV
Spreadsheet-compatible output:
```bash
cfad --format csv zone list > zones.csv
```

---

## Configuration File

**Location:**
- Linux/Mac: `~/.config/cfad/config.toml`
- Windows: `%APPDATA%\cfad\config.toml`

**Format:**
```toml
default_profile = "default"

[profiles.default]
api_token = "your_cloudflare_api_token"
default_zone = "example.com"
output_format = "table"

[profiles.production]
api_token = "prod_token"
default_zone = "prod-example.com"
output_format = "json"

[profiles.staging]
api_token = "staging_token"
default_zone = "staging-example.com"
```

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         CFAD CLI (v0.1.0)                       │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Config     │  │  Command     │  │   Output     │          │
│  │   Manager    │  │   Parser     │  │  Formatter   │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
│         │                 │                  │                  │
│         └─────────┬───────┴──────────────────┘                  │
│                   │                                              │
│         ┌─────────▼──────────────────────┐                      │
│         │   CloudflareClient             │                      │
│         │  (Async HTTP + Rate Limiting)  │                      │
│         └─────────┬──────────────────────┘                      │
│                   │                                              │
│    ┌──────────────┼──────────────┬──────────────┐               │
│    │              │              │              │               │
│  ┌─▼───────┐  ┌──▼─────┐  ┌────▼────┐  ┌──────▼──────┐        │
│  │  DNS    │  │  Zone  │  │  Cache  │  │   Config    │        │
│  │ Module  │  │ Module │  │ Module  │  │   Module    │        │
│  └─────────┘  └────────┘  └─────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
              ┌─────────────────────────────┐
              │  Cloudflare REST API v4     │
              │  https://api.cloudflare.com │
              └─────────────────────────────┘
```

### Project Structure

```
cfad/
├── src/
│   ├── main.rs                   # Entry point, command routing
│   ├── cli/                      # CLI definitions
│   │   ├── mod.rs                # Main CLI structure
│   │   ├── config.rs             # Config commands
│   │   ├── dns.rs                # DNS commands
│   │   ├── zone.rs               # Zone commands
│   │   └── cache.rs              # Cache commands
│   ├── client/                   # HTTP client
│   │   ├── mod.rs                # CloudflareClient
│   │   └── retry.rs              # Retry logic
│   ├── config/                   # Configuration
│   │   ├── mod.rs                # Profile management
│   │   └── validation.rs         # Validators
│   ├── error/                    # Error handling
│   │   ├── mod.rs                # Error types
│   │   └── category.rs           # Error categories
│   ├── api/                      # API models
│   │   ├── dns.rs                # DNS models
│   │   ├── zone.rs               # Zone models
│   │   ├── cache.rs              # Cache models
│   │   └── response.rs           # Response wrappers
│   ├── ops/                      # Operations
│   │   ├── dns.rs                # DNS operations
│   │   ├── zone.rs               # Zone operations
│   │   └── cache.rs              # Cache operations
│   ├── output/                   # Output formatting
│   │   └── table.rs              # Table formatter
│   ├── utils/                    # Utilities
│   │   └── validation.rs         # Input validators
│   └── metrics/                  # Metrics (stub)
├── Cargo.toml                    # Dependencies
├── LICENSE                       # MIT License
└── README.md                     # This file
```

### Key Design Patterns

- **Async-First**: Tokio runtime for all I/O operations
- **Type-Safe**: Rust's type system for API request/response validation
- **Modular**: Clear separation of concerns (CLI → Ops → Client → API)
- **Error Resilient**: Comprehensive error handling with automatic retries
- **User-Friendly**: Colored output, progress indicators, helpful error messages
- **Configurable**: Multiple credential sources with priority order

---

## Error Handling

CFAD provides clear error messages with categories:

- **API Errors** - Issues with Cloudflare API responses
- **Authentication Errors** - Invalid or missing credentials
- **Network Errors** - Connection issues (auto-retried)
- **Validation Errors** - Invalid input parameters
- **Configuration Errors** - Config file or profile issues

### Automatic Retries

Network errors are automatically retried with exponential backoff:
- Max attempts: 3
- Initial delay: 100ms
- Max delay: 30s
- Multiplier: 2x

### Rate Limiting

CFAD respects Cloudflare's rate limits:
- Default: 4 requests/second
- Automatic throttling via tokio::sync::Semaphore
- Prevents API quota exhaustion

---

## Advanced Usage

### Scripting

```bash
#!/bin/bash
# Update all zones to strict SSL

for zone in $(cfad --format json zone list | jq -r '.[].name'); do
  echo "Updating $zone..."
  cfad zone update "$zone" --ssl strict --always-https on
done
```

### Multi-Profile Workflow

```bash
# Development
cfad --profile dev zone list

# Staging
cfad --profile staging dns list example-staging.com

# Production
cfad --profile production cache purge example.com --all
```

---

## Troubleshooting

### Command Not Found

```bash
# Ensure ~/.cargo/bin is in PATH
echo $PATH | grep cargo

# Add to PATH if needed (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Authentication Errors

```bash
# Verify API token
cfad config show

# Test with explicit token
cfad --api-token <your-token> zone list

# Check token permissions at:
# https://dash.cloudflare.com/profile/api-tokens
```

### Rate Limit Errors

```bash
# Use --verbose to see retry attempts
cfad --verbose dns list example.com

# Wait a few minutes and try again
# CFAD automatically retries with backoff
```

### Network Errors

```bash
# Check connectivity
ping api.cloudflare.com

# Use verbose mode
cfad --verbose zone list

# Check proxy settings if behind corporate firewall
```

---

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/yourusername/cfad
cd cfad

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run tests
cargo test

# Check for errors
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Quality Metrics

- ✅ **Compilation Errors:** 0
- ✅ **Compilation Warnings:** 0
- ✅ **Clippy Warnings:** 0
- ✅ **Binary Size:** 5.3 MB (release)
- ✅ **Build Time:** ~55s (release)

---

## Dependencies

### Core Dependencies (49 total)

- **CLI:** clap 4.5, clap_complete 4.5
- **Async:** tokio 1.40, futures 0.3
- **HTTP:** reqwest 0.12
- **Serialization:** serde 1.0, serde_json 1.0, toml 0.8
- **Error Handling:** thiserror 2.0, anyhow 1.0
- **Logging:** tracing 0.1, tracing-subscriber 0.3
- **Config:** dirs 5.0
- **UI:** indicatif 0.17, colored 2.1, comfy-table 7.1
- **Utils:** regex 1.10, chrono 0.4, url 2.5

### Dev Dependencies

- **Testing:** assert_cmd 2.0, predicates 3.1, tempfile 3.13
- **Mocking:** wiremock 0.6
- **Sync:** serial_test 3.1

---

## Roadmap

### v0.2.0 - Bulk Operations (Planned)
- DNS import from BIND zone files
- DNS import from CSV files
- Bulk DNS record updates
- Zone migration tools

### v0.3.0 - Security Features (Planned)
- Firewall rule management
- IP access rules (whitelist/block/challenge)
- Country-based blocking
- WAF custom rules

### v0.4.0 - Analytics & Reporting (Planned)
- Dashboard analytics queries
- Request/bandwidth/threat metrics
- Time-range filtering
- CSV/JSON report export

### v0.5.0 - Workers & Edge (Planned)
- Worker script deployment
- Worker log tailing
- KV namespace management
- Durable Objects support

### v1.0.0 - Full Integration (Planned)
- R2 bucket management (integrate cfr2)
- Pages deployment
- Stream video management
- Shell completions (bash/zsh/fish)
- Comprehensive test coverage

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Areas for Contribution

1. **DNS Import** - BIND and CSV file parsers
2. **Firewall Management** - Firewall rule CRUD operations
3. **Analytics** - Cloudflare Analytics API integration
4. **Workers** - Worker deployment and management
5. **Test Coverage** - Integration and unit tests
6. **Documentation** - Usage examples and guides

---

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI framework: [Clap](https://github.com/clap-rs/clap)
- HTTP client: [Reqwest](https://github.com/seanmonstar/reqwest)
- Table formatting: [Comfy Table](https://github.com/Nukesor/comfy-table)
- Async runtime: [Tokio](https://tokio.rs/)
- Inspired by [cloudflare-cli](https://github.com/jordantrizz/cloudflare-cli)
- Architecture patterns from [cfr2](https://github.com/yourusername/cfr2)

---

## Support

- **Documentation:** This README
- **Issues:** GitHub Issues
- **Cloudflare API Docs:** https://developers.cloudflare.com/api/

---

## Migration Guide

### Upgrading from v0.1.0 to v0.2.0

**Breaking Changes:**

v0.2.0 introduces breaking changes to DNS record operations to align with Cloudflare API requirements and industry standards.

#### DNS Show Command

**v0.1.0 (non-functional):**
```bash
cfad dns show <record-id>  # Did not work
```

**v0.2.0:**
```bash
cfad dns show <zone> <record-id>
```

#### DNS Update Command

**v0.1.0 (non-functional):**
```bash
cfad dns update <record-id> --content 1.2.3.4  # Did not work
```

**v0.2.0:**
```bash
cfad dns update <zone> <record-id> --content 1.2.3.4
```

#### DNS Delete Command

**v0.1.0 (non-functional):**
```bash
cfad dns delete <record-id> --confirm  # Did not work
```

**v0.2.0:**
```bash
cfad dns delete <zone> <record-id> --confirm
```

#### Why the Change?

The Cloudflare API requires both `zone_id` and `record_id` for all DNS record operations. There is no API endpoint to search for a DNS record across all zones. This change:

- Matches industry standard ([cloudflare-cli](https://github.com/jordantrizz/cloudflare-cli))
- Provides clear user intent
- Eliminates unnecessary API calls
- Ensures predictable performance

#### No Breaking Changes

These commands remain unchanged:
- `cfad dns list <zone>` - No change
- `cfad dns add <zone> <type> <name> <content>` - No change
- All zone, cache, config commands - No change

---

## Changelog

### v0.2.0 (2026-02-02)

**Completed:**
- ✅ DNS show command - View detailed record information
- ✅ DNS update command - Modify existing records (now functional)
- ✅ DNS delete command - Remove records (now functional)
- ✅ DNS import - Bulk import from CSV files
- ✅ DNS import - Bulk import from BIND zone files
- ✅ Auto-detect file format (CSV vs BIND)
- ✅ Support for A, AAAA, CNAME, MX, TXT, NS record types
- ✅ Progress indicators for bulk operations
- ✅ Comprehensive error handling with partial import support
- ✅ Zero compilation errors/warnings
- ✅ Zero clippy warnings

**Breaking Changes:**
- DNS show, update, delete commands now require `<zone>` parameter
- Old: `cfad dns update <record-id> --content X`
- New: `cfad dns update <zone> <record-id> --content X`
- See Migration Guide above for details

**Architecture:**
- Aligned with Cloudflare API zone-scoped requirements
- Matches industry standard (jordantrizz/cloudflare-cli)
- Single API call per operation (improved performance)

### v0.1.0 (2026-02-01)

**Implemented:**
- ✅ DNS management (list, add, update, delete)
- ✅ Zone management (list, show, create, delete, update settings)
- ✅ Cache management (purge all, files, tags, hosts, prefixes)
- ✅ Configuration management with profiles
- ✅ Multiple authentication methods (API token, legacy key/email)
- ✅ Colored table output with comfy-table
- ✅ Automatic retries with exponential backoff
- ✅ Rate limiting (4 req/s)
- ✅ JSON/CSV/Table output formats
- ✅ Zero compilation warnings
- ✅ Production-ready release build

**Not Implemented (Future):**
- DNS import (BIND/CSV)
- Firewall rules
- Analytics queries
- Workers management
- R2 integration
- Shell completions

---

**Made with ❤️ using Rust**
