//! Error response integration tests
//!
//! Tests that verify the client properly handles various error responses from the Cloudflare API.

use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::zone;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

#[tokio::test]
async fn test_401_unauthorized_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "success": false,
            "errors": [
                {
                    "code": 10000,
                    "message": "Authentication error"
                }
            ],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::list_zones(&client, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{}", err);
    assert!(err_str.contains("10000") || err_str.contains("Authentication"));
}

#[tokio::test]
async fn test_403_forbidden_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/zone123"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "success": false,
            "errors": [
                {
                    "code": 10001,
                    "message": "You do not have permission to access this resource"
                }
            ],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::get_zone(&client, "zone123").await;

    // Should fail with some kind of error (403, permission, or API error)
    assert!(result.is_err());
}

#[tokio::test]
async fn test_404_not_found_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones/invalid-zone-id"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "success": false,
            "errors": [
                {
                    "code": 7003,
                    "message": "Could not route to /zones/invalid-zone-id, perhaps your object identifier is invalid?"
                }
            ],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::get_zone(&client, "invalid-zone-id").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_429_rate_limit_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(
            ResponseTemplate::new(429)
                .set_body_json(serde_json::json!({
                    "success": false,
                    "errors": [
                        {
                            "code": 10015,
                            "message": "Rate limit reached. Please wait before making another request."
                        }
                    ],
                    "messages": [],
                    "result": null
                }))
                .insert_header("Retry-After", "60"),
        )
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::list_zones(&client, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{}", err);
    assert!(err_str.contains("Rate limit") || err_str.contains("10015"));
}

#[tokio::test]
async fn test_500_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "success": false,
            "errors": [
                {
                    "code": 10000,
                    "message": "Internal server error"
                }
            ],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::list_zones(&client, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_errors_in_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "success": false,
            "errors": [
                {
                    "code": 1001,
                    "message": "Invalid zone name"
                },
                {
                    "code": 1002,
                    "message": "Account ID required"
                }
            ],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::create_zone(&client, "invalid", "acc123").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{}", err);
    // Should contain error codes or messages
    assert!(err_str.contains("1001") || err_str.contains("Invalid"));
}

#[tokio::test]
async fn test_zone_list_with_null_values_in_response() {
    let mock_server = MockServer::start().await;

    // Use realistic fixture data with null values
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "zone123",
                    "name": "example.com",
                    "status": "active",
                    "paused": false,
                    "development_mode": 0,
                    "name_servers": ["ns1.cloudflare.com"],
                    "original_name_servers": null,
                    "owner": {
                        "id": null,
                        "type": "user",
                        "email": "user@example.com"
                    },
                    "account": {
                        "id": "acc123",
                        "name": "Test Account"
                    },
                    "created_on": "2024-01-01T00:00:00Z",
                    "modified_on": "2024-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::list_zones(&client, None).await;

    // This should succeed even with null values
    assert!(result.is_ok());
    let zones = result.unwrap();
    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].name, "example.com");
    assert!(zones[0].owner.id.is_none());
    assert!(zones[0].original_name_servers.is_empty());
}
