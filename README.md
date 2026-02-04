# CFAD - CloudFlare Admin CLI

A fast, type-safe Rust CLI for managing Cloudflare DNS, zones, cache, D1 databases, and R2 storage from the command line.

[![CI](https://github.com/davidcforbes/cloudflare-cli-rs/workflows/CI/badge.svg)](https://github.com/davidcforbes/cloudflare-cli-rs/actions/workflows/ci.yml)
[![Release](https://github.com/davidcforbes/cloudflare-cli-rs/workflows/Release/badge.svg)](https://github.com/davidcforbes/cloudflare-cli-rs/actions/workflows/release.yml)
[![Security Audit](https://github.com/davidcforbes/cloudflare-cli-rs/workflows/Security%20Audit/badge.svg)](https://github.com/davidcforbes/cloudflare-cli-rs/actions/workflows/security-audit.yml)
[![Known Vulnerabilities](https://snyk.io/test/github/davidcforbes/cloudflare-cli-rs/badge.svg)](https://snyk.io/test/github/davidcforbes/cloudflare-cli-rs)
[![codecov](https://codecov.io/gh/davidcforbes/cloudflare-cli-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/davidcforbes/cloudflare-cli-rs)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/098be488ba4a46199247ae6fde2b8a71)](https://app.codacy.com/gh/davidcforbes/cloudflare-cli-rs/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![GitHub release](https://img.shields.io/github/release/davidcforbes/cloudflare-cli-rs.svg)](https://github.com/davidcforbes/cloudflare-cli-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Dependencies](https://deps.rs/repo/github/davidcforbes/cloudflare-cli-rs/status.svg)](https://deps.rs/repo/github/davidcforbes/cloudflare-cli-rs)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)](https://github.com/davidcforbes/cloudflare-cli-rs/releases)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org)
[![Cloudflare API](https://img.shields.io/badge/Cloudflare%20API-v4-orange?logo=cloudflare)](https://api.cloudflare.com/)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/davidcforbes?style=flat&logo=github&label=Sponsor&color=EA4AAA)](https://github.com/sponsors/davidcforbes)

> **Current Status:** v0.3.0 - D1 Database and R2 Storage support complete

---

## Executive Overview

### About Cloudflare

**Global Edge Platform**

Cloudflare operates a global edge network spanning **300+ cities worldwide**, providing enterprise-grade infrastructure for businesses of all sizes. The platform serves as a reverse proxy, CDN, and distributed computing environment that sits between your users and your infrastructure.

### Key Platform Services

#### 🚀 Cloudflare Workers (Serverless Compute)

- Deploy JavaScript/TypeScript, Rust (WASM), Python, and other languages to the edge
- Sub-millisecond startup times with V8 isolates (not containers)
- Execute code in 300+ locations worldwide, closest to your users
- Pay-per-request pricing with a generous free tier
- Durable Objects for stateful applications and distributed coordination

#### 💾 R2 Storage (Object Storage)

- S3-compatible object storage **without egress fees**
- Global availability with automatic replication
- Lower costs than AWS S3, Azure Blob, or Google Cloud Storage
- Perfect for static assets, backups, data lakes, and CDN origins
- Seamless integration with Workers for edge processing

#### 🗄️ D1 Databases (Serverless SQL)

- SQLite-based distributed SQL databases at the edge
- Automatic replication across multiple regions
- Low-latency reads from the nearest location
- ACID compliance with global consistency
- Integrated with Workers for edge-native applications

#### 🔧 Additional Services

- **KV (Key-Value)**: Eventually-consistent edge storage for configuration and session data
- **Queues**: Message queuing for async workflows between Workers
- **Stream**: Live and on-demand video platform with adaptive bitrate streaming
- **Pages**: JAMstack deployment platform with Git integration
- **DNS**: Authoritative DNS with the fastest response times globally
- **CDN**: Content delivery with smart routing and caching

### Why a CLI Matters

While Cloudflare provides an excellent web dashboard and official Wrangler CLI for Workers, **cfad** (CloudFlare Admin CLI) fills a critical gap for infrastructure automation and DevOps workflows:

- 🎯 **Multi-Service Management**: Unified interface across DNS, caching, zones, and security settings
- 🔄 **CI/CD Integration**: Scriptable commands for deployment pipelines
- 📦 **Bulk Operations**: Import/export DNS records, batch zone updates, mass cache purges
- ⚡ **Performance**: Rust-based implementation with async I/O and smart rate limiting
- 🏢 **Enterprise Workflows**: Profile management for multiple accounts/environments

The CLI complements Cloudflare's Workers and R2 services by providing **programmatic control over the infrastructure layer** - DNS records, cache policies, firewall rules, and zone configurations - while your application code runs on Workers and stores data in R2/D1.

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
| **D1 Databases** | ✅ Complete | list, show, create, update, delete, query, export, import, bookmark, restore |
| **R2 Storage** | ✅ Complete | buckets, cors, domains, lifecycle, locks, metrics, sippy, notifications, migrate, temp-creds |
| **Firewall Rules** | 🔮 Planned | Firewall rule CRUD, IP access rules |
| **Analytics** | 🔮 Planned | Dashboard queries, metrics export |
| **Workers** | 🔮 Planned | Worker deployment and management |

---

## Installation

### From Release Binary (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/davidcforbes/cfad/releases).

#### Windows

```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/davidcforbes/cfad/releases/latest/download/cfad-0.2.0-x86_64-pc-windows-msvc.zip" -OutFile cfad.zip
Expand-Archive cfad.zip
Move-Item cfad\cfad.exe $env:USERPROFILE\.cargo\bin\

# Verify installation
cfad --version
```

#### Linux (Ubuntu/Debian)

```bash
# Download and install
curl -LO https://github.com/davidcforbes/cfad/releases/latest/download/cfad-0.2.0-x86_64-unknown-linux-gnu.tar.gz
tar xzf cfad-0.2.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv cfad /usr/local/bin/

# Verify installation
cfad --version
```

#### macOS (Intel)

```bash
# Download and install
curl -LO https://github.com/davidcforbes/cfad/releases/latest/download/cfad-0.2.0-x86_64-apple-darwin.tar.gz
tar xzf cfad-0.2.0-x86_64-apple-darwin.tar.gz
sudo mv cfad /usr/local/bin/

# Verify installation
cfad --version
```

#### macOS (Apple Silicon - M1/M2/M3)

```bash
# Download and install
curl -LO https://github.com/davidcforbes/cfad/releases/latest/download/cfad-0.2.0-aarch64-apple-darwin.tar.gz
tar xzf cfad-0.2.0-aarch64-apple-darwin.tar.gz
sudo mv cfad /usr/local/bin/

# Verify installation
cfad --version
```

### From Source

```bash
# Clone repository
git clone https://github.com/davidcforbes/cfad
cd cfad

# Build and install
cargo build --release
cargo install --path .

# Verify installation
cfad --version
```

### Using Cargo

```bash
# Install directly from source (once published to crates.io)
cargo install cfad

# Verify installation
cfad --version
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

Create an API token at <https://dash.cloudflare.com/profile/api-tokens>

Required permissions:

- Zone:Read (for zone list/show)
- Zone:Edit (for zone create/update/delete)
- DNS:Read (for DNS list/show)
- DNS:Edit (for DNS create/update/delete)
- Cache Purge (for cache operations)
- D1:Read (for D1 database list/show/query)
- D1:Edit (for D1 database create/update/delete/import)
- R2:Read (for R2 bucket list/show/metrics)
- R2:Edit (for R2 bucket create/delete, CORS, domains, lifecycle, etc.)

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

### Account ID Configuration

For D1 and R2 commands that require an account ID, you can configure it once instead of passing `--account-id` on every command:

**Environment Variable (Recommended):**

```bash
export CLOUDFLARE_ACCOUNT_ID="your_account_id"
cfad d1 list   # No --account-id needed
cfad r2 list   # No --account-id needed
```

**Configuration File:**

```toml
[profiles.default]
api_token = "your_api_token"
account_id = "your_account_id"
```

**CLI Override:**

You can still override the account ID on any command:

```bash
cfad d1 list --account-id different_account_id
```

**Resolution Order:** CLI flag > Environment variable > Config file

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

### D1 Database Management

D1 is Cloudflare's serverless SQLite database. CFAD provides comprehensive D1 management capabilities.

> **Note:** The `--account-id` flag is optional if you have set `CLOUDFLARE_ACCOUNT_ID` environment variable or `account_id` in your config file. See [Account ID Configuration](#account-id-configuration).

#### List D1 Databases

```bash
# With environment variable set (recommended)
cfad d1 list

# Or with explicit account ID
cfad d1 list --account-id <account-id>
```

**Output:**

```text
D1 Databases:

╔═══════════════════╦════════╦══════════╦══════════╗
║ Name              ║ Tables ║ Size     ║ ID       ║
╠═══════════════════╬════════╬══════════╬══════════╣
║ production-db     ║ 12     ║ 4.2 MB   ║ abc12345 ║
║ staging-db        ║ 8      ║ 1.1 MB   ║ def67890 ║
╚═══════════════════╩════════╩══════════╩══════════╝

Total: 2 databases
```

#### Show D1 Database Details

```bash
cfad d1 show --account-id <account-id> <database-id>
```

#### Create D1 Database

```bash
# Create a new database
cfad d1 create --account-id <account-id> my-database

# With location hint for optimal latency
cfad d1 create --account-id <account-id> my-database --location wnam
```

**Available locations:** `wnam` (Western North America), `enam` (Eastern North America), `weur` (Western Europe), `eeur` (Eastern Europe), `apac` (Asia Pacific)

#### Update D1 Database

```bash
cfad d1 update --account-id <account-id> <database-id> --name new-name
```

#### Delete D1 Database

```bash
cfad d1 delete --account-id <account-id> <database-id> --confirm
```

#### Execute SQL Queries

```bash
# Execute a SQL query
cfad d1 query --account-id <account-id> <database-id> "SELECT * FROM users LIMIT 10"

# Use raw format for better performance (array output)
cfad d1 query --account-id <account-id> <database-id> "SELECT * FROM users" --raw
```

#### Execute SQL from File

```bash
cfad d1 query-file --account-id <account-id> <database-id> schema.sql
cfad d1 query-file --account-id <account-id> <database-id> migrations/001.sql --raw
```

#### Export D1 Database

```bash
cfad d1 export --account-id <account-id> <database-id>
```

**Output:**

```text
Export initiated:
  Task ID: task_abc123
  Status: pending
  Download URL: https://... (when complete)
```

#### Import SQL into D1 Database

```bash
cfad d1 import --account-id <account-id> <database-id> backup.sql
```

#### Time Travel - Get Bookmark

D1 supports point-in-time recovery via Time Travel.

```bash
# Get current bookmark
cfad d1 bookmark --account-id <account-id> <database-id>

# Get bookmark nearest to a timestamp
cfad d1 bookmark --account-id <account-id> <database-id> --timestamp "2026-02-01T12:00:00Z"
```

#### Time Travel - Restore Database

```bash
# Restore to a specific bookmark
cfad d1 restore --account-id <account-id> <database-id> --bookmark <bookmark-id> --confirm

# Restore to a specific timestamp
cfad d1 restore --account-id <account-id> <database-id> --timestamp "2026-02-01T12:00:00Z" --confirm
```

---

### R2 Storage Management

R2 is Cloudflare's S3-compatible object storage with zero egress fees. CFAD provides comprehensive R2 management.

> **Note:** The `--account-id` flag is optional if you have set `CLOUDFLARE_ACCOUNT_ID` environment variable or `account_id` in your config file. See [Account ID Configuration](#account-id-configuration).

#### List R2 Buckets

```bash
# With environment variable set (recommended)
cfad r2 list

# Or with explicit account ID
cfad r2 list --account-id <account-id>
```

**Output:**

```text
R2 Buckets:

╔═══════════════════╦══════════╦═════════════════════╗
║ Name              ║ Location ║ Created             ║
╠═══════════════════╬══════════╬═════════════════════╣
║ assets-bucket     ║ wnam     ║ 2026-01-15T10:30:00 ║
║ backups-bucket    ║ eeur     ║ 2026-01-20T14:22:00 ║
╚═══════════════════╩══════════╩═════════════════════╝

Total: 2 buckets
```

#### Show R2 Bucket Details

```bash
cfad r2 show --account-id <account-id> my-bucket
```

#### Create R2 Bucket

```bash
# Create a bucket
cfad r2 create --account-id <account-id> my-bucket

# With location hint
cfad r2 create --account-id <account-id> my-bucket --location weur

# With storage class
cfad r2 create --account-id <account-id> my-bucket --storage-class Standard
```

#### Delete R2 Bucket

```bash
cfad r2 delete --account-id <account-id> my-bucket --confirm
```

---

#### R2 CORS Management

```bash
# Show CORS configuration
cfad r2 cors show --account-id <account-id> my-bucket

# Set CORS from JSON file
cfad r2 cors set --account-id <account-id> my-bucket --file cors.json

# Delete CORS configuration
cfad r2 cors delete --account-id <account-id> my-bucket --confirm
```

**CORS JSON format (`cors.json`):**

```json
[
  {
    "allowedOrigins": ["https://example.com"],
    "allowedMethods": ["GET", "PUT", "POST"],
    "allowedHeaders": ["Content-Type"],
    "exposeHeaders": ["ETag"],
    "maxAgeSeconds": 3600
  }
]
```

---

#### R2 Custom Domain Management

```bash
# List custom domains
cfad r2 domain list --account-id <account-id> my-bucket

# Show domain details
cfad r2 domain show --account-id <account-id> my-bucket cdn.example.com

# Add a custom domain
cfad r2 domain add --account-id <account-id> my-bucket cdn.example.com

# With zone ID and TLS settings
cfad r2 domain add --account-id <account-id> my-bucket cdn.example.com \
  --zone-id <zone-id> --min-tls 1.2

# Update custom domain
cfad r2 domain update --account-id <account-id> my-bucket cdn.example.com \
  --enabled true --min-tls 1.3

# Delete custom domain
cfad r2 domain delete --account-id <account-id> my-bucket cdn.example.com --confirm
```

---

#### R2 Public Access (r2.dev Domain)

```bash
# Show public access status
cfad r2 public-access show --account-id <account-id> my-bucket

# Enable public access via r2.dev
cfad r2 public-access enable --account-id <account-id> my-bucket

# Disable public access
cfad r2 public-access disable --account-id <account-id> my-bucket
```

---

#### R2 Lifecycle Rules

```bash
# Show lifecycle rules
cfad r2 lifecycle show --account-id <account-id> my-bucket

# Set lifecycle rules from JSON file
cfad r2 lifecycle set --account-id <account-id> my-bucket --file lifecycle.json
```

**Lifecycle JSON format (`lifecycle.json`):**

```json
{
  "rules": [
    {
      "id": "delete-old-logs",
      "enabled": true,
      "conditions": { "prefix": "logs/" },
      "actions": { "deleteAfterDays": 30 }
    },
    {
      "id": "cleanup-temp",
      "enabled": true,
      "conditions": { "prefix": "temp/" },
      "actions": { "deleteAfterDays": 7 }
    }
  ]
}
```

---

#### R2 Bucket Locks (Object Lock)

```bash
# Show lock configuration
cfad r2 lock show --account-id <account-id> my-bucket

# Enable bucket lock (governance mode)
cfad r2 lock enable --account-id <account-id> my-bucket --mode governance --days 90

# Enable bucket lock (compliance mode - cannot be deleted)
cfad r2 lock enable --account-id <account-id> my-bucket --mode compliance --days 365

# Disable bucket lock
cfad r2 lock disable --account-id <account-id> my-bucket --confirm
```

---

#### R2 Storage Metrics

```bash
cfad r2 metrics --account-id <account-id>
```

**Output:**

```text
R2 Metrics:

╔═══════════════════╦════════════╦═══════════╗
║ Bucket            ║ Objects    ║ Storage   ║
╠═══════════════════╬════════════╬═══════════╣
║ assets-bucket     ║ 15,234     ║ 2.3 GB    ║
║ backups-bucket    ║ 892        ║ 45.6 GB   ║
╚═══════════════════╩════════════╩═══════════╝

Total: 16,126 objects, 47.9 GB
```

---

#### R2 Sippy (Incremental Migration)

Sippy enables incremental migration from other S3-compatible providers.

```bash
# Show Sippy configuration
cfad r2 sippy show --account-id <account-id> my-bucket

# Enable Sippy from AWS S3
cfad r2 sippy enable --account-id <account-id> my-bucket \
  --provider aws \
  --source-bucket source-bucket-name \
  --region us-east-1 \
  --access-key-id <key> \
  --secret-access-key <secret>

# Enable Sippy from GCS
cfad r2 sippy enable --account-id <account-id> my-bucket \
  --provider gcs \
  --source-bucket source-bucket-name

# Disable Sippy
cfad r2 sippy disable --account-id <account-id> my-bucket --confirm
```

---

#### R2 Event Notifications

```bash
# List notification rules
cfad r2 notifications list --account-id <account-id> my-bucket

# Show notification rule details
cfad r2 notifications show --account-id <account-id> my-bucket <queue-id>

# Create notification rule
cfad r2 notifications create --account-id <account-id> my-bucket <queue-id> \
  --events object:create,object:delete \
  --prefix uploads/ \
  --suffix .jpg

# Delete notification rule
cfad r2 notifications delete --account-id <account-id> my-bucket <queue-id> --confirm
```

---

#### R2 Super Slurper (Bulk Migration)

Super Slurper performs bulk data migration from other cloud providers.

```bash
# List migration jobs
cfad r2 migrate list --account-id <account-id>

# Show job details
cfad r2 migrate show --account-id <account-id> <job-id>

# Create migration job
cfad r2 migrate create --account-id <account-id> \
  --source-provider aws \
  --source-bucket source-bucket \
  --source-region us-east-1 \
  --target-bucket my-r2-bucket \
  --access-key-id <key> \
  --secret-access-key <secret>

# Pause migration
cfad r2 migrate pause --account-id <account-id> <job-id>

# Resume migration
cfad r2 migrate resume --account-id <account-id> <job-id>

# Abort migration
cfad r2 migrate abort --account-id <account-id> <job-id> --confirm

# Check progress
cfad r2 migrate progress --account-id <account-id> <job-id>

# View logs
cfad r2 migrate logs --account-id <account-id> <job-id>
```

---

#### R2 Temporary Credentials

Generate scoped temporary credentials for S3-compatible access.

```bash
# Create read-only credentials for a bucket
cfad r2 temp-creds create --account-id <account-id> \
  --bucket my-bucket \
  --permission read \
  --ttl 3600

# Create read-write credentials scoped to a prefix
cfad r2 temp-creds create --account-id <account-id> \
  --bucket my-bucket \
  --prefix uploads/ \
  --permission readwrite \
  --ttl 7200
```

**Output:**

```text
Temporary Credentials:
  Access Key ID: AKIAIOSFODNN7EXAMPLE
  Secret Access Key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
  Session Token: FwoGZXIvYXdzEBYaDK...
  Expiration: 2026-02-04T15:00:00Z
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

Beautifully formatted tables with colors

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

```text
┌──────────────────────────────────────────────────────────────────────────┐
│                            CFAD CLI (v0.3.0)                             │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                    │
│  │   Config     │  │  Command     │  │   Output     │                    │
│  │   Manager    │  │   Parser     │  │  Formatter   │                    │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                    │
│         │                 │                  │                           │
│         └─────────┬───────┴──────────────────┘                           │
│                   │                                                      │
│         ┌─────────▼──────────────────────┐                               │
│         │   CloudflareClient             │                               │
│         │  (Async HTTP + Rate Limiting)  │                               │
│         └─────────┬──────────────────────┘                               │
│                   │                                                      │
│    ┌──────┬───────┼───────┬──────────┬──────────┐                        │
│    │      │       │       │          │          │                        │
│  ┌─▼────┐ ▼─────┐ ▼─────┐ ▼────────┐ ▼────────┐ ▼──────────┐             │
│  │ DNS  │ │Zone │ │Cache│ │   D1   │ │   R2   │ │  Config  │             │
│  │Module│ │Mod. │ │Mod. │ │ Module │ │ Module │ │  Module  │             │
│  └──────┘ └─────┘ └─────┘ └────────┘ └────────┘ └──────────┘             │
└──────────────────────────────────────────────────────────────────────────┘
                            │
                            ▼
              ┌─────────────────────────────┐
              │  Cloudflare REST API v4     │
              │  https://api.cloudflare.com │
              └─────────────────────────────┘
```

### Project Structure

```text
cfad/
├── src/
│   ├── main.rs                   # Entry point, command routing
│   ├── cli/                      # CLI definitions
│   │   ├── mod.rs                # Main CLI structure
│   │   ├── config.rs             # Config commands
│   │   ├── dns.rs                # DNS commands
│   │   ├── zone.rs               # Zone commands
│   │   ├── cache.rs              # Cache commands
│   │   ├── d1.rs                 # D1 database commands
│   │   └── r2.rs                 # R2 storage commands
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
│   │   ├── d1.rs                 # D1 database models
│   │   ├── r2.rs                 # R2 storage models
│   │   └── response.rs           # Response wrappers
│   ├── ops/                      # Operations
│   │   ├── dns.rs                # DNS operations
│   │   ├── zone.rs               # Zone operations
│   │   ├── cache.rs              # Cache operations
│   │   ├── d1.rs                 # D1 database operations
│   │   └── r2.rs                 # R2 storage operations
│   ├── output/                   # Output formatting
│   │   └── table.rs              # Table formatter (DNS, Zone, D1, R2)
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
- ✅ **Tests:** 126 (unit + integration)
- ✅ **Binary Size:** ~6 MB (release)
- ✅ **Build Time:** ~2 min (release)

### Code Quality Checks

Run all quality checks before pushing:

```bash
# Using Claude Code (recommended)
/quality

# Using scripts
# Windows: .\scripts\quality-check.ps1
# Linux/macOS: ./scripts/quality-check.sh

# Using Make
make quality-check
```

The `/quality` Claude Skill runs comprehensive checks:
- Code formatting (cargo fmt)
- Linting (cargo clippy with zero warnings)
- Tests (all 68 tests)
- Security audit (cargo audit)
- Release build verification

**See [DEVELOPMENT.md](docs/DEVELOPMENT.md) for:**

- Setting up development tools
- Running local code quality checks
- Git hooks for automated checking
- Complexity analysis
- Contributing guidelines

---

## Dependencies

### Core Dependencies (14 production dependencies - 30% reduction)

- **CLI:** clap 4.5
- **Async:** tokio 1.40
- **HTTP:** reqwest 0.13
- **Serialization:** serde 1.0, serde_json 1.0, toml 0.9, csv 1.3
- **Error Handling:** thiserror 2.0
- **Logging:** tracing 0.1, tracing-subscriber 0.3
- **Config:** dirs 6.0
- **UI:** comfy-table 7.1
- **Utils:** regex 1.10, url 2.5

### Dev Dependencies

- **Mocking:** wiremock 0.6

**Removed unused dependencies:** chrono, futures, anyhow, colored, clap_complete, indicatif, assert_cmd, predicates, tempfile, serial_test

---

## Roadmap

### v0.2.0 - Bulk Operations ✅ Complete

- DNS import from BIND zone files
- DNS import from CSV files
- Bulk DNS record updates
- Zone migration tools

### v0.3.0 - D1 & R2 Support ✅ Complete

- D1 Database Management (CRUD, query, export, import, time travel)
- R2 Storage Management (buckets, CORS, domains, lifecycle, locks)
- R2 Advanced Features (metrics, Sippy, notifications, migrations, temp creds)

### v0.4.0 - Security Features (Planned)

- Firewall rule management
- IP access rules (whitelist/block/challenge)
- Country-based blocking
- WAF custom rules

### v0.5.0 - Analytics & Reporting (Planned)

- Dashboard analytics queries
- Request/bandwidth/threat metrics
- Time-range filtering
- CSV/JSON report export

### v0.6.0 - Workers & Edge (Planned)

- Worker script deployment
- Worker log tailing
- KV namespace management
- Durable Objects support

### v1.0.0 - Full Integration (Planned)

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
- **Cloudflare API Docs:** <https://developers.cloudflare.com/api/>

### 💖 Sponsor This Project

If you find CFAD useful, consider supporting its development:

[![GitHub Sponsors](https://img.shields.io/github/sponsors/davidcforbes?style=for-the-badge&logo=github&label=Sponsor&color=EA4AAA)](https://github.com/sponsors/davidcforbes)

Your sponsorship helps:
- 🚀 Accelerate new feature development
- 🐛 Improve bug fixes and maintenance
- 📚 Enhance documentation and examples
- 🔧 Add support for more Cloudflare services

[Become a sponsor](https://github.com/sponsors/davidcforbes) to get priority support and influence the roadmap!

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

### v0.3.0 (2026-02-04)

**D1 Database Support:**

- ✅ D1 database CRUD operations (list, show, create, update, delete)
- ✅ SQL query execution (query, query-file, raw mode)
- ✅ Database export to SQL
- ✅ Database import from SQL files
- ✅ Time Travel support (bookmark, restore)

**R2 Storage Support:**

- ✅ R2 bucket management (list, show, create, delete)
- ✅ CORS configuration (show, set, delete)
- ✅ Custom domain management (list, show, add, update, delete)
- ✅ Public access via r2.dev (show, enable, disable)
- ✅ Lifecycle rules (show, set)
- ✅ Bucket locks / Object Lock (show, enable, disable)
- ✅ Storage metrics across all buckets
- ✅ Sippy incremental migration (show, enable, disable)
- ✅ Event notifications (list, show, create, delete)
- ✅ Super Slurper bulk migration (list, show, create, pause, resume, abort, progress, logs)
- ✅ Temporary credentials generation

**Technical:**

- Added `src/api/d1.rs` - D1 API models
- Added `src/api/r2.rs` - R2 API models
- Added `src/ops/d1.rs` - D1 operations
- Added `src/ops/r2.rs` - R2 operations (40+ functions)
- Added `src/cli/d1.rs` - D1 CLI commands
- Added `src/cli/r2.rs` - R2 CLI commands with subcommands
- Updated `src/output/table.rs` - D1/R2 table formatters
- Zero compilation errors/warnings
- Zero clippy warnings
- All 126 tests passing

---

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
