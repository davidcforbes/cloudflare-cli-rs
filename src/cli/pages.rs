use clap::Subcommand;

#[derive(Subcommand)]
pub enum PagesCommand {
    /// List all Pages projects
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Show Pages project details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,
    },

    /// Create a new Pages project
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        name: String,

        /// Production branch name
        #[arg(long, default_value = "main")]
        branch: String,

        /// Build command (e.g., "npm run build")
        #[arg(long)]
        build_command: Option<String>,

        /// Build output directory (e.g., "dist", "build")
        #[arg(long)]
        output_dir: Option<String>,
    },

    /// Update a Pages project
    Update {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// New project name
        #[arg(long)]
        name: Option<String>,

        /// New production branch
        #[arg(long)]
        branch: Option<String>,

        /// Build command
        #[arg(long)]
        build_command: Option<String>,

        /// Build output directory
        #[arg(long)]
        output_dir: Option<String>,
    },

    /// Delete a Pages project
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Purge build cache for a project
    #[command(name = "purge-cache")]
    PurgeCache {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,
    },

    /// Deployment management commands
    #[command(subcommand)]
    Deploy(DeployCommand),

    /// Custom domain management commands
    #[command(subcommand)]
    Domain(DomainCommand),
}

#[derive(Subcommand)]
pub enum DeployCommand {
    /// List deployments for a project
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,
    },

    /// Show deployment details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Deployment ID
        deployment_id: String,
    },

    /// Trigger a new deployment
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,
    },

    /// Delete a deployment
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Deployment ID
        deployment_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Retry a failed deployment
    Retry {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Deployment ID to retry
        deployment_id: String,
    },

    /// Rollback to a previous deployment
    Rollback {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Deployment ID to rollback to
        deployment_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// View deployment build logs
    Logs {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Deployment ID
        deployment_id: String,
    },
}

#[derive(Subcommand)]
pub enum DomainCommand {
    /// List custom domains for a project
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,
    },

    /// Show custom domain details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Domain name
        domain: String,
    },

    /// Add a custom domain
    Add {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Domain name to add
        domain: String,
    },

    /// Verify/retry domain verification
    Verify {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Domain name to verify
        domain: String,
    },

    /// Delete a custom domain
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Project name
        project: String,

        /// Domain name to delete
        domain: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}
