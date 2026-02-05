use cfad::api::token::{
    CreatePermissionGroupRef, CreateToken, CreateTokenPolicy, PermissionGroup, Token,
    TokenCreateResponse, TokenVerification, UpdateToken,
};
use cfad::client::{CfResponse, CloudflareClient};
use cfad::config::AuthMethod;
use cfad::ops::token;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

// =============================================================================
// DESERIALIZATION TESTS
// =============================================================================

#[test]
fn test_token_list_response_deserialization() {
    let json = r#"{
        "success": true,
        "errors": [],
        "messages": [],
        "result": [
            {
                "id": "token-abc123",
                "name": "My API Token",
                "status": "active",
                "issued_on": "2026-01-01T00:00:00Z",
                "last_used_on": "2026-02-04T10:00:00Z",
                "policies": []
            }
        ]
    }"#;

    let response: CfResponse<Vec<Token>> =
        serde_json::from_str(json).expect("Failed to deserialize token list response");

    assert!(response.success);
    let tokens = response.result.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].id, "token-abc123");
    assert_eq!(tokens[0].status, "active");
}

#[test]
fn test_token_verification_deserialization() {
    let json = r#"{
        "id": "token-xyz789",
        "status": "active",
        "not_before": null,
        "expires_on": "2027-01-01T00:00:00Z"
    }"#;

    let verification: TokenVerification =
        serde_json::from_str(json).expect("Failed to deserialize token verification");

    assert_eq!(verification.id, "token-xyz789");
    assert_eq!(verification.status, "active");
    assert!(verification.not_before.is_none());
}

#[test]
fn test_permission_group_deserialization() {
    let json = r#"{
        "id": "perm-123",
        "name": "Zone Read",
        "description": "Grants read access to zones",
        "scopes": ["com.cloudflare.api.account.zone"]
    }"#;

    let group: PermissionGroup =
        serde_json::from_str(json).expect("Failed to deserialize permission group");

    assert_eq!(group.id, "perm-123");
    assert_eq!(group.name, "Zone Read");
    assert_eq!(group.scopes.len(), 1);
}

#[test]
fn test_token_create_response_deserialization() {
    let json = r#"{
        "id": "new-token-id",
        "name": "New Token",
        "status": "active",
        "value": "secret-token-value-abc123",
        "issued_on": "2026-02-04T12:00:00Z",
        "policies": []
    }"#;

    let response: TokenCreateResponse =
        serde_json::from_str(json).expect("Failed to deserialize token create response");

    assert_eq!(response.id, "new-token-id");
    assert_eq!(response.value, "secret-token-value-abc123");
}

// =============================================================================
// INTEGRATION TESTS
// =============================================================================

#[tokio::test]
async fn test_list_tokens_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "token-1",
                    "name": "Token One",
                    "status": "active",
                    "policies": []
                },
                {
                    "id": "token-2",
                    "name": "Token Two",
                    "status": "disabled",
                    "policies": []
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let tokens = token::list_tokens(&client).await.unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].name, "Token One");
    assert_eq!(tokens[1].status, "disabled");
}

#[tokio::test]
async fn test_list_tokens_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let tokens = token::list_tokens(&client).await.unwrap();

    assert!(tokens.is_empty());
}

#[tokio::test]
async fn test_get_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens/token-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "token-abc",
                "name": "Test Token",
                "status": "active",
                "issued_on": "2026-01-01T00:00:00Z",
                "policies": [
                    {
                        "id": "policy-1",
                        "effect": "allow",
                        "resources": {"com.cloudflare.api.account.*": "*"},
                        "permission_groups": [
                            {"id": "perm-1", "name": "Zone Read"}
                        ]
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let t = token::get_token(&client, "token-abc").await.unwrap();

    assert_eq!(t.id, "token-abc");
    assert_eq!(t.name, "Test Token");
    assert_eq!(t.policies.len(), 1);
}

#[tokio::test]
async fn test_get_token_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens/nonexistent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = token::get_token(&client, "nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/user/tokens"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "new-token-id",
                "name": "New API Token",
                "status": "active",
                "value": "generated-secret-value",
                "issued_on": "2026-02-04T12:00:00Z",
                "policies": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let create_request = CreateToken {
        name: "New API Token".to_string(),
        policies: vec![CreateTokenPolicy {
            effect: "allow".to_string(),
            resources: serde_json::json!({"com.cloudflare.api.account.*": "*"}),
            permission_groups: vec![CreatePermissionGroupRef {
                id: "perm-123".to_string(),
            }],
        }],
        not_before: None,
        expires_on: None,
        condition: None,
    };

    let response = token::create_token(&client, create_request).await.unwrap();

    assert_eq!(response.id, "new-token-id");
    assert_eq!(response.value, "generated-secret-value");
}

#[tokio::test]
async fn test_update_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/user/tokens/token-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "token-abc",
                "name": "Renamed Token",
                "status": "active",
                "policies": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let update_request = UpdateToken {
        name: Some("Renamed Token".to_string()),
        status: None,
        policies: None,
        not_before: None,
        expires_on: None,
        condition: None,
    };

    let t = token::update_token(&client, "token-abc", update_request)
        .await
        .unwrap();

    assert_eq!(t.name, "Renamed Token");
}

#[tokio::test]
async fn test_delete_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/user/tokens/token-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = token::delete_token(&client, "token-abc").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_verify_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens/verify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "current-token-id",
                "status": "active",
                "not_before": null,
                "expires_on": "2027-01-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let verification = token::verify_token(&client).await.unwrap();

    assert_eq!(verification.id, "current-token-id");
    assert_eq!(verification.status, "active");
}

#[tokio::test]
async fn test_list_permission_groups_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/tokens/permission_groups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "perm-1",
                    "name": "Zone Read",
                    "description": "Read zone settings",
                    "scopes": ["com.cloudflare.api.account.zone"]
                },
                {
                    "id": "perm-2",
                    "name": "Zone Write",
                    "description": "Write zone settings",
                    "scopes": ["com.cloudflare.api.account.zone"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let groups = token::list_permission_groups(&client).await.unwrap();

    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].name, "Zone Read");
    assert_eq!(groups[1].name, "Zone Write");
}

#[tokio::test]
async fn test_roll_token_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/user/tokens/token-abc/value"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "token-abc",
                "name": "Existing Token",
                "status": "active",
                "value": "new-regenerated-secret-value",
                "issued_on": "2026-02-04T14:00:00Z",
                "policies": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let response = token::roll_token(&client, "token-abc").await.unwrap();

    assert_eq!(response.id, "token-abc");
    assert_eq!(response.value, "new-regenerated-secret-value");
}
