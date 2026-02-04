use clap::Subcommand;

#[derive(Subcommand)]
pub enum D1Command {
    /// List all D1 databases
    List {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Show D1 database details
    Show {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,
    },

    /// Create a new D1 database
    Create {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database name
        name: String,

        /// Location hint (e.g., wnam, enam, weur, eeur, apac)
        #[arg(long)]
        location: Option<String>,
    },

    /// Update a D1 database
    Update {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// New name for the database
        #[arg(long)]
        name: Option<String>,
    },

    /// Delete a D1 database
    Delete {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Execute a SQL query
    Query {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// SQL query to execute
        sql: String,

        /// Use raw output format (arrays instead of objects)
        #[arg(long)]
        raw: bool,
    },

    /// Execute SQL from a file
    #[command(name = "query-file")]
    QueryFile {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// Path to SQL file
        file: String,

        /// Use raw output format (arrays instead of objects)
        #[arg(long)]
        raw: bool,
    },

    /// Export a D1 database to SQL
    Export {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,
    },

    /// Import SQL into a D1 database
    Import {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// Path to SQL file
        file: String,
    },

    /// Get time travel bookmark
    Bookmark {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// Timestamp to find nearest bookmark
        #[arg(long)]
        timestamp: Option<String>,
    },

    /// Restore database to a point in time
    Restore {
        /// Account ID (uses CLOUDFLARE_ACCOUNT_ID env var or config if not provided)
        #[arg(long)]
        account_id: Option<String>,

        /// Database ID
        database_id: String,

        /// Bookmark to restore to
        #[arg(long)]
        bookmark: Option<String>,

        /// Timestamp to restore to
        #[arg(long)]
        timestamp: Option<String>,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}
