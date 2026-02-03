#![allow(dead_code)]
#![allow(unused_imports)]

use clap::Parser;
use std::process;
use tracing::error;

mod cli;
mod client;
mod config;
mod error;
mod output;
mod utils;

// API and operations modules (stubs for Phase 1)
mod api;
mod ops;
mod metrics;

use cli::{Cli, Commands};
use config::{Config, Profile};
use error::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Error: {}", e);
        eprintln!("{}", e);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    cli::setup_logging(cli.verbose, cli.quiet);

    // Handle config commands first (they don't need API auth)
    if let Commands::Config(cmd) = cli.command {
        return handle_config_command(cmd).await;
    }

    // Load configuration
    let mut profile = Config::load(cli.profile.as_deref())?;

    // Override with CLI flags if provided
    if let Some(api_token) = &cli.api_token {
        profile.api_token = Some(api_token.clone());
        profile.api_key = None;
        profile.api_email = None;
    }

    if let Some(api_key) = &cli.api_key {
        profile.api_key = Some(api_key.clone());
    }

    if let Some(api_email) = &cli.api_email {
        profile.api_email = Some(api_email.clone());
    }

    // Get auth method
    let auth = profile.auth_method()?;

    // Create client
    let client = client::CloudflareClient::new(auth)?;

    // Handle commands
    match cli.command {
        Commands::Config(_) => unreachable!("Config handled above"),
        Commands::Dns(cmd) => handle_dns_command(&client, cmd).await?,
        Commands::Zone(cmd) => handle_zone_command(&client, cmd).await?,
        Commands::Cache(cmd) => handle_cache_command(&client, cmd).await?,
    }

    Ok(())
}

async fn handle_dns_command(client: &client::CloudflareClient, cmd: cli::dns::DnsCommand) -> Result<()> {
    use cli::dns::DnsCommand;
    use crate::api::dns::{CreateDnsRecord, UpdateDnsRecord};

    match cmd {
        DnsCommand::List { zone, r#type, name } => {
            // First get zone ID from zone name
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            let records = ops::dns::list_records(client, &zone_obj.id, r#type.as_deref(), name.as_deref()).await?;

            println!("\nDNS Records for {}:\n", zone);
            output::table::print_dns_records(&records);
        }
        DnsCommand::Show { zone, record_id } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            let record = ops::dns::get_record(client, &zone_obj.id, &record_id).await?;
            output::table::print_dns_record(&record);
        }
        DnsCommand::Add { zone, r#type, name, content, ttl, proxied, priority } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            let record = CreateDnsRecord {
                record_type: r#type,
                name,
                content,
                ttl: Some(ttl),
                proxied: Some(proxied),
                priority,
                data: None,
            };
            ops::dns::create_record(client, &zone_obj.id, record).await?;
        }
        DnsCommand::Update { zone, record_id, name, content, ttl, proxied, priority } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            let update = UpdateDnsRecord {
                record_type: None,
                name,
                content,
                ttl,
                proxied,
                priority,
            };
            ops::dns::update_record(client, &zone_obj.id, &record_id, update).await?;
        }
        DnsCommand::Delete { zone, record_id, confirm } => {
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            ops::dns::delete_record(client, &zone_obj.id, &record_id).await?;
        }
        DnsCommand::Import { zone, file } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            ops::dns::import_records(client, &zone_obj.id, &file).await?;
        }
    }

    Ok(())
}

async fn handle_zone_command(client: &client::CloudflareClient, cmd: cli::zone::ZoneCommand) -> Result<()> {
    use cli::zone::ZoneCommand;
    use crate::api::zone::ZoneSettings;

    match cmd {
        ZoneCommand::List { status } => {
            let zones = ops::zone::list_zones(client, status.as_deref()).await?;
            println!("\nZones:\n");
            output::table::print_zones(&zones);
        }
        ZoneCommand::Show { zone } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            println!("Zone: {}", zone_obj.name);
            println!("  ID: {}", zone_obj.id);
            println!("  Status: {}", zone_obj.status);
            println!("  Name Servers: {:?}", zone_obj.name_servers);
        }
        ZoneCommand::Create { zone, account_id } => {
            let account_id = account_id.ok_or_else(|| {
                crate::error::CfadError::validation("Account ID required for zone creation")
            })?;
            ops::zone::create_zone(client, &zone, &account_id).await?;
        }
        ZoneCommand::Delete { zone_id, confirm } => {
            if !confirm {
                println!("⚠ Zone deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::zone::delete_zone(client, &zone_id).await?;
        }
        ZoneCommand::Settings { zone } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            println!("Settings for zone: {}", zone_obj.name);
            println!("  Development mode: {}", zone_obj.development_mode);
        }
        ZoneCommand::Update {
            zone,
            security_level,
            cache_level,
            dev_mode,
            ipv6,
            ssl,
            always_https,
        } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;

            let settings = ZoneSettings {
                security_level,
                cache_level,
                development_mode: dev_mode.map(|v| v == "on"),
                ipv6: ipv6.map(|v| v == "on"),
                ssl,
                always_use_https: always_https.map(|v| v == "on"),
                minify: None,
            };

            ops::zone::update_zone_settings(client, &zone_obj.id, settings).await?;
        }
    }

    Ok(())
}

async fn handle_cache_command(client: &client::CloudflareClient, cmd: cli::cache::CacheCommand) -> Result<()> {
    use cli::cache::CacheCommand;

    match cmd {
        CacheCommand::Purge { zone, all, files, tags, hosts, prefixes } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;

            if all {
                ops::cache::purge_all(client, &zone_obj.id).await?;
            } else if let Some(files) = files {
                ops::cache::purge_files(client, &zone_obj.id, files).await?;
            } else if let Some(tags) = tags {
                ops::cache::purge_tags(client, &zone_obj.id, tags).await?;
            } else if let Some(hosts) = hosts {
                ops::cache::purge_hosts(client, &zone_obj.id, hosts).await?;
            } else if let Some(prefixes) = prefixes {
                ops::cache::purge_prefixes(client, &zone_obj.id, prefixes).await?;
            } else {
                return Err(crate::error::CfadError::validation(
                    "Must specify purge type: --all, --files, --tags, --hosts, or --prefixes"
                ));
            }
        }
    }

    Ok(())
}

async fn handle_config_command(cmd: cli::config::ConfigCommand) -> Result<()> {
    use cli::config::{ConfigCommand, ProfileCommand};

    match cmd {
        ConfigCommand::Init => {
            let config = Config::new("default".to_string());
            config.save()?;
            println!("✓ Configuration initialized at {:?}", Config::config_path()?);
        }
        ConfigCommand::Show { profile } => {
            let loaded = Config::load(profile.as_deref())?;
            let redacted = loaded.redacted();
            println!("Profile configuration:");
            println!("  API Token: {:?}", redacted.api_token);
            println!("  API Key: {:?}", redacted.api_key);
            println!("  API Email: {:?}", redacted.api_email);
            println!("  Default Zone: {:?}", redacted.default_zone);
            println!("  Output Format: {:?}", redacted.output_format);
        }
        ConfigCommand::Profiles(ProfileCommand::List) => {
            match Config::from_file() {
                Ok(config) => {
                    println!("Available profiles:");
                    for name in config.profiles.keys() {
                        if name == &config.default_profile {
                            println!("  {} (default)", name);
                        } else {
                            println!("  {}", name);
                        }
                    }
                }
                Err(_) => {
                    println!("No configuration file found. Run 'cfad config init' first.");
                }
            }
        }
        ConfigCommand::Profiles(ProfileCommand::Add { name }) => {
            let mut config = Config::from_file().unwrap_or_else(|_| Config::new("default".to_string()));
            let profile = Profile {
                api_token: None,
                api_key: None,
                api_email: None,
                default_zone: None,
                output_format: None,
            };
            config.profiles.insert(name.clone(), profile);
            config.save()?;
            println!("✓ Profile '{}' added", name);
        }
        ConfigCommand::Profiles(ProfileCommand::SetDefault { name }) => {
            let mut config = Config::from_file()?;
            if !config.profiles.contains_key(&name) {
                return Err(error::CfadError::config(format!("Profile '{}' not found", name)));
            }
            config.default_profile = name.clone();
            config.save()?;
            println!("✓ Default profile set to '{}'", name);
        }
    }

    Ok(())
}
