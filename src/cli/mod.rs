use clap::{Parser, Subcommand};

pub mod cache;
pub mod config;
pub mod d1;
pub mod dns;
pub mod r2;
pub mod token;
pub mod zone;

#[derive(Parser)]
#[command(
    name = "cfad",
    version = "0.1.0",
    about = "CloudFlare Admin CLI - Manage Cloudflare services from the command line",
    long_about = "A fast, type-safe Rust CLI for managing Cloudflare DNS, zones, cache, firewall rules, and analytics"
)]
pub struct Cli {
    /// Configuration profile to use
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Cloudflare API token (overrides config/env)
    #[arg(long, global = true)]
    pub api_token: Option<String>,

    /// Cloudflare API key (requires --api-email)
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Cloudflare account email (used with --api-key)
    #[arg(long, global = true)]
    pub api_email: Option<String>,

    /// Output format [default: table] [possible: table, json, csv]
    #[arg(short, long, global = true, default_value = "table", value_parser = ["table", "json", "csv"], hide_default_value = true, hide_possible_values = true)]
    pub format: String,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Enable debug output
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

    /// D1 database management
    #[command(subcommand)]
    D1(d1::D1Command),

    /// R2 object storage management
    #[command(subcommand)]
    R2(r2::R2Command),

    /// API token management
    #[command(subcommand)]
    Token(token::TokenCommand),

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

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(format!("cfad={}", level)),
    )
    .format_target(false)
    .format_timestamp(None)
    .init();
}
