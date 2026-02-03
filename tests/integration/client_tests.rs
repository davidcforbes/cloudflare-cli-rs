use cfad::client::CloudflareClient;
use cfad::config::{AuthMethod, Profile};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_client_get_success() {
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

    let profile = Profile {
        api_token: Some("test_token".to_string()),
        api_key: None,
        api_email: None,
        default_zone: None,
        output_format: None,
    };

    let auth = profile.auth_method().unwrap();
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let result: Result<cfad::client::CfResponse<Vec<serde_json::Value>>, _> =
        client.get("/zones").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_auth_token_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(header("Authorization", "Bearer test_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {},
            "errors": [],
            "messages": []
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiToken("test_token_12345".to_string());
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let _result: Result<cfad::client::CfResponse<serde_json::Value>, _> = client.get("/test").await;

    // If the test passes, it means the Authorization header was sent correctly
}

#[tokio::test]
async fn test_client_auth_key_email_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(header("X-Auth-Key", "test_key_12345"))
        .and(header("X-Auth-Email", "user@example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {},
            "errors": [],
            "messages": []
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiKeyEmail {
        key: "test_key_12345".to_string(),
        email: "user@example.com".to_string(),
    };
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let _result: Result<cfad::client::CfResponse<serde_json::Value>, _> = client.get("/test").await;

    // If the test passes, it means the headers were sent correctly
}

#[tokio::test]
async fn test_client_post_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "result": {"id": "123"},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiToken("test_token".to_string());
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let payload = serde_json::json!({"name": "test"});
    let result: Result<cfad::client::CfResponse<serde_json::Value>, _> =
        client.post("/test", payload).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_put_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/test/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {"id": "123", "updated": true},
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiToken("test_token".to_string());
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let payload = serde_json::json!({"name": "updated"});
    let result: Result<cfad::client::CfResponse<serde_json::Value>, _> =
        client.put("/test/123", payload).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_delete_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/test/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": null,
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiToken("test_token".to_string());
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let result: Result<cfad::client::CfResponse<serde_json::Value>, _> =
        client.delete("/test/123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_api_error_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "success": false,
            "result": null,
            "errors": [
                {
                    "code": 1003,
                    "message": "Invalid request"
                }
            ],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let auth = AuthMethod::ApiToken("test_token".to_string());
    let client = CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap();

    let result: Result<cfad::client::CfResponse<serde_json::Value>, _> = client.get("/test").await;

    assert!(result.is_err());
}
