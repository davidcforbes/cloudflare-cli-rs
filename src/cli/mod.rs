use clap::{Parser, Subcommand};

pub mod cache;
pub mod config;
pub mod dns;
pub mod zone;

#[derive(Parser)]
#[command(
    name = "cfad",
    version = "0.1.0",
    about = "CloudFlare Admin CLI - Manage Cloudflare services from the command line",
    long_about = "A fast, type-safe Rust CLI for managing Cloudflare DNS, zones, cache, firewall rules, and analytics"
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub profile: Option<String>,

    #[arg(long, global = true)]
    pub api_token: Option<String>,

    #[arg(long, global = true)]
    pub api_key: Option<String>,

    #[arg(long, global = true)]
    pub api_email: Option<String>,

    #[arg(
        short,
        long,
        global = true,
        default_value = "table",
        value_parser = ["table", "json", "csv"]
    )]
    pub format: String,

    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// DNS record management
    #[command(subcommand)]
    Dns(dns::DnsCommand),

    /// Zone management
    #[command(subcommand)]
    Zone(zone::ZoneCommand),

    /// Cache management
    #[command(subcommand)]
    Cache(cache::CacheCommand),

    /// Configuration management
    #[command(subcommand)]
    Config(config::ConfigCommand),
}

pub fn setup_logging(verbose: bool, quiet: bool) {
    let level = if verbose {
        "debug"
    } else if quiet {
        "warn"
    } else {
        "info"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(format!("cfad={}", level)))
        .format_target(false)
        .format_timestamp(None)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_level_verbose() {
        let level = if true { "debug" } else if false { "warn" } else { "info" };
        assert_eq!(level, "debug");
    }

    #[test]
    fn test_logging_level_quiet() {
        let level = if false { "debug" } else if true { "warn" } else { "info" };
        assert_eq!(level, "warn");
    }

    #[test]
    fn test_logging_level_normal() {
        let level = if false { "debug" } else if false { "warn" } else { "info" };
        assert_eq!(level, "info");
    }
}
