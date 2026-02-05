use crate::api::pages::{
    AddDomain, CreateProject, Deployment, DeploymentLogs, PagesDomain, PagesProject, UpdateProject,
};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;

// ============================================================================
// Project Operations
// ============================================================================

/// List all Pages projects for an account
pub async fn list_projects(
    client: &CloudflareClient,
    account_id: &str,
) -> Result<Vec<PagesProject>> {
    let endpoint = format!("/accounts/{}/pages/projects", account_id);
    let response: CfResponse<Vec<PagesProject>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific Pages project by name
pub async fn get_project(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<PagesProject> {
    let endpoint = format!("/accounts/{}/pages/projects/{}", account_id, project_name);
    let response: CfResponse<PagesProject> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Pages project", project_name))
}

/// Create a new Pages project
pub async fn create_project(
    client: &CloudflareClient,
    account_id: &str,
    project: CreateProject,
) -> Result<PagesProject> {
    let endpoint = format!("/accounts/{}/pages/projects", account_id);
    let response: CfResponse<PagesProject> = client.post(&endpoint, project).await?;
    let project = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create project"))?;
    println!("✓ Created Pages project: {}", project.name);
    Ok(project)
}

/// Update a Pages project
pub async fn update_project(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    update: UpdateProject,
) -> Result<PagesProject> {
    let endpoint = format!("/accounts/{}/pages/projects/{}", account_id, project_name);
    let response: CfResponse<PagesProject> = client.patch(&endpoint, update).await?;
    let project = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from update project"))?;
    println!("✓ Updated Pages project: {}", project.name);
    Ok(project)
}

/// Delete a Pages project
pub async fn delete_project(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/pages/projects/{}", account_id, project_name);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted Pages project: {}", project_name);
    Ok(())
}

/// Purge build cache for a Pages project
pub async fn purge_build_cache(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/purge_build_cache",
        account_id, project_name
    );
    let _response: CfResponse<serde_json::Value> =
        client.post(&endpoint, serde_json::json!({})).await?;
    println!("✓ Purged build cache for: {}", project_name);
    Ok(())
}

// ============================================================================
// Deployment Operations
// ============================================================================

/// List deployments for a Pages project
pub async fn list_deployments(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<Vec<Deployment>> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments",
        account_id, project_name
    );
    let response: CfResponse<Vec<Deployment>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific deployment
pub async fn get_deployment(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    deployment_id: &str,
) -> Result<Deployment> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments/{}",
        account_id, project_name, deployment_id
    );
    let response: CfResponse<Deployment> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Deployment", deployment_id))
}

/// Create a new deployment (trigger build from production branch)
pub async fn create_deployment(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<Deployment> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments",
        account_id, project_name
    );
    let response: CfResponse<Deployment> = client.post(&endpoint, serde_json::json!({})).await?;
    let deployment = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create deployment"))?;
    println!("✓ Created deployment: {}", deployment.id);
    Ok(deployment)
}

/// Delete a deployment
pub async fn delete_deployment(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    deployment_id: &str,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments/{}",
        account_id, project_name, deployment_id
    );
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted deployment: {}", deployment_id);
    Ok(())
}

/// Retry a failed deployment
pub async fn retry_deployment(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    deployment_id: &str,
) -> Result<Deployment> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments/{}/retry",
        account_id, project_name, deployment_id
    );
    let response: CfResponse<Deployment> = client.post(&endpoint, serde_json::json!({})).await?;
    let deployment = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from retry deployment"))?;
    println!("✓ Retried deployment: {}", deployment.id);
    Ok(deployment)
}

/// Rollback to a previous deployment
pub async fn rollback_deployment(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    deployment_id: &str,
) -> Result<Deployment> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments/{}/rollback",
        account_id, project_name, deployment_id
    );
    let response: CfResponse<Deployment> = client.post(&endpoint, serde_json::json!({})).await?;
    let deployment = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from rollback"))?;
    println!("✓ Rolled back to deployment: {}", deployment_id);
    Ok(deployment)
}

/// Get deployment build logs
pub async fn get_deployment_logs(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    deployment_id: &str,
) -> Result<DeploymentLogs> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/deployments/{}/history/logs",
        account_id, project_name, deployment_id
    );
    let response: CfResponse<DeploymentLogs> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Deployment logs", deployment_id))
}

// ============================================================================
// Domain Operations
// ============================================================================

/// List custom domains for a Pages project
pub async fn list_domains(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
) -> Result<Vec<PagesDomain>> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/domains",
        account_id, project_name
    );
    let response: CfResponse<Vec<PagesDomain>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific domain
pub async fn get_domain(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    domain_name: &str,
) -> Result<PagesDomain> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/domains/{}",
        account_id, project_name, domain_name
    );
    let response: CfResponse<PagesDomain> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Domain", domain_name))
}

/// Add a custom domain to a Pages project
pub async fn add_domain(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    domain: AddDomain,
) -> Result<PagesDomain> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/domains",
        account_id, project_name
    );
    let response: CfResponse<PagesDomain> = client.post(&endpoint, domain).await?;
    let domain = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from add domain"))?;
    println!("✓ Added domain: {}", domain.name);
    Ok(domain)
}

/// Verify/retry domain verification
pub async fn verify_domain(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    domain_name: &str,
) -> Result<PagesDomain> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/domains/{}",
        account_id, project_name, domain_name
    );
    let response: CfResponse<PagesDomain> =
        client.patch(&endpoint, serde_json::json!({})).await?;
    let domain = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from verify domain"))?;
    println!("✓ Verification triggered for: {}", domain.name);
    Ok(domain)
}

/// Delete a custom domain from a Pages project
pub async fn delete_domain(
    client: &CloudflareClient,
    account_id: &str,
    project_name: &str,
    domain_name: &str,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/pages/projects/{}/domains/{}",
        account_id, project_name, domain_name
    );
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted domain: {}", domain_name);
    Ok(())
}
