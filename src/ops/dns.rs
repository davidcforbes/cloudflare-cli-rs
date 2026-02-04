use crate::api::dns::{CreateDnsRecord, DnsRecord, UpdateDnsRecord};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;
use serde::Deserialize;

pub async fn list_records(
    client: &CloudflareClient,
    zone_id: &str,
    record_type: Option<&str>,
    name: Option<&str>,
) -> Result<Vec<DnsRecord>> {
    let mut endpoint = format!("/zones/{}/dns_records", zone_id);
    let mut params = Vec::new();

    if let Some(rtype) = record_type {
        params.push(format!("type={}", rtype));
    }
    if let Some(n) = name {
        params.push(format!("name={}", n));
    }

    if !params.is_empty() {
        endpoint.push_str(&format!("?{}", params.join("&")));
    }

    let response: CfResponse<Vec<DnsRecord>> = client.get(&endpoint).await?;

    Ok(response.result.unwrap_or_default())
}

pub async fn get_record(
    client: &CloudflareClient,
    zone_id: &str,
    record_id: &str,
) -> Result<DnsRecord> {
    let endpoint = format!("/zones/{}/dns_records/{}", zone_id, record_id);
    let response: CfResponse<DnsRecord> = client.get(&endpoint).await?;

    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("DNS record", record_id))
}

pub async fn create_record(
    client: &CloudflareClient,
    zone_id: &str,
    record: CreateDnsRecord,
) -> Result<DnsRecord> {
    let endpoint = format!("/zones/{}/dns_records", zone_id);
    let response: CfResponse<DnsRecord> = client.post(&endpoint, record).await?;

    let record = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create record"))?;

    println!("✓ Created DNS record: {}", record.name);
    Ok(record)
}

pub async fn update_record(
    client: &CloudflareClient,
    zone_id: &str,
    record_id: &str,
    update: UpdateDnsRecord,
) -> Result<DnsRecord> {
    let endpoint = format!("/zones/{}/dns_records/{}", zone_id, record_id);
    let response: CfResponse<DnsRecord> = client.put(&endpoint, update).await?;

    let record = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from update record"))?;

    println!("✓ Updated DNS record: {}", record.name);
    Ok(record)
}

pub async fn delete_record(
    client: &CloudflareClient,
    zone_id: &str,
    record_id: &str,
) -> Result<()> {
    let endpoint = format!("/zones/{}/dns_records/{}", zone_id, record_id);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;

    println!("✓ Deleted DNS record");
    Ok(())
}

#[derive(Debug, Deserialize)]
struct CsvRecord {
    r#type: String,
    name: String,
    content: String,
    #[serde(default = "default_ttl")]
    ttl: u32,
    #[serde(default)]
    proxied: bool,
    #[serde(default)]
    priority: Option<u16>,
}

fn default_ttl() -> u32 {
    1
}

#[derive(Debug, Default)]
pub struct ImportStats {
    pub success: usize,
    pub failed: usize,
    pub total: usize,
}

pub async fn import_records(
    client: &CloudflareClient,
    zone_id: &str,
    file_path: &str,
) -> Result<ImportStats> {
    let contents = std::fs::read_to_string(file_path)?;
    let records = detect_and_parse_format(&contents)?;
    let stats = import_records_batch(client, zone_id, records).await?;
    print_import_summary(&stats);
    Ok(stats)
}

fn detect_and_parse_format(contents: &str) -> Result<Vec<CreateDnsRecord>> {
    if is_bind_format(contents) {
        println!("Detected BIND zone file format");
        parse_bind_format(contents)
    } else {
        println!("Detected CSV format");
        parse_csv_format(contents)
    }
}

fn is_bind_format(contents: &str) -> bool {
    contents.contains("$ORIGIN") || contents.contains("$TTL") || contents.contains(" IN ")
}

async fn import_records_batch(
    client: &CloudflareClient,
    zone_id: &str,
    records: Vec<CreateDnsRecord>,
) -> Result<ImportStats> {
    let mut stats = ImportStats {
        total: records.len(),
        ..Default::default()
    };

    println!("\nImporting {} DNS records...\n", stats.total);

    for (i, record) in records.into_iter().enumerate() {
        import_single_record(client, zone_id, record, i + 1, stats.total, &mut stats).await;
    }

    Ok(stats)
}

async fn import_single_record(
    client: &CloudflareClient,
    zone_id: &str,
    record: CreateDnsRecord,
    current: usize,
    total: usize,
    stats: &mut ImportStats,
) {
    print!(
        "[{}/{}] Importing {} record for {}... ",
        current, total, record.record_type, record.name
    );

    match create_record(client, zone_id, record).await {
        Ok(_) => {
            stats.success += 1;
            println!("✓");
        }
        Err(e) => {
            stats.failed += 1;
            println!("✗");
            eprintln!("  Error: {}", e);
        }
    }
}

fn print_import_summary(stats: &ImportStats) {
    println!("\nImport complete!");
    println!("  Success: {}", stats.success);
    println!("  Failed: {}", stats.failed);
    println!("  Total: {}", stats.total);
}

fn parse_csv_format(contents: &str) -> Result<Vec<CreateDnsRecord>> {
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let csv_record: CsvRecord = result.map_err(|e| {
            crate::error::CfadError::validation(format!("Invalid CSV format: {}", e))
        })?;

        records.push(CreateDnsRecord {
            record_type: csv_record.r#type,
            name: csv_record.name,
            content: csv_record.content,
            ttl: Some(csv_record.ttl),
            proxied: Some(csv_record.proxied),
            priority: csv_record.priority,
            data: None,
        });
    }

    Ok(records)
}

fn parse_bind_format(contents: &str) -> Result<Vec<CreateDnsRecord>> {
    let mut records = Vec::new();
    let mut default_origin = String::new();
    let mut default_ttl = 1u32;

    for line in contents.lines() {
        process_bind_line(
            line.trim(),
            &mut records,
            &mut default_origin,
            &mut default_ttl,
        );
    }

    Ok(records)
}

fn process_bind_line(
    line: &str,
    records: &mut Vec<CreateDnsRecord>,
    default_origin: &mut String,
    default_ttl: &mut u32,
) {
    if should_skip_line(line) {
        return;
    }

    if process_bind_directive(line, default_origin, default_ttl) {
        return;
    }

    if let Some(record) = parse_bind_record_line(line, default_origin, *default_ttl) {
        records.push(record);
    }
}

fn should_skip_line(line: &str) -> bool {
    line.is_empty() || line.starts_with(';')
}

fn process_bind_directive(line: &str, default_origin: &mut String, default_ttl: &mut u32) -> bool {
    // Parse $ORIGIN directive
    if line.starts_with("$ORIGIN") {
        *default_origin = line
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .trim_end_matches('.')
            .to_string();
        return true;
    }

    // Parse $TTL directive
    if line.starts_with("$TTL") {
        if let Some(ttl_str) = line.split_whitespace().nth(1) {
            *default_ttl = ttl_str.parse().unwrap_or(1);
        }
        return true;
    }

    false
}

fn parse_bind_record_line(
    line: &str,
    default_origin: &str,
    default_ttl: u32,
) -> Option<CreateDnsRecord> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }

    let mut idx = 0;
    let name = parts[idx];
    idx += 1;

    let (ttl, idx) = extract_ttl_and_advance(&parts, idx, default_ttl);
    let (record_type, idx) = extract_record_type(&parts, idx)?;
    let full_name = build_full_domain_name(name, default_origin);
    let (content, priority) = parse_record_content(&record_type, &parts, idx)?;

    Some(CreateDnsRecord {
        record_type,
        name: full_name,
        content,
        ttl: Some(ttl),
        proxied: Some(false),
        priority,
        data: None,
    })
}

fn extract_ttl_and_advance(parts: &[&str], idx: usize, default_ttl: u32) -> (u32, usize) {
    if parts[idx].parse::<u32>().is_ok() {
        let ttl = parts[idx].parse().unwrap_or(default_ttl);
        (ttl, idx + 1)
    } else {
        (default_ttl, idx)
    }
}

fn extract_record_type(parts: &[&str], mut idx: usize) -> Option<(String, usize)> {
    // Skip IN class if present
    if parts.get(idx)? == &"IN" {
        idx += 1;
    }

    let record_type = parts.get(idx)?.to_uppercase();
    Some((record_type, idx + 1))
}

fn build_full_domain_name(name: &str, default_origin: &str) -> String {
    if name == "@" {
        default_origin.to_string()
    } else if name.ends_with('.') {
        name.trim_end_matches('.').to_string()
    } else if !default_origin.is_empty() {
        format!("{}.{}", name, default_origin)
    } else {
        name.to_string()
    }
}

fn parse_record_content(
    record_type: &str,
    parts: &[&str],
    idx: usize,
) -> Option<(String, Option<u16>)> {
    match record_type {
        "A" | "AAAA" | "CNAME" | "NS" => {
            let content = parts.get(idx)?.trim_end_matches('.').to_string();
            Some((content, None))
        }
        "MX" => {
            let priority = parts.get(idx)?.parse().ok();
            let content = parts.get(idx + 1)?.trim_end_matches('.').to_string();
            Some((content, priority))
        }
        "TXT" => {
            let content = parts[idx..].join(" ").trim_matches('"').to_string();
            Some((content, None))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // CSV Parser Tests
    // ========================================

    #[test]
    fn test_parse_csv_valid_records() {
        let csv = "type,name,content,ttl,proxied,priority
A,www,203.0.113.1,3600,true,
AAAA,www,2001:db8::1,3600,false,
MX,@,mail.example.com,3600,false,10
TXT,@,v=spf1 mx ~all,3600,false,";

        let records = parse_csv_format(csv).unwrap();
        assert_eq!(records.len(), 4);

        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].name, "www");
        assert_eq!(records[0].content, "203.0.113.1");
        assert_eq!(records[0].ttl, Some(3600));
        assert_eq!(records[0].proxied, Some(true));
        assert_eq!(records[0].priority, None);

        assert_eq!(records[2].record_type, "MX");
        assert_eq!(records[2].priority, Some(10));
    }

    #[test]
    fn test_parse_csv_minimal_fields() {
        let csv = "type,name,content
A,www,203.0.113.1
CNAME,blog,www.example.com";

        let records = parse_csv_format(csv).unwrap();
        assert_eq!(records.len(), 2);

        // Default values should be applied
        assert_eq!(records[0].ttl, Some(1)); // default_ttl
        assert_eq!(records[0].proxied, Some(false)); // default
        assert_eq!(records[0].priority, None);
    }

    #[test]
    fn test_parse_csv_with_quotes() {
        let csv = r#"type,name,content,ttl,proxied,priority
TXT,@,"v=spf1 mx ~all",3600,false,
TXT,_dmarc,"v=DMARC1; p=quarantine",3600,false,"#;

        let records = parse_csv_format(csv).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].content, "v=spf1 mx ~all");
        assert_eq!(records[1].content, "v=DMARC1; p=quarantine");
    }

    #[test]
    fn test_parse_csv_empty_file() {
        let csv = "type,name,content";
        let records = parse_csv_format(csv).unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_parse_csv_invalid_format() {
        let csv = "invalid,csv,without,proper,headers
some,data,here";

        // Should fail due to missing required fields
        let result = parse_csv_format(csv);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_csv_missing_headers() {
        let csv = "A,www,203.0.113.1";

        // CSV crate treats first row as headers, so this returns empty vec
        let result = parse_csv_format(csv);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    // ========================================
    // BIND Parser Tests
    // ========================================

    #[test]
    fn test_parse_bind_with_origin() {
        let bind = "$ORIGIN example.com.
$TTL 3600
@       IN  A       203.0.113.1
www     IN  A       203.0.113.2";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "example.com");
        assert_eq!(records[1].name, "www.example.com");
    }

    #[test]
    fn test_parse_bind_with_ttl() {
        let bind = "$ORIGIN example.com.
$TTL 7200
www     IN  A       203.0.113.1
mail    300 IN  A   203.0.113.2";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].ttl, Some(7200)); // Default TTL
        assert_eq!(records[1].ttl, Some(300)); // Explicit TTL
    }

    #[test]
    fn test_parse_bind_a_records() {
        let bind = "$ORIGIN example.com.
www     IN  A       203.0.113.1
ftp     IN  A       203.0.113.2";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].content, "203.0.113.1");
    }

    #[test]
    fn test_parse_bind_mx_records() {
        let bind = "$ORIGIN example.com.
@       IN  MX  10  mail.example.com.
@       IN  MX  20  mail2.example.com.";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_type, "MX");
        assert_eq!(records[0].priority, Some(10));
        assert_eq!(records[0].content, "mail.example.com");
        assert_eq!(records[1].priority, Some(20));
    }

    #[test]
    fn test_parse_bind_txt_records() {
        let bind = r#"$ORIGIN example.com.
@       IN  TXT     "v=spf1 mx ~all"
_dmarc  IN  TXT     "v=DMARC1; p=quarantine""#;

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_type, "TXT");
        assert_eq!(records[0].content, "v=spf1 mx ~all");
        assert_eq!(records[1].content, "v=DMARC1; p=quarantine");
    }

    #[test]
    fn test_parse_bind_comments() {
        let bind = "; This is a comment
$ORIGIN example.com.
; Another comment
www     IN  A       203.0.113.1
; Yet another comment";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "www.example.com");
    }

    #[test]
    fn test_parse_bind_at_symbol() {
        let bind = "$ORIGIN example.com.
@       IN  A       203.0.113.1";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "example.com");
    }

    #[test]
    fn test_parse_bind_trailing_dots() {
        let bind = "$ORIGIN example.com.
www.example.com.    IN  A       203.0.113.1
mail.example.com.   IN  A       203.0.113.2";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "www.example.com");
        assert_eq!(records[1].name, "mail.example.com");
    }

    #[test]
    fn test_parse_bind_empty_lines() {
        let bind = "$ORIGIN example.com.

www     IN  A       203.0.113.1

mail    IN  A       203.0.113.2

";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_parse_bind_multiple_record_types() {
        let bind = "$ORIGIN example.com.
$TTL 3600
@       IN  A       203.0.113.1
@       IN  AAAA    2001:db8::1
www     IN  CNAME   example.com.
@       IN  MX  10  mail.example.com.
@       IN  TXT     \"v=spf1 mx ~all\"
@       IN  NS      ns1.cloudflare.com.";

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 6);

        let types: Vec<String> = records.iter().map(|r| r.record_type.clone()).collect();
        assert!(types.contains(&"A".to_string()));
        assert!(types.contains(&"AAAA".to_string()));
        assert!(types.contains(&"CNAME".to_string()));
        assert!(types.contains(&"MX".to_string()));
        assert!(types.contains(&"TXT".to_string()));
        assert!(types.contains(&"NS".to_string()));
    }

    #[test]
    fn test_parse_bind_invalid_line_too_short() {
        let bind = "$ORIGIN example.com.
www IN";  // Only 2 parts, need at least 4

        let records = parse_bind_format(bind).unwrap();
        // Should skip invalid line
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_parse_bind_unsupported_record_type() {
        let bind = "$ORIGIN example.com.
www IN SRV 10 5 5060 sipserver.example.com.";  // SRV not supported in parse_record_content

        let records = parse_bind_format(bind).unwrap();
        // Should skip unsupported record type
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_parse_bind_with_short_line() {
        let bind = "short";  // Too short to parse

        let records = parse_bind_format(bind).unwrap();
        assert_eq!(records.len(), 0);
    }
}
