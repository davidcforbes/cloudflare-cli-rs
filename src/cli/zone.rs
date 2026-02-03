use clap::Subcommand;

#[derive(Subcommand)]
pub enum ZoneCommand {
    /// List all zones
    List {
        /// Filter by status (active, pending, etc.)
        #[arg(long)]
        status: Option<String>,
    },

    /// Show zone details
    Show {
        /// Zone name or ID
        zone: String,
    },

    /// Create a new zone
    Create {
        /// Zone name
        zone: String,

        /// Cloudflare account ID
        #[arg(long)]
        account_id: Option<String>,
    },

    /// Delete a zone
    Delete {
        /// Zone ID
        zone_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Show zone settings
    Settings {
        /// Zone name or ID
        zone: String,
    },

    /// Update zone settings
    Update {
        /// Zone name or ID
        zone: String,

        /// Security level (off, low, medium, high, under_attack)
        #[arg(long, value_parser = ["off", "low", "medium", "high", "under_attack"])]
        security_level: Option<String>,

        /// Cache level (aggressive, basic, simplified)
        #[arg(long, value_parser = ["aggressive", "basic", "simplified"])]
        cache_level: Option<String>,

        /// Development mode (on, off)
        #[arg(long, value_parser = ["on", "off"])]
        dev_mode: Option<String>,

        /// IPv6 support (on, off)
        #[arg(long, value_parser = ["on", "off"])]
        ipv6: Option<String>,

        /// SSL/TLS mode (off, flexible, full, strict)
        #[arg(long, value_parser = ["off", "flexible", "full", "strict"])]
        ssl: Option<String>,

        /// Always use HTTPS (on, off)
        #[arg(long, value_parser = ["on", "off"])]
        always_https: Option<String>,
    },
}
