use cfad::api::zone::ZoneSettings;
use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::zone;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

fn create_zone_json(id: &str, name: &str, status: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "status": status,
        "paused": false,
        "development_mode": 0,
        "name_servers": ["ns1.cloudflare.com", "ns2.cloudflare.com"],
        "original_name_servers": ["ns1.example.com"],
        "owner": {
            "id": "owner123",
            "type": "user",
            "email": "user@example.com"
        },
        "account": {
            "id": "account123",
            "name": "Test Account"
        },
        "created_on": "2026-01-01T00:00:00Z",
        "modified_on": "2026-01-01T00:00:00Z"
    })
}

#[tokio::test]
async fn test_list_zones_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [
                create_zone_json("zone1", "example.com", "active"),
                create_zone_json("zone2", "test.com", "active"),
            ],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zones = zone::list_zones(&client, None).await.unwrap();

    assert_eq!(zones.len(), 2);
    assert_eq!(zones[0].name, "example.com");
    assert_eq!(zones[1].name, "test.com");
}

#[tokio::test]
async fn test_list_zones_with_status_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .and(query_param("status", "active"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [
                create_zone_json("zone1", "active-zone.com", "active"),
            ],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zones = zone::list_zones(&client, Some("active")).await.unwrap();

    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].status, "active");
}

#[tokio::test]
async fn test_list_zones_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zones = zone::list_zones(&client, None).await.unwrap();

    assert_eq!(zones.len(), 0);
}

#[tokio::test]
async fn test_get_zone_by_id() {
    let mock_server = MockServer::start().await;

    let zone_id = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";

    Mock::given(method("GET"))
        .and(path(format!("/zones/{}", zone_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": create_zone_json(zone_id, "example.com", "active"),
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zone = zone::get_zone(&client, zone_id).await.unwrap();

    assert_eq!(zone.id, zone_id);
    assert_eq!(zone.name, "example.com");
}

#[tokio::test]
async fn test_get_zone_by_name() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .and(query_param("name", "example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": create_zone_json("zone123", "example.com", "active"),
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zone = zone::get_zone(&client, "example.com").await.unwrap();

    assert_eq!(zone.name, "example.com");
}

#[tokio::test]
async fn test_get_zone_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/zones"))
        .and(query_param("name", "notfound.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::get_zone(&client, "notfound.com").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_zone_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": create_zone_json("new_zone_123", "newdomain.com", "pending"),
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let zone = zone::create_zone(&client, "newdomain.com", "account123")
        .await
        .unwrap();

    assert_eq!(zone.name, "newdomain.com");
    assert_eq!(zone.status, "pending");
}

#[tokio::test]
async fn test_delete_zone_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/zones/zone123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = zone::delete_zone(&client, "zone123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_zone_settings_security_level() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/security_level"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "high"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let settings = ZoneSettings {
        security_level: Some("high".to_string()),
        ..Default::default()
    };

    let result = zone::update_zone_settings(&client, "zone123", settings).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_zone_settings_multiple() {
    let mock_server = MockServer::start().await;

    // Mock multiple setting updates
    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/security_level"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "high"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/ssl"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "flexible"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/development_mode"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "on"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let settings = ZoneSettings {
        security_level: Some("high".to_string()),
        ssl: Some("flexible".to_string()),
        development_mode: Some(true),
        ..Default::default()
    };

    let result = zone::update_zone_settings(&client, "zone123", settings).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_zone_settings_cache_level() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/cache_level"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "aggressive"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let settings = ZoneSettings {
        cache_level: Some("aggressive".to_string()),
        ..Default::default()
    };

    let result = zone::update_zone_settings(&client, "zone123", settings).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_zone_settings_ipv6() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/ipv6"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "on"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let settings = ZoneSettings {
        ipv6: Some(true),
        ..Default::default()
    };

    let result = zone::update_zone_settings(&client, "zone123", settings).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_zone_settings_always_https() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/zones/zone123/settings/always_use_https"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"value": "on"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let settings = ZoneSettings {
        always_use_https: Some(true),
        ..Default::default()
    };

    let result = zone::update_zone_settings(&client, "zone123", settings).await;
    assert!(result.is_ok());
}
