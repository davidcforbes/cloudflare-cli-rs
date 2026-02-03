#!/usr/bin/env bash
# Quality Check Script - Run before pushing to catch Codacy issues
# Usage: ./scripts/quality-check.sh

set -e

CYAN='\033[0;36m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
GRAY='\033[0;37m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
echo "========================================"
echo "Running Code Quality Checks"
echo "========================================"
echo -e "${NC}"

failed=false

# 1. Cargo Format Check
echo -e "${YELLOW}1. Checking code formatting...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}   ✓ Format check passed${NC}"
else
    echo -e "${RED}   ✗ Format check failed${NC}"
    echo -e "${GRAY}   Run: cargo fmt --all${NC}"
    failed=true
fi

# 2. Clippy Lints
echo -e "\n${YELLOW}2. Running Clippy...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}   ✓ Clippy passed${NC}"
else
    echo -e "${RED}   ✗ Clippy found issues${NC}"
    failed=true
fi

# 3. Tests
echo -e "\n${YELLOW}3. Running tests...${NC}"
if cargo test --quiet; then
    echo -e "${GREEN}   ✓ All tests passed${NC}"
else
    echo -e "${RED}   ✗ Tests failed${NC}"
    failed=true
fi

# 4. Markdown Linting (non-blocking)
echo -e "\n${YELLOW}4. Checking markdown files...${NC}"
if command -v markdownlint &> /dev/null; then
    if markdownlint "**/*.md" --ignore node_modules --ignore target --ignore CHANGELOG.md; then
        echo -e "${GREEN}   ✓ Markdown linting passed${NC}"
    else
        echo -e "${YELLOW}   ⚠ Markdown linting found issues (non-blocking)${NC}"
    fi
else
    echo -e "${YELLOW}   ⚠ markdownlint not installed (npm install -g markdownlint-cli)${NC}"
fi

# 5. Build Check
echo -e "\n${YELLOW}5. Building release binary...${NC}"
if cargo build --release --quiet; then
    echo -e "${GREEN}   ✓ Build successful${NC}"
else
    echo -e "${RED}   ✗ Build failed${NC}"
    failed=true
fi

# Summary
echo -e "\n${CYAN}========================================${NC}"
if [ "$failed" = true ]; then
    echo -e "${RED}✗ Quality checks FAILED${NC}"
    echo -e "${CYAN}========================================${NC}\n"
    exit 1
else
    echo -e "${GREEN}✓ All quality checks PASSED${NC}"
    echo -e "${CYAN}========================================${NC}\n"
    exit 0
fi
