//! Integration tests for the `cfad::runner` dispatch layer.
//!
//! Each `handle_*` function was previously inlined in `main.rs` and had no
//! coverage. After the refactor extracting dispatch into `src/runner.rs`,
//! these tests exercise the top-level command handlers end-to-end with a
//! mock Cloudflare API.

use cfad::cli;
use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::runner;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn mock_client(mock_server: &MockServer) -> CloudflareClient {
    CloudflareClient::new_with_base_url(
        AuthMethod::ApiToken("test_token".to_string()),
        mock_server.uri(),
    )
    .unwrap()
}

fn zone_body() -> serde_json::Value {
    serde_json::json!({
        "id": "zone123abc",
        "name": "example.com",
        "status": "active",
        "paused": false,
        "development_mode": 0,
        "name_servers": ["ns1.cloudflare.com"],
        "original_name_servers": [],
        "owner": {"id": "o1", "type": "user", "email": "u@example.com"},
        "account": {"id": "a1", "name": "Test"},
        "created_on": "2026-01-01T00:00:00Z",
        "modified_on": "2026-01-01T00:00:00Z"
    })
}

fn dns_record_body() -> serde_json::Value {
    serde_json::json!({
        "id": "rec1",
        "zone_id": "zone123abc",
        "zone_name": "example.com",
        "type": "A",
        "name": "www.example.com",
        "content": "203.0.113.1",
        "ttl": 3600,
        "proxiable": true,
        "proxied": false,
        "locked": false,
        "created_on": "2026-01-01T00:00:00Z",
        "modified_on": "2026-01-01T00:00:00Z"
    })
}

// ------------------ DNS handler coverage ------------------

#[tokio::test]
async fn test_handle_dns_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/zones/zone123abc/dns_records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [dns_record_body()]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::List {
        zone: "example.com".to_string(),
        r#type: None,
        name: None,
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_dns_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/zones/zone123abc/dns_records/rec1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": dns_record_body()
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::Show {
        zone: "example.com".to_string(),
        record_id: "rec1".to_string(),
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_dns_add_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/dns_records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": dns_record_body()
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::Add {
        zone: "example.com".to_string(),
        r#type: "A".to_string(),
        name: "www".to_string(),
        content: "203.0.113.1".to_string(),
        ttl: 3600,
        proxied: false,
        priority: None,
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_dns_update_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/zones/zone123abc/dns_records/rec1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": dns_record_body()
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::Update {
        zone: "example.com".to_string(),
        record_id: "rec1".to_string(),
        name: Some("www2".to_string()),
        content: Some("203.0.113.2".to_string()),
        ttl: Some(7200),
        proxied: Some(true),
        priority: None,
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_dns_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::Delete {
        zone: "example.com".to_string(),
        record_id: "rec1".to_string(),
        confirm: false,
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_dns_delete_with_confirm_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/zones/zone123abc/dns_records/rec1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::dns::DnsCommand::Delete {
        zone: "example.com".to_string(),
        record_id: "rec1".to_string(),
        confirm: true,
    };
    assert!(runner::handle_dns_command(&client, cmd).await.is_ok());
}

// ------------------ Zone handler coverage ------------------

#[tokio::test]
async fn test_handle_zone_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::List { status: None };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_zone_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Show {
        zone: "example.com".to_string(),
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_zone_create_missing_account_errors() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Create {
        zone: "new.example.com".to_string(),
        account_id: None,
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_zone_create_with_account_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": zone_body()
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Create {
        zone: "example.com".to_string(),
        account_id: Some("acc123".to_string()),
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_zone_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Delete {
        zone_id: "zone123".to_string(),
        confirm: false,
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_zone_delete_with_confirm_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/zones/zone123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Delete {
        zone_id: "zone123".to_string(),
        confirm: true,
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_zone_settings_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/zones/zone123abc/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [
                {"id": "ssl", "value": "flexible"},
                {"id": "security_level", "value": "high"},
                {"id": "development_mode", "value": "off"},
                {"id": "ipv6", "value": "on"},
                {"id": "minify", "value": {"css": "on"}},
                {"id": "http3", "value": true},
                {"id": "cache_level", "value": "aggressive"},
                {"id": "numeric_setting", "value": 42},
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Settings {
        zone: "example.com".to_string(),
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_zone_update_dispatches_all_settings() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    for setting in [
        "security_level",
        "cache_level",
        "development_mode",
        "ipv6",
        "ssl",
        "always_use_https",
    ] {
        Mock::given(method("PATCH"))
            .and(path(format!("/zones/zone123abc/settings/{}", setting)))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true, "errors": [], "messages": [],
                "result": {"value": "on"}
            })))
            .mount(&mock_server)
            .await;
    }

    let client = mock_client(&mock_server).await;
    let cmd = cli::zone::ZoneCommand::Update {
        zone: "example.com".to_string(),
        security_level: Some("high".to_string()),
        cache_level: Some("aggressive".to_string()),
        dev_mode: Some("on".to_string()),
        ipv6: Some("on".to_string()),
        ssl: Some("flexible".to_string()),
        always_https: Some("on".to_string()),
    };
    assert!(runner::handle_zone_command(&client, cmd).await.is_ok());
}

// ------------------ Cache handler coverage ------------------

#[tokio::test]
async fn test_handle_cache_purge_all_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {"id": "zone123abc"}
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: true,
        files: None,
        tags: None,
        hosts: None,
        prefixes: None,
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_cache_purge_no_options_errors() {
    // No --all/--files/--tags/--hosts/--prefixes -> validation error
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: false,
        files: None,
        tags: None,
        hosts: None,
        prefixes: None,
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_err());
}

// ------------------ D1 handler coverage ------------------

fn d1_db_body(uuid: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "uuid": uuid,
        "name": name,
        "version": "production",
        "num_tables": 1,
        "file_size": 1024,
        "created_at": "2026-01-01T00:00:00Z"
    })
}

#[tokio::test]
async fn test_handle_d1_list_dispatches() {
    std::env::remove_var("CLOUDFLARE_ACCOUNT_ID");
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::List {
        account_id: Some("acc1".to_string()),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_show_dispatches() {
    let mock_server = MockServer::start().await;
    // Show resolves DB ID first (list + find)
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database/db-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": d1_db_body("db-1", "my-db")
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Show {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_create_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": d1_db_body("db-1", "my-db")
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Create {
        account_id: Some("acc1".to_string()),
        name: "my-db".to_string(),
        location: Some("wnam".to_string()),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Delete {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        confirm: false,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_d1_query_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [{ "success": true, "results": [], "meta": {} }]
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Query {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        sql: "SELECT 1".to_string(),
        raw: false,
        format: "table".to_string(),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

// ------------------ Token handler coverage ------------------

#[tokio::test]
async fn test_handle_token_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/user/tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": []
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::List;
    assert!(runner::handle_token_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_token_verify_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/user/tokens/verify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "tok-1", "status": "active", "not_before": null, "expires_on": null }
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::Verify;
    assert!(runner::handle_token_command(&client, cmd).await.is_ok());
}

// ------------------ Pages handler coverage ------------------

#[tokio::test]
async fn test_handle_pages_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/pages/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": []
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::List {
        account_id: Some("acc1".to_string()),
    };
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

// ------------------ R2 handler coverage ------------------

#[tokio::test]
async fn test_handle_r2_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "buckets": [] }
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::List {
        account_id: Some("acc1".to_string()),
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

// ------------------ Cache purge with other flavors ------------------

#[tokio::test]
async fn test_handle_cache_purge_files_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {"id": "zone123abc"}
        })))
        .mount(&mock_server)
        .await;

    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: false,
        files: Some(vec!["https://example.com/a.js".to_string()]),
        tags: None,
        hosts: None,
        prefixes: None,
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_cache_purge_tags_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": {"id": "zone123abc"}
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: false,
        files: None,
        tags: Some(vec!["tag1".to_string()]),
        hosts: None,
        prefixes: None,
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_cache_purge_hosts_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": {"id": "zone123abc"}
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: false,
        files: None,
        tags: None,
        hosts: Some(vec!["example.com".to_string()]),
        prefixes: None,
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_cache_purge_prefixes_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [zone_body()]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/zones/zone123abc/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": {"id": "zone123abc"}
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::cache::CacheCommand::Purge {
        zone: "example.com".to_string(),
        all: false,
        files: None,
        tags: None,
        hosts: None,
        prefixes: Some(vec!["/api/".to_string()]),
    };
    assert!(runner::handle_cache_command(&client, cmd).await.is_ok());
}

// ------------------ D1 extended handler coverage ------------------

#[tokio::test]
async fn test_handle_d1_update_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/accounts/acc1/d1/database/db-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": d1_db_body("db-1", "renamed")
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Update {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        name: Some("renamed".to_string()),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_delete_confirmed_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/accounts/acc1/d1/database/db-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Delete {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        confirm: true,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_query_raw_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/raw"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [{ "success": true, "columns": [], "rows": [], "meta": {} }]
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Query {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        sql: "SELECT 1".to_string(),
        raw: true,
        format: "table".to_string(),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_query_json_format_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [{ "success": true, "results": [], "meta": {} }]
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Query {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        sql: "SELECT 1".to_string(),
        raw: false,
        format: "json".to_string(),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_bookmark_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database/db-1/time_travel/bookmark"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "bookmark": "bk-1", "timestamp": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Bookmark {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        timestamp: None,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_restore_requires_confirm() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Restore {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        bookmark: Some("bk1".to_string()),
        timestamp: None,
        confirm: false,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_d1_export_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/export"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "task_id": "t1", "status": "processing", "signed_url": null }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Export {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

// ------------------ R2 sub-command handler coverage ------------------

#[tokio::test]
async fn test_handle_r2_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "name": "b1", "creation_date": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_create_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/r2/buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "name": "b1", "creation_date": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Create {
        account_id: Some("acc1".to_string()),
        name: "b1".to_string(),
        location: None,
        storage_class: None,
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Delete {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        confirm: false,
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_r2_metrics_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "buckets": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Metrics {
        account_id: Some("acc1".to_string()),
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

// ------------------ Pages sub-command handler coverage ------------------

#[tokio::test]
async fn test_handle_pages_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/pages/projects/p1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {
                "id": "p1", "name": "p1", "subdomain": "p1.pages.dev",
                "domains": [], "created_on": "2026-01-01T00:00:00Z",
                "production_branch": "main"
            }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Show {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
    };
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Delete {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        confirm: false,
    };
    assert!(runner::handle_pages_command(&client, cmd).await.is_err());
}

// ------------------ R2 Cors / Domain / PublicAccess / Lifecycle / Lock / Sippy / Notifications / Migrate ------------------

#[tokio::test]
async fn test_handle_r2_cors_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/cors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "rules": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Cors(cli::r2::R2CorsCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_cors_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Cors(cli::r2::R2CorsCommand::Delete {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        confirm: false,
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_r2_domain_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/domains/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Domain(cli::r2::R2DomainCommand::List {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_public_access_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/domains/managed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "enabled": true, "domain": "pub-abc.r2.dev" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::PublicAccess(cli::r2::R2PublicAccessCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_public_access_enable_disable_dispatch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/accounts/acc1/r2/buckets/b1/domains/managed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "enabled": true, "domain": "pub-abc.r2.dev" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let enable = cli::r2::R2Command::PublicAccess(cli::r2::R2PublicAccessCommand::Enable {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, enable).await.is_ok());
    let disable = cli::r2::R2Command::PublicAccess(cli::r2::R2PublicAccessCommand::Disable {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, disable).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_lifecycle_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/lifecycle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "rules": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Lifecycle(cli::r2::R2LifecycleCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_lock_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/lock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "enabled": false }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Lock(cli::r2::R2LockCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_sippy_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/r2/buckets/b1/sippy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "enabled": false }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Sippy(cli::r2::R2SippyCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_notifications_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc1/event_notifications/r2/b1/configuration",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Notifications(cli::r2::R2NotificationCommand::List {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_migrate_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/slurper/jobs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::List {
        account_id: Some("acc1".to_string()),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

// ------------------ Pages Deployment / Domain sub-commands ------------------

#[tokio::test]
async fn test_handle_pages_deployment_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/pages/projects/p1/deployments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::List {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_domain_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/pages/projects/p1/domains"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Domain(cli::pages::DomainCommand::List {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

// ------------------ Token additional ------------------

#[tokio::test]
async fn test_handle_token_permissions_list_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/user/tokens/permission_groups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::Permissions { scope: None };
    assert!(runner::handle_token_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_token_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::Delete {
        token_id: "tok1".to_string(),
        confirm: false,
    };
    assert!(runner::handle_token_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_token_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/user/tokens/tok1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "tok1", "name": "Test", "status": "active", "policies": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::Show {
        token_id: "tok1".to_string(),
    };
    assert!(runner::handle_token_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_token_delete_confirmed_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/user/tokens/tok1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::token::TokenCommand::Delete {
        token_id: "tok1".to_string(),
        confirm: true,
    };
    assert!(runner::handle_token_command(&client, cmd).await.is_ok());
}

// ------------------ Additional D1 ------------------

#[tokio::test]
async fn test_handle_d1_schema_all_tables_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [{
                "success": true,
                "results": [{"name": "users", "sql": "CREATE TABLE users(id INTEGER)"}],
                "meta": {}
            }]
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Schema {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        table: None,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_schema_specific_table_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": [{
                "success": true,
                "results": [
                    {"name": "id", "type": "INTEGER", "notnull": 1, "pk": 1, "dflt_value": null},
                    {"name": "name", "type": "TEXT", "notnull": 0, "pk": 0, "dflt_value": "'unknown'"}
                ],
                "meta": {}
            }]
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Schema {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        table: Some("users".to_string()),
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_d1_restore_confirmed_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": [d1_db_body("db-1", "my-db")]
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/d1/database/db-1/time_travel/restore"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "restored_bookmark": "bk2", "timestamp": "2026-01-02T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::d1::D1Command::Restore {
        account_id: Some("acc1".to_string()),
        database_id: "db-1".to_string(),
        bookmark: Some("bk2".to_string()),
        timestamp: None,
        confirm: true,
    };
    assert!(runner::handle_d1_command(&client, cmd).await.is_ok());
}

// ------------------ R2 additional ------------------

#[tokio::test]
async fn test_handle_r2_delete_confirmed_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/accounts/acc1/r2/buckets/b1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Delete {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        confirm: true,
    };
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_domain_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc1/r2/buckets/b1/domains/custom/cdn.example.com",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "domain": "cdn.example.com", "enabled": true, "status": "active" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Domain(cli::r2::R2DomainCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        domain: "cdn.example.com".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_lock_enable_disable_dispatch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/accounts/acc1/r2/buckets/b1/lock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let enable = cli::r2::R2Command::Lock(cli::r2::R2LockCommand::Enable {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        mode: "governance".to_string(),
        days: Some(30),
    });
    assert!(runner::handle_r2_command(&client, enable).await.is_ok());
    let disable = cli::r2::R2Command::Lock(cli::r2::R2LockCommand::Disable {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        confirm: true,
    });
    assert!(runner::handle_r2_command(&client, disable).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_sippy_disable_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/accounts/acc1/r2/buckets/b1/sippy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Sippy(cli::r2::R2SippyCommand::Disable {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        confirm: true,
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_notifications_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc1/event_notifications/r2/b1/configuration/queues/q1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "queueId": "q1", "rules": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Notifications(cli::r2::R2NotificationCommand::Show {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        queue_id: "q1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_notifications_delete_confirmed_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/accounts/acc1/event_notifications/r2/b1/configuration/queues/q1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Notifications(cli::r2::R2NotificationCommand::Delete {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        queue_id: "q1".to_string(),
        confirm: true,
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_migrate_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/slurper/jobs/j1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "j1", "status": "running", "filesMigrated": 0, "bytesMigrated": 0, "filesSkipped": 0, "filesFailed": 0 }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Show {
        account_id: Some("acc1".to_string()),
        job_id: "j1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

// ------------------ Pages deploy/domain additional ------------------

#[tokio::test]
async fn test_handle_pages_deploy_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/pages/projects/p1/deployments/d1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {
                "id": "d1", "short_id": "abc", "project_id": "p1", "project_name": "p1",
                "environment": "production", "url": "https://abc.p1.pages.dev",
                "created_on": "2026-01-01T00:00:00Z", "modified_on": "2026-01-01T00:00:00Z",
                "is_skipped": false,
                "latest_stage": { "name": "deploy", "status": "success", "started_on": "2026-01-01T00:00:00Z", "ended_on": "2026-01-01T00:05:00Z" },
                "deployment_trigger": { "type": "push", "metadata": { "branch": "main", "commit_hash": "abc", "commit_message": "m" } }
            }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Show {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        deployment_id: "d1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_domain_show_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc1/pages/projects/p1/domains/custom.example.com",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "dm1", "name": "custom.example.com", "status": "active",
                        "verification_data": {"status": "active"},
                        "validation_data": {"status": "active"},
                        "created_on": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Domain(cli::pages::DomainCommand::Show {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        domain: "custom.example.com".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_purge_cache_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/pages/projects/p1/purge_build_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": null
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::PurgeCache {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
    };
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

// ------------------ print_help_json + build_command_json ------------------

#[test]
fn test_build_command_json_produces_structure() {
    use clap::CommandFactory;
    let cmd = cli::Cli::command();
    let json = runner::build_command_json(&cmd);
    assert!(json.get("name").is_some());
    assert!(json.get("options").is_some());
    assert!(json.get("subcommands").is_some());
    let subs = json["subcommands"].as_array().unwrap();
    assert!(subs.len() >= 7);
}

#[test]
fn test_print_help_json_does_not_panic() {
    runner::print_help_json();
}

// ------------------ Pages deploy/domain sub-command bulk coverage ------------------

#[tokio::test]
async fn test_handle_pages_deploy_create_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/pages/projects/p1/deployments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {
                "id": "d1", "short_id": "abc", "project_id": "p1", "project_name": "p1",
                "environment": "production", "url": "https://abc.p1.pages.dev",
                "created_on": "2026-01-01T00:00:00Z", "modified_on": "2026-01-01T00:00:00Z",
                "is_skipped": false,
                "latest_stage": { "name": "deploy", "status": "success", "started_on": "2026-01-01T00:00:00Z", "ended_on": "2026-01-01T00:05:00Z" },
                "deployment_trigger": { "type": "push", "metadata": { "branch": "main", "commit_hash": "abc", "commit_message": "m" } }
            }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Create {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_deploy_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Delete {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        deployment_id: "d1".to_string(),
        confirm: false,
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_pages_deploy_retry_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/pages/projects/p1/deployments/d1/retry"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": {
                "id": "d1", "short_id": "abc", "project_id": "p1", "project_name": "p1",
                "environment": "production", "url": "https://abc.p1.pages.dev",
                "created_on": "2026-01-01T00:00:00Z", "modified_on": "2026-01-01T00:00:00Z",
                "is_skipped": false,
                "latest_stage": { "name": "deploy", "status": "success", "started_on": "2026-01-01T00:00:00Z", "ended_on": "2026-01-01T00:05:00Z" },
                "deployment_trigger": { "type": "push", "metadata": { "branch": "main", "commit_hash": "abc", "commit_message": "m" } }
            }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Retry {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        deployment_id: "d1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_deploy_rollback_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Rollback {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        deployment_id: "d1".to_string(),
        confirm: false,
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_err());
}

#[tokio::test]
async fn test_handle_pages_deploy_logs_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc1/pages/projects/p1/deployments/d1/history/logs",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "total": 0, "includes_container_logs": false, "data": [] }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Deploy(cli::pages::DeployCommand::Logs {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        deployment_id: "d1".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_domain_add_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/pages/projects/p1/domains"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "dm1", "name": "custom.example.com", "status": "pending",
                        "verification_data": { "status": "pending" },
                        "validation_data": { "status": "pending" },
                        "created_on": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Domain(cli::pages::DomainCommand::Add {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        domain: "custom.example.com".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_domain_verify_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path(
            "/accounts/acc1/pages/projects/p1/domains/custom.example.com",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "id": "dm1", "name": "custom.example.com", "status": "active",
                        "verification_data": { "status": "active" },
                        "validation_data": { "status": "active" },
                        "created_on": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Domain(cli::pages::DomainCommand::Verify {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        domain: "custom.example.com".to_string(),
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_pages_domain_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::pages::PagesCommand::Domain(cli::pages::DomainCommand::Delete {
        account_id: Some("acc1".to_string()),
        project: "p1".to_string(),
        domain: "custom.example.com".to_string(),
        confirm: false,
    });
    assert!(runner::handle_pages_command(&client, cmd).await.is_err());
}

// ------------------ R2 migrate sub-command bulk coverage ------------------

#[tokio::test]
async fn test_handle_r2_migrate_pause_resume_abort() {
    let mock_server = MockServer::start().await;
    for action in ["pause", "resume", "abort"] {
        Mock::given(method("PUT"))
            .and(path(format!("/accounts/acc1/slurper/jobs/j1/{}", action)))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true, "errors": [], "messages": [], "result": null
            })))
            .mount(&mock_server)
            .await;
    }
    let client = mock_client(&mock_server).await;
    for cmd in [
        cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Pause {
            account_id: Some("acc1".to_string()),
            job_id: "j1".to_string(),
        }),
        cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Resume {
            account_id: Some("acc1".to_string()),
            job_id: "j1".to_string(),
        }),
        cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Abort {
            account_id: Some("acc1".to_string()),
            job_id: "j1".to_string(),
            confirm: true,
        }),
    ] {
        assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
    }
}

#[tokio::test]
async fn test_handle_r2_migrate_progress_and_logs() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/slurper/jobs/j1/progress"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "totalFiles": 0, "processedFiles": 0, "errorCount": 0,
                        "totalBytes": 0, "processedBytes": 0,
                        "estimatedCompletion": null, "startedAt": "2026-01-01T00:00:00Z" }
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/accounts/acc1/slurper/jobs/j1/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [], "result": []
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let progress = cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Progress {
        account_id: Some("acc1".to_string()),
        job_id: "j1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, progress).await.is_ok());
    let logs = cli::r2::R2Command::Migrate(cli::r2::R2MigrateCommand::Logs {
        account_id: Some("acc1".to_string()),
        job_id: "j1".to_string(),
    });
    assert!(runner::handle_r2_command(&client, logs).await.is_ok());
}

// ------------------ R2 domain add/update/delete ------------------

#[tokio::test]
async fn test_handle_r2_domain_add_dispatches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/accounts/acc1/r2/buckets/b1/domains/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true, "errors": [], "messages": [],
            "result": { "domain": "cdn.example.com", "enabled": false, "status": "pending" }
        })))
        .mount(&mock_server)
        .await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Domain(cli::r2::R2DomainCommand::Add {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        domain: "cdn.example.com".to_string(),
        zone_id: None,
        min_tls: None,
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_ok());
}

#[tokio::test]
async fn test_handle_r2_domain_delete_requires_confirm() {
    let mock_server = MockServer::start().await;
    let client = mock_client(&mock_server).await;
    let cmd = cli::r2::R2Command::Domain(cli::r2::R2DomainCommand::Delete {
        account_id: Some("acc1".to_string()),
        bucket: "b1".to_string(),
        domain: "cdn.example.com".to_string(),
        confirm: false,
    });
    assert!(runner::handle_r2_command(&client, cmd).await.is_err());
}
