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
mod metrics;
mod ops;

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
    cli::setup_logging(cli.verbose, cli.quiet);

    // Handle config commands first (they don't need API auth)
    if let Commands::Config(cmd) = cli.command {
        return handle_config_command(cmd).await;
    }

    // Load configuration and apply CLI overrides
    let mut profile = Config::load(cli.profile.as_deref())?;
    apply_cli_overrides(&mut profile, &cli);

    // Get auth method and create client
    let auth = profile.auth_method()?;
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

fn apply_cli_overrides(profile: &mut Profile, cli: &Cli) {
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
}

async fn handle_dns_command(
    client: &client::CloudflareClient,
    cmd: cli::dns::DnsCommand,
) -> Result<()> {
    use cli::dns::DnsCommand;

    match cmd {
        DnsCommand::List { zone, r#type, name } => {
            return handle_dns_list(client, &zone, r#type, name).await;
        }
        DnsCommand::Show { zone, record_id } => {
            return handle_dns_show(client, &zone, &record_id).await;
        }
        DnsCommand::Add {
            zone,
            r#type,
            name,
            content,
            ttl,
            proxied,
            priority,
        } => {
            return handle_dns_add(client, &zone, r#type, name, content, ttl, proxied, priority)
                .await;
        }
        DnsCommand::Update {
            zone,
            record_id,
            name,
            content,
            ttl,
            proxied,
            priority,
        } => {
            return handle_dns_update(
                client, &zone, &record_id, name, content, ttl, proxied, priority,
            )
            .await;
        }
        DnsCommand::Delete {
            zone,
            record_id,
            confirm,
        } => {
            return handle_dns_delete(client, &zone, &record_id, confirm).await;
        }
        DnsCommand::Import { zone, file } => {
            return handle_dns_import(client, &zone, &file).await;
        }
    }
}

async fn handle_dns_list(
    client: &client::CloudflareClient,
    zone: &str,
    record_type: Option<String>,
    name: Option<String>,
) -> Result<()> {
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    let records = ops::dns::list_records(
        client,
        &zone_obj.id,
        record_type.as_deref(),
        name.as_deref(),
    )
    .await?;
    println!("\nDNS Records for {}:\n", zone);
    output::table::print_dns_records(&records);
    Ok(())
}

async fn handle_dns_show(
    client: &client::CloudflareClient,
    zone: &str,
    record_id: &str,
) -> Result<()> {
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    let record = ops::dns::get_record(client, &zone_obj.id, record_id).await?;
    output::table::print_dns_record(&record);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn handle_dns_add(
    client: &client::CloudflareClient,
    zone: &str,
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
    priority: Option<u16>,
) -> Result<()> {
    use crate::api::dns::CreateDnsRecord;

    let zone_obj = ops::zone::get_zone(client, zone).await?;
    let record = CreateDnsRecord {
        record_type,
        name,
        content,
        ttl: Some(ttl),
        proxied: Some(proxied),
        priority,
        data: None,
    };
    ops::dns::create_record(client, &zone_obj.id, record).await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn handle_dns_update(
    client: &client::CloudflareClient,
    zone: &str,
    record_id: &str,
    name: Option<String>,
    content: Option<String>,
    ttl: Option<u32>,
    proxied: Option<bool>,
    priority: Option<u16>,
) -> Result<()> {
    use crate::api::dns::UpdateDnsRecord;

    let zone_obj = ops::zone::get_zone(client, zone).await?;
    let update = UpdateDnsRecord {
        record_type: None,
        name,
        content,
        ttl,
        proxied,
        priority,
    };
    ops::dns::update_record(client, &zone_obj.id, record_id, update).await?;
    Ok(())
}

async fn handle_dns_delete(
    client: &client::CloudflareClient,
    zone: &str,
    record_id: &str,
    confirm: bool,
) -> Result<()> {
    if !confirm {
        println!("⚠ Deletion requires --confirm flag");
        return Err(crate::error::CfadError::validation("Confirmation required"));
    }
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    ops::dns::delete_record(client, &zone_obj.id, record_id).await
}

async fn handle_dns_import(
    client: &client::CloudflareClient,
    zone: &str,
    file: &str,
) -> Result<()> {
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    ops::dns::import_records(client, &zone_obj.id, file).await?;
    Ok(())
}

async fn handle_zone_command(
    client: &client::CloudflareClient,
    cmd: cli::zone::ZoneCommand,
) -> Result<()> {
    use cli::zone::ZoneCommand;

    match cmd {
        ZoneCommand::List { status } => return handle_zone_list(client, status).await,
        ZoneCommand::Show { zone } => return handle_zone_show(client, &zone).await,
        ZoneCommand::Create { zone, account_id } => {
            return handle_zone_create(client, &zone, account_id).await
        }
        ZoneCommand::Delete { zone_id, confirm } => {
            return handle_zone_delete(client, &zone_id, confirm).await
        }
        ZoneCommand::Settings { zone } => return handle_zone_settings(client, &zone).await,
        ZoneCommand::Update {
            zone,
            security_level,
            cache_level,
            dev_mode,
            ipv6,
            ssl,
            always_https,
        } => {
            return handle_zone_update(
                client,
                &zone,
                security_level,
                cache_level,
                dev_mode,
                ipv6,
                ssl,
                always_https,
            )
            .await;
        }
    }
}

async fn handle_zone_list(client: &client::CloudflareClient, status: Option<String>) -> Result<()> {
    let zones = ops::zone::list_zones(client, status.as_deref()).await?;
    println!("\nZones:\n");
    output::table::print_zones(&zones);
    Ok(())
}

async fn handle_zone_show(client: &client::CloudflareClient, zone: &str) -> Result<()> {
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    println!("Zone: {}", zone_obj.name);
    println!("  ID: {}", zone_obj.id);
    println!("  Status: {}", zone_obj.status);
    println!("  Name Servers: {:?}", zone_obj.name_servers);
    Ok(())
}

async fn handle_zone_create(
    client: &client::CloudflareClient,
    zone: &str,
    account_id: Option<String>,
) -> Result<()> {
    let account_id = account_id.ok_or_else(|| {
        crate::error::CfadError::validation("Account ID required for zone creation")
    })?;
    ops::zone::create_zone(client, zone, &account_id).await?;
    Ok(())
}

async fn handle_zone_delete(
    client: &client::CloudflareClient,
    zone_id: &str,
    confirm: bool,
) -> Result<()> {
    if !confirm {
        println!("⚠ Zone deletion requires --confirm flag");
        return Err(crate::error::CfadError::validation("Confirmation required"));
    }
    ops::zone::delete_zone(client, zone_id).await
}

async fn handle_zone_settings(client: &client::CloudflareClient, zone: &str) -> Result<()> {
    let zone_obj = ops::zone::get_zone(client, zone).await?;
    println!("Settings for zone: {}", zone_obj.name);
    println!("  Development mode: {}", zone_obj.development_mode);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn handle_zone_update(
    client: &client::CloudflareClient,
    zone: &str,
    security_level: Option<String>,
    cache_level: Option<String>,
    dev_mode: Option<String>,
    ipv6: Option<String>,
    ssl: Option<String>,
    always_https: Option<String>,
) -> Result<()> {
    use crate::api::zone::ZoneSettings;

    let zone_obj = ops::zone::get_zone(client, zone).await?;

    let settings = ZoneSettings {
        security_level,
        cache_level,
        development_mode: dev_mode.map(|v| v == "on"),
        ipv6: ipv6.map(|v| v == "on"),
        ssl,
        always_use_https: always_https.map(|v| v == "on"),
        minify: None,
    };

    ops::zone::update_zone_settings(client, &zone_obj.id, settings).await
}

async fn handle_cache_command(
    client: &client::CloudflareClient,
    cmd: cli::cache::CacheCommand,
) -> Result<()> {
    use cli::cache::CacheCommand;

    match cmd {
        CacheCommand::Purge {
            zone,
            all,
            files,
            tags,
            hosts,
            prefixes,
        } => {
            let zone_obj = ops::zone::get_zone(client, &zone).await?;
            execute_cache_purge(client, &zone_obj.id, all, files, tags, hosts, prefixes).await?;
        }
    }

    Ok(())
}

async fn execute_cache_purge(
    client: &client::CloudflareClient,
    zone_id: &str,
    all: bool,
    files: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    hosts: Option<Vec<String>>,
    prefixes: Option<Vec<String>>,
) -> Result<()> {
    if all {
        return ops::cache::purge_all(client, zone_id).await;
    }

    if let Some(files) = files {
        return ops::cache::purge_files(client, zone_id, files).await;
    }

    if let Some(tags) = tags {
        return ops::cache::purge_tags(client, zone_id, tags).await;
    }

    if let Some(hosts) = hosts {
        return ops::cache::purge_hosts(client, zone_id, hosts).await;
    }

    if let Some(prefixes) = prefixes {
        return ops::cache::purge_prefixes(client, zone_id, prefixes).await;
    }

    Err(crate::error::CfadError::validation(
        "Must specify purge type: --all, --files, --tags, --hosts, or --prefixes",
    ))
}

async fn handle_config_command(cmd: cli::config::ConfigCommand) -> Result<()> {
    use cli::config::{ConfigCommand, ProfileCommand};

    match cmd {
        ConfigCommand::Init => handle_config_init().await,
        ConfigCommand::Show { profile } => handle_config_show(profile).await,
        ConfigCommand::Profiles(ProfileCommand::List) => handle_profile_list().await,
        ConfigCommand::Profiles(ProfileCommand::Add { name }) => handle_profile_add(name).await,
        ConfigCommand::Profiles(ProfileCommand::SetDefault { name }) => {
            handle_profile_set_default(name).await
        }
    }
}

async fn handle_config_init() -> Result<()> {
    let config = Config::new("default".to_string());
    config.save()?;
    println!(
        "✓ Configuration initialized at {:?}",
        Config::config_path()?
    );
    Ok(())
}

async fn handle_config_show(profile: Option<String>) -> Result<()> {
    let loaded = Config::load(profile.as_deref())?;
    let redacted = loaded.redacted();
    println!("Profile configuration:");
    println!("  API Token: {:?}", redacted.api_token);
    println!("  API Key: {:?}", redacted.api_key);
    println!("  API Email: {:?}", redacted.api_email);
    println!("  Default Zone: {:?}", redacted.default_zone);
    println!("  Output Format: {:?}", redacted.output_format);
    Ok(())
}

async fn handle_profile_list() -> Result<()> {
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
    Ok(())
}

async fn handle_profile_add(name: String) -> Result<()> {
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
    Ok(())
}

async fn handle_profile_set_default(name: String) -> Result<()> {
    let mut config = Config::from_file()?;
    if !config.profiles.contains_key(&name) {
        return Err(error::CfadError::config(format!(
            "Profile '{}' not found",
            name
        )));
    }
    config.default_profile = name.clone();
    config.save()?;
    println!("✓ Default profile set to '{}'", name);
    Ok(())
}
