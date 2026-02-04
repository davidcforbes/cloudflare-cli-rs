use crate::api::d1::D1Database;
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
            Cell::new(&record.id[..8]),
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
    println!("  Created: {}", record.created_on);
    println!("  Modified: {}", record.modified_on);
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
            Cell::new(&zone.id[..8]),
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
            Cell::new(&db.uuid[..8.min(db.uuid.len())]),
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
            Cell::new(&job.id[..8.min(job.id.len())]),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::dns::DnsRecord;
    use crate::api::zone::{Account, Owner, Zone};

    fn create_test_record(ttl: u32, proxied: bool, priority: Option<u16>) -> DnsRecord {
        DnsRecord {
            id: "test123abc".to_string(),
            zone_id: "zone123".to_string(),
            zone_name: "example.com".to_string(),
            name: "www.example.com".to_string(),
            record_type: "A".to_string(),
            content: "203.0.113.1".to_string(),
            ttl,
            proxied,
            priority,
            locked: false,
            created_on: "2026-01-01T00:00:00Z".to_string(),
            modified_on: "2026-01-01T00:00:00Z".to_string(),
            data: None,
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
