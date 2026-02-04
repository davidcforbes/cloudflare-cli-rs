use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::cache;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn create_test_client(mock_server: &MockServer) -> CloudflareClient {
    let auth = AuthMethod::ApiToken("test_token".to_string());
    CloudflareClient::new_with_base_url(auth, mock_server.uri()).unwrap()
}

#[tokio::test]
async fn test_purge_all_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_all_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = cache::purge_all(&client, "zone123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_files_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_files_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let urls = vec![
        "https://example.com/image.jpg".to_string(),
        "https://example.com/style.css".to_string(),
    ];
    let result = cache::purge_files(&client, "zone123", urls).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_files_empty_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_empty_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = cache::purge_files(&client, "zone123", vec![]).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_tags_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_tags_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
    let result = cache::purge_tags(&client, "zone123", tags).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_hosts_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_hosts_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let hosts = vec![
        "www.example.com".to_string(),
        "blog.example.com".to_string(),
    ];
    let result = cache::purge_hosts(&client, "zone123", hosts).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_prefixes_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {
                "id": "purge_prefixes_result"
            },
            "errors": [],
            "messages": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let prefixes = vec![
        "https://example.com/images/".to_string(),
        "https://example.com/static/".to_string(),
    ];
    let result = cache::purge_prefixes(&client, "zone123", prefixes).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_files_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/zones/zone123/purge_cache"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "success": false,
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

    let client = create_test_client(&mock_server).await;
    let urls = vec!["https://example.com/file.jpg".to_string()];
    let result = cache::purge_files(&client, "zone123", urls).await;

    assert!(result.is_err());
}
