//! CLI integration tests
//!
//! Tests that verify the CLI binary works correctly with all subcommands and flags.

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cfad() -> Command {
    Command::cargo_bin("cfad").unwrap()
}

// =============================================================================
// Help and Version Tests
// =============================================================================

#[test]
fn test_help_flag() {
    cfad()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloudflare DNS"))
        .stdout(predicate::str::contains("dns"))
        .stdout(predicate::str::contains("zone"))
        .stdout(predicate::str::contains("cache"))
        .stdout(predicate::str::contains("d1"))
        .stdout(predicate::str::contains("r2"));
}

#[test]
fn test_version_flag() {
    cfad()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("cfad"));
}

// =============================================================================
// Subcommand Recognition Tests
// =============================================================================

#[test]
fn test_dns_subcommand_recognized() {
    cfad()
        .arg("dns")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("DNS record management"));
}

#[test]
fn test_zone_subcommand_recognized() {
    cfad()
        .arg("zone")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Zone management"));
}

#[test]
fn test_cache_subcommand_recognized() {
    cfad()
        .arg("cache")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cache management"));
}

#[test]
fn test_d1_subcommand_recognized() {
    cfad()
        .arg("d1")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("D1 database management"));
}

#[test]
fn test_r2_subcommand_recognized() {
    cfad()
        .arg("r2")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("R2 object storage management"));
}

#[test]
fn test_config_subcommand_recognized() {
    cfad()
        .arg("config")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration management"));
}

// =============================================================================
// Invalid Subcommand Tests
// =============================================================================

#[test]
fn test_invalid_subcommand_error() {
    cfad()
        .arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

// =============================================================================
// DNS Subcommand Tests
// =============================================================================

#[test]
fn test_dns_list_help() {
    cfad()
        .args(["dns", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ZONE"));
}

#[test]
fn test_dns_show_help() {
    cfad()
        .args(["dns", "show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ZONE"))
        .stdout(predicate::str::contains("RECORD_ID"));
}

#[test]
fn test_dns_add_help() {
    cfad()
        .args(["dns", "add", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TYPE"))
        .stdout(predicate::str::contains("CONTENT"));
}

#[test]
fn test_dns_import_help() {
    cfad()
        .args(["dns", "import", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ZONE"))
        .stdout(predicate::str::contains("FILE"));
}

// =============================================================================
// Zone Subcommand Tests
// =============================================================================

#[test]
fn test_zone_list_help() {
    cfad()
        .args(["zone", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--status"));
}

#[test]
fn test_zone_show_help() {
    cfad()
        .args(["zone", "show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ZONE"));
}

#[test]
fn test_zone_create_help() {
    cfad()
        .args(["zone", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--account-id"));
}

#[test]
fn test_zone_delete_help() {
    cfad()
        .args(["zone", "delete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--confirm"));
}

// =============================================================================
// D1 Subcommand Tests
// =============================================================================

#[test]
fn test_d1_list_help() {
    cfad()
        .args(["d1", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--account-id"));
}

#[test]
fn test_d1_show_help() {
    cfad()
        .args(["d1", "show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DATABASE_ID"));
}

#[test]
fn test_d1_create_help() {
    cfad()
        .args(["d1", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"))
        .stdout(predicate::str::contains("--location"));
}

#[test]
fn test_d1_query_help() {
    cfad()
        .args(["d1", "query", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SQL"))
        .stdout(predicate::str::contains("--raw"));
}

#[test]
fn test_d1_export_help() {
    cfad()
        .args(["d1", "export", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DATABASE_ID"));
}

#[test]
fn test_d1_import_help() {
    cfad()
        .args(["d1", "import", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FILE"));
}

// =============================================================================
// R2 Subcommand Tests
// =============================================================================

#[test]
fn test_r2_list_help() {
    cfad()
        .args(["r2", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--account-id"));
}

#[test]
fn test_r2_create_help() {
    cfad()
        .args(["r2", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"))
        .stdout(predicate::str::contains("--location"));
}

#[test]
fn test_r2_cors_help() {
    cfad()
        .args(["r2", "cors", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("set"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn test_r2_domain_help() {
    cfad()
        .args(["r2", "domain", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"));
}

#[test]
fn test_r2_metrics_help() {
    cfad()
        .args(["r2", "metrics", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--account-id"));
}

// =============================================================================
// Cache Subcommand Tests
// =============================================================================

#[test]
fn test_cache_purge_help() {
    cfad()
        .args(["cache", "purge", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--all"))
        .stdout(predicate::str::contains("--files"))
        .stdout(predicate::str::contains("--tags"));
}

// =============================================================================
// Config Subcommand Tests
// =============================================================================

#[test]
fn test_config_init_help() {
    cfad().args(["config", "init", "--help"]).assert().success();
}

#[test]
fn test_config_show_help() {
    cfad()
        .args(["config", "show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PROFILE"));
}

#[test]
fn test_config_profiles_help() {
    cfad()
        .args(["config", "profiles", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"));
}

// =============================================================================
// Global Flag Tests
// =============================================================================

#[test]
fn test_global_format_flag() {
    cfad()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("table"))
        .stdout(predicate::str::contains("json"));
}

#[test]
fn test_global_profile_flag() {
    cfad()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--profile"));
}

#[test]
fn test_global_quiet_flag() {
    cfad()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--quiet"));
}

#[test]
fn test_global_verbose_flag() {
    cfad()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--verbose"));
}

// =============================================================================
// Error Message Tests
// =============================================================================

#[test]
fn test_missing_required_arg_dns_list() {
    cfad()
        .args(["dns", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ZONE"));
}

#[test]
fn test_missing_required_arg_dns_show() {
    cfad()
        .args(["dns", "show", "example.com"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("RECORD_ID"));
}

#[test]
fn test_missing_required_arg_d1_show() {
    cfad()
        .args(["d1", "show"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("DATABASE_ID"));
}

// =============================================================================
// Token Subcommand Tests
// =============================================================================

#[test]
fn test_token_subcommand_recognized() {
    cfad()
        .arg("token")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("API token management"));
}

#[test]
fn test_token_list_help() {
    cfad()
        .args(["token", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List all API tokens"));
}

#[test]
fn test_token_show_help() {
    cfad()
        .args(["token", "show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TOKEN_ID"));
}

#[test]
fn test_token_create_help() {
    cfad()
        .args(["token", "create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"))
        .stdout(predicate::str::contains("--permissions"));
}

#[test]
fn test_token_update_help() {
    cfad()
        .args(["token", "update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TOKEN_ID"));
}

#[test]
fn test_token_delete_help() {
    cfad()
        .args(["token", "delete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--confirm"));
}

#[test]
fn test_token_verify_help() {
    cfad()
        .args(["token", "verify", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verify the current token"));
}

#[test]
fn test_token_permissions_help() {
    cfad()
        .args(["token", "permissions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--scope"));
}

#[test]
fn test_token_roll_help() {
    cfad()
        .args(["token", "roll", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--confirm"));
}
