use serde::{Deserialize, Serialize};

/// R2 Bucket representation from Cloudflare API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2Bucket {
    /// Name of the bucket
    pub name: String,
    /// When the bucket was created
    #[serde(default)]
    pub creation_date: String,
    /// Location of the bucket
    #[serde(default)]
    pub location: Option<String>,
    /// Storage class of the bucket
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// Request payload for creating an R2 bucket
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2Bucket {
    /// Name for the new bucket
    pub name: String,
    /// Location hint for the bucket (e.g., "wnam", "enam", "weur", "eeur", "apac")
    #[serde(rename = "locationHint", skip_serializing_if = "Option::is_none")]
    pub location_hint: Option<String>,
    /// Storage class for the bucket
    #[serde(rename = "storageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// Request payload for updating an R2 bucket
#[derive(Debug, Clone, Serialize)]
pub struct UpdateR2Bucket {
    /// Storage class for the bucket
    #[serde(rename = "storageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// CORS rule for an R2 bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2CorsRule {
    /// Allowed origins (e.g., ["https://example.com"] or ["*"])
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Vec<String>,
    /// Allowed HTTP methods
    #[serde(rename = "allowedMethods")]
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    #[serde(rename = "allowedHeaders", default)]
    pub allowed_headers: Vec<String>,
    /// Headers exposed to the browser
    #[serde(rename = "exposeHeaders", default)]
    pub expose_headers: Vec<String>,
    /// Max age in seconds for preflight cache
    #[serde(rename = "maxAgeSeconds", default)]
    pub max_age_seconds: u32,
}

/// CORS configuration for an R2 bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2CorsConfig {
    /// List of CORS rules
    pub rules: Vec<R2CorsRule>,
}

/// Custom domain for an R2 bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2CustomDomain {
    /// Domain name
    pub domain: String,
    /// Whether the domain is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Status of the domain (e.g., "active", "pending")
    #[serde(default)]
    pub status: String,
    /// Minimum TLS version
    #[serde(rename = "minTLS", default)]
    pub min_tls: Option<String>,
    /// Zone ID the domain belongs to
    #[serde(rename = "zoneId", default)]
    pub zone_id: Option<String>,
    /// Zone name the domain belongs to
    #[serde(rename = "zoneName", default)]
    pub zone_name: Option<String>,
}

/// Request to register a custom domain
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2CustomDomain {
    /// Domain name to register
    pub domain: String,
    /// Zone ID to use (optional, auto-detected if not provided)
    #[serde(rename = "zoneId", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// Minimum TLS version
    #[serde(rename = "minTLS", skip_serializing_if = "Option::is_none")]
    pub min_tls: Option<String>,
}

/// Request to update a custom domain
#[derive(Debug, Clone, Serialize)]
pub struct UpdateR2CustomDomain {
    /// Whether the domain is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Minimum TLS version
    #[serde(rename = "minTLS", skip_serializing_if = "Option::is_none")]
    pub min_tls: Option<String>,
}

/// Managed domain (r2.dev) configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2ManagedDomain {
    /// Whether public access is enabled
    #[serde(default)]
    pub enabled: bool,
    /// The public URL for the bucket
    #[serde(default)]
    pub domain: Option<String>,
}

/// Request to update managed domain settings
#[derive(Debug, Clone, Serialize)]
pub struct UpdateR2ManagedDomain {
    /// Whether to enable public access
    pub enabled: bool,
}

/// Lifecycle rule for an R2 bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2LifecycleRule {
    /// Rule ID
    #[serde(default)]
    pub id: String,
    /// Whether the rule is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Conditions for the rule to apply
    #[serde(default)]
    pub conditions: R2LifecycleConditions,
    /// Actions to take when conditions are met
    #[serde(default)]
    pub actions: R2LifecycleActions,
}

/// Conditions for a lifecycle rule
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct R2LifecycleConditions {
    /// Object key prefix to match
    #[serde(default)]
    pub prefix: Option<String>,
}

/// Actions for a lifecycle rule
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct R2LifecycleActions {
    /// Delete objects after this many days
    #[serde(rename = "deleteAfterDays", default)]
    pub delete_after_days: Option<u32>,
    /// Abort incomplete multipart uploads after this many days
    #[serde(rename = "abortIncompleteMultipartUploadAfterDays", default)]
    pub abort_incomplete_multipart_upload_after_days: Option<u32>,
}

/// Lifecycle configuration for an R2 bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2LifecycleConfig {
    /// List of lifecycle rules
    pub rules: Vec<R2LifecycleRule>,
}

/// Bucket lock configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2LockConfig {
    /// Whether object lock is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Default retention mode (governance or compliance)
    #[serde(default)]
    pub mode: Option<String>,
    /// Default retention period in days
    #[serde(rename = "defaultRetentionDays", default)]
    pub default_retention_days: Option<u32>,
}

/// Request to update bucket lock configuration
#[derive(Debug, Clone, Serialize)]
pub struct UpdateR2LockConfig {
    /// Whether to enable object lock
    pub enabled: bool,
    /// Retention mode (governance or compliance)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Default retention period in days
    #[serde(
        rename = "defaultRetentionDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_retention_days: Option<u32>,
}

/// R2 storage metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2Metrics {
    /// Storage metrics by bucket
    #[serde(default)]
    pub buckets: Vec<R2BucketMetrics>,
}

/// Metrics for a single bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2BucketMetrics {
    /// Bucket name
    #[serde(default)]
    pub bucket_name: String,
    /// Storage used in bytes
    #[serde(default)]
    pub storage_bytes: u64,
    /// Number of objects
    #[serde(default)]
    pub object_count: u64,
    /// Upload count
    #[serde(default)]
    pub upload_count: u64,
}

/// Sippy (incremental migration) configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2SippyConfig {
    /// Whether Sippy is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Source provider (e.g., "aws", "gcs")
    #[serde(default)]
    pub provider: Option<String>,
    /// Source bucket name
    #[serde(default)]
    pub bucket: Option<String>,
    /// Source region
    #[serde(default)]
    pub region: Option<String>,
}

/// Request to configure Sippy
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2SippyConfig {
    /// Source provider (e.g., "aws", "gcs")
    pub provider: String,
    /// Source bucket name
    pub bucket: String,
    /// Source region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Access key ID for source
    #[serde(rename = "accessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// Secret access key for source
    #[serde(rename = "secretAccessKey", skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

/// Event notification rule
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2EventNotification {
    /// Queue ID to send notifications to
    #[serde(rename = "queueId", default)]
    pub queue_id: String,
    /// Event types to notify on
    #[serde(default)]
    pub events: Vec<String>,
    /// Object key prefix filter
    #[serde(default)]
    pub prefix: Option<String>,
    /// Object key suffix filter
    #[serde(default)]
    pub suffix: Option<String>,
}

/// Request to create an event notification rule
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2EventNotification {
    /// Event types to notify on
    pub events: Vec<String>,
    /// Object key prefix filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Object key suffix filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

/// Super Slurper migration job
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2MigrationJob {
    /// Job ID
    #[serde(default)]
    pub id: String,
    /// Job status (e.g., "running", "completed", "failed")
    #[serde(default)]
    pub status: String,
    /// Source provider
    #[serde(default)]
    pub source_provider: String,
    /// Source bucket
    #[serde(default)]
    pub source_bucket: String,
    /// Target bucket
    #[serde(default)]
    pub target_bucket: String,
    /// Created timestamp
    #[serde(default)]
    pub created_at: String,
    /// Completed timestamp
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// Request to create a migration job
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2MigrationJob {
    /// Source provider (e.g., "aws", "gcs", "azure")
    pub source_provider: String,
    /// Source bucket name
    pub source_bucket: String,
    /// Source region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    /// Target R2 bucket name
    pub target_bucket: String,
    /// Access key ID for source
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
    /// Secret access key for source
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
}

/// Migration job progress
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2MigrationProgress {
    /// Number of objects migrated
    #[serde(default)]
    pub objects_migrated: u64,
    /// Number of objects total
    #[serde(default)]
    pub objects_total: u64,
    /// Bytes migrated
    #[serde(default)]
    pub bytes_migrated: u64,
    /// Bytes total
    #[serde(default)]
    pub bytes_total: u64,
    /// Number of errors
    #[serde(default)]
    pub errors: u64,
}

/// Temporary credentials for R2 access
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct R2TempCredentials {
    /// Access key ID
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
    /// Secret access key
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    /// Session token
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    /// Expiration timestamp
    pub expiration: String,
}

/// Request to generate temporary credentials
#[derive(Debug, Clone, Serialize)]
pub struct CreateR2TempCredentials {
    /// Bucket to scope credentials to
    pub bucket: String,
    /// Object key prefix to scope credentials to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Permission level (read, write, readwrite)
    pub permission: String,
    /// TTL in seconds
    #[serde(rename = "ttlSeconds")]
    pub ttl_seconds: u32,
}
