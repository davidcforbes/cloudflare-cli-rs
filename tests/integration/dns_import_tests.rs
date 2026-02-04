use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::dns;
use std::fs;
use std::path::PathBuf;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

fn create_temp_file(content: &str, extension: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir().join("cfad_dns_import_tests");
    fs::create_dir_all(&temp_dir).ok();

    // Use a unique filename based on thread ID and timestamp to avoid conflicts
    let thread_id = std::thread::current().id();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file_path = temp_dir.join(format!(
        "test_import_{:?}_{}.{}",
        thread_id, timestamp, extension
    ));
    fs::write(&file_path, content).unwrap();
    file_path
}

fn cleanup_temp_files() {
    let temp_dir = std::env::temp_dir().join("cfad_dns_import_tests");
    fs::remove_dir_all(&temp_dir).ok();
}

#[tokio::test]
async fn test_import_records_csv_success() {
    let mock_server = MockServer::start().await;

    // Mock multiple record creation
    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "test.example.com",
                "type": "A",
                "content": "192.0.2.1",
                "ttl": 3600,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-03T00:00:00Z",
                "modified_on": "2026-02-03T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .expect(2)
        .mount(&mock_server)
        .await;

    let csv_content = "type,name,content,ttl,proxied,priority\nA,test1.example.com,192.0.2.1,3600,false,\nA,test2.example.com,192.0.2.2,3600,false,";
    let file_path = create_temp_file(csv_content, "csv");

    let client = create_test_client(&mock_server).await;
    let result = dns::import_records(&client, "zone123", file_path.to_str().unwrap()).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total, 2);
    assert_eq!(stats.success, 2);
    assert_eq!(stats.failed, 0);

    cleanup_temp_files();
}

#[tokio::test]
async fn test_import_records_bind_format() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "www.example.com",
                "type": "A",
                "content": "192.0.2.1",
                "ttl": 3600,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-03T00:00:00Z",
                "modified_on": "2026-02-03T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let bind_content = "$ORIGIN example.com.\n$TTL 3600\nwww IN A 192.0.2.1";
    let file_path = create_temp_file(bind_content, "zone");

    let client = create_test_client(&mock_server).await;
    let result = dns::import_records(&client, "zone123", file_path.to_str().unwrap()).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total, 1);

    cleanup_temp_files();
}

#[tokio::test]
async fn test_import_records_with_failures() {
    let mock_server = MockServer::start().await;

    // First record succeeds
    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "test1.example.com",
                "type": "A",
                "content": "192.0.2.1",
                "ttl": 3600,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-03T00:00:00Z",
                "modified_on": "2026-02-03T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Second record fails
    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "success": false,
            "errors": [{
                "code": 81057,
                "message": "Record already exists"
            }],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let csv_content = "type,name,content,ttl,proxied,priority\nA,test1.example.com,192.0.2.1,3600,false,\nA,test2.example.com,192.0.2.2,3600,false,";
    let file_path = create_temp_file(csv_content, "csv");

    let client = create_test_client(&mock_server).await;
    let result = dns::import_records(&client, "zone123", file_path.to_str().unwrap()).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total, 2);
    assert_eq!(stats.success, 1);
    assert_eq!(stats.failed, 1);

    cleanup_temp_files();
}

#[tokio::test]
async fn test_import_records_file_not_found() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    let result = dns::import_records(&client, "zone123", "/nonexistent/file.csv").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_import_records_detect_bind_format() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "www.example.com",
                "type": "A",
                "content": "192.0.2.1",
                "ttl": 86400,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-03T00:00:00Z",
                "modified_on": "2026-02-03T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    // BIND format with $TTL directive
    let bind_content = "$TTL 86400\nwww 3600 IN A 192.0.2.1";
    let file_path = create_temp_file(bind_content, "zone");

    let client = create_test_client(&mock_server).await;
    let result = dns::import_records(&client, "zone123", file_path.to_str().unwrap()).await;

    assert!(result.is_ok());
    cleanup_temp_files();
}

#[tokio::test]
async fn test_import_records_bind_with_in_class() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "mail.example.com",
                "type": "A",
                "content": "192.0.2.5",
                "ttl": 3600,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-03T00:00:00Z",
                "modified_on": "2026-02-03T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    // BIND format with IN class
    let bind_content = "mail 3600 IN A 192.0.2.5";
    let file_path = create_temp_file(bind_content, "zone");

    let client = create_test_client(&mock_server).await;
    let result = dns::import_records(&client, "zone123", file_path.to_str().unwrap()).await;

    assert!(result.is_ok());
    cleanup_temp_files();
}
