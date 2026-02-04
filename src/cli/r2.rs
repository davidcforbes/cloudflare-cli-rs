use clap::Subcommand;

#[derive(Subcommand)]
pub enum R2Command {
    /// List all R2 buckets
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Show R2 bucket details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Create a new R2 bucket
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        name: String,

        /// Location hint (e.g., wnam, enam, weur, eeur, apac)
        #[arg(long)]
        location: Option<String>,

        /// Storage class
        #[arg(long)]
        storage_class: Option<String>,
    },

    /// Delete an R2 bucket
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// CORS management commands
    #[command(subcommand)]
    Cors(R2CorsCommand),

    /// Custom domain management commands
    #[command(subcommand)]
    Domain(R2DomainCommand),

    /// Managed domain (r2.dev) access commands
    #[command(subcommand, name = "public-access")]
    PublicAccess(R2PublicAccessCommand),

    /// Lifecycle rule management commands
    #[command(subcommand)]
    Lifecycle(R2LifecycleCommand),

    /// Bucket lock management commands
    #[command(subcommand)]
    Lock(R2LockCommand),

    /// R2 storage metrics
    Metrics {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Sippy (incremental migration) commands
    #[command(subcommand)]
    Sippy(R2SippyCommand),

    /// Event notification commands
    #[command(subcommand)]
    Notifications(R2NotificationCommand),

    /// Migration (Super Slurper) commands
    #[command(subcommand)]
    Migrate(R2MigrateCommand),

    /// Temporary credentials commands
    #[command(subcommand, name = "temp-creds")]
    TempCreds(R2TempCredsCommand),
}

#[derive(Subcommand)]
pub enum R2CorsCommand {
    /// Show CORS configuration
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Set CORS configuration from JSON file
    Set {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Path to CORS JSON file
        #[arg(long)]
        file: String,
    },

    /// Delete CORS configuration
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum R2DomainCommand {
    /// List custom domains
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Show custom domain details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Domain name
        domain: String,
    },

    /// Add a custom domain
    Add {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Domain name
        domain: String,

        /// Zone ID (optional, auto-detected if not provided)
        #[arg(long)]
        zone_id: Option<String>,

        /// Minimum TLS version
        #[arg(long)]
        min_tls: Option<String>,
    },

    /// Update a custom domain
    Update {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Domain name
        domain: String,

        /// Enable or disable the domain
        #[arg(long)]
        enabled: Option<bool>,

        /// Minimum TLS version
        #[arg(long)]
        min_tls: Option<String>,
    },

    /// Delete a custom domain
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Domain name
        domain: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum R2PublicAccessCommand {
    /// Show public access status
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Enable public access via r2.dev
    Enable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Disable public access
    Disable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },
}

#[derive(Subcommand)]
pub enum R2LifecycleCommand {
    /// Show lifecycle rules
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Set lifecycle rules from JSON file
    Set {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Path to lifecycle JSON file
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
pub enum R2LockCommand {
    /// Show bucket lock configuration
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Enable bucket lock
    Enable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Lock mode (governance or compliance)
        #[arg(long, value_parser = ["governance", "compliance"])]
        mode: String,

        /// Default retention period in days
        #[arg(long)]
        days: Option<u32>,
    },

    /// Disable bucket lock
    Disable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum R2SippyCommand {
    /// Show Sippy configuration
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Enable Sippy
    Enable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Source provider (e.g., aws, gcs)
        #[arg(long)]
        provider: String,

        /// Source bucket name
        #[arg(long)]
        source_bucket: String,

        /// Source region
        #[arg(long)]
        region: Option<String>,

        /// Access key ID for source
        #[arg(long)]
        access_key_id: Option<String>,

        /// Secret access key for source
        #[arg(long)]
        secret_access_key: Option<String>,
    },

    /// Disable Sippy
    Disable {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum R2NotificationCommand {
    /// List event notification rules
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,
    },

    /// Show event notification rule details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Queue ID
        queue_id: String,
    },

    /// Create an event notification rule
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Queue ID to send notifications to
        queue_id: String,

        /// Event types (e.g., object:create, object:delete)
        #[arg(long, value_delimiter = ',')]
        events: Vec<String>,

        /// Object key prefix filter
        #[arg(long)]
        prefix: Option<String>,

        /// Object key suffix filter
        #[arg(long)]
        suffix: Option<String>,
    },

    /// Delete an event notification rule
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket name
        bucket: String,

        /// Queue ID
        queue_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum R2MigrateCommand {
    /// List migration jobs
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Show migration job details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,
    },

    /// Create a migration job
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Source provider (e.g., aws, gcs, azure)
        #[arg(long)]
        source_provider: String,

        /// Source bucket name
        #[arg(long)]
        source_bucket: String,

        /// Source region
        #[arg(long)]
        source_region: Option<String>,

        /// Target R2 bucket name
        #[arg(long)]
        target_bucket: String,

        /// Access key ID for source
        #[arg(long)]
        access_key_id: String,

        /// Secret access key for source
        #[arg(long)]
        secret_access_key: String,
    },

    /// Pause a migration job
    Pause {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,
    },

    /// Resume a migration job
    Resume {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,
    },

    /// Abort a migration job
    Abort {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Show migration job progress
    Progress {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,
    },

    /// Show migration job logs
    Logs {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Job ID
        job_id: String,
    },
}

#[derive(Subcommand)]
pub enum R2TempCredsCommand {
    /// Generate temporary credentials
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Bucket to scope credentials to
        #[arg(long)]
        bucket: String,

        /// Object key prefix to scope credentials to
        #[arg(long)]
        prefix: Option<String>,

        /// Permission level (read, write, readwrite)
        #[arg(long, value_parser = ["read", "write", "readwrite"])]
        permission: String,

        /// TTL in seconds (default: 3600)
        #[arg(long, default_value = "3600")]
        ttl: u32,
    },
}
