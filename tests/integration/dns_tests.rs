use cfad::api::dns::{CreateDnsRecord, UpdateDnsRecord};
use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::dns;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

#[tokio::test]
async fn test_list_records_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [
                {
                    "id": "rec123",
                    "zone_id": "zone123",
                    "zone_name": "example.com",
                    "name": "www.example.com",
                    "type": "A",
                    "content": "203.0.113.1",
                    "ttl": 3600,
                    "proxied": true,
                    "locked": false,
                    "created_on": "2026-01-01T00:00:00Z",
                    "modified_on": "2026-01-01T00:00:00Z"
                }
            ],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let records = dns::list_records(&client, "zone123", None, None)
        .await
        .unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].name, "www.example.com");
}

#[tokio::test]
async fn test_list_records_with_filters() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/zone123/dns_records"))
        .and(query_param("type", "A"))
        .and(query_param("name", "www.example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let records = dns::list_records(&client, "zone123", Some("A"), Some("www.example.com"))
        .await
        .unwrap();

    assert_eq!(records.len(), 0);
}

#[tokio::test]
async fn test_get_record_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/zone123/dns_records/rec123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec123",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "www.example.com",
                "type": "A",
                "content": "203.0.113.1",
                "ttl": 3600,
                "proxied": true,
                "locked": false,
                "created_on": "2026-01-01T00:00:00Z",
                "modified_on": "2026-01-01T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let record = dns::get_record(&client, "zone123", "rec123").await.unwrap();

    assert_eq!(record.id, "rec123");
    assert_eq!(record.name, "www.example.com");
}

#[tokio::test]
async fn test_get_record_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/zone123/dns_records/notfound"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = dns::get_record(&client, "zone123", "notfound").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_record_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec_new",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "new.example.com",
                "type": "A",
                "content": "203.0.113.10",
                "ttl": 3600,
                "proxied": false,
                "locked": false,
                "created_on": "2026-02-02T00:00:00Z",
                "modified_on": "2026-02-02T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let new_record = CreateDnsRecord {
        record_type: "A".to_string(),
        name: "new.example.com".to_string(),
        content: "203.0.113.10".to_string(),
        ttl: Some(3600),
        proxied: Some(false),
        priority: None,
        data: None,
    };

    let record = dns::create_record(&client, "zone123", new_record)
        .await
        .unwrap();

    assert_eq!(record.name, "new.example.com");
    assert_eq!(record.content, "203.0.113.10");
}

#[tokio::test]
async fn test_update_record_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/zones/zone123/dns_records/rec123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "rec123",
                "zone_id": "zone123",
                "zone_name": "example.com",
                "name": "www.example.com",
                "type": "A",
                "content": "203.0.113.99",
                "ttl": 3600,
                "proxied": true,
                "locked": false,
                "created_on": "2026-01-01T00:00:00Z",
                "modified_on": "2026-02-02T00:00:00Z"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let update = UpdateDnsRecord {
        record_type: None,
        name: None,
        content: Some("203.0.113.99".to_string()),
        ttl: None,
        proxied: None,
        priority: None,
    };

    let record = dns::update_record(&client, "zone123", "rec123", update)
        .await
        .unwrap();

    assert_eq!(record.content, "203.0.113.99");
}

#[tokio::test]
async fn test_delete_record_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/zones/zone123/dns_records/rec123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = dns::delete_record(&client, "zone123", "rec123").await;

    assert!(result.is_ok());
}
