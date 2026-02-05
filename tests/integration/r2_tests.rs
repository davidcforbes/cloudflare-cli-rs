use cfad::api::r2::{
    CreateR2Bucket, CreateR2EventNotification, CreateR2MigrationJob, CreateR2SippyConfig,
    CreateR2TempCredentials, R2Bucket, R2BucketList, R2CorsConfig, R2CorsRule, R2CustomDomain,
    R2EventNotification, R2LifecycleConfig, R2LockConfig, R2ManagedDomain, R2Metrics,
    R2MigrationJob, R2MigrationProgress, R2SippyConfig, R2TempCredentials, UpdateR2Bucket,
};
use cfad::client::{CfResponse, CloudflareClient};
use cfad::config::AuthMethod;
use cfad::ops::r2;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

// =============================================================================
// DESERIALIZATION TESTS - Test actual API response format
// =============================================================================

/// Test that R2 bucket list response uses wrapper format
/// API returns {"result": {"buckets": [...]}} not {"result": [...]}
#[test]
fn test_r2_bucket_list_wrapper_format() {
    // This is the ACTUAL response format from Cloudflare R2 API
    let json = r#"{
        "success": true,
        "errors": [],
        "messages": [],
        "result": {
            "buckets": [
                {
                    "name": "my-bucket",
                    "creation_date": "2026-02-01T00:07:29.595Z"
                }
            ]
        }
    }"#;

    // This would have FAILED before the fix because we expected result to be Vec<R2Bucket>
    let response: CfResponse<R2BucketList> = serde_json::from_str(json)
        .expect("Failed to deserialize R2 bucket list - wrapper format issue?");

    assert!(response.success);
    let bucket_list = response.result.unwrap();
    assert_eq!(bucket_list.buckets.len(), 1);
    assert_eq!(bucket_list.buckets[0].name, "my-bucket");
}

/// Test R2Bucket deserialization with minimal fields
#[test]
fn test_r2_bucket_minimal() {
    let json = r#"{
        "name": "test-bucket",
        "creation_date": "2026-01-15T08:00:00.000Z"
    }"#;

    let bucket: R2Bucket =
        serde_json::from_str(json).expect("Failed to deserialize R2Bucket with minimal fields");

    assert_eq!(bucket.name, "test-bucket");
    assert_eq!(bucket.creation_date, "2026-01-15T08:00:00.000Z");
    assert!(bucket.location.is_none());
    assert!(bucket.storage_class.is_none());
}

/// Test R2Bucket deserialization with all fields
#[test]
fn test_r2_bucket_full() {
    let json = r#"{
        "name": "test-bucket",
        "creation_date": "2026-01-15T08:00:00.000Z",
        "location": "wnam",
        "storage_class": "Standard"
    }"#;

    let bucket: R2Bucket =
        serde_json::from_str(json).expect("Failed to deserialize R2Bucket with all fields");

    assert_eq!(bucket.name, "test-bucket");
    assert_eq!(bucket.location, Some("wnam".to_string()));
    assert_eq!(bucket.storage_class, Some("Standard".to_string()));
}

// =============================================================================
// INTEGRATION TESTS - Test full API flow with wiremock
// =============================================================================

#[tokio::test]
async fn test_list_buckets_success() {
    let mock_server = MockServer::start().await;

    // Use realistic response with wrapper format
    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "buckets": [
                    {
                        "name": "my-bucket",
                        "creation_date": "2026-01-01T00:00:00Z"
                    },
                    {
                        "name": "another-bucket",
                        "creation_date": "2026-01-15T12:00:00Z",
                        "location": "eeur"
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let buckets = r2::list_buckets(&client, "acc123").await.unwrap();

    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0].name, "my-bucket");
    assert_eq!(buckets[1].name, "another-bucket");
    assert_eq!(buckets[1].location, Some("eeur".to_string()));
}

#[tokio::test]
async fn test_list_buckets_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "buckets": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let buckets = r2::list_buckets(&client, "acc123").await.unwrap();

    assert!(buckets.is_empty());
}

#[tokio::test]
async fn test_get_bucket_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "name": "my-bucket",
                "creation_date": "2026-01-01T00:00:00Z",
                "location": "wnam",
                "storage_class": "Standard"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let bucket = r2::get_bucket(&client, "acc123", "my-bucket")
        .await
        .unwrap();

    assert_eq!(bucket.name, "my-bucket");
    assert_eq!(bucket.location, Some("wnam".to_string()));
}

#[tokio::test]
async fn test_get_bucket_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/nonexistent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::get_bucket(&client, "acc123", "nonexistent").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[tokio::test]
async fn test_delete_bucket_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::delete_bucket(&client, "acc123", "my-bucket").await;

    assert!(result.is_ok());
}

// =============================================================================
// BUCKET CRUD TESTS
// =============================================================================

#[tokio::test]
async fn test_create_bucket_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/r2/buckets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "name": "new-bucket",
                "creation_date": "2026-02-04T12:00:00Z",
                "location": "wnam",
                "storage_class": "Standard"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateR2Bucket {
        name: "new-bucket".to_string(),
        location_hint: Some("wnam".to_string()),
        storage_class: Some("Standard".to_string()),
    };

    let bucket = r2::create_bucket(&client, "acc123", create_request)
        .await
        .unwrap();

    assert_eq!(bucket.name, "new-bucket");
    assert_eq!(bucket.location, Some("wnam".to_string()));
}

#[tokio::test]
async fn test_update_bucket_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "name": "my-bucket",
                "creation_date": "2026-01-01T00:00:00Z",
                "storage_class": "InfrequentAccess"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let update_request = UpdateR2Bucket {
        storage_class: Some("InfrequentAccess".to_string()),
    };

    let bucket = r2::update_bucket(&client, "acc123", "my-bucket", update_request)
        .await
        .unwrap();

    assert_eq!(bucket.storage_class, Some("InfrequentAccess".to_string()));
}

// =============================================================================
// CORS TESTS
// =============================================================================

#[test]
fn test_r2_cors_rule_deserialization() {
    let json = r#"{
        "allowedOrigins": ["https://example.com", "https://app.example.com"],
        "allowedMethods": ["GET", "PUT", "POST"],
        "allowedHeaders": ["Content-Type", "Authorization"],
        "exposeHeaders": ["ETag"],
        "maxAgeSeconds": 3600
    }"#;

    let rule: R2CorsRule = serde_json::from_str(json).expect("Failed to deserialize R2CorsRule");

    assert_eq!(rule.allowed_origins.len(), 2);
    assert_eq!(rule.allowed_methods, vec!["GET", "PUT", "POST"]);
    assert_eq!(rule.max_age_seconds, 3600);
}

#[test]
fn test_r2_cors_config_deserialization() {
    let json = r#"{
        "rules": [
            {
                "allowedOrigins": ["*"],
                "allowedMethods": ["GET"],
                "allowedHeaders": [],
                "exposeHeaders": [],
                "maxAgeSeconds": 0
            }
        ]
    }"#;

    let config: R2CorsConfig =
        serde_json::from_str(json).expect("Failed to deserialize R2CorsConfig");

    assert_eq!(config.rules.len(), 1);
    assert_eq!(config.rules[0].allowed_origins, vec!["*"]);
}

#[tokio::test]
async fn test_get_cors_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/cors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "rules": [
                    {
                        "allowedOrigins": ["https://example.com"],
                        "allowedMethods": ["GET", "PUT"],
                        "allowedHeaders": ["Content-Type"],
                        "exposeHeaders": [],
                        "maxAgeSeconds": 3600
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let cors = r2::get_cors(&client, "acc123", "my-bucket").await.unwrap();

    assert_eq!(cors.rules.len(), 1);
    assert_eq!(cors.rules[0].allowed_origins, vec!["https://example.com"]);
}

#[tokio::test]
async fn test_set_cors_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/cors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let cors_rules = vec![R2CorsRule {
        allowed_origins: vec!["*".to_string()],
        allowed_methods: vec!["GET".to_string()],
        allowed_headers: vec![],
        expose_headers: vec![],
        max_age_seconds: 0,
    }];

    let result = r2::set_cors(&client, "acc123", "my-bucket", cors_rules).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_cors_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/cors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::delete_cors(&client, "acc123", "my-bucket").await;

    assert!(result.is_ok());
}

// =============================================================================
// CUSTOM DOMAIN TESTS
// =============================================================================

#[test]
fn test_r2_custom_domain_deserialization() {
    let json = r#"{
        "domain": "cdn.example.com",
        "enabled": true,
        "status": "active",
        "minTLS": "1.2",
        "zoneId": "zone123",
        "zoneName": "example.com"
    }"#;

    let domain: R2CustomDomain =
        serde_json::from_str(json).expect("Failed to deserialize R2CustomDomain");

    assert_eq!(domain.domain, "cdn.example.com");
    assert!(domain.enabled);
    assert_eq!(domain.status, "active");
}

#[tokio::test]
async fn test_list_custom_domains_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/domains/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "domain": "cdn.example.com",
                    "enabled": true,
                    "status": "active"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let domains = r2::list_custom_domains(&client, "acc123", "my-bucket")
        .await
        .unwrap();

    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0].domain, "cdn.example.com");
}

// =============================================================================
// MANAGED DOMAIN TESTS
// =============================================================================

#[test]
fn test_r2_managed_domain_deserialization() {
    let json = r#"{
        "enabled": true,
        "domain": "pub-abc123.r2.dev",
        "bucket_id": "bucket-id-123"
    }"#;

    let domain: R2ManagedDomain =
        serde_json::from_str(json).expect("Failed to deserialize R2ManagedDomain");

    assert!(domain.enabled);
    assert_eq!(domain.domain, Some("pub-abc123.r2.dev".to_string()));
}

#[tokio::test]
async fn test_get_managed_domain_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc123/r2/buckets/my-bucket/domains/managed",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "enabled": true,
                "domain": "pub-abc123.r2.dev"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let domain = r2::get_managed_domain(&client, "acc123", "my-bucket")
        .await
        .unwrap();

    assert!(domain.enabled);
}

// =============================================================================
// LIFECYCLE TESTS
// =============================================================================

#[test]
fn test_r2_lifecycle_config_deserialization() {
    let json = r#"{
        "rules": [
            {
                "id": "rule-1",
                "enabled": true,
                "conditions": {
                    "prefix": "logs/"
                },
                "actions": {
                    "deleteAfterDays": 30
                }
            }
        ]
    }"#;

    let config: R2LifecycleConfig =
        serde_json::from_str(json).expect("Failed to deserialize R2LifecycleConfig");

    assert_eq!(config.rules.len(), 1);
}

#[tokio::test]
async fn test_get_lifecycle_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/lifecycle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "rules": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let lifecycle = r2::get_lifecycle(&client, "acc123", "my-bucket")
        .await
        .unwrap();

    assert!(lifecycle.rules.is_empty());
}

// =============================================================================
// LOCK TESTS
// =============================================================================

#[test]
fn test_r2_lock_config_deserialization() {
    let json = r#"{
        "enabled": true
    }"#;

    let config: R2LockConfig =
        serde_json::from_str(json).expect("Failed to deserialize R2LockConfig");

    assert!(config.enabled);
}

#[tokio::test]
async fn test_get_lock_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/lock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "enabled": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let lock = r2::get_lock(&client, "acc123", "my-bucket").await.unwrap();

    assert!(!lock.enabled);
}

// =============================================================================
// METRICS TESTS
// =============================================================================

#[test]
fn test_r2_metrics_deserialization() {
    let json = r#"{
        "buckets": [
            {
                "bucket_name": "my-bucket",
                "storage_bytes": 107374182,
                "object_count": 1000,
                "upload_count": 500
            }
        ]
    }"#;

    let metrics: R2Metrics = serde_json::from_str(json).expect("Failed to deserialize R2Metrics");

    assert_eq!(metrics.buckets.len(), 1);
    assert_eq!(metrics.buckets[0].object_count, 1000);
}

#[tokio::test]
async fn test_get_metrics_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "buckets": [
                    {
                        "bucket_name": "bucket-1",
                        "storage_bytes": 536870912,
                        "object_count": 5000,
                        "upload_count": 2500
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let metrics = r2::get_metrics(&client, "acc123").await.unwrap();

    assert_eq!(metrics.buckets.len(), 1);
    assert_eq!(metrics.buckets[0].object_count, 5000);
}

// =============================================================================
// SIPPY TESTS
// =============================================================================

#[test]
fn test_r2_sippy_config_deserialization() {
    let json = r#"{
        "enabled": true,
        "provider": "aws",
        "bucket": "source-bucket",
        "region": "us-east-1"
    }"#;

    let config: R2SippyConfig =
        serde_json::from_str(json).expect("Failed to deserialize R2SippyConfig");

    assert!(config.enabled);
    assert_eq!(config.provider, Some("aws".to_string()));
    assert_eq!(config.bucket, Some("source-bucket".to_string()));
    assert_eq!(config.region, Some("us-east-1".to_string()));
}

#[test]
fn test_r2_sippy_config_deserialization_disabled() {
    let json = r#"{
        "enabled": false
    }"#;

    let config: R2SippyConfig =
        serde_json::from_str(json).expect("Failed to deserialize disabled R2SippyConfig");

    assert!(!config.enabled);
    assert!(config.provider.is_none());
}

#[tokio::test]
async fn test_get_sippy_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/sippy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "enabled": true,
                "provider": "aws",
                "bucket": "source-bucket",
                "region": "us-west-2"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let sippy = r2::get_sippy(&client, "acc123", "my-bucket").await.unwrap();

    assert!(sippy.enabled);
    assert_eq!(sippy.provider, Some("aws".to_string()));
}

#[tokio::test]
async fn test_set_sippy_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/sippy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "enabled": true,
                "provider": "aws",
                "bucket": "source-bucket",
                "region": "us-east-1"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let config = CreateR2SippyConfig {
        provider: "aws".to_string(),
        bucket: "source-bucket".to_string(),
        region: Some("us-east-1".to_string()),
        access_key_id: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
        secret_access_key: Some("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string()),
    };

    let sippy = r2::set_sippy(&client, "acc123", "my-bucket", config)
        .await
        .unwrap();

    assert!(sippy.enabled);
    assert_eq!(sippy.provider, Some("aws".to_string()));
}

#[tokio::test]
async fn test_delete_sippy_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/acc123/r2/buckets/my-bucket/sippy"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::delete_sippy(&client, "acc123", "my-bucket").await;

    assert!(result.is_ok());
}

// =============================================================================
// EVENT NOTIFICATION TESTS
// =============================================================================

#[test]
fn test_r2_event_notification_deserialization() {
    let json = r#"{
        "queueId": "queue-abc123",
        "events": ["object-create", "object-delete"],
        "prefix": "uploads/",
        "suffix": ".json"
    }"#;

    let notification: R2EventNotification =
        serde_json::from_str(json).expect("Failed to deserialize R2EventNotification");

    assert_eq!(notification.queue_id, "queue-abc123");
    assert_eq!(notification.events.len(), 2);
    assert_eq!(notification.prefix, Some("uploads/".to_string()));
    assert_eq!(notification.suffix, Some(".json".to_string()));
}

#[test]
fn test_r2_event_notification_deserialization_minimal() {
    let json = r#"{
        "queueId": "queue-xyz",
        "events": ["object-create"]
    }"#;

    let notification: R2EventNotification =
        serde_json::from_str(json).expect("Failed to deserialize minimal R2EventNotification");

    assert_eq!(notification.queue_id, "queue-xyz");
    assert!(notification.prefix.is_none());
    assert!(notification.suffix.is_none());
}

#[tokio::test]
async fn test_list_notifications_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc123/event_notifications/r2/my-bucket/configuration",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "queueId": "queue-1",
                    "events": ["object-create"],
                    "prefix": "uploads/"
                },
                {
                    "queueId": "queue-2",
                    "events": ["object-delete"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let notifications = r2::list_notifications(&client, "acc123", "my-bucket")
        .await
        .unwrap();

    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications[0].queue_id, "queue-1");
    assert_eq!(notifications[1].queue_id, "queue-2");
}

#[tokio::test]
async fn test_get_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/accounts/acc123/event_notifications/r2/my-bucket/configuration/queues/queue-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "queueId": "queue-1",
                "events": ["object-create", "object-delete"],
                "prefix": "logs/"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let notification = r2::get_notification(&client, "acc123", "my-bucket", "queue-1")
        .await
        .unwrap();

    assert_eq!(notification.queue_id, "queue-1");
    assert_eq!(notification.events.len(), 2);
}

#[tokio::test]
async fn test_create_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/accounts/acc123/event_notifications/r2/my-bucket/configuration/queues/queue-abc",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "queueId": "queue-abc",
                "events": ["object-create"],
                "prefix": "data/"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateR2EventNotification {
        events: vec!["object-create".to_string()],
        prefix: Some("data/".to_string()),
        suffix: None,
    };

    let notification =
        r2::create_notification(&client, "acc123", "my-bucket", "queue-abc", create_request)
            .await
            .unwrap();

    assert_eq!(notification.queue_id, "queue-abc");
}

#[tokio::test]
async fn test_delete_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/accounts/acc123/event_notifications/r2/my-bucket/configuration/queues/queue-1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::delete_notification(&client, "acc123", "my-bucket", "queue-1").await;

    assert!(result.is_ok());
}

// =============================================================================
// MIGRATION (SUPER SLURPER) TESTS
// =============================================================================

#[test]
fn test_r2_migration_job_deserialization() {
    let json = r#"{
        "id": "job-abc123",
        "status": "running",
        "source_provider": "aws",
        "source_bucket": "source-bucket",
        "target_bucket": "target-bucket",
        "created_at": "2026-02-04T10:00:00Z"
    }"#;

    let job: R2MigrationJob =
        serde_json::from_str(json).expect("Failed to deserialize R2MigrationJob");

    assert_eq!(job.id, "job-abc123");
    assert_eq!(job.status, "running");
    assert_eq!(job.source_provider, "aws");
    assert!(job.completed_at.is_none());
}

#[test]
fn test_r2_migration_job_deserialization_completed() {
    let json = r#"{
        "id": "job-xyz",
        "status": "completed",
        "source_provider": "gcs",
        "source_bucket": "my-gcs-bucket",
        "target_bucket": "my-r2-bucket",
        "created_at": "2026-02-01T00:00:00Z",
        "completed_at": "2026-02-02T12:00:00Z"
    }"#;

    let job: R2MigrationJob =
        serde_json::from_str(json).expect("Failed to deserialize completed R2MigrationJob");

    assert_eq!(job.status, "completed");
    assert_eq!(job.completed_at, Some("2026-02-02T12:00:00Z".to_string()));
}

#[test]
fn test_r2_migration_progress_deserialization() {
    let json = r#"{
        "objects_migrated": 5000,
        "objects_total": 10000,
        "bytes_migrated": 536870912,
        "bytes_total": 1073741824,
        "errors": 2
    }"#;

    let progress: R2MigrationProgress =
        serde_json::from_str(json).expect("Failed to deserialize R2MigrationProgress");

    assert_eq!(progress.objects_migrated, 5000);
    assert_eq!(progress.objects_total, 10000);
    assert_eq!(progress.errors, 2);
}

#[tokio::test]
async fn test_list_migration_jobs_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/slurper/jobs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "job-1",
                    "status": "running",
                    "source_provider": "aws",
                    "source_bucket": "bucket-a",
                    "target_bucket": "r2-bucket-a",
                    "created_at": "2026-02-01T00:00:00Z"
                },
                {
                    "id": "job-2",
                    "status": "completed",
                    "source_provider": "gcs",
                    "source_bucket": "bucket-b",
                    "target_bucket": "r2-bucket-b",
                    "created_at": "2026-01-15T00:00:00Z",
                    "completed_at": "2026-01-16T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let jobs = r2::list_migration_jobs(&client, "acc123").await.unwrap();

    assert_eq!(jobs.len(), 2);
    assert_eq!(jobs[0].id, "job-1");
    assert_eq!(jobs[0].status, "running");
    assert_eq!(jobs[1].status, "completed");
}

#[tokio::test]
async fn test_get_migration_job_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "job-abc",
                "status": "running",
                "source_provider": "aws",
                "source_bucket": "source",
                "target_bucket": "target",
                "created_at": "2026-02-04T10:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let job = r2::get_migration_job(&client, "acc123", "job-abc")
        .await
        .unwrap();

    assert_eq!(job.id, "job-abc");
    assert_eq!(job.status, "running");
}

#[tokio::test]
async fn test_create_migration_job_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/slurper/jobs"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "new-job-123",
                "status": "pending",
                "source_provider": "aws",
                "source_bucket": "aws-bucket",
                "target_bucket": "r2-bucket",
                "created_at": "2026-02-04T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateR2MigrationJob {
        source_provider: "aws".to_string(),
        source_bucket: "aws-bucket".to_string(),
        source_region: Some("us-east-1".to_string()),
        target_bucket: "r2-bucket".to_string(),
        access_key_id: "AKIAIOSFODNN7EXAMPLE".to_string(),
        secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
    };

    let job = r2::create_migration_job(&client, "acc123", create_request)
        .await
        .unwrap();

    assert_eq!(job.id, "new-job-123");
    assert_eq!(job.status, "pending");
}

#[tokio::test]
async fn test_pause_migration_job_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc/pause"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::pause_migration_job(&client, "acc123", "job-abc").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resume_migration_job_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc/resume"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::resume_migration_job(&client, "acc123", "job-abc").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_abort_migration_job_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc/abort"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = r2::abort_migration_job(&client, "acc123", "job-abc").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_migration_progress_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc/progress"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "objects_migrated": 7500,
                "objects_total": 10000,
                "bytes_migrated": 805306368,
                "bytes_total": 1073741824,
                "errors": 0
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let progress = r2::get_migration_progress(&client, "acc123", "job-abc")
        .await
        .unwrap();

    assert_eq!(progress.objects_migrated, 7500);
    assert_eq!(progress.objects_total, 10000);
    assert_eq!(progress.errors, 0);
}

#[tokio::test]
async fn test_get_migration_logs_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/slurper/jobs/job-abc/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                "2026-02-04T10:00:00Z: Migration started",
                "2026-02-04T10:30:00Z: 50% complete",
                "2026-02-04T11:00:00Z: Migration completed"
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let logs = r2::get_migration_logs(&client, "acc123", "job-abc")
        .await
        .unwrap();

    assert_eq!(logs.len(), 3);
    assert!(logs[0].contains("Migration started"));
}

// =============================================================================
// TEMPORARY CREDENTIALS TESTS
// =============================================================================

#[test]
fn test_r2_temp_credentials_deserialization() {
    let json = r#"{
        "accessKeyId": "temp-access-key-id",
        "secretAccessKey": "temp-secret-access-key",
        "sessionToken": "temp-session-token-abc123",
        "expiration": "2026-02-04T14:00:00Z"
    }"#;

    let creds: R2TempCredentials =
        serde_json::from_str(json).expect("Failed to deserialize R2TempCredentials");

    assert_eq!(creds.access_key_id, "temp-access-key-id");
    assert_eq!(creds.secret_access_key, "temp-secret-access-key");
    assert_eq!(creds.session_token, "temp-session-token-abc123");
    assert_eq!(creds.expiration, "2026-02-04T14:00:00Z");
}

#[tokio::test]
async fn test_create_temp_credentials_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/r2/temp-access-credentials"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "accessKeyId": "TEMP_ACCESS_KEY_ID",
                "secretAccessKey": "TEMP_SECRET_ACCESS_KEY",
                "sessionToken": "SESSION_TOKEN_XYZ",
                "expiration": "2026-02-04T15:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateR2TempCredentials {
        bucket: "my-bucket".to_string(),
        prefix: Some("uploads/".to_string()),
        permission: "readwrite".to_string(),
        ttl_seconds: 3600,
    };

    let creds = r2::create_temp_credentials(&client, "acc123", create_request)
        .await
        .unwrap();

    assert_eq!(creds.access_key_id, "TEMP_ACCESS_KEY_ID");
    assert_eq!(creds.session_token, "SESSION_TOKEN_XYZ");
}

#[tokio::test]
async fn test_create_temp_credentials_read_only() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/r2/temp-access-credentials"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "accessKeyId": "READ_ONLY_KEY",
                "secretAccessKey": "READ_ONLY_SECRET",
                "sessionToken": "READ_ONLY_TOKEN",
                "expiration": "2026-02-04T13:30:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateR2TempCredentials {
        bucket: "public-bucket".to_string(),
        prefix: None,
        permission: "read".to_string(),
        ttl_seconds: 1800,
    };

    let creds = r2::create_temp_credentials(&client, "acc123", create_request)
        .await
        .unwrap();

    assert_eq!(creds.access_key_id, "READ_ONLY_KEY");
}
