//! Token CLI commands
//!
//! Subcommands for managing Cloudflare API tokens.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum TokenCommand {
    /// List all API tokens
    List,

    /// Show token details
    Show {
        /// Token ID
        token_id: String,
    },

    /// Create a new API token
    Create {
        /// Token name
        #[arg(long)]
        name: String,

        /// Permission group IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        permissions: Vec<String>,

        /// Resource scope (e.g., "com.cloudflare.api.account.*" or zone ID)
        #[arg(long, default_value = "com.cloudflare.api.account.*")]
        resources: String,

        /// Expiration date (ISO 8601 format, e.g., "2025-12-31T23:59:59Z")
        #[arg(long)]
        expires: Option<String>,

        /// Not valid before date (ISO 8601 format)
        #[arg(long)]
        not_before: Option<String>,
    },

    /// Update an existing token
    Update {
        /// Token ID
        token_id: String,

        /// New name for the token
        #[arg(long)]
        name: Option<String>,

        /// Status (active or disabled)
        #[arg(long)]
        status: Option<String>,

        /// New expiration date (ISO 8601 format)
        #[arg(long)]
        expires: Option<String>,
    },

    /// Delete a token
    Delete {
        /// Token ID
        token_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Verify the current token
    Verify,

    /// List available permission groups
    Permissions {
        /// Filter by scope (account, zone, user)
        #[arg(long)]
        scope: Option<String>,
    },

    /// Roll (regenerate) a token's value
    Roll {
        /// Token ID
        token_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },
}
