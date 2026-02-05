use cfad::api::pages::{
    AddDomain, CreateProject, Deployment, DeploymentLogs, PagesDomain, PagesProject,
};
use cfad::client::CloudflareClient;
use cfad::config::AuthMethod;
use cfad::ops::pages;
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
fn test_pages_project_minimal_deserialize() {
    let json = r#"{
        "id": "project-123",
        "name": "my-project"
    }"#;

    let project: PagesProject =
        serde_json::from_str(json).expect("Failed to deserialize minimal project");

    assert_eq!(project.id, "project-123");
    assert_eq!(project.name, "my-project");
    assert!(project.domains.is_empty());
    assert!(!project.uses_functions);
}

#[test]
fn test_pages_project_full_deserialize() {
    let json = r#"{
        "id": "project-123",
        "name": "my-project",
        "subdomain": "my-project.pages.dev",
        "production_branch": "main",
        "domains": ["example.com", "www.example.com"],
        "framework": "next-js",
        "framework_version": "14.0.0",
        "created_on": "2024-01-01T00:00:00Z",
        "uses_functions": true,
        "build_config": {
            "build_command": "npm run build",
            "destination_dir": "out",
            "root_dir": "/",
            "build_caching": true
        },
        "source": {
            "type": "github",
            "config": {
                "owner": "myuser",
                "repo_name": "myrepo",
                "production_branch": "main"
            }
        }
    }"#;

    let project: PagesProject =
        serde_json::from_str(json).expect("Failed to deserialize full project");

    assert_eq!(project.subdomain, "my-project.pages.dev");
    assert_eq!(project.production_branch, "main");
    assert_eq!(project.domains.len(), 2);
    assert_eq!(project.framework, Some("next-js".to_string()));
    assert!(project.uses_functions);
    assert_eq!(
        project.build_config.build_command,
        Some("npm run build".to_string())
    );
    assert!(project.source.is_some());
}

#[test]
fn test_deployment_deserialize() {
    let json = r#"{
        "id": "deploy-abc123",
        "project_id": "project-123",
        "project_name": "my-project",
        "url": "https://abc123.my-project.pages.dev",
        "environment": "production",
        "created_on": "2024-01-01T00:00:00Z",
        "aliases": ["my-project.pages.dev"],
        "stages": [
            {"name": "queued", "status": "success"},
            {"name": "initialize", "status": "success"},
            {"name": "clone_repo", "status": "success"},
            {"name": "build", "status": "success"},
            {"name": "deploy", "status": "success"}
        ],
        "latest_stage": {"name": "deploy", "status": "success"},
        "is_skipped": false,
        "deployment_trigger": {
            "type": "github:push",
            "metadata": {
                "branch": "main",
                "commit_hash": "abc123def456",
                "commit_message": "Update README"
            }
        }
    }"#;

    let deployment: Deployment =
        serde_json::from_str(json).expect("Failed to deserialize deployment");

    assert_eq!(deployment.id, "deploy-abc123");
    assert_eq!(deployment.environment, "production");
    assert_eq!(deployment.stages.len(), 5);
    assert!(!deployment.is_skipped);
    assert!(deployment.deployment_trigger.is_some());
}

#[test]
fn test_pages_domain_deserialize() {
    let json = r#"{
        "id": "domain-123",
        "name": "example.com",
        "status": "active",
        "verification_status": "verified",
        "certificate_status": "active",
        "created_on": "2024-01-01T00:00:00Z"
    }"#;

    let domain: PagesDomain =
        serde_json::from_str(json).expect("Failed to deserialize domain");

    assert_eq!(domain.name, "example.com");
    assert_eq!(domain.status, "active");
    assert_eq!(domain.certificate_status, Some("active".to_string()));
}

#[test]
fn test_deployment_logs_deserialize() {
    let json = r#"{
        "data": [
            {"ts": "2024-01-01T00:00:00Z", "line": "Installing dependencies..."},
            {"ts": "2024-01-01T00:00:01Z", "line": "Building project..."},
            {"line": "Build complete!"}
        ],
        "has_more": false
    }"#;

    let logs: DeploymentLogs =
        serde_json::from_str(json).expect("Failed to deserialize logs");

    assert_eq!(logs.data.len(), 3);
    assert!(!logs.has_more);
    assert!(logs.data[0].ts.is_some());
    assert!(logs.data[2].ts.is_none());
}

// =============================================================================
// API OPERATION TESTS
// =============================================================================

#[tokio::test]
async fn test_list_projects_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "project-1",
                    "name": "project-one",
                    "subdomain": "project-one.pages.dev",
                    "production_branch": "main"
                },
                {
                    "id": "project-2",
                    "name": "project-two",
                    "subdomain": "project-two.pages.dev",
                    "production_branch": "master"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let projects = pages::list_projects(&client, "test-account")
        .await
        .expect("Failed to list projects");

    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].name, "project-one");
    assert_eq!(projects[1].name, "project-two");
}

#[tokio::test]
async fn test_list_projects_empty() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": []
        })))
        .mount(&mock_server)
        .await;

    let projects = pages::list_projects(&client, "test-account")
        .await
        .expect("Failed to list projects");

    assert!(projects.is_empty());
}

#[tokio::test]
async fn test_get_project_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects/my-project"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "project-123",
                "name": "my-project",
                "subdomain": "my-project.pages.dev",
                "production_branch": "main",
                "framework": "react"
            }
        })))
        .mount(&mock_server)
        .await;

    let project = pages::get_project(&client, "test-account", "my-project")
        .await
        .expect("Failed to get project");

    assert_eq!(project.id, "project-123");
    assert_eq!(project.name, "my-project");
    assert_eq!(project.framework, Some("react".to_string()));
}

#[tokio::test]
async fn test_get_project_not_found() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects/nonexistent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let result = pages::get_project(&client, "test-account", "nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_project_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/accounts/test-account/pages/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "new-project-id",
                "name": "new-project",
                "subdomain": "new-project.pages.dev",
                "production_branch": "main"
            }
        })))
        .mount(&mock_server)
        .await;

    let create = CreateProject {
        name: "new-project".to_string(),
        production_branch: Some("main".to_string()),
        build_config: None,
    };

    let project = pages::create_project(&client, "test-account", create)
        .await
        .expect("Failed to create project");

    assert_eq!(project.name, "new-project");
    assert_eq!(project.production_branch, "main");
}

#[tokio::test]
async fn test_delete_project_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/test-account/pages/projects/my-project"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let result = pages::delete_project(&client, "test-account", "my-project").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_deployments_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects/my-project/deployments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {
                    "id": "deploy-1",
                    "environment": "production",
                    "url": "https://my-project.pages.dev"
                },
                {
                    "id": "deploy-2",
                    "environment": "preview",
                    "url": "https://abc123.my-project.pages.dev"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let deployments = pages::list_deployments(&client, "test-account", "my-project")
        .await
        .expect("Failed to list deployments");

    assert_eq!(deployments.len(), 2);
    assert_eq!(deployments[0].environment, "production");
    assert_eq!(deployments[1].environment, "preview");
}

#[tokio::test]
async fn test_create_deployment_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/accounts/test-account/pages/projects/my-project/deployments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "new-deploy-id",
                "environment": "production",
                "url": "https://my-project.pages.dev",
                "stages": [
                    {"name": "queued", "status": "active"}
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let deployment = pages::create_deployment(&client, "test-account", "my-project")
        .await
        .expect("Failed to create deployment");

    assert_eq!(deployment.id, "new-deploy-id");
    assert_eq!(deployment.environment, "production");
}

#[tokio::test]
async fn test_rollback_deployment_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/accounts/test-account/pages/projects/my-project/deployments/old-deploy/rollback"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "id": "rollback-deploy-id",
                "environment": "production"
            }
        })))
        .mount(&mock_server)
        .await;

    let deployment = pages::rollback_deployment(&client, "test-account", "my-project", "old-deploy")
        .await
        .expect("Failed to rollback deployment");

    assert_eq!(deployment.id, "rollback-deploy-id");
}

#[tokio::test]
async fn test_list_domains_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/accounts/test-account/pages/projects/my-project/domains"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": [
                {"name": "example.com", "status": "active"},
                {"name": "www.example.com", "status": "pending"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let domains = pages::list_domains(&client, "test-account", "my-project")
        .await
        .expect("Failed to list domains");

    assert_eq!(domains.len(), 2);
    assert_eq!(domains[0].name, "example.com");
    assert_eq!(domains[0].status, "active");
}

#[tokio::test]
async fn test_add_domain_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/accounts/test-account/pages/projects/my-project/domains"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": {
                "name": "new-domain.com",
                "status": "pending"
            }
        })))
        .mount(&mock_server)
        .await;

    let add = AddDomain {
        name: "new-domain.com".to_string(),
    };

    let domain = pages::add_domain(&client, "test-account", "my-project", add)
        .await
        .expect("Failed to add domain");

    assert_eq!(domain.name, "new-domain.com");
    assert_eq!(domain.status, "pending");
}

#[tokio::test]
async fn test_delete_domain_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("DELETE"))
        .and(path("/accounts/test-account/pages/projects/my-project/domains/example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let result = pages::delete_domain(&client, "test-account", "my-project", "example.com").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_purge_build_cache_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/accounts/test-account/pages/projects/my-project/purge_build_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "errors": [],
            "messages": [],
            "result": null
        })))
        .mount(&mock_server)
        .await;

    let result = pages::purge_build_cache(&client, "test-account", "my-project").await;

    assert!(result.is_ok());
}
