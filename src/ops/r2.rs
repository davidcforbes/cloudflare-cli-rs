use crate::api::r2::{
    CreateR2Bucket, CreateR2CustomDomain, CreateR2EventNotification, CreateR2MigrationJob,
    CreateR2SippyConfig, CreateR2TempCredentials, R2Bucket, R2CorsConfig, R2CorsRule,
    R2CustomDomain, R2EventNotification, R2LifecycleConfig, R2LockConfig, R2ManagedDomain,
    R2Metrics, R2MigrationJob, R2MigrationProgress, R2SippyConfig, R2TempCredentials,
    UpdateR2Bucket, UpdateR2CustomDomain, UpdateR2LockConfig, UpdateR2ManagedDomain,
};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;

// ============================================================================
// Bucket Operations
// ============================================================================

/// List all R2 buckets for an account
pub async fn list_buckets(client: &CloudflareClient, account_id: &str) -> Result<Vec<R2Bucket>> {
    let endpoint = format!("/accounts/{}/r2/buckets", account_id);
    let response: CfResponse<Vec<R2Bucket>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific R2 bucket by name
pub async fn get_bucket(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2Bucket> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}", account_id, bucket_name);
    let response: CfResponse<R2Bucket> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("R2 bucket", bucket_name))
}

/// Create a new R2 bucket
pub async fn create_bucket(
    client: &CloudflareClient,
    account_id: &str,
    bucket: CreateR2Bucket,
) -> Result<R2Bucket> {
    let endpoint = format!("/accounts/{}/r2/buckets", account_id);
    let response: CfResponse<R2Bucket> = client.post(&endpoint, bucket).await?;
    let bucket = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create bucket"))?;
    println!("✓ Created R2 bucket: {}", bucket.name);
    Ok(bucket)
}

/// Update an R2 bucket
pub async fn update_bucket(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    update: UpdateR2Bucket,
) -> Result<R2Bucket> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}", account_id, bucket_name);
    let response: CfResponse<R2Bucket> = client.patch(&endpoint, update).await?;
    let bucket = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from update bucket"))?;
    println!("✓ Updated R2 bucket: {}", bucket.name);
    Ok(bucket)
}

/// Delete an R2 bucket
pub async fn delete_bucket(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}", account_id, bucket_name);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted R2 bucket: {}", bucket_name);
    Ok(())
}

// ============================================================================
// CORS Operations
// ============================================================================

/// Get CORS configuration for a bucket
pub async fn get_cors(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2CorsConfig> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/cors", account_id, bucket_name);
    let response: CfResponse<R2CorsConfig> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2CorsConfig { rules: vec![] }))
}

/// Set CORS configuration for a bucket
pub async fn set_cors(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    rules: Vec<R2CorsRule>,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/cors", account_id, bucket_name);
    let config = R2CorsConfig { rules };
    let _response: CfResponse<serde_json::Value> = client.put(&endpoint, config).await?;
    println!("✓ Updated CORS configuration for bucket: {}", bucket_name);
    Ok(())
}

/// Delete CORS configuration for a bucket
pub async fn delete_cors(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/cors", account_id, bucket_name);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted CORS configuration for bucket: {}", bucket_name);
    Ok(())
}

// ============================================================================
// Custom Domain Operations
// ============================================================================

/// List custom domains for a bucket
pub async fn list_custom_domains(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<Vec<R2CustomDomain>> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/custom",
        account_id, bucket_name
    );
    let response: CfResponse<Vec<R2CustomDomain>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific custom domain
pub async fn get_custom_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    domain: &str,
) -> Result<R2CustomDomain> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/custom/{}",
        account_id, bucket_name, domain
    );
    let response: CfResponse<R2CustomDomain> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Custom domain", domain))
}

/// Register a custom domain for a bucket
pub async fn create_custom_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    domain: CreateR2CustomDomain,
) -> Result<R2CustomDomain> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/custom",
        account_id, bucket_name
    );
    let response: CfResponse<R2CustomDomain> = client.post(&endpoint, domain).await?;
    let result = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create domain"))?;
    println!("✓ Registered custom domain: {}", result.domain);
    Ok(result)
}

/// Update a custom domain
pub async fn update_custom_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    domain: &str,
    update: UpdateR2CustomDomain,
) -> Result<R2CustomDomain> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/custom/{}",
        account_id, bucket_name, domain
    );
    let response: CfResponse<R2CustomDomain> = client.put(&endpoint, update).await?;
    let result = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from update domain"))?;
    println!("✓ Updated custom domain: {}", result.domain);
    Ok(result)
}

/// Delete a custom domain
pub async fn delete_custom_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    domain: &str,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/custom/{}",
        account_id, bucket_name, domain
    );
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted custom domain: {}", domain);
    Ok(())
}

// ============================================================================
// Managed Domain Operations
// ============================================================================

/// Get managed domain (r2.dev) settings
pub async fn get_managed_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2ManagedDomain> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/managed",
        account_id, bucket_name
    );
    let response: CfResponse<R2ManagedDomain> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2ManagedDomain {
        enabled: false,
        domain: None,
    }))
}

/// Update managed domain (r2.dev) settings
pub async fn update_managed_domain(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    enabled: bool,
) -> Result<R2ManagedDomain> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/domains/managed",
        account_id, bucket_name
    );
    let update = UpdateR2ManagedDomain { enabled };
    let response: CfResponse<R2ManagedDomain> = client.put(&endpoint, update).await?;
    let result = response.result.unwrap_or(R2ManagedDomain {
        enabled,
        domain: None,
    });
    println!(
        "✓ {} public access for bucket: {}",
        if enabled { "Enabled" } else { "Disabled" },
        bucket_name
    );
    Ok(result)
}

// ============================================================================
// Lifecycle Operations
// ============================================================================

/// Get lifecycle configuration for a bucket
pub async fn get_lifecycle(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2LifecycleConfig> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/lifecycle",
        account_id, bucket_name
    );
    let response: CfResponse<R2LifecycleConfig> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2LifecycleConfig { rules: vec![] }))
}

/// Set lifecycle configuration for a bucket
pub async fn set_lifecycle(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    config: R2LifecycleConfig,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/r2/buckets/{}/lifecycle",
        account_id, bucket_name
    );
    let _response: CfResponse<serde_json::Value> = client.put(&endpoint, config).await?;
    println!(
        "✓ Updated lifecycle configuration for bucket: {}",
        bucket_name
    );
    Ok(())
}

// ============================================================================
// Lock Operations
// ============================================================================

/// Get bucket lock configuration
pub async fn get_lock(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2LockConfig> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/lock", account_id, bucket_name);
    let response: CfResponse<R2LockConfig> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2LockConfig {
        enabled: false,
        mode: None,
        default_retention_days: None,
    }))
}

/// Set bucket lock configuration
pub async fn set_lock(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    config: UpdateR2LockConfig,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/lock", account_id, bucket_name);
    let _response: CfResponse<serde_json::Value> = client.put(&endpoint, config).await?;
    println!("✓ Updated lock configuration for bucket: {}", bucket_name);
    Ok(())
}

// ============================================================================
// Metrics Operations
// ============================================================================

/// Get R2 storage metrics
pub async fn get_metrics(client: &CloudflareClient, account_id: &str) -> Result<R2Metrics> {
    let endpoint = format!("/accounts/{}/r2/metrics", account_id);
    let response: CfResponse<R2Metrics> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2Metrics { buckets: vec![] }))
}

// ============================================================================
// Sippy Operations
// ============================================================================

/// Get Sippy configuration for a bucket
pub async fn get_sippy(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<R2SippyConfig> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/sippy", account_id, bucket_name);
    let response: CfResponse<R2SippyConfig> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or(R2SippyConfig {
        enabled: false,
        provider: None,
        bucket: None,
        region: None,
    }))
}

/// Configure Sippy for a bucket
pub async fn set_sippy(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    config: CreateR2SippyConfig,
) -> Result<R2SippyConfig> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/sippy", account_id, bucket_name);
    let response: CfResponse<R2SippyConfig> = client.put(&endpoint, config).await?;
    let result = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from set sippy"))?;
    println!("✓ Configured Sippy for bucket: {}", bucket_name);
    Ok(result)
}

/// Disable Sippy for a bucket
pub async fn delete_sippy(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/r2/buckets/{}/sippy", account_id, bucket_name);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Disabled Sippy for bucket: {}", bucket_name);
    Ok(())
}

// ============================================================================
// Event Notification Operations
// ============================================================================

/// List event notification rules for a bucket
pub async fn list_notifications(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
) -> Result<Vec<R2EventNotification>> {
    let endpoint = format!(
        "/accounts/{}/event_notifications/r2/{}/configuration",
        account_id, bucket_name
    );
    let response: CfResponse<Vec<R2EventNotification>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific event notification rule
pub async fn get_notification(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    queue_id: &str,
) -> Result<R2EventNotification> {
    let endpoint = format!(
        "/accounts/{}/event_notifications/r2/{}/configuration/queues/{}",
        account_id, bucket_name, queue_id
    );
    let response: CfResponse<R2EventNotification> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Event notification", queue_id))
}

/// Create an event notification rule
pub async fn create_notification(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    queue_id: &str,
    notification: CreateR2EventNotification,
) -> Result<R2EventNotification> {
    let endpoint = format!(
        "/accounts/{}/event_notifications/r2/{}/configuration/queues/{}",
        account_id, bucket_name, queue_id
    );
    let response: CfResponse<R2EventNotification> = client.put(&endpoint, notification).await?;
    let result = response.result.ok_or_else(|| {
        crate::error::CfadError::api("No result returned from create notification")
    })?;
    println!(
        "✓ Created event notification for bucket: {} -> queue: {}",
        bucket_name, queue_id
    );
    Ok(result)
}

/// Delete an event notification rule
pub async fn delete_notification(
    client: &CloudflareClient,
    account_id: &str,
    bucket_name: &str,
    queue_id: &str,
) -> Result<()> {
    let endpoint = format!(
        "/accounts/{}/event_notifications/r2/{}/configuration/queues/{}",
        account_id, bucket_name, queue_id
    );
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!(
        "✓ Deleted event notification for bucket: {} -> queue: {}",
        bucket_name, queue_id
    );
    Ok(())
}

// ============================================================================
// Migration (Super Slurper) Operations
// ============================================================================

/// List migration jobs
pub async fn list_migration_jobs(
    client: &CloudflareClient,
    account_id: &str,
) -> Result<Vec<R2MigrationJob>> {
    let endpoint = format!("/accounts/{}/slurper/jobs", account_id);
    let response: CfResponse<Vec<R2MigrationJob>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a migration job by ID
pub async fn get_migration_job(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<R2MigrationJob> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}", account_id, job_id);
    let response: CfResponse<R2MigrationJob> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Migration job", job_id))
}

/// Create a migration job
pub async fn create_migration_job(
    client: &CloudflareClient,
    account_id: &str,
    job: CreateR2MigrationJob,
) -> Result<R2MigrationJob> {
    let endpoint = format!("/accounts/{}/slurper/jobs", account_id);
    let response: CfResponse<R2MigrationJob> = client.post(&endpoint, job).await?;
    let result = response.result.ok_or_else(|| {
        crate::error::CfadError::api("No result returned from create migration job")
    })?;
    println!("✓ Created migration job: {}", result.id);
    Ok(result)
}

/// Pause a migration job
pub async fn pause_migration_job(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}/pause", account_id, job_id);
    let _response: CfResponse<serde_json::Value> =
        client.put(&endpoint, serde_json::json!({})).await?;
    println!("✓ Paused migration job: {}", job_id);
    Ok(())
}

/// Resume a migration job
pub async fn resume_migration_job(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}/resume", account_id, job_id);
    let _response: CfResponse<serde_json::Value> =
        client.put(&endpoint, serde_json::json!({})).await?;
    println!("✓ Resumed migration job: {}", job_id);
    Ok(())
}

/// Abort a migration job
pub async fn abort_migration_job(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}/abort", account_id, job_id);
    let _response: CfResponse<serde_json::Value> =
        client.put(&endpoint, serde_json::json!({})).await?;
    println!("✓ Aborted migration job: {}", job_id);
    Ok(())
}

/// Get migration job progress
pub async fn get_migration_progress(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<R2MigrationProgress> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}/progress", account_id, job_id);
    let response: CfResponse<R2MigrationProgress> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No progress returned"))
}

/// Get migration job logs
pub async fn get_migration_logs(
    client: &CloudflareClient,
    account_id: &str,
    job_id: &str,
) -> Result<Vec<String>> {
    let endpoint = format!("/accounts/{}/slurper/jobs/{}/logs", account_id, job_id);
    let response: CfResponse<Vec<String>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

// ============================================================================
// Temporary Credentials Operations
// ============================================================================

/// Generate temporary credentials for R2 access
pub async fn create_temp_credentials(
    client: &CloudflareClient,
    account_id: &str,
    request: CreateR2TempCredentials,
) -> Result<R2TempCredentials> {
    let endpoint = format!("/accounts/{}/r2/temp-access-credentials", account_id);
    let response: CfResponse<R2TempCredentials> = client.post(&endpoint, request).await?;
    let result = response.result.ok_or_else(|| {
        crate::error::CfadError::api("No result returned from create temp credentials")
    })?;
    println!("✓ Generated temporary credentials (expires: {})", result.expiration);
    Ok(result)
}
