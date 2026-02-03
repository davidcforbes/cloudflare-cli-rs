# Quality Check Script - Run before pushing to catch Codacy issues
# Usage: ./scripts/quality-check.ps1

$ErrorActionPreference = "Stop"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "Running Code Quality Checks" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

$failed = $false

# 1. Cargo Format Check
Write-Host "1. Checking code formatting..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ✗ Format check failed" -ForegroundColor Red
    Write-Host "   Run: cargo fmt --all" -ForegroundColor Gray
    $failed = $true
} else {
    Write-Host "   ✓ Format check passed" -ForegroundColor Green
}

# 2. Clippy Lints
Write-Host "`n2. Running Clippy..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ✗ Clippy found issues" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "   ✓ Clippy passed" -ForegroundColor Green
}

# 3. Tests
Write-Host "`n3. Running tests..." -ForegroundColor Yellow
cargo test --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ✗ Tests failed" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "   ✓ All tests passed" -ForegroundColor Green
}

# 4. Markdown Linting (non-blocking)
Write-Host "`n4. Checking markdown files..." -ForegroundColor Yellow
if (Get-Command markdownlint -ErrorAction SilentlyContinue) {
    markdownlint "**/*.md" --ignore node_modules --ignore target --ignore CHANGELOG.md
    if ($LASTEXITCODE -ne 0) {
        Write-Host "   ⚠ Markdown linting found issues (non-blocking)" -ForegroundColor Yellow
    } else {
        Write-Host "   ✓ Markdown linting passed" -ForegroundColor Green
    }
} else {
    Write-Host "   ⚠ markdownlint not installed (npm install -g markdownlint-cli)" -ForegroundColor Yellow
}

# 5. Build Check
Write-Host "`n5. Building release binary..." -ForegroundColor Yellow
cargo build --release --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ✗ Build failed" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "   ✓ Build successful" -ForegroundColor Green
}

# Summary
Write-Host "`n========================================" -ForegroundColor Cyan
if ($failed) {
    Write-Host "✗ Quality checks FAILED" -ForegroundColor Red
    Write-Host "========================================`n" -ForegroundColor Cyan
    exit 1
} else {
    Write-Host "✓ All quality checks PASSED" -ForegroundColor Green
    Write-Host "========================================`n" -ForegroundColor Cyan
    exit 0
}
