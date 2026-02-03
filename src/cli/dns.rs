use clap::Subcommand;

#[derive(Subcommand)]
pub enum DnsCommand {
    /// List DNS records
    List {
        /// Zone name (e.g., example.com)
        zone: String,

        /// Filter by record type (A, AAAA, CNAME, MX, TXT, etc.)
        #[arg(long)]
        r#type: Option<String>,

        /// Filter by record name
        #[arg(long)]
        name: Option<String>,
    },

    /// Show DNS record details
    Show {
        /// Zone name (e.g., example.com)
        zone: String,

        /// Record ID
        record_id: String,
    },

    /// Create a new DNS record
    Add {
        /// Zone name
        zone: String,

        /// Record type (A, AAAA, CNAME, MX, TXT, SRV, NS)
        #[arg(value_parser = ["A", "AAAA", "CNAME", "MX", "TXT", "SRV", "NS", "SPF", "LOC"])]
        r#type: String,

        /// Record name
        name: String,

        /// Record content
        content: String,

        /// TTL (1 for automatic)
        #[arg(long, default_value = "1")]
        ttl: u32,

        /// Proxied (orange cloud) for A/AAAA/CNAME records
        #[arg(long)]
        proxied: bool,

        /// Priority for MX/SRV records
        #[arg(long)]
        priority: Option<u16>,
    },

    /// Update a DNS record
    Update {
        /// Zone name (e.g., example.com)
        zone: String,

        /// Record ID
        record_id: String,

        /// New record name
        #[arg(long)]
        name: Option<String>,

        /// New record content
        #[arg(long)]
        content: Option<String>,

        /// New TTL
        #[arg(long)]
        ttl: Option<u32>,

        /// New proxied status
        #[arg(long)]
        proxied: Option<bool>,

        /// New priority (for MX/SRV)
        #[arg(long)]
        priority: Option<u16>,
    },

    /// Delete a DNS record
    Delete {
        /// Zone name (e.g., example.com)
        zone: String,

        /// Record ID
        record_id: String,

        /// Skip confirmation
        #[arg(long)]
        confirm: bool,
    },

    /// Import DNS records from file
    Import {
        /// Zone name
        zone: String,

        /// File path (BIND or CSV format)
        file: String,
    },
}
