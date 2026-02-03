use clap::Subcommand;

#[derive(Subcommand)]
pub enum CacheCommand {
    /// Purge cache
    Purge {
        /// Zone name or ID
        zone: String,

        /// Purge everything
        #[arg(long, group = "purge_type")]
        all: bool,

        /// Purge specific files (URLs)
        #[arg(long, group = "purge_type", value_delimiter = ',')]
        files: Option<Vec<String>>,

        /// Purge by cache tags
        #[arg(long, group = "purge_type", value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Purge by hosts
        #[arg(long, group = "purge_type", value_delimiter = ',')]
        hosts: Option<Vec<String>>,

        /// Purge by prefixes
        #[arg(long, group = "purge_type", value_delimiter = ',')]
        prefixes: Option<Vec<String>>,
    },
}
