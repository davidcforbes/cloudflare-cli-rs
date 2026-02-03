# Release Process

This document outlines the automated release process for CFAD.

## Prerequisites

- Ensure all tests pass: `cargo test`
- Ensure no clippy warnings: `cargo clippy -- -D warnings`
- Ensure code is formatted: `cargo fmt --check`
- Update version in `Cargo.toml`
- Update `CHANGELOG.md` with release notes
- Update version references in `README.md`

## Release Steps

### 1. Prepare the Release

```bash
# Update version in Cargo.toml
# Current: version = "0.2.0"
# New:     version = "0.3.0"

# Update CHANGELOG.md
# Add new section for the version with changes

# Update version badges in README.md if needed
```

### 2. Commit and Tag

```bash
# Commit version changes
git add Cargo.toml CHANGELOG.md README.md
git commit -m "chore: bump version to 0.3.0"

# Create and push tag (this triggers the release workflow)
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin master
git push origin v0.3.0
```

### 3. Automated Build Process

Once you push the tag, GitHub Actions will automatically:

1. ✅ Create a GitHub Release draft
2. ✅ Build binaries for all platforms:
   - Windows (x86_64-pc-windows-msvc)
   - Linux GNU (x86_64-unknown-linux-gnu)
   - Linux musl/static (x86_64-unknown-linux-musl)
   - macOS Intel (x86_64-apple-darwin)
   - macOS Apple Silicon (aarch64-apple-darwin)
3. ✅ Generate SHA256 checksums for each binary
4. ✅ Upload all artifacts to the GitHub Release
5. ✅ Create a unified checksums.txt file

### 4. Monitor the Release

```bash
# Watch the GitHub Actions workflow
# Visit: https://github.com/davidcforbes/cfad/actions

# The workflow takes approximately 10-15 minutes to complete
```

### 5. Verify the Release

After the workflow completes:

1. Visit the [Releases page](https://github.com/davidcforbes/cfad/releases)
2. Verify all binaries are attached:
   - ✅ cfad-X.X.X-x86_64-pc-windows-msvc.zip
   - ✅ cfad-X.X.X-x86_64-unknown-linux-gnu.tar.gz
   - ✅ cfad-X.X.X-x86_64-unknown-linux-musl.tar.gz
   - ✅ cfad-X.X.X-x86_64-apple-darwin.tar.gz
   - ✅ cfad-X.X.X-aarch64-apple-darwin.tar.gz
   - ✅ checksums.txt
3. Download and test a binary for your platform
4. Edit the release notes if needed

### 6. Announce the Release

- Update README.md badges if needed
- Announce on social media/forums
- Update documentation site (if applicable)

## Manual Release (Fallback)

If automated release fails, you can build manually:

```bash
# Install cross-compilation targets
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for target platform
cargo build --release --target <target>

# Create archive
cd target/<target>/release
tar czf cfad-<version>-<target>.tar.gz cfad

# Generate checksum
shasum -a 256 cfad-<version>-<target>.tar.gz > cfad-<version>-<target>.tar.gz.sha256

# Upload manually to GitHub Release
```

## Troubleshooting

### Workflow Fails

1. Check the GitHub Actions logs
2. Ensure GITHUB_TOKEN has proper permissions
3. Verify tag format matches `v*.*.*` pattern
4. Check cargo.toml syntax

### Build Fails for Specific Platform

1. Check platform-specific dependencies
2. Verify Rust toolchain supports the target
3. Check cross-compilation requirements (e.g., musl-tools for musl builds)

### Checksum Generation Fails

1. Verify the archive was created successfully
2. Check file permissions
3. Ensure shasum/Get-FileHash is available

## Version Numbering

Follow Semantic Versioning (SemVer):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

Examples:

- v0.2.0 → v0.3.0: Added firewall management features
- v0.3.0 → v0.3.1: Fixed authentication bug
- v0.9.0 → v1.0.0: First stable release with API stability guarantee

## Post-Release Checklist

- [ ] Verify release on GitHub
- [ ] Test binaries on each platform
- [ ] Update README badges
- [ ] Close milestone (if using GitHub milestones)
- [ ] Update project board (if using GitHub projects)
- [ ] Create announcement (social media, blog, etc.)
- [ ] Update documentation site

## Automation Details

The release workflow (`.github/workflows/release.yml`) is triggered by:

1. **Tag push**: Pushing a tag matching `v*.*.*`
2. **Manual trigger**: Using GitHub Actions "workflow_dispatch"

### Workflow Jobs

1. **create-release**: Creates the GitHub Release
2. **build**: Builds binaries for all platforms (runs in parallel)
3. **checksums**: Generates unified checksums file

### Build Matrix

| Target | OS | Archive | Notes |
| -------- | ------- | ------- | ------------------------------- |
| x86_64-pc-windows-msvc | Windows | zip | Standard Windows binary |
| x86_64-unknown-linux-gnu | Ubuntu | tar.gz | Standard Linux binary (glibc) |
| x86_64-unknown-linux-musl | Ubuntu | tar.gz | Static Linux binary (musl) |
| x86_64-apple-darwin | macOS | tar.gz | Intel Macs |
| aarch64-apple-darwin | macOS | tar.gz | M1/M2/M3 Macs |

## First Release (v0.2.0)

For the first automated release:

```bash
# Verify current state
git status
cargo test
cargo clippy

# Tag the current commit (already at v0.2.0)
git tag -a v0.2.0 -m "Release v0.2.0 - DNS features complete"
git push origin master
git push origin v0.2.0

# Monitor the release
# Visit: https://github.com/davidcforbes/cfad/actions
```

The workflow will build and publish v0.2.0 binaries automatically.
