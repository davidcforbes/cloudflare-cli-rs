use cfad::api::r2::{R2Bucket, R2BucketList};
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
