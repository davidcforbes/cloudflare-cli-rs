use crate::api::dns::DnsRecord;
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
                id: "owner123".to_string(),
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
                id: "owner123".to_string(),
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
