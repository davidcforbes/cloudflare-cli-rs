use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize null or missing values as the default for the type
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// A Cloudflare Pages project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagesProject {
    /// Unique project identifier
    pub id: String,
    /// Project name (used in URLs)
    pub name: String,
    /// Default subdomain (e.g., "my-project.pages.dev")
    #[serde(default)]
    pub subdomain: String,
    /// Production branch name
    #[serde(default)]
    pub production_branch: String,
    /// Custom domains attached to the project
    #[serde(default, deserialize_with = "null_to_default")]
    pub domains: Vec<String>,
    /// Detected or configured framework (e.g., "next-js", "react")
    #[serde(default)]
    pub framework: Option<String>,
    /// Framework version
    #[serde(default)]
    pub framework_version: Option<String>,
    /// When the project was created
    #[serde(default)]
    pub created_on: Option<String>,
    /// Build configuration
    #[serde(default)]
    pub build_config: BuildConfig,
    /// Deployment configurations for preview and production
    #[serde(default)]
    pub deployment_configs: Option<DeploymentConfigs>,
    /// Source repository configuration
    #[serde(default)]
    pub source: Option<SourceConfig>,
    /// Most recent deployment
    #[serde(default)]
    pub latest_deployment: Option<Deployment>,
    /// Current canonical (production) deployment
    #[serde(default)]
    pub canonical_deployment: Option<Deployment>,
    /// Whether the project uses Pages Functions
    #[serde(default, deserialize_with = "null_to_default")]
    pub uses_functions: bool,
    /// Production worker script name
    #[serde(default)]
    pub production_script_name: Option<String>,
    /// Preview worker script name
    #[serde(default)]
    pub preview_script_name: Option<String>,
    /// Catch any additional fields
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Build configuration for a Pages project
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildConfig {
    /// Command to run for building (e.g., "npm run build")
    #[serde(default)]
    pub build_command: Option<String>,
    /// Output directory (e.g., "dist", "build", "out")
    #[serde(default)]
    pub destination_dir: Option<String>,
    /// Root directory for the build
    #[serde(default)]
    pub root_dir: Option<String>,
    /// Whether build caching is enabled
    #[serde(default)]
    pub build_caching: Option<bool>,
    /// Web analytics tag
    #[serde(default)]
    pub web_analytics_tag: Option<String>,
    /// Web analytics token
    #[serde(default)]
    pub web_analytics_token: Option<String>,
}

/// Deployment configurations for different environments
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentConfigs {
    /// Preview environment configuration
    #[serde(default)]
    pub preview: Option<EnvironmentConfig>,
    /// Production environment configuration
    #[serde(default)]
    pub production: Option<EnvironmentConfig>,
}

/// Environment-specific configuration
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentConfig {
    /// Environment variables
    #[serde(default)]
    pub env_vars: Option<serde_json::Value>,
    /// Compatibility date
    #[serde(default)]
    pub compatibility_date: Option<String>,
    /// Compatibility flags
    #[serde(default, deserialize_with = "null_to_default")]
    pub compatibility_flags: Vec<String>,
    /// Whether to fail open on errors
    #[serde(default)]
    pub fail_open: Option<bool>,
    /// Always use latest compatibility date
    #[serde(default)]
    pub always_use_latest_compatibility_date: Option<bool>,
    /// D1 database bindings
    #[serde(default)]
    pub d1_databases: Option<serde_json::Value>,
    /// KV namespace bindings
    #[serde(default)]
    pub kv_namespaces: Option<serde_json::Value>,
    /// R2 bucket bindings
    #[serde(default)]
    pub r2_buckets: Option<serde_json::Value>,
    /// Durable Object bindings
    #[serde(default)]
    pub durable_object_namespaces: Option<serde_json::Value>,
    /// Service bindings
    #[serde(default)]
    pub services: Option<serde_json::Value>,
    /// Queue bindings
    #[serde(default)]
    pub queues: Option<serde_json::Value>,
    /// Catch any additional fields
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Source repository configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    /// Source type ("github" or "gitlab")
    #[serde(rename = "type")]
    pub source_type: String,
    /// Repository configuration
    #[serde(default)]
    pub config: Option<RepoConfig>,
}

/// Repository configuration
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepoConfig {
    /// Repository owner (user or organization)
    #[serde(default)]
    pub owner: Option<String>,
    /// Repository name
    #[serde(default)]
    pub repo_name: Option<String>,
    /// Production branch
    #[serde(default)]
    pub production_branch: Option<String>,
    /// Whether to deploy PRs
    #[serde(default)]
    pub pr_comments_enabled: Option<bool>,
    /// Whether deployments are enabled
    #[serde(default)]
    pub deployments_enabled: Option<bool>,
    /// Branch patterns for preview deployments
    #[serde(default, deserialize_with = "null_to_default")]
    pub preview_branch_includes: Vec<String>,
    /// Branch patterns to exclude from preview
    #[serde(default, deserialize_with = "null_to_default")]
    pub preview_branch_excludes: Vec<String>,
}

/// A deployment of a Pages project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deployment {
    /// Unique deployment identifier
    pub id: String,
    /// Project ID this deployment belongs to
    #[serde(default)]
    pub project_id: Option<String>,
    /// Project name
    #[serde(default)]
    pub project_name: Option<String>,
    /// Full deployment URL
    #[serde(default)]
    pub url: Option<String>,
    /// Environment ("production" or "preview")
    #[serde(default)]
    pub environment: String,
    /// When the deployment was created
    #[serde(default)]
    pub created_on: Option<String>,
    /// When the deployment was last modified
    #[serde(default)]
    pub modified_on: Option<String>,
    /// Alias URLs for this deployment
    #[serde(default, deserialize_with = "null_to_default")]
    pub aliases: Vec<String>,
    /// Build and deploy stages
    #[serde(default, deserialize_with = "null_to_default")]
    pub stages: Vec<Stage>,
    /// Current/latest stage
    #[serde(default)]
    pub latest_stage: Option<Stage>,
    /// Whether the build was skipped
    #[serde(default, deserialize_with = "null_to_default")]
    pub is_skipped: bool,
    /// What triggered this deployment
    #[serde(default)]
    pub deployment_trigger: Option<DeploymentTrigger>,
    /// Build configuration used
    #[serde(default)]
    pub build_config: Option<BuildConfig>,
    /// Environment variables
    #[serde(default)]
    pub env_vars: Option<serde_json::Value>,
    /// Whether the deployment uses Functions
    #[serde(default, deserialize_with = "null_to_default")]
    pub uses_functions: bool,
    /// Short deployment ID
    #[serde(default)]
    pub short_id: Option<String>,
    /// Catch any additional fields
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// A stage in the deployment pipeline
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stage {
    /// Stage name (queued, initialize, clone_repo, build, deploy)
    #[serde(default)]
    pub name: String,
    /// Stage status (idle, active, success, failure, skipped)
    #[serde(default)]
    pub status: String,
    /// When the stage started
    #[serde(default)]
    pub started_on: Option<String>,
    /// When the stage ended
    #[serde(default)]
    pub ended_on: Option<String>,
}

/// Information about what triggered a deployment
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentTrigger {
    /// Trigger type (e.g., "ad_hoc", "github:push")
    #[serde(rename = "type", default)]
    pub trigger_type: String,
    /// Additional trigger metadata
    #[serde(default)]
    pub metadata: Option<TriggerMetadata>,
}

/// Metadata about a deployment trigger
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerMetadata {
    /// Git branch
    #[serde(default)]
    pub branch: Option<String>,
    /// Git commit hash
    #[serde(default)]
    pub commit_hash: Option<String>,
    /// Commit message
    #[serde(default)]
    pub commit_message: Option<String>,
    /// Commit author
    #[serde(default)]
    pub commit_author: Option<String>,
}

/// A custom domain attached to a Pages project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagesDomain {
    /// Domain ID
    #[serde(default)]
    pub id: Option<String>,
    /// Domain name
    pub name: String,
    /// Domain status (active, pending, etc.)
    #[serde(default)]
    pub status: String,
    /// Verification status
    #[serde(default)]
    pub verification_status: Option<String>,
    /// Certificate status
    #[serde(default)]
    pub certificate_status: Option<String>,
    /// When the domain was created
    #[serde(default)]
    pub created_on: Option<String>,
}

/// Deployment logs response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentLogs {
    /// Log lines
    #[serde(default)]
    pub data: Vec<LogLine>,
    /// Whether there are more logs
    #[serde(default)]
    pub has_more: bool,
}

/// A single log line
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogLine {
    /// Log timestamp
    #[serde(default)]
    pub ts: Option<String>,
    /// Log line content
    #[serde(default)]
    pub line: String,
}

// Request structs

/// Request body for creating a new project
#[derive(Debug, Clone, Serialize)]
pub struct CreateProject {
    /// Project name
    pub name: String,
    /// Production branch name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<String>,
    /// Build configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_config: Option<BuildConfig>,
}

/// Request body for updating a project
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateProject {
    /// New project name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New production branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<String>,
    /// Updated build configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_config: Option<BuildConfig>,
    /// Updated deployment configurations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configs: Option<DeploymentConfigs>,
}

/// Request body for adding a custom domain
#[derive(Debug, Clone, Serialize)]
pub struct AddDomain {
    /// Domain name to add
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pages_project_deserialize_minimal() {
        let json = r#"{
            "id": "project-123",
            "name": "my-project"
        }"#;

        let project: PagesProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "project-123");
        assert_eq!(project.name, "my-project");
        assert!(project.domains.is_empty());
    }

    #[test]
    fn test_pages_project_deserialize_full() {
        let json = r#"{
            "id": "project-123",
            "name": "my-project",
            "subdomain": "my-project.pages.dev",
            "production_branch": "main",
            "domains": ["example.com"],
            "framework": "next-js",
            "created_on": "2024-01-01T00:00:00Z",
            "build_config": {
                "build_command": "npm run build",
                "destination_dir": "out"
            },
            "uses_functions": true
        }"#;

        let project: PagesProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.subdomain, "my-project.pages.dev");
        assert_eq!(project.production_branch, "main");
        assert_eq!(project.framework, Some("next-js".to_string()));
        assert!(project.uses_functions);
        assert_eq!(
            project.build_config.build_command,
            Some("npm run build".to_string())
        );
    }

    #[test]
    fn test_deployment_deserialize() {
        let json = r#"{
            "id": "deploy-123",
            "url": "https://abc123.my-project.pages.dev",
            "environment": "production",
            "created_on": "2024-01-01T00:00:00Z",
            "stages": [
                {"name": "queued", "status": "success"},
                {"name": "build", "status": "active"}
            ],
            "is_skipped": false
        }"#;

        let deployment: Deployment = serde_json::from_str(json).unwrap();
        assert_eq!(deployment.id, "deploy-123");
        assert_eq!(deployment.environment, "production");
        assert_eq!(deployment.stages.len(), 2);
        assert!(!deployment.is_skipped);
    }

    #[test]
    fn test_stage_deserialize() {
        let json = r#"{
            "name": "build",
            "status": "success",
            "started_on": "2024-01-01T00:00:00Z",
            "ended_on": "2024-01-01T00:01:00Z"
        }"#;

        let stage: Stage = serde_json::from_str(json).unwrap();
        assert_eq!(stage.name, "build");
        assert_eq!(stage.status, "success");
        assert!(stage.started_on.is_some());
        assert!(stage.ended_on.is_some());
    }

    #[test]
    fn test_pages_domain_deserialize() {
        let json = r#"{
            "name": "example.com",
            "status": "active",
            "certificate_status": "active"
        }"#;

        let domain: PagesDomain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.status, "active");
    }

    #[test]
    fn test_create_project_serialize() {
        let create = CreateProject {
            name: "my-project".to_string(),
            production_branch: Some("main".to_string()),
            build_config: None,
        };

        let json = serde_json::to_string(&create).unwrap();
        assert!(json.contains("my-project"));
        assert!(json.contains("main"));
    }

    #[test]
    fn test_source_config_deserialize() {
        let json = r#"{
            "type": "github",
            "config": {
                "owner": "myuser",
                "repo_name": "myrepo",
                "production_branch": "main"
            }
        }"#;

        let source: SourceConfig = serde_json::from_str(json).unwrap();
        assert_eq!(source.source_type, "github");
        assert_eq!(source.config.as_ref().unwrap().owner, Some("myuser".to_string()));
    }
}
