use crate::api::d1::{D1Database, D1QueryResult, D1RawQueryResult};
use crate::api::dns::DnsRecord;
use crate::api::r2::{R2Bucket, R2CustomDomain, R2EventNotification, R2Metrics, R2MigrationJob};
use crate::api::token::{PermissionGroup, Token};
use crate::api::zone::Zone;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};

pub fn print_dns_records(records: &[DnsRecord]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Type")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Content")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("TTL")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Proxied")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for record in records {
        table.add_row(vec![
            Cell::new(&record.record_type),
            Cell::new(&record.name),
            Cell::new(&record.content),
            Cell::new(if record.ttl == 1 {
                "Auto".to_string()
            } else {
                record.ttl.to_string()
            }),
            Cell::new(if record.proxied { "✓" } else { "✗" }),
            Cell::new(&record.id),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} records", records.len());
}

pub fn print_dns_record(record: &DnsRecord) {
    println!("\nDNS Record Details:\n");
    println!("  ID: {}", record.id);
    println!("  Type: {}", record.record_type);
    println!("  Name: {}", record.name);
    println!("  Content: {}", record.content);
    println!(
        "  TTL: {}",
        if record.ttl == 1 {
            "Auto".to_string()
        } else {
            record.ttl.to_string()
        }
    );
    println!("  Proxied: {}", if record.proxied { "✓" } else { "✗" });
    if let Some(priority) = record.priority {
        println!("  Priority: {}", priority);
    }
    if let Some(created) = &record.created_on {
        println!("  Created: {}", created);
    }
    if let Some(modified) = &record.modified_on {
        println!("  Modified: {}", modified);
    }
}

pub fn print_zones(zones: &[Zone]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for zone in zones {
        let status_cell = Cell::new(&zone.status);
        let status_cell = if zone.status == "active" {
            status_cell.fg(Color::Green)
        } else {
            status_cell.fg(Color::Yellow)
        };

        table.add_row(vec![
            Cell::new(&zone.name),
            status_cell,
            Cell::new(&zone.id),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} zones", zones.len());
}

pub fn print_d1_databases(databases: &[D1Database]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Tables")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Size")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for db in databases {
        table.add_row(vec![
            Cell::new(&db.name),
            Cell::new(db.num_tables.to_string()),
            Cell::new(format_bytes(db.file_size)),
            Cell::new(&db.uuid),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} databases", databases.len());
}

pub fn print_d1_database(db: &D1Database) {
    println!("\nD1 Database Details:\n");
    println!("  ID: {}", db.uuid);
    println!("  Name: {}", db.name);
    println!("  Version: {}", db.version);
    println!("  Tables: {}", db.num_tables);
    println!("  Size: {}", format_bytes(db.file_size));
    println!("  Created: {}", db.created_at);
}

/// Print D1 query results (object format) as a table
pub fn print_d1_query_results(results: &[D1QueryResult]) {
    for (i, result) in results.iter().enumerate() {
        if results.len() > 1 {
            println!("\n--- Result Set {} ---", i + 1);
        }

        if result.results.is_empty() {
            println!("\nNo rows returned.");
            continue;
        }

        // Extract column names from first row
        let first_row = &result.results[0];
        let columns: Vec<String> = if let Some(obj) = first_row.as_object() {
            obj.keys().cloned().collect()
        } else {
            vec!["value".to_string()]
        };

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Set header
        let header: Vec<Cell> = columns
            .iter()
            .map(|col| {
                Cell::new(col)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan)
            })
            .collect();
        table.set_header(header);

        // Add rows
        for row in &result.results {
            let cells: Vec<Cell> = columns
                .iter()
                .map(|col| {
                    let value = row.get(col).unwrap_or(&serde_json::Value::Null);
                    Cell::new(format_json_value(value))
                })
                .collect();
            table.add_row(cells);
        }

        println!("{}", table);
        println!(
            "\n{} row(s) returned in {:.3}s",
            result.results.len(),
            result.meta.duration
        );
    }
}

/// Print D1 raw query results (array format) as a table
pub fn print_d1_raw_query_results(results: &[D1RawQueryResult]) {
    for (i, result) in results.iter().enumerate() {
        if results.len() > 1 {
            println!("\n--- Result Set {} ---", i + 1);
        }

        if result.rows.is_empty() {
            println!("\nNo rows returned.");
            continue;
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Set header from columns
        let header: Vec<Cell> = result
            .columns
            .iter()
            .map(|col| {
                Cell::new(col)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan)
            })
            .collect();
        table.set_header(header);

        // Add rows
        for row in &result.rows {
            let cells: Vec<Cell> = row.iter().map(|v| Cell::new(format_json_value(v))).collect();
            table.add_row(cells);
        }

        println!("{}", table);
        println!(
            "\n{} row(s) returned in {:.3}s",
            result.rows.len(),
            result.meta.duration
        );
    }
}

/// Format a JSON value for table display
fn format_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "NULL".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            // Truncate long strings for table display
            if s.len() > 50 {
                format!("{}...", &s[..47])
            } else {
                s.clone()
            }
        }
        serde_json::Value::Array(arr) => {
            let json = serde_json::to_string(arr).unwrap_or_default();
            if json.len() > 50 {
                format!("{}...", &json[..47])
            } else {
                json
            }
        }
        serde_json::Value::Object(obj) => {
            let json = serde_json::to_string(obj).unwrap_or_default();
            if json.len() > 50 {
                format!("{}...", &json[..47])
            } else {
                json
            }
        }
    }
}

pub fn print_r2_buckets(buckets: &[R2Bucket]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Location")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Created")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for bucket in buckets {
        table.add_row(vec![
            Cell::new(&bucket.name),
            Cell::new(bucket.location.as_deref().unwrap_or("-")),
            Cell::new(&bucket.creation_date),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} buckets", buckets.len());
}

pub fn print_r2_bucket(bucket: &R2Bucket) {
    println!("\nR2 Bucket Details:\n");
    println!("  Name: {}", bucket.name);
    if let Some(location) = &bucket.location {
        println!("  Location: {}", location);
    }
    if let Some(storage_class) = &bucket.storage_class {
        println!("  Storage Class: {}", storage_class);
    }
    println!("  Created: {}", bucket.creation_date);
}

pub fn print_r2_custom_domains(domains: &[R2CustomDomain]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Domain")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Enabled")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for domain in domains {
        let status_cell = Cell::new(&domain.status);
        let status_cell = if domain.status == "active" {
            status_cell.fg(Color::Green)
        } else {
            status_cell.fg(Color::Yellow)
        };

        table.add_row(vec![
            Cell::new(&domain.domain),
            status_cell,
            Cell::new(if domain.enabled { "✓" } else { "✗" }),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} domains", domains.len());
}

pub fn print_r2_metrics(metrics: &R2Metrics) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Bucket")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Objects")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Storage")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    let mut total_objects = 0u64;
    let mut total_storage = 0u64;

    for bucket in &metrics.buckets {
        table.add_row(vec![
            Cell::new(&bucket.bucket_name),
            Cell::new(bucket.object_count.to_string()),
            Cell::new(format_bytes(bucket.storage_bytes)),
        ]);
        total_objects += bucket.object_count;
        total_storage += bucket.storage_bytes;
    }

    println!("{}", table);
    println!(
        "\nTotal: {} objects, {}",
        total_objects,
        format_bytes(total_storage)
    );
}

pub fn print_r2_notifications(notifications: &[R2EventNotification]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Queue ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Events")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Prefix")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for notification in notifications {
        table.add_row(vec![
            Cell::new(&notification.queue_id),
            Cell::new(notification.events.join(", ")),
            Cell::new(notification.prefix.as_deref().unwrap_or("-")),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} rules", notifications.len());
}

pub fn print_r2_migration_jobs(jobs: &[R2MigrationJob]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Source")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Target")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for job in jobs {
        let status_cell = Cell::new(&job.status);
        let status_cell = match job.status.as_str() {
            "completed" => status_cell.fg(Color::Green),
            "running" => status_cell.fg(Color::Blue),
            "failed" => status_cell.fg(Color::Red),
            _ => status_cell.fg(Color::Yellow),
        };

        table.add_row(vec![
            Cell::new(&job.id),
            status_cell,
            Cell::new(format!("{} ({})", job.source_bucket, job.source_provider)),
            Cell::new(&job.target_bucket),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} jobs", jobs.len());
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn print_tokens(tokens: &[Token]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Last Used")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Expires")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for token in tokens {
        let status_cell = Cell::new(&token.status);
        let status_cell = match token.status.as_str() {
            "active" => status_cell.fg(Color::Green),
            "disabled" => status_cell.fg(Color::Red),
            "expired" => status_cell.fg(Color::Yellow),
            _ => status_cell,
        };

        table.add_row(vec![
            Cell::new(&token.name),
            status_cell,
            Cell::new(token.last_used_on.as_deref().unwrap_or("-")),
            Cell::new(token.expires_on.as_deref().unwrap_or("Never")),
            Cell::new(&token.id),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} tokens", tokens.len());
}

pub fn print_token(token: &Token) {
    println!("\nToken Details:\n");
    println!("  ID: {}", token.id);
    println!("  Name: {}", token.name);
    println!("  Status: {}", token.status);
    if let Some(issued) = &token.issued_on {
        println!("  Issued: {}", issued);
    }
    if let Some(modified) = &token.modified_on {
        println!("  Modified: {}", modified);
    }
    if let Some(not_before) = &token.not_before {
        println!("  Not Before: {}", not_before);
    }
    if let Some(expires) = &token.expires_on {
        println!("  Expires: {}", expires);
    }
    if let Some(last_used) = &token.last_used_on {
        println!("  Last Used: {}", last_used);
    }
    if !token.policies.is_empty() {
        println!("\n  Policies:");
        for policy in &token.policies {
            println!("    - Effect: {}", policy.effect);
            println!("      Resources: {}", policy.resources);
            for pg in &policy.permission_groups {
                println!(
                    "      Permission: {} ({})",
                    pg.name.as_deref().unwrap_or("unknown"),
                    pg.id
                );
            }
        }
    }
}

pub fn print_permission_groups(groups: &[PermissionGroup], scope_filter: Option<&str>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Scopes")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    let filtered_groups: Vec<&PermissionGroup> = groups
        .iter()
        .filter(|g| {
            if let Some(scope) = scope_filter {
                g.scopes.iter().any(|s| s.contains(scope))
            } else {
                true
            }
        })
        .collect();

    for group in &filtered_groups {
        table.add_row(vec![
            Cell::new(&group.name),
            Cell::new(group.scopes.join(", ")),
            Cell::new(&group.id),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} permission groups", filtered_groups.len());
}

// ============================================================================
// Pages Output Functions
// ============================================================================

use crate::api::pages::{Deployment, PagesDomain, PagesProject};

pub fn print_pages_projects(projects: &[PagesProject]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Name")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Subdomain")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Branch")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Framework")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Domains")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for project in projects {
        table.add_row(vec![
            Cell::new(&project.name),
            Cell::new(&project.subdomain),
            Cell::new(&project.production_branch),
            Cell::new(project.framework.as_deref().unwrap_or("-")),
            Cell::new(project.domains.len().to_string()),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} projects", projects.len());
}

pub fn print_pages_project(project: &PagesProject) {
    println!("\nPages Project Details:\n");
    println!("  Name: {}", project.name);
    println!("  ID: {}", project.id);
    println!("  Subdomain: {}", project.subdomain);
    println!("  Production Branch: {}", project.production_branch);
    if let Some(framework) = &project.framework {
        println!("  Framework: {}", framework);
    }
    if let Some(created) = &project.created_on {
        println!("  Created: {}", created);
    }
    println!("  Uses Functions: {}", if project.uses_functions { "Yes" } else { "No" });

    if !project.domains.is_empty() {
        println!("\n  Custom Domains:");
        for domain in &project.domains {
            println!("    - {}", domain);
        }
    }

    let build = &project.build_config;
    if build.build_command.is_some() || build.destination_dir.is_some() {
        println!("\n  Build Config:");
        if let Some(cmd) = &build.build_command {
            println!("    Command: {}", cmd);
        }
        if let Some(dir) = &build.destination_dir {
            println!("    Output Dir: {}", dir);
        }
        if let Some(root) = &build.root_dir {
            println!("    Root Dir: {}", root);
        }
        if let Some(caching) = build.build_caching {
            println!("    Caching: {}", if caching { "Enabled" } else { "Disabled" });
        }
    }

    if let Some(source) = &project.source {
        println!("\n  Source:");
        println!("    Type: {}", source.source_type);
        if let Some(config) = &source.config {
            if let Some(owner) = &config.owner {
                println!("    Owner: {}", owner);
            }
            if let Some(repo) = &config.repo_name {
                println!("    Repo: {}", repo);
            }
        }
    }

    if let Some(deployment) = &project.latest_deployment {
        println!("\n  Latest Deployment:");
        println!("    ID: {}", deployment.id);
        if let Some(url) = &deployment.url {
            println!("    URL: {}", url);
        }
        println!("    Environment: {}", deployment.environment);
        if let Some(stage) = &deployment.latest_stage {
            println!("    Status: {} ({})", stage.name, stage.status);
        }
    }
}

pub fn print_deployments(deployments: &[Deployment]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Environment")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("URL")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Created")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for deployment in deployments {
        let status = deployment
            .latest_stage
            .as_ref()
            .map(|s| format!("{}: {}", s.name, s.status))
            .unwrap_or_else(|| "-".to_string());

        let status_cell = Cell::new(&status);
        let status_cell = if status.contains("success") {
            status_cell.fg(Color::Green)
        } else if status.contains("failure") {
            status_cell.fg(Color::Red)
        } else if status.contains("active") {
            status_cell.fg(Color::Blue)
        } else {
            status_cell
        };

        let env_cell = Cell::new(&deployment.environment);
        let env_cell = if deployment.environment == "production" {
            env_cell.fg(Color::Green)
        } else {
            env_cell.fg(Color::Yellow)
        };

        table.add_row(vec![
            Cell::new(&deployment.id[..8.min(deployment.id.len())]),
            env_cell,
            status_cell,
            Cell::new(deployment.url.as_deref().unwrap_or("-")),
            Cell::new(deployment.created_on.as_deref().unwrap_or("-")),
        ]);
    }

    println!("{}", table);
    println!("\nTotal: {} deployments", deployments.len());
}

pub fn print_deployment(deployment: &Deployment) {
    println!("\nDeployment Details:\n");
    println!("  ID: {}", deployment.id);
    if let Some(short_id) = &deployment.short_id {
        println!("  Short ID: {}", short_id);
    }
    println!("  Environment: {}", deployment.environment);
    if let Some(url) = &deployment.url {
        println!("  URL: {}", url);
    }
    if let Some(created) = &deployment.created_on {
        println!("  Created: {}", created);
    }
    if let Some(modified) = &deployment.modified_on {
        println!("  Modified: {}", modified);
    }
    println!("  Skipped: {}", if deployment.is_skipped { "Yes" } else { "No" });
    println!("  Uses Functions: {}", if deployment.uses_functions { "Yes" } else { "No" });

    if !deployment.aliases.is_empty() {
        println!("\n  Aliases:");
        for alias in &deployment.aliases {
            println!("    - {}", alias);
        }
    }

    if let Some(trigger) = &deployment.deployment_trigger {
        println!("\n  Trigger:");
        println!("    Type: {}", trigger.trigger_type);
        if let Some(meta) = &trigger.metadata {
            if let Some(branch) = &meta.branch {
                println!("    Branch: {}", branch);
            }
            if let Some(hash) = &meta.commit_hash {
                println!("    Commit: {}", &hash[..7.min(hash.len())]);
            }
            if let Some(msg) = &meta.commit_message {
                let msg_short = if msg.len() > 60 {
                    format!("{}...", &msg[..57])
                } else {
                    msg.clone()
                };
                println!("    Message: {}", msg_short);
            }
        }
    }

    if !deployment.stages.is_empty() {
        println!("\n  Stages:");
        for stage in &deployment.stages {
            let status_icon = match stage.status.as_str() {
                "success" => "✓",
                "failure" => "✗",
                "active" => "●",
                "skipped" => "○",
                _ => "-",
            };
            println!("    {} {} ({})", status_icon, stage.name, stage.status);
        }
    }
}

pub fn print_pages_domains(domains: &[PagesDomain]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Domain")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Certificate")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for domain in domains {
        let status_cell = Cell::new(&domain.status);
        let status_cell = if domain.status == "active" {
            status_cell.fg(Color::Green)
        } else if domain.status == "pending" {
            status_cell.fg(Color::Yellow)
        } else {
            status_cell
        };

        let cert_status = domain.certificate_status.as_deref().unwrap_or("-");
        let cert_cell = Cell::new(cert_status);
        let cert_cell = if cert_status == "active" {
            cert_cell.fg(Color::Green)
        } else {
            cert_cell
        };

        table.add_row(vec![Cell::new(&domain.name), status_cell, cert_cell]);
    }

    println!("{}", table);
    println!("\nTotal: {} domains", domains.len());
}

pub fn print_pages_domain(domain: &PagesDomain) {
    println!("\nDomain Details:\n");
    println!("  Name: {}", domain.name);
    if let Some(id) = &domain.id {
        println!("  ID: {}", id);
    }
    println!("  Status: {}", domain.status);
    if let Some(verification) = &domain.verification_status {
        println!("  Verification: {}", verification);
    }
    if let Some(cert) = &domain.certificate_status {
        println!("  Certificate: {}", cert);
    }
    if let Some(created) = &domain.created_on {
        println!("  Created: {}", created);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::dns::DnsRecord;
    use crate::api::zone::{Account, Owner, Zone};

    fn create_test_record(ttl: u32, proxied: bool, priority: Option<u16>) -> DnsRecord {
        DnsRecord {
            id: "test123abc".to_string(),
            zone_id: Some("zone123".to_string()),
            zone_name: Some("example.com".to_string()),
            name: "www.example.com".to_string(),
            record_type: "A".to_string(),
            content: "203.0.113.1".to_string(),
            ttl,
            proxiable: true,
            proxied,
            priority,
            locked: false,
            created_on: Some("2026-01-01T00:00:00Z".to_string()),
            modified_on: Some("2026-01-01T00:00:00Z".to_string()),
            data: None,
            comment: None,
            tags: vec![],
        }
    }

    #[test]
    fn test_print_dns_record_basic() {
        // Should not panic with basic record
        let record = create_test_record(3600, true, None);
        print_dns_record(&record);
    }

    #[test]
    fn test_print_dns_record_with_auto_ttl() {
        // Should display "Auto" for TTL = 1
        let record = create_test_record(1, false, None);
        print_dns_record(&record);
    }

    #[test]
    fn test_print_dns_record_with_priority() {
        // Should display priority for MX records
        let mut record = create_test_record(3600, false, Some(10));
        record.record_type = "MX".to_string();
        print_dns_record(&record);
    }

    #[test]
    fn test_print_dns_records_empty() {
        // Should handle empty list
        let records: Vec<DnsRecord> = vec![];
        print_dns_records(&records);
    }

    #[test]
    fn test_print_dns_records_single() {
        // Should handle single record
        let records = vec![create_test_record(3600, true, None)];
        print_dns_records(&records);
    }

    #[test]
    fn test_print_dns_records_multiple() {
        // Should handle multiple records
        let records = vec![
            create_test_record(3600, true, None),
            create_test_record(1, false, None),
            create_test_record(7200, true, Some(10)),
        ];
        print_dns_records(&records);
    }

    #[test]
    fn test_print_zones_empty() {
        // Should handle empty list
        let zones: Vec<Zone> = vec![];
        print_zones(&zones);
    }

    #[test]
    fn test_print_zones_active() {
        // Should handle active zone
        let zones = vec![Zone {
            id: "zone123abc".to_string(),
            name: "example.com".to_string(),
            status: "active".to_string(),
            paused: false,
            development_mode: 0,
            name_servers: vec![],
            original_name_servers: vec![],
            owner: Owner {
                id: Some("owner123".to_string()),
                owner_type: "user".to_string(),
                email: Some("user@example.com".to_string()),
            },
            account: Account {
                id: "account123".to_string(),
                name: "Test Account".to_string(),
            },
            created_on: "2026-01-01T00:00:00Z".to_string(),
            modified_on: "2026-01-01T00:00:00Z".to_string(),
        }];
        print_zones(&zones);
    }

    #[test]
    fn test_print_zones_pending() {
        // Should handle non-active zone
        let zones = vec![Zone {
            id: "zone123abc".to_string(),
            name: "example.com".to_string(),
            status: "pending".to_string(),
            paused: false,
            development_mode: 0,
            name_servers: vec![],
            original_name_servers: vec![],
            owner: Owner {
                id: Some("owner123".to_string()),
                owner_type: "user".to_string(),
                email: Some("user@example.com".to_string()),
            },
            account: Account {
                id: "account123".to_string(),
                name: "Test Account".to_string(),
            },
            created_on: "2026-01-01T00:00:00Z".to_string(),
            modified_on: "2026-01-01T00:00:00Z".to_string(),
        }];
        print_zones(&zones);
    }
}
