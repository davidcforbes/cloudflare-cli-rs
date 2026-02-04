#![allow(dead_code)]
#![allow(unused_imports)]

use clap::Parser;
use log::error;
use std::process;

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
use config::{resolve_account_id, Config, Profile};
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
        Commands::D1(cmd) => handle_d1_command(&client, cmd).await?,
        Commands::R2(cmd) => handle_r2_command(&client, cmd).await?,
        Commands::Token(cmd) => handle_token_command(&client, cmd).await?,
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

async fn handle_d1_command(
    client: &client::CloudflareClient,
    cmd: cli::d1::D1Command,
) -> Result<()> {
    use cli::d1::D1Command;

    match cmd {
        D1Command::List { account_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let databases = ops::d1::list_databases(client, &account_id).await?;
            println!("\nD1 Databases:\n");
            output::table::print_d1_databases(&databases);
            Ok(())
        }
        D1Command::Show {
            account_id,
            database_id,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let db = ops::d1::get_database(client, &account_id, &database_id).await?;
            output::table::print_d1_database(&db);
            Ok(())
        }
        D1Command::Create {
            account_id,
            name,
            location,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let db = api::d1::CreateD1Database {
                name,
                primary_location_hint: location,
            };
            ops::d1::create_database(client, &account_id, db).await?;
            Ok(())
        }
        D1Command::Update {
            account_id,
            database_id,
            name,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let update = api::d1::UpdateD1Database { name };
            ops::d1::update_database(client, &account_id, &database_id, update).await?;
            Ok(())
        }
        D1Command::Delete {
            account_id,
            database_id,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::d1::delete_database(client, &account_id, &database_id).await
        }
        D1Command::Query {
            account_id,
            database_id,
            sql,
            raw,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if raw {
                let results =
                    ops::d1::query_database_raw(client, &account_id, &database_id, &sql, None)
                        .await?;
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                let results =
                    ops::d1::query_database(client, &account_id, &database_id, &sql, None).await?;
                println!("{}", serde_json::to_string_pretty(&results)?);
            }
            Ok(())
        }
        D1Command::QueryFile {
            account_id,
            database_id,
            file,
            raw,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let sql = std::fs::read_to_string(&file)?;
            if raw {
                let results =
                    ops::d1::query_database_raw(client, &account_id, &database_id, &sql, None)
                        .await?;
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                let results =
                    ops::d1::query_database(client, &account_id, &database_id, &sql, None).await?;
                println!("{}", serde_json::to_string_pretty(&results)?);
            }
            Ok(())
        }
        D1Command::Export {
            account_id,
            database_id,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let result = ops::d1::export_database(client, &account_id, &database_id).await?;
            println!("\nExport initiated:");
            println!("  Task ID: {}", result.task_id);
            println!("  Status: {}", result.status);
            if let Some(url) = result.signed_url {
                println!("  Download URL: {}", url);
            }
            Ok(())
        }
        D1Command::Import {
            account_id,
            database_id,
            file,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let sql = std::fs::read_to_string(&file)?;
            ops::d1::import_database(client, &account_id, &database_id, &sql).await?;
            Ok(())
        }
        D1Command::Bookmark {
            account_id,
            database_id,
            timestamp,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let bookmark =
                ops::d1::get_bookmark(client, &account_id, &database_id, timestamp.as_deref())
                    .await?;
            println!("\nBookmark:");
            println!("  ID: {}", bookmark.bookmark);
            println!("  Timestamp: {}", bookmark.timestamp);
            Ok(())
        }
        D1Command::Restore {
            account_id,
            database_id,
            bookmark,
            timestamp,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Restore requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::d1::restore_database(
                client,
                &account_id,
                &database_id,
                bookmark.as_deref(),
                timestamp.as_deref(),
            )
            .await?;
            Ok(())
        }
    }
}

async fn handle_r2_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2Command,
) -> Result<()> {
    use cli::r2::R2Command;

    match cmd {
        R2Command::List { account_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let buckets = ops::r2::list_buckets(client, &account_id).await?;
            println!("\nR2 Buckets:\n");
            output::table::print_r2_buckets(&buckets);
            Ok(())
        }
        R2Command::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let bucket_info = ops::r2::get_bucket(client, &account_id, &bucket).await?;
            output::table::print_r2_bucket(&bucket_info);
            Ok(())
        }
        R2Command::Create {
            account_id,
            name,
            location,
            storage_class,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let bucket = api::r2::CreateR2Bucket {
                name,
                location_hint: location,
                storage_class,
            };
            ops::r2::create_bucket(client, &account_id, bucket).await?;
            Ok(())
        }
        R2Command::Delete {
            account_id,
            bucket,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::delete_bucket(client, &account_id, &bucket).await
        }
        R2Command::Cors(cmd) => handle_r2_cors_command(client, cmd).await,
        R2Command::Domain(cmd) => handle_r2_domain_command(client, cmd).await,
        R2Command::PublicAccess(cmd) => handle_r2_public_access_command(client, cmd).await,
        R2Command::Lifecycle(cmd) => handle_r2_lifecycle_command(client, cmd).await,
        R2Command::Lock(cmd) => handle_r2_lock_command(client, cmd).await,
        R2Command::Metrics { account_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let metrics = ops::r2::get_metrics(client, &account_id).await?;
            output::table::print_r2_metrics(&metrics);
            Ok(())
        }
        R2Command::Sippy(cmd) => handle_r2_sippy_command(client, cmd).await,
        R2Command::Notifications(cmd) => handle_r2_notification_command(client, cmd).await,
        R2Command::Migrate(cmd) => handle_r2_migrate_command(client, cmd).await,
        R2Command::TempCreds(cmd) => handle_r2_temp_creds_command(client, cmd).await,
    }
}

async fn handle_r2_cors_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2CorsCommand,
) -> Result<()> {
    use cli::r2::R2CorsCommand;

    match cmd {
        R2CorsCommand::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = ops::r2::get_cors(client, &account_id, &bucket).await?;
            println!("{}", serde_json::to_string_pretty(&config)?);
            Ok(())
        }
        R2CorsCommand::Set {
            account_id,
            bucket,
            file,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let contents = std::fs::read_to_string(&file)?;
            let rules: Vec<api::r2::R2CorsRule> = serde_json::from_str(&contents)?;
            ops::r2::set_cors(client, &account_id, &bucket, rules).await
        }
        R2CorsCommand::Delete {
            account_id,
            bucket,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::delete_cors(client, &account_id, &bucket).await
        }
    }
}

async fn handle_r2_domain_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2DomainCommand,
) -> Result<()> {
    use cli::r2::R2DomainCommand;

    match cmd {
        R2DomainCommand::List { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let domains = ops::r2::list_custom_domains(client, &account_id, &bucket).await?;
            output::table::print_r2_custom_domains(&domains);
            Ok(())
        }
        R2DomainCommand::Show {
            account_id,
            bucket,
            domain,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let domain_info =
                ops::r2::get_custom_domain(client, &account_id, &bucket, &domain).await?;
            println!("\nCustom Domain Details:");
            println!("  Domain: {}", domain_info.domain);
            println!("  Enabled: {}", domain_info.enabled);
            println!("  Status: {}", domain_info.status);
            if let Some(min_tls) = &domain_info.min_tls {
                println!("  Min TLS: {}", min_tls);
            }
            Ok(())
        }
        R2DomainCommand::Add {
            account_id,
            bucket,
            domain,
            zone_id,
            min_tls,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let create = api::r2::CreateR2CustomDomain {
                domain,
                zone_id,
                min_tls,
            };
            ops::r2::create_custom_domain(client, &account_id, &bucket, create).await?;
            Ok(())
        }
        R2DomainCommand::Update {
            account_id,
            bucket,
            domain,
            enabled,
            min_tls,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let update = api::r2::UpdateR2CustomDomain { enabled, min_tls };
            ops::r2::update_custom_domain(client, &account_id, &bucket, &domain, update).await?;
            Ok(())
        }
        R2DomainCommand::Delete {
            account_id,
            bucket,
            domain,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::delete_custom_domain(client, &account_id, &bucket, &domain).await
        }
    }
}

async fn handle_r2_public_access_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2PublicAccessCommand,
) -> Result<()> {
    use cli::r2::R2PublicAccessCommand;

    match cmd {
        R2PublicAccessCommand::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = ops::r2::get_managed_domain(client, &account_id, &bucket).await?;
            println!("\nPublic Access (r2.dev):");
            println!("  Enabled: {}", config.enabled);
            if let Some(domain) = &config.domain {
                println!("  URL: https://{}", domain);
            }
            Ok(())
        }
        R2PublicAccessCommand::Enable { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let result = ops::r2::update_managed_domain(client, &account_id, &bucket, true).await?;
            if let Some(domain) = &result.domain {
                println!("  Public URL: https://{}", domain);
            }
            Ok(())
        }
        R2PublicAccessCommand::Disable { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            ops::r2::update_managed_domain(client, &account_id, &bucket, false).await?;
            Ok(())
        }
    }
}

async fn handle_r2_lifecycle_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2LifecycleCommand,
) -> Result<()> {
    use cli::r2::R2LifecycleCommand;

    match cmd {
        R2LifecycleCommand::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = ops::r2::get_lifecycle(client, &account_id, &bucket).await?;
            println!("{}", serde_json::to_string_pretty(&config)?);
            Ok(())
        }
        R2LifecycleCommand::Set {
            account_id,
            bucket,
            file,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let contents = std::fs::read_to_string(&file)?;
            let config: api::r2::R2LifecycleConfig = serde_json::from_str(&contents)?;
            ops::r2::set_lifecycle(client, &account_id, &bucket, config).await
        }
    }
}

async fn handle_r2_lock_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2LockCommand,
) -> Result<()> {
    use cli::r2::R2LockCommand;

    match cmd {
        R2LockCommand::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = ops::r2::get_lock(client, &account_id, &bucket).await?;
            println!("\nBucket Lock Configuration:");
            println!("  Enabled: {}", config.enabled);
            if let Some(mode) = &config.mode {
                println!("  Mode: {}", mode);
            }
            if let Some(days) = config.default_retention_days {
                println!("  Default Retention: {} days", days);
            }
            Ok(())
        }
        R2LockCommand::Enable {
            account_id,
            bucket,
            mode,
            days,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = api::r2::UpdateR2LockConfig {
                enabled: true,
                mode: Some(mode),
                default_retention_days: days,
            };
            ops::r2::set_lock(client, &account_id, &bucket, config).await
        }
        R2LockCommand::Disable {
            account_id,
            bucket,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Disabling lock requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            let config = api::r2::UpdateR2LockConfig {
                enabled: false,
                mode: None,
                default_retention_days: None,
            };
            ops::r2::set_lock(client, &account_id, &bucket, config).await
        }
    }
}

async fn handle_r2_sippy_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2SippyCommand,
) -> Result<()> {
    use cli::r2::R2SippyCommand;

    match cmd {
        R2SippyCommand::Show { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = ops::r2::get_sippy(client, &account_id, &bucket).await?;
            println!("\nSippy Configuration:");
            println!("  Enabled: {}", config.enabled);
            if let Some(provider) = &config.provider {
                println!("  Provider: {}", provider);
            }
            if let Some(source_bucket) = &config.bucket {
                println!("  Source Bucket: {}", source_bucket);
            }
            if let Some(region) = &config.region {
                println!("  Region: {}", region);
            }
            Ok(())
        }
        R2SippyCommand::Enable {
            account_id,
            bucket,
            provider,
            source_bucket,
            region,
            access_key_id,
            secret_access_key,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let config = api::r2::CreateR2SippyConfig {
                provider,
                bucket: source_bucket,
                region,
                access_key_id,
                secret_access_key,
            };
            ops::r2::set_sippy(client, &account_id, &bucket, config).await?;
            Ok(())
        }
        R2SippyCommand::Disable {
            account_id,
            bucket,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Disabling Sippy requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::delete_sippy(client, &account_id, &bucket).await
        }
    }
}

async fn handle_r2_notification_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2NotificationCommand,
) -> Result<()> {
    use cli::r2::R2NotificationCommand;

    match cmd {
        R2NotificationCommand::List { account_id, bucket } => {
            let account_id = resolve_account_id(account_id, None)?;
            let notifications = ops::r2::list_notifications(client, &account_id, &bucket).await?;
            output::table::print_r2_notifications(&notifications);
            Ok(())
        }
        R2NotificationCommand::Show {
            account_id,
            bucket,
            queue_id,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let notification =
                ops::r2::get_notification(client, &account_id, &bucket, &queue_id).await?;
            println!("\nEvent Notification:");
            println!("  Queue ID: {}", notification.queue_id);
            println!("  Events: {:?}", notification.events);
            if let Some(prefix) = &notification.prefix {
                println!("  Prefix: {}", prefix);
            }
            if let Some(suffix) = &notification.suffix {
                println!("  Suffix: {}", suffix);
            }
            Ok(())
        }
        R2NotificationCommand::Create {
            account_id,
            bucket,
            queue_id,
            events,
            prefix,
            suffix,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let notification = api::r2::CreateR2EventNotification {
                events,
                prefix,
                suffix,
            };
            ops::r2::create_notification(client, &account_id, &bucket, &queue_id, notification)
                .await?;
            Ok(())
        }
        R2NotificationCommand::Delete {
            account_id,
            bucket,
            queue_id,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::delete_notification(client, &account_id, &bucket, &queue_id).await
        }
    }
}

async fn handle_r2_migrate_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2MigrateCommand,
) -> Result<()> {
    use cli::r2::R2MigrateCommand;

    match cmd {
        R2MigrateCommand::List { account_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let jobs = ops::r2::list_migration_jobs(client, &account_id).await?;
            output::table::print_r2_migration_jobs(&jobs);
            Ok(())
        }
        R2MigrateCommand::Show { account_id, job_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let job = ops::r2::get_migration_job(client, &account_id, &job_id).await?;
            println!("\nMigration Job Details:");
            println!("  ID: {}", job.id);
            println!("  Status: {}", job.status);
            println!("  Source: {} ({})", job.source_bucket, job.source_provider);
            println!("  Target: {}", job.target_bucket);
            println!("  Created: {}", job.created_at);
            if let Some(completed) = &job.completed_at {
                println!("  Completed: {}", completed);
            }
            Ok(())
        }
        R2MigrateCommand::Create {
            account_id,
            source_provider,
            source_bucket,
            source_region,
            target_bucket,
            access_key_id,
            secret_access_key,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let job = api::r2::CreateR2MigrationJob {
                source_provider,
                source_bucket,
                source_region,
                target_bucket,
                access_key_id,
                secret_access_key,
            };
            ops::r2::create_migration_job(client, &account_id, job).await?;
            Ok(())
        }
        R2MigrateCommand::Pause { account_id, job_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            ops::r2::pause_migration_job(client, &account_id, &job_id).await
        }
        R2MigrateCommand::Resume { account_id, job_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            ops::r2::resume_migration_job(client, &account_id, &job_id).await
        }
        R2MigrateCommand::Abort {
            account_id,
            job_id,
            confirm,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            if !confirm {
                println!("⚠ Abort requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::r2::abort_migration_job(client, &account_id, &job_id).await
        }
        R2MigrateCommand::Progress { account_id, job_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let progress = ops::r2::get_migration_progress(client, &account_id, &job_id).await?;
            println!("\nMigration Progress:");
            println!(
                "  Objects: {}/{}",
                progress.objects_migrated, progress.objects_total
            );
            println!(
                "  Bytes: {}/{}",
                progress.bytes_migrated, progress.bytes_total
            );
            println!("  Errors: {}", progress.errors);
            if progress.objects_total > 0 {
                let pct =
                    (progress.objects_migrated as f64 / progress.objects_total as f64) * 100.0;
                println!("  Progress: {:.1}%", pct);
            }
            Ok(())
        }
        R2MigrateCommand::Logs { account_id, job_id } => {
            let account_id = resolve_account_id(account_id, None)?;
            let logs = ops::r2::get_migration_logs(client, &account_id, &job_id).await?;
            for log in logs {
                println!("{}", log);
            }
            Ok(())
        }
    }
}

async fn handle_r2_temp_creds_command(
    client: &client::CloudflareClient,
    cmd: cli::r2::R2TempCredsCommand,
) -> Result<()> {
    use cli::r2::R2TempCredsCommand;

    match cmd {
        R2TempCredsCommand::Create {
            account_id,
            bucket,
            prefix,
            permission,
            ttl,
        } => {
            let account_id = resolve_account_id(account_id, None)?;
            let request = api::r2::CreateR2TempCredentials {
                bucket,
                prefix,
                permission,
                ttl_seconds: ttl,
            };
            let creds = ops::r2::create_temp_credentials(client, &account_id, request).await?;
            println!("\nTemporary Credentials:");
            println!("  Access Key ID: {}", creds.access_key_id);
            println!("  Secret Access Key: {}", creds.secret_access_key);
            println!("  Session Token: {}", creds.session_token);
            println!("  Expiration: {}", creds.expiration);
            Ok(())
        }
    }
}

async fn handle_token_command(
    client: &client::CloudflareClient,
    cmd: cli::token::TokenCommand,
) -> Result<()> {
    use cli::token::TokenCommand;

    match cmd {
        TokenCommand::List => {
            let tokens = ops::token::list_tokens(client).await?;
            println!("\nAPI Tokens:\n");
            output::table::print_tokens(&tokens);
            Ok(())
        }
        TokenCommand::Show { token_id } => {
            let token = ops::token::get_token(client, &token_id).await?;
            output::table::print_token(&token);
            Ok(())
        }
        TokenCommand::Create {
            name,
            permissions,
            resources,
            expires,
            not_before,
        } => {
            use crate::api::token::{CreatePermissionGroupRef, CreateToken, CreateTokenPolicy};

            let policy = CreateTokenPolicy {
                effect: "allow".to_string(),
                resources: serde_json::json!({ &resources: "*" }),
                permission_groups: permissions
                    .into_iter()
                    .map(|id| CreatePermissionGroupRef { id })
                    .collect(),
            };

            let create = CreateToken {
                name,
                policies: vec![policy],
                not_before,
                expires_on: expires,
                condition: None,
            };

            let result = ops::token::create_token(client, create).await?;
            println!("\n✓ Token created successfully!\n");
            println!("  ID: {}", result.id);
            println!("  Name: {}", result.name);
            println!("  Status: {}", result.status);
            println!("\n  TOKEN VALUE (save this - it won't be shown again):");
            println!("  {}", result.value);
            Ok(())
        }
        TokenCommand::Update {
            token_id,
            name,
            status,
            expires,
        } => {
            use crate::api::token::UpdateToken;

            let update = UpdateToken {
                name,
                status,
                policies: None,
                not_before: None,
                expires_on: expires,
                condition: None,
            };

            let token = ops::token::update_token(client, &token_id, update).await?;
            println!("✓ Token updated: {}", token.name);
            Ok(())
        }
        TokenCommand::Delete { token_id, confirm } => {
            if !confirm {
                println!("⚠ Deletion requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            ops::token::delete_token(client, &token_id).await
        }
        TokenCommand::Verify => {
            let verification = ops::token::verify_token(client).await?;
            println!("\nToken Verification:\n");
            println!("  ID: {}", verification.id);
            println!("  Status: {}", verification.status);
            if let Some(not_before) = &verification.not_before {
                println!("  Not Before: {}", not_before);
            }
            if let Some(expires) = &verification.expires_on {
                println!("  Expires: {}", expires);
            }
            Ok(())
        }
        TokenCommand::Permissions { scope } => {
            let groups = ops::token::list_permission_groups(client).await?;
            println!("\nAvailable Permission Groups:\n");
            output::table::print_permission_groups(&groups, scope.as_deref());
            Ok(())
        }
        TokenCommand::Roll { token_id, confirm } => {
            if !confirm {
                println!("⚠ Rolling a token requires --confirm flag");
                return Err(crate::error::CfadError::validation("Confirmation required"));
            }
            let result = ops::token::roll_token(client, &token_id).await?;
            println!("\n✓ Token rolled successfully!\n");
            println!("  ID: {}", result.id);
            println!("\n  NEW TOKEN VALUE (save this - it won't be shown again):");
            println!("  {}", result.value);
            Ok(())
        }
    }
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
        account_id: None,
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
