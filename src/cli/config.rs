use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Initialize configuration file
    Init,

    /// Show current configuration
    #[command(alias = "display")]
    Show { profile: Option<String> },

    /// List all profiles
    #[command(subcommand)]
    Profiles(ProfileCommand),
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// List all profiles
    List,

    /// Add a new profile
    Add { name: String },

    /// Set default profile
    #[command(alias = "default")]
    SetDefault { name: String },
}
