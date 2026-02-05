use cfad::api::d1::{CreateD1Database, D1Database, UpdateD1Database};
use cfad::client::{CfResponse, CloudflareClient, ResultInfo};
use cfad::config::AuthMethod;
use cfad::ops::d1;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

// =============================================================================
// DESERIALIZATION TESTS - These would have caught the total_pages bug
// =============================================================================

/// Test that we can deserialize REAL API responses (without total_pages)
/// This is the exact response format from Cloudflare's D1 API
#[test]
fn test_d1_list_response_without_total_pages() {
    // This is the ACTUAL response format from Cloudflare D1 API
    // Note: result_info does NOT include total_pages
    let json = r#"{
        "result": [{
            "uuid": "9b783f77-78de-47e1-abea-94deae6a0038",
            "name": "policy-factory-discussions",
            "created_at": "2026-02-01T10:36:58.688Z",
            "version": "production",
            "num_tables": 0,
            "file_size": 151552,
            "jurisdiction": null
        }],
        "errors": [],
        "messages": [],
        "success": true,
        "result_info": {
            "count": 1,
            "page": 1,
            "per_page": 100,
            "total_count": 1
        }
    }"#;

    // This would have FAILED before the fix because total_pages was required
    let response: CfResponse<Vec<D1Database>> = serde_json::from_str(json)
        .expect("Failed to deserialize D1 list response - missing total_pages handling?");

    assert!(response.success);
    assert_eq!(response.result.unwrap().len(), 1);

    // Verify result_info parsed correctly with default total_pages
    let result_info = response.result_info.unwrap();
    assert_eq!(result_info.count, 1);
    assert_eq!(result_info.page, 1);
    assert_eq!(result_info.total_pages, 0); // Default value
}

/// Test D1Database deserialization with null fields
#[test]
fn test_d1_database_with_null_fields() {
    // Real API can return null for version, num_tables, file_size, created_at
    let json = r#"{
        "uuid": "test-uuid",
        "name": "test-db",
        "version": null,
        "num_tables": null,
        "file_size": null,
        "created_at": null
    }"#;

    let db: D1Database = serde_json::from_str(json)
        .expect("Failed to deserialize D1Database with null fields");

    assert_eq!(db.uuid, "test-uuid");
    assert_eq!(db.name, "test-db");
    assert_eq!(db.version, ""); // Default for null
    assert_eq!(db.num_tables, 0); // Default for null
    assert_eq!(db.file_size, 0); // Default for null
    assert_eq!(db.created_at, ""); // Default for null
}

/// Test D1Database deserialization with missing optional fields
#[test]
fn test_d1_database_with_missing_fields() {
    // API might omit optional fields entirely
    let json = r#"{
        "uuid": "test-uuid",
        "name": "test-db"
    }"#;

    let db: D1Database = serde_json::from_str(json)
        .expect("Failed to deserialize D1Database with missing fields");

    assert_eq!(db.uuid, "test-uuid");
    assert_eq!(db.name, "test-db");
    assert_eq!(db.version, "");
    assert_eq!(db.num_tables, 0);
}

/// Test ResultInfo deserialization without total_pages
#[test]
fn test_result_info_without_total_pages() {
    let json = r#"{
        "count": 5,
        "page": 1,
        "per_page": 20,
        "total_count": 100
    }"#;

    let info: ResultInfo = serde_json::from_str(json)
        .expect("Failed to deserialize ResultInfo without total_pages");

    assert_eq!(info.count, 5);
    assert_eq!(info.page, 1);
    assert_eq!(info.per_page, 20);
    assert_eq!(info.total_count, 100);
    assert_eq!(info.total_pages, 0); // Should default to 0
}

/// Load and parse the actual fixture file
#[test]
fn test_d1_list_fixture_parses() {
    let json = include_str!("../fixtures/d1_database_list_response.json");
    let response: CfResponse<Vec<D1Database>> = serde_json::from_str(json)
        .expect("Failed to parse d1_database_list_response.json fixture");

    assert!(response.success);
    let databases = response.result.unwrap();
    assert_eq!(databases.len(), 2);
    assert_eq!(databases[0].name, "policy-factory-discussions");
    assert_eq!(databases[1].name, "my-app-database");
}

// =============================================================================
// INTEGRATION TESTS - Test full API flow with wiremock
// =============================================================================

#[tokio::test]
async fn test_list_databases_success() {
    let mock_server = MockServer::start().await;

    // Use realistic response without total_pages
    Mock::given(method("GET"))
        .and(path("/accounts/acc123/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [
                {
                    "uuid": "db-uuid-1",
                    "name": "my-database",
                    "created_at": "2026-01-01T00:00:00Z",
                    "version": "production",
                    "num_tables": 5,
                    "file_size": 102400
                }
            ],
            "errors": [],
            "messages": [],
            "result_info": {
                "count": 1,
                "page": 1,
                "per_page": 100,
                "total_count": 1
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let databases = d1::list_databases(&client, "acc123").await.unwrap();

    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0].uuid, "db-uuid-1");
    assert_eq!(databases[0].name, "my-database");
}

#[tokio::test]
async fn test_list_databases_with_null_fields() {
    let mock_server = MockServer::start().await;

    // API returns null for optional fields
    Mock::given(method("GET"))
        .and(path("/accounts/acc123/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [
                {
                    "uuid": "db-uuid-1",
                    "name": "new-database",
                    "created_at": "2026-01-01T00:00:00Z",
                    "version": null,
                    "num_tables": null,
                    "file_size": null
                }
            ],
            "errors": [],
            "messages": [],
            "result_info": {
                "count": 1,
                "page": 1,
                "per_page": 100,
                "total_count": 1
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let databases = d1::list_databases(&client, "acc123").await.unwrap();

    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0].version, "");
    assert_eq!(databases[0].num_tables, 0);
    assert_eq!(databases[0].file_size, 0);
}

#[tokio::test]
async fn test_get_database_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/d1/database/db-uuid-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "uuid": "db-uuid-1",
                "name": "my-database",
                "created_at": "2026-01-01T00:00:00Z",
                "version": "production",
                "num_tables": 5,
                "file_size": 102400
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let database = d1::get_database(&client, "acc123", "db-uuid-1")
        .await
        .unwrap();

    assert_eq!(database.uuid, "db-uuid-1");
    assert_eq!(database.name, "my-database");
    assert_eq!(database.num_tables, 5);
}

#[tokio::test]
async fn test_create_database_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/d1/database"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "uuid": "new-db-uuid",
                "name": "new-database",
                "created_at": "2026-02-04T12:00:00Z",
                "version": "production",
                "num_tables": 0,
                "file_size": 0
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateD1Database {
        name: "new-database".to_string(),
        primary_location_hint: None,
    };

    let database = d1::create_database(&client, "acc123", create_request)
        .await
        .unwrap();

    assert_eq!(database.uuid, "new-db-uuid");
    assert_eq!(database.name, "new-database");
}

#[tokio::test]
async fn test_update_database_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/accounts/acc123/d1/database/db-uuid-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "uuid": "db-uuid-1",
                "name": "renamed-database",
                "created_at": "2026-01-01T00:00:00Z",
                "version": "production",
                "num_tables": 5,
                "file_size": 102400
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let update_request = UpdateD1Database {
        name: Some("renamed-database".to_string()),
    };

    let database = d1::update_database(&client, "acc123", "db-uuid-1", update_request)
        .await
        .unwrap();

    assert_eq!(database.name, "renamed-database");
}

#[tokio::test]
async fn test_delete_database_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/acc123/d1/database/db-uuid-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = d1::delete_database(&client, "acc123", "db-uuid-1").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_database_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/accounts/acc123/d1/database/db-uuid-1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [{
                "results": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ],
                "success": true,
                "meta": {
                    "duration": 0.5,
                    "rows_read": 2,
                    "rows_written": 0,
                    "last_row_id": 0,
                    "changes": 0,
                    "size_after": 102400,
                    "served_by_cache": false
                }
            }],
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let results = d1::query_database(&client, "acc123", "db-uuid-1", "SELECT * FROM users", None)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].success);
    assert_eq!(results[0].results.len(), 2);
}

#[tokio::test]
async fn test_get_database_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/d1/database/nonexistent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = d1::get_database(&client, "acc123", "nonexistent").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[tokio::test]
async fn test_list_databases_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/accounts/acc123/d1/database"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [],
            "errors": [],
            "messages": [],
            "result_info": {
                "count": 0,
                "page": 1,
                "per_page": 100,
                "total_count": 0
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let databases = d1::list_databases(&client, "acc123").await.unwrap();

    assert!(databases.is_empty());
}
